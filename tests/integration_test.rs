use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{routing::{get, post}, Json, Router};
use clap::Parser;
use hawkeye::{
    features::browse::handler::handle_browse,
    features::graph::handler::{handle_entity_graph, handle_document_graph},
    features::ingest::scanner::scan_directory,
    features::queue::{
        consumer::spawn_consumers,
        RedisQueue,
    },
    features::search::facets::handle_facets,
    features::search::handler::handle_search,
    features::search::SearchIndexer,
    features::semantic::EmbedClient,
    features::semantic::MilvusClient,
    features::summary::{handler::handle_summary, Relationship, Summary},
    shared::config::{AppConfig, DEFAULT_WORKSPACE_ID},
    shared::db::documents,
    shared::health::handle_health,
    shared::inference::client::InferenceClient,
    shared::state::AppState,
    shared::status::handle_status,
};
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{watch, Mutex};
use chrono::Utc;
use tower::ServiceExt;
use uuid::Uuid;

const TEST_POSTGRES_URL: &str = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye";
const TEST_REDIS_URL: &str = "redis://localhost:6379";

async fn mock_llm_handler() -> Json<serde_json::Value> {
    Json(json!({
        "choices": [{
            "message": {
                "content": "{\"tldr\": \"A test document about testing.\", \"title\": \"Test Doc\", \"tags\": [\"test\", \"docs\"], \"entities\": [\"TestEntity\"], \"topics\": [\"testing\"], \"relationships\": [{\"from\": \"TestEntity\", \"rel\": \"related_to\", \"to\": \"testing\", \"context\": \"test relationship\"}]}"
            }
        }]
    }))
}

async fn wait_for_completion(queue: &RedisQueue, expected: usize, timeout: Duration) {
    let start = Instant::now();
    loop {
        if let Ok(status) = queue.read_status().await {
            if status.completed + status.failed >= expected {
                return;
            }
        }
        if start.elapsed() > timeout {
            let status = queue.read_status().await.unwrap();
            panic!(
                "Timed out waiting for {} completions (completed={}, failed={})",
                expected, status.completed, status.failed
            );
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn create_redis_pool() -> deadpool_redis::Pool {
    let cfg = deadpool_redis::Config::from_url(TEST_REDIS_URL);
    cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool")
}

#[tokio::test]
async fn test_full_pipeline() {
    // 1. Temp directory with 5 md files
    let dir = tempfile::tempdir().unwrap();
    for i in 0..5 {
        std::fs::write(
            dir.path().join(format!("doc{}.md", i)),
            format!("Document {} contains information about testing and validation.", i),
        )
        .unwrap();
    }

    // 2. Mock LLM server
    let mock_app = Router::new().route("/v1/chat/completions", post(mock_llm_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock_addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, mock_app).await.unwrap();
    });

    // 3. Setup infrastructure
    let index_dir = tempfile::tempdir().unwrap();
    let mlx_url = format!("http://{}", mock_addr);
    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();

    let redis_pool = create_redis_pool();
    let ws_id = Uuid::new_v4();
    let queue = RedisQueue::new(redis_pool.clone(), ws_id);
    queue.ensure_group().await.unwrap();

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    // Embed/Milvus pointed at non-existent URLs — worker logs warnings but doesn't fail
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    // 4. Scan and publish
    let scan = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan.to_process.len(), 5, "Expected 5 files to process");
    assert_eq!(scan.skipped, 0);

    queue.publish_files(&scan.to_process).await.unwrap();

    // 5. Spawn consumers and wait for completion
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    let handles = spawn_consumers(
        queue.clone(),
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        embed.clone(),
        milvus.clone(),
        None,
        2,
        shutdown_rx,
    );

    wait_for_completion(&queue, 5, Duration::from_secs(30)).await;

    // 6. Verify queue status
    let status = queue.read_status().await.unwrap();
    assert_eq!(status.completed, 5, "All 5 files should be completed");
    assert_eq!(status.failed, 0, "No failures expected");
    assert_eq!(status.in_progress, 0);

    // 7. Verify summaries in Postgres
    for i in 0..5 {
        let md_path = dir.path().join(format!("doc{}.md", i));
        let source_path = md_path.display().to_string();

        let doc = documents::get_document_by_path(&pg_pool, DEFAULT_WORKSPACE_ID, &source_path)
            .await
            .unwrap();
        assert!(doc.is_some(), "Missing document for doc{}.md", i);
        let doc = doc.unwrap();
        assert!(!doc.source_hash.is_empty());

        let summary = documents::get_summary_by_document(&pg_pool, doc.id)
            .await
            .unwrap();
        assert!(summary.is_some(), "Missing summary for doc{}.md", i);
        let summary = summary.unwrap();
        assert_eq!(summary.title, "Test Doc");
        assert!(summary.word_count > 0);

        let rels: Vec<serde_json::Value> =
            serde_json::from_value(summary.relationships.clone()).unwrap_or_default();
        assert_eq!(rels.len(), 1, "Expected 1 relationship for doc{}.md", i);
    }

    // 8. Verify search works
    let indexer = indexer.lock().await;
    let results = indexer.search("testing", 20).unwrap();
    assert_eq!(results.len(), 5, "All 5 docs should be findable");

    let results = indexer.search("tags:test", 20).unwrap();
    assert_eq!(results.len(), 5, "All 5 docs should have tag 'test'");

    // Cleanup
    for h in handles {
        h.abort();
    }
    queue.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_skip_logic_on_rerun() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..3 {
        std::fs::write(dir.path().join(format!("note{}.md", i)), "Some content").unwrap();
    }

    let mock_app = Router::new().route("/v1/chat/completions", post(mock_llm_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock_addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, mock_app).await.unwrap();
    });

    let index_dir = tempfile::tempdir().unwrap();
    let mlx_url = format!("http://{}", mock_addr);
    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();

    let redis_pool = create_redis_pool();
    let ws_id = Uuid::new_v4();
    let queue = RedisQueue::new(redis_pool.clone(), ws_id);
    queue.ensure_group().await.unwrap();

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    // First run — process all 3
    let scan1 = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan1.to_process.len(), 3);
    queue.publish_files(&scan1.to_process).await.unwrap();

    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    let handles = spawn_consumers(
        queue.clone(),
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        embed.clone(),
        milvus.clone(),
        None,
        2,
        shutdown_rx,
    );

    wait_for_completion(&queue, 3, Duration::from_secs(30)).await;

    let status = queue.read_status().await.unwrap();
    assert_eq!(status.completed, 3);

    for h in handles {
        h.abort();
    }

    // Second run — fetch known hashes from Postgres, all 3 should be skipped
    let hashes = documents::get_source_hashes(&pg_pool, DEFAULT_WORKSPACE_ID)
        .await
        .unwrap();
    let known: HashMap<String, String> = hashes
        .into_iter()
        .map(|h| (h.source_path, h.source_hash))
        .collect();

    let scan2 = scan_directory(dir.path(), &known).unwrap();
    assert_eq!(
        scan2.to_process.len(),
        0,
        "All files should be skipped on rerun"
    );
    assert_eq!(scan2.skipped, 3);

    // Cleanup
    queue.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_cancel_ingestion() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..10 {
        std::fs::write(
            dir.path().join(format!("cancel{}.md", i)),
            format!("Cancel test document {}.", i),
        )
        .unwrap();
    }

    // Use a mock LLM that's slow so we can cancel mid-flight
    async fn slow_llm_handler() -> Json<serde_json::Value> {
        tokio::time::sleep(Duration::from_secs(2)).await;
        Json(json!({
            "choices": [{
                "message": {
                    "content": "{\"tldr\": \"Cancelled test.\", \"title\": \"Cancel Doc\", \"tags\": [], \"entities\": [], \"topics\": [], \"relationships\": []}"
                }
            }]
        }))
    }

    let mock_app = Router::new().route("/v1/chat/completions", post(slow_llm_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock_addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, mock_app).await.unwrap();
    });

    let index_dir = tempfile::tempdir().unwrap();
    let mlx_url = format!("http://{}", mock_addr);
    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();

    let redis_pool = create_redis_pool();
    let ws_id = Uuid::new_v4();
    let queue = RedisQueue::new(redis_pool.clone(), ws_id);
    queue.ensure_group().await.unwrap();

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    // Publish all 10 files
    let scan = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan.to_process.len(), 10);
    queue.publish_files(&scan.to_process).await.unwrap();

    // Spawn 1 slow consumer (each file takes 2s)
    let (_shutdown_tx, shutdown_rx) = watch::channel(false);
    let handles = spawn_consumers(
        queue.clone(),
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        embed.clone(),
        milvus.clone(),
        None,
        1,
        shutdown_rx,
    );

    // Wait briefly for at least 1 to start processing
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Cancel the remaining jobs
    let result = queue.cancel().await.unwrap();
    assert!(
        result.cancelled > 0,
        "Expected some jobs to be cancelled, got {}",
        result.cancelled
    );

    // After cancel, in_progress should be 0
    let status = queue.read_status().await.unwrap();
    assert_eq!(status.in_progress, 0, "in_progress should be 0 after cancel");

    // A new ingest should work after cancel (consumer group was recreated)
    let dir2 = tempfile::tempdir().unwrap();
    std::fs::write(dir2.path().join("post_cancel.md"), "After cancel.").unwrap();
    let scan2 = scan_directory(dir2.path(), &HashMap::new()).unwrap();
    assert_eq!(scan2.to_process.len(), 1);
    queue.publish_files(&scan2.to_process).await.unwrap();

    // Cleanup
    for h in handles {
        h.abort();
    }
    queue.cleanup().await.unwrap();
}

/// Build a test AppState with real Postgres/Redis and the given Tantivy indexer.
async fn create_test_app_state(
    indexer: Arc<Mutex<SearchIndexer>>,
) -> (Arc<AppState>, PgPool) {
    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();
    let redis_pool = create_redis_pool();
    let config = AppConfig::parse_from(["hawkeye"]);
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());
    let (shutdown_tx, _) = watch::channel(false);

    let pg_clone = pg_pool.clone();
    let state = Arc::new(AppState {
        config,
        redis_pool,
        indexer,
        pg_pool,
        embed,
        milvus,
        neo4j: None,
        shutdown: shutdown_tx,
        shutdown_docker: AtomicBool::new(false),
    });
    (state, pg_clone)
}

/// Helper: send a GET request to the app and return (status, body as Vec<u8>).
async fn get_request(app: Router, uri: &str) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes().to_vec();
    (status, body)
}

#[tokio::test]
async fn test_search_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));

    // Index test documents
    {
        let mut idx = indexer.lock().await;
        idx.index_summary(&Summary {
            source: "auth.md".to_string(),
            source_hash: "sha256:aaa".to_string(),
            created_at: Utc::now(),
            tldr: "Guide to setting up OAuth2 authentication".to_string(),
            title: "OAuth2 Setup Guide".to_string(),
            tags: vec!["auth".to_string(), "oauth2".to_string()],
            entities: vec!["OAuth2".to_string()],
            topics: vec!["authentication".to_string()],
            relationships: vec![],
            word_count: 100,
        }).unwrap();
        idx.index_summary(&Summary {
            source: "deploy.md".to_string(),
            source_hash: "sha256:bbb".to_string(),
            created_at: Utc::now(),
            tldr: "Steps for deploying to Kubernetes cluster".to_string(),
            title: "K8s Deployment".to_string(),
            tags: vec!["kubernetes".to_string(), "devops".to_string()],
            entities: vec!["Kubernetes".to_string()],
            topics: vec!["deployment".to_string()],
            relationships: vec![],
            word_count: 200,
        }).unwrap();
    }

    let (state, _pg) = create_test_app_state(indexer).await;
    let app = Router::new()
        .route("/search", get(handle_search))
        .with_state(state);

    // Search for "OAuth2" — should find auth.md
    let (status, body) = get_request(app.clone(), "/search?q=OAuth2").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(!results.is_empty(), "Expected search results for 'OAuth2'");
    assert_eq!(results[0]["file"], "auth.md");

    // Search for "kubernetes" — should find deploy.md
    let (status, body) = get_request(app.clone(), "/search?q=kubernetes").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(!results.is_empty(), "Expected search results for 'kubernetes'");
    assert_eq!(results[0]["file"], "deploy.md");

    // Search with limit
    let (status, body) = get_request(app.clone(), "/search?q=OAuth2&limit=1").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(results.len() <= 1, "Expected at most 1 result with limit=1");

    // Search with no results
    let (status, body) = get_request(app.clone(), "/search?q=nonexistent_xyzzy").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(results.is_empty(), "Expected no results for nonsense query");
}

#[tokio::test]
async fn test_browse_handler() {
    // Create a temp directory with subdirs and .md files
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("readme.md"), "# Hello").unwrap();
    std::fs::write(dir.path().join("notes.md"), "# Notes").unwrap();
    std::fs::write(dir.path().join("data.txt"), "not markdown").unwrap();
    std::fs::create_dir(dir.path().join("subdir_a")).unwrap();
    std::fs::create_dir(dir.path().join("subdir_b")).unwrap();
    std::fs::create_dir(dir.path().join(".hidden")).unwrap();

    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, _pg) = create_test_app_state(indexer).await;
    let app = Router::new()
        .route("/browse", get(handle_browse))
        .with_state(state);

    // Browse the temp directory
    let path_str = dir.path().to_string_lossy();
    let (status, body) = get_request(app.clone(), &format!("/browse?path={}", path_str)).await;
    assert_eq!(status, StatusCode::OK);
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Should list only non-hidden subdirectories
    let entries = result["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2, "Expected 2 visible subdirectories");
    assert!(entries.contains(&json!("subdir_a")));
    assert!(entries.contains(&json!("subdir_b")));

    // Should count only .md files
    assert_eq!(result["md_file_count"], 2);

    // Should have a parent
    assert!(result["parent"].is_string());

    // Browse non-existent path should return 400
    let (status, _) = get_request(app.clone(), "/browse?path=/nonexistent_xyzzy_path").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // Browse a file (not a directory) should return 400
    let file_path = dir.path().join("readme.md");
    let (status, _) = get_request(
        app.clone(),
        &format!("/browse?path={}", file_path.to_string_lossy()),
    ).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_summary_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, pg_pool) = create_test_app_state(indexer).await;

    // Use a unique filename (no slashes) so it works with the {file} path segment
    let source_path = format!("test-summary-{}.md", Uuid::new_v4());
    let doc = documents::upsert_document(
        &pg_pool,
        DEFAULT_WORKSPACE_ID,
        &source_path,
        "sha256:summary_test_hash",
        Some(512),
    )
    .await
    .unwrap();

    documents::insert_summary(
        &pg_pool,
        &documents::InsertSummary {
            document_id: doc.id,
            tldr: "A comprehensive guide to Rust ownership.",
            title: "Rust Ownership Guide",
            tags: &["rust".to_string(), "ownership".to_string()],
            entities: &["Rust".to_string()],
            topics: &["programming".to_string()],
            relationships: &[Relationship {
                from: "Rust".to_string(),
                rel: hawkeye::features::summary::types::RelationType::Uses,
                to: "Ownership".to_string(),
                context: "core concept".to_string(),
            }],
            word_count: 350,
        },
    )
    .await
    .unwrap();

    let app = Router::new()
        .route("/summary/{file}", get(handle_summary))
        .with_state(state);

    // Fetch the summary via the handler
    let (status, body) = get_request(app.clone(), &format!("/summary/{}", source_path)).await;
    assert_eq!(status, StatusCode::OK);
    let summary: Summary = serde_json::from_slice(&body).unwrap();
    assert_eq!(summary.title, "Rust Ownership Guide");
    assert_eq!(summary.tldr, "A comprehensive guide to Rust ownership.");
    assert_eq!(summary.tags, vec!["rust", "ownership"]);
    assert_eq!(summary.entities, vec!["Rust"]);
    assert_eq!(summary.topics, vec!["programming"]);
    assert_eq!(summary.word_count, 350);
    assert_eq!(summary.relationships.len(), 1);
    assert_eq!(summary.relationships[0].from, "Rust");
    assert_eq!(summary.relationships[0].to, "Ownership");

    // Request a non-existent summary should return 404
    let (status, _) = get_request(app.clone(), "/summary/nonexistent-file.md").await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Clean up test data
    sqlx::query("DELETE FROM summaries WHERE document_id = $1")
        .bind(doc.id)
        .execute(&pg_pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM documents WHERE id = $1")
        .bind(doc.id)
        .execute(&pg_pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_health_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, _pg) = create_test_app_state(indexer).await;
    let app = Router::new()
        .route("/health", get(handle_health))
        .with_state(state);

    let (status, body) = get_request(app, "/health").await;
    assert_eq!(status, StatusCode::OK);
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Should have services array with 7 entries
    let services = result["services"].as_array().unwrap();
    assert_eq!(services.len(), 7, "Expected 7 service health checks");

    // Should have checked_at timestamp
    assert!(result["checked_at"].is_string());

    // Each service should have name, status, and latency_ms fields
    for svc in services {
        assert!(svc["name"].is_string(), "Service missing 'name' field");
        assert!(svc["status"].is_string(), "Service missing 'status' field");
    }

    // Redis and Postgres should be up (required for integration tests)
    let redis = services.iter().find(|s| s["name"] == "redis").unwrap();
    assert_eq!(redis["status"], "up", "Redis should be up for integration tests");
    assert!(redis["latency_ms"].is_number());

    let pg = services.iter().find(|s| s["name"] == "postgres").unwrap();
    assert_eq!(pg["status"], "up", "Postgres should be up for integration tests");
    assert!(pg["latency_ms"].is_number());
}

#[tokio::test]
async fn test_facets_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));

    // Index documents with known tags, topics, entities
    {
        let mut idx = indexer.lock().await;
        idx.index_summary(&Summary {
            source: "auth.md".to_string(),
            source_hash: "sha256:f1".to_string(),
            created_at: Utc::now(),
            tldr: "Auth guide".to_string(),
            title: "Auth".to_string(),
            tags: vec!["security".to_string(), "auth".to_string()],
            entities: vec!["OAuth2".to_string()],
            topics: vec!["authentication".to_string()],
            relationships: vec![],
            word_count: 50,
        }).unwrap();
        idx.index_summary(&Summary {
            source: "deploy.md".to_string(),
            source_hash: "sha256:f2".to_string(),
            created_at: Utc::now(),
            tldr: "Deploy guide".to_string(),
            title: "Deploy".to_string(),
            tags: vec!["devops".to_string(), "auth".to_string()],
            entities: vec!["Kubernetes".to_string()],
            topics: vec!["deployment".to_string()],
            relationships: vec![],
            word_count: 80,
        }).unwrap();
    }

    let (state, _pg) = create_test_app_state(indexer).await;
    let app = Router::new()
        .route("/facets", get(handle_facets))
        .with_state(state);

    let (status, body) = get_request(app, "/facets").await;
    assert_eq!(status, StatusCode::OK);
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Should have tags, topics, entities arrays
    let tags = result["tags"].as_array().unwrap();
    let topics = result["topics"].as_array().unwrap();
    let entities = result["entities"].as_array().unwrap();

    assert!(!tags.is_empty(), "Expected at least one tag facet");
    assert!(!topics.is_empty(), "Expected at least one topic facet");
    assert!(!entities.is_empty(), "Expected at least one entity facet");

    // "auth" appears in both docs, so it should be the top tag with count 2
    let auth_tag = tags.iter().find(|t| t["name"] == "auth").unwrap();
    assert_eq!(auth_tag["count"], 2);

    // Each entry should have name and count fields
    for tag in tags {
        assert!(tag["name"].is_string(), "Tag missing 'name' field");
        assert!(tag["count"].is_number(), "Tag missing 'count' field");
    }
}

#[tokio::test]
async fn test_status_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, _pg) = create_test_app_state(indexer).await;

    // Clean up any leftover state from previous test runs, then re-create the group
    let queue = RedisQueue::new(state.redis_pool.clone(), DEFAULT_WORKSPACE_ID);
    queue.cleanup().await.unwrap();
    queue.ensure_group().await.unwrap();

    let app = Router::new()
        .route("/status", get(handle_status))
        .with_state(state);

    let (status, body) = get_request(app, "/status").await;
    assert_eq!(status, StatusCode::OK);
    let result: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Should have standard queue status fields
    assert!(result["total"].is_number(), "Missing 'total' field");
    assert!(result["in_progress"].is_number(), "Missing 'in_progress' field");
    assert!(result["completed"].is_number(), "Missing 'completed' field");
    assert!(result["failed"].is_number(), "Missing 'failed' field");

    // Fresh queue should have all zeros
    assert_eq!(result["total"], 0);
    assert_eq!(result["in_progress"], 0);

    // Cleanup
    queue.cleanup().await.unwrap();
}

#[tokio::test]
async fn test_graph_handler_without_neo4j() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, _pg) = create_test_app_state(indexer).await;

    // state.neo4j is None — handlers should return 500 "Knowledge graph not available"
    let app = Router::new()
        .route("/graph/entity/{name}", get(handle_entity_graph))
        .route("/graph/document/{id}", get(handle_document_graph))
        .with_state(state);

    let (status, body) = get_request(app.clone(), "/graph/entity/TestEntity").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let text = String::from_utf8(body).unwrap();
    assert!(
        text.contains("Knowledge graph not available"),
        "Expected 'Knowledge graph not available' error, got: {}",
        text
    );

    let (status, body) = get_request(app.clone(), "/graph/document/some-doc-id").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let text = String::from_utf8(body).unwrap();
    assert!(
        text.contains("Knowledge graph not available"),
        "Expected 'Knowledge graph not available' error, got: {}",
        text
    );
}

#[tokio::test]
async fn test_db_migrations_and_document_crud() {
    let pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let workspace_id = DEFAULT_WORKSPACE_ID;

    // Insert a document
    let doc = documents::upsert_document(
        &pool,
        workspace_id,
        "/tmp/test/notes.md",
        "sha256:abc123",
        Some(1024),
    )
    .await
    .unwrap();

    assert_eq!(doc.source_path, "/tmp/test/notes.md");
    assert_eq!(doc.source_hash, "sha256:abc123");
    assert_eq!(doc.file_size_bytes, Some(1024));
    assert_eq!(doc.workspace_id, workspace_id);

    // Query it back by path
    let found = documents::get_document_by_path(&pool, workspace_id, "/tmp/test/notes.md")
        .await
        .unwrap();
    assert!(found.is_some());
    let found = found.unwrap();
    assert_eq!(found.id, doc.id);
    assert_eq!(found.source_hash, "sha256:abc123");

    // Upsert with a new hash (simulating file change)
    let updated = documents::upsert_document(
        &pool,
        workspace_id,
        "/tmp/test/notes.md",
        "sha256:def456",
        Some(2048),
    )
    .await
    .unwrap();
    assert_eq!(updated.id, doc.id, "Upsert should reuse the same row");
    assert_eq!(updated.source_hash, "sha256:def456");

    // Insert a summary for the document
    let summary = documents::insert_summary(
        &pool,
        &documents::InsertSummary {
            document_id: doc.id,
            tldr: "A test document about testing.",
            title: "Test Doc",
            tags: &["test".to_string(), "docs".to_string()],
            entities: &["TestEntity".to_string()],
            topics: &["testing".to_string()],
            relationships: &[],
            word_count: 42,
        },
    )
    .await
    .unwrap();
    assert_eq!(summary.document_id, doc.id);
    assert_eq!(summary.title, "Test Doc");
    assert_eq!(summary.word_count, 42);

    // Query summary back
    let found_summary = documents::get_summary_by_document(&pool, doc.id)
        .await
        .unwrap();
    assert!(found_summary.is_some());
    let found_summary = found_summary.unwrap();
    assert_eq!(found_summary.tldr, "A test document about testing.");

    // Verify source hash listing for skip logic
    let hashes = documents::get_source_hashes(&pool, workspace_id)
        .await
        .unwrap();
    let entry = hashes
        .iter()
        .find(|h| h.source_path == "/tmp/test/notes.md");
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().source_hash, "sha256:def456");

    // Query non-existent path returns None
    let missing = documents::get_document_by_path(&pool, workspace_id, "/no/such/file.md")
        .await
        .unwrap();
    assert!(missing.is_none());

    // Verify get_summary_by_source_path (the joined query)
    let full = documents::get_summary_by_source_path(&pool, workspace_id, "/tmp/test/notes.md")
        .await
        .unwrap();
    assert!(full.is_some());
    let full = full.unwrap();
    assert_eq!(full.title, "Test Doc");
    assert_eq!(full.source_hash, "sha256:def456");

    // Clean up test data
    sqlx::query("DELETE FROM summaries WHERE document_id = $1")
        .bind(doc.id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM documents WHERE id = $1")
        .bind(doc.id)
        .execute(&pool)
        .await
        .unwrap();
}
