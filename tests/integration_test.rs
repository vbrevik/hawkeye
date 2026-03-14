use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::{routing::{delete, get, post}, Json, Router};
use clap::Parser;
use hawkeye::{
    features::auth::handler::{handle_create_workspace, handle_create_api_key, handle_list_workspace_docs, handle_revoke_api_key},
    features::auth::middleware::require_auth,
    features::browse::handler::handle_browse,
    features::events::{
        handler::handle_events,
        publish_document_event, DocumentEvent,
    },
    features::graph::handler::{handle_entity_graph, handle_document_graph},
    features::ingest::scanner::scan_directory,
    features::queue::QueueManager,
    features::search::facets::handle_facets,
    features::search::handler::{handle_search, handle_reindex},
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
use std::time::Duration;
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

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    // 4. Scan
    let scan = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan.to_process.len(), 5, "Expected 5 files to process");
    assert_eq!(scan.skipped, 0);

    // 5. Process files with in-memory queue
    let queue = Arc::new(QueueManager::new(2));
    queue
        .process_files(
            scan.to_process,
            inference.clone(),
            indexer.clone(),
            pg_pool.clone(),
            embed.clone(),
            milvus.clone(),
            None,
            redis_pool,
        )
        .await;

    // 6. Verify queue status
    let status = queue.status().await;
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

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    // First run — process all 3
    let scan1 = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan1.to_process.len(), 3);

    let queue = Arc::new(QueueManager::new(2));
    queue
        .process_files(
            scan1.to_process,
            inference.clone(),
            indexer.clone(),
            pg_pool.clone(),
            embed.clone(),
            milvus.clone(),
            None,
            redis_pool,
        )
        .await;

    let status = queue.status().await;
    assert_eq!(status.completed, 3);

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

    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model", 0.1).unwrap());
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock-model").unwrap());
    let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());

    let scan = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan.to_process.len(), 10);

    // Process with 1 slow worker in background, cancel after a brief delay
    let queue = Arc::new(QueueManager::new(1));
    let queue_bg = queue.clone();
    let handle = tokio::spawn(async move {
        queue_bg
            .process_files(
                scan.to_process,
                inference.clone(),
                indexer.clone(),
                pg_pool.clone(),
                embed.clone(),
                milvus.clone(),
                None,
                redis_pool,
            )
            .await;
    });

    // Wait briefly for at least 1 to start processing
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Cancel the remaining jobs
    let result = queue.cancel().await;
    assert!(
        result.cancelled > 0 || result.already_completed > 0,
        "Expected some jobs to be cancelled or completed",
    );

    // Wait for the background task to finish
    let _ = tokio::time::timeout(Duration::from_secs(30), handle).await;

    // Verify status reflects cancel
    let status = queue.status().await;
    assert_eq!(status.in_progress, 0, "in_progress should be 0 after cancel");
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
    let inference = Arc::new(InferenceClient::new("http://127.0.0.1:1", "mock-model", 0.1).unwrap());
    let queue = Arc::new(QueueManager::new(4));
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
        inference,
        queue,
        shutdown: shutdown_tx,
        shutdown_docker: AtomicBool::new(false),
    });
    (state, pg_clone)
}

/// Helper: send a POST request with JSON body.
async fn post_json(app: Router, uri: &str, body: serde_json::Value) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes().to_vec();
    (status, body)
}

/// Helper: send a GET request with an Authorization Bearer header.
async fn get_with_auth(app: Router, uri: &str, token: &str) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .uri(uri)
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes().to_vec();
    (status, body)
}

/// Helper: send a DELETE request with an Authorization Bearer header.
async fn delete_with_auth(app: Router, uri: &str, token: &str) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .method("DELETE")
        .uri(uri)
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes().to_vec();
    (status, body)
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
}

#[tokio::test]
async fn test_sse_events() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, _pg) = create_test_app_state(indexer).await;
    let redis_pool = state.redis_pool.clone();

    // Start a real HTTP server (SSE is streaming — can't use oneshot)
    let app = Router::new()
        .route("/events", get(handle_events))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Connect SSE client
    let client = reqwest::Client::new();
    let mut response = client
        .get(format!("http://{}/events", addr))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Wait for Redis pub/sub subscription to establish
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Publish a document_done event via Redis pub/sub
    publish_document_event(
        &redis_pool,
        DEFAULT_WORKSPACE_ID,
        &DocumentEvent::Done {
            file: "test-sse.md".to_string(),
        },
    )
    .await
    .unwrap();

    // Read SSE data with timeout
    let chunk = tokio::time::timeout(Duration::from_secs(5), response.chunk())
        .await
        .expect("Timed out waiting for SSE event")
        .unwrap()
        .expect("SSE stream ended unexpectedly");

    let text = String::from_utf8(chunk.to_vec()).unwrap();

    // Verify SSE format: must contain event type and data
    assert!(
        text.contains("event: document_done"),
        "Missing 'event: document_done' in SSE output: {}",
        text
    );
    assert!(
        text.contains("test-sse.md"),
        "Missing file name in SSE output: {}",
        text
    );

    // Parse the JSON data line
    let data_line = text
        .lines()
        .find(|l| l.starts_with("data: "))
        .expect("No 'data:' line in SSE event");
    let json: serde_json::Value = serde_json::from_str(&data_line[6..]).unwrap();
    assert_eq!(json["type"], "document_done");
    assert_eq!(json["file"], "test-sse.md");
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
async fn test_workspace_and_auth() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));
    let (state, pg_pool) = create_test_app_state(indexer).await;

    let protected = Router::new()
        .route("/workspaces/{id}/docs", get(handle_list_workspace_docs))
        .route("/api-keys/{id}", delete(handle_revoke_api_key))
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), require_auth));

    let app = Router::new()
        .route("/workspaces", post(handle_create_workspace))
        .route("/api-keys", post(handle_create_api_key))
        .merge(protected)
        .with_state(state);

    // 1. Create workspace
    let (status, body) = post_json(app.clone(), "/workspaces", json!({"name": "Test Workspace"})).await;
    assert_eq!(status, StatusCode::OK);
    let ws: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let ws_id_str = ws["id"].as_str().unwrap();
    let ws_id: Uuid = ws_id_str.parse().unwrap();
    assert_eq!(ws["name"], "Test Workspace");

    // 2. Create workspace with empty name → 400
    let (status, _) = post_json(app.clone(), "/workspaces", json!({"name": "  "})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    // 3. Create API key for non-existent workspace → 404
    let (status, _) = post_json(app.clone(), "/api-keys", json!({
        "workspace_id": Uuid::new_v4().to_string(),
        "label": "bad"
    })).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // 4. Create API key for the workspace
    let (status, body) = post_json(app.clone(), "/api-keys", json!({
        "workspace_id": ws_id_str,
        "label": "test-key"
    })).await;
    assert_eq!(status, StatusCode::OK);
    let key_resp: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let api_key = key_resp["key"].as_str().unwrap().to_string();
    assert!(api_key.starts_with("hke_"), "Key should start with hke_");
    assert_eq!(api_key.len(), 36, "Key should be 36 chars (hke_ + 32)");
    assert_eq!(key_resp["workspace_id"], ws_id_str);
    assert_eq!(key_resp["label"], "test-key");
    assert!(key_resp["key_prefix"].as_str().unwrap().starts_with("hke_"));

    // 5. GET /workspaces/:id/docs without auth → 401
    let (status, body) = get_request(app.clone(), &format!("/workspaces/{}/docs", ws_id)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let err: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(err["error"].as_str().unwrap().contains("Authorization"));

    // 6. GET /workspaces/:id/docs with invalid key → 401
    let (status, _) = get_with_auth(
        app.clone(),
        &format!("/workspaces/{}/docs", ws_id),
        "hke_invalidkey00000000000000000000",
    ).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // 7. GET /workspaces/:id/docs with valid key → 200 (empty)
    let (status, body) = get_with_auth(
        app.clone(),
        &format!("/workspaces/{}/docs", ws_id),
        &api_key,
    ).await;
    assert_eq!(status, StatusCode::OK);
    let docs: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(docs["workspace_id"], ws_id_str);
    assert!(docs["documents"].as_array().unwrap().is_empty());

    // 8. Insert a document and verify it appears
    documents::upsert_document(&pg_pool, ws_id, "/test/auth.md", "sha256:authtest", Some(100))
        .await
        .unwrap();
    let (status, body) = get_with_auth(
        app.clone(),
        &format!("/workspaces/{}/docs", ws_id),
        &api_key,
    ).await;
    assert_eq!(status, StatusCode::OK);
    let docs: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let doc_list = docs["documents"].as_array().unwrap();
    assert_eq!(doc_list.len(), 1);
    assert_eq!(doc_list[0]["source_path"], "/test/auth.md");

    // 9. Access another workspace with this key → 401
    let other_ws_id = Uuid::new_v4();
    let (status, _) = get_with_auth(
        app.clone(),
        &format!("/workspaces/{}/docs", other_ws_id),
        &api_key,
    ).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // 10. Non-Bearer auth format → 401
    let req = Request::builder()
        .uri(format!("/workspaces/{}/docs", ws_id))
        .header("authorization", "Basic dXNlcjpwYXNz")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 11. DELETE /api-keys/:id without auth → 401
    let key_id = key_resp["id"].as_str().unwrap();
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/api-keys/{}", key_id))
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 12. DELETE /api-keys/:id for non-existent key → 404
    let (status, _) = delete_with_auth(
        app.clone(),
        &format!("/api-keys/{}", Uuid::new_v4()),
        &api_key,
    ).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // 13. Create a second API key, then revoke the first using the second
    let (status, body) = post_json(app.clone(), "/api-keys", json!({
        "workspace_id": ws_id_str,
        "label": "second-key"
    })).await;
    assert_eq!(status, StatusCode::OK);
    let second_key_resp: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let second_api_key = second_key_resp["key"].as_str().unwrap().to_string();

    // 14. Revoke the first key using the second key → 200
    let (status, body) = delete_with_auth(
        app.clone(),
        &format!("/api-keys/{}", key_id),
        &second_api_key,
    ).await;
    assert_eq!(status, StatusCode::OK);
    let revoke_resp: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(revoke_resp["id"], key_id);
    assert!(revoke_resp["revoked_at"].as_str().is_some());

    // 15. Revoke again (idempotent) → 200
    let (status, _) = delete_with_auth(
        app.clone(),
        &format!("/api-keys/{}", key_id),
        &second_api_key,
    ).await;
    assert_eq!(status, StatusCode::OK);

    // 16. Using the revoked key for auth → 401 "revoked"
    let (status, body) = get_with_auth(
        app.clone(),
        &format!("/workspaces/{}/docs", ws_id),
        &api_key,
    ).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let err: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(err["error"].as_str().unwrap().contains("revoked"));

    // Cleanup
    sqlx::query("DELETE FROM documents WHERE workspace_id = $1")
        .bind(ws_id)
        .execute(&pg_pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM api_keys WHERE workspace_id = $1")
        .bind(ws_id)
        .execute(&pg_pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM workspaces WHERE id = $1")
        .bind(ws_id)
        .execute(&pg_pool)
        .await
        .unwrap();
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

#[tokio::test]
async fn test_reindex_handler() {
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = Arc::new(Mutex::new(
        SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
    ));

    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();

    let workspace_id = DEFAULT_WORKSPACE_ID;

    // Insert 3 documents + summaries into Postgres
    let titles = ["OAuth2 Setup Guide", "K8s Deployment", "Rust Ownership"];
    let tldrs = [
        "Guide to setting up OAuth2 authentication",
        "Steps for deploying to Kubernetes cluster",
        "Introduction to Rust ownership and borrowing",
    ];
    let tags_list: [Vec<String>; 3] = [
        vec!["auth".into(), "oauth2".into()],
        vec!["kubernetes".into(), "devops".into()],
        vec!["rust".into(), "programming".into()],
    ];
    let sources = ["auth.md", "deploy.md", "rust.md"];

    let mut doc_ids = Vec::new();
    for i in 0..3 {
        let doc = documents::upsert_document(
            &pg_pool,
            workspace_id,
            &format!("/tmp/test/{}", sources[i]),
            &format!("sha256:reindex{}", i),
            Some(100),
        )
        .await
        .unwrap();
        documents::insert_summary(
            &pg_pool,
            &documents::InsertSummary {
                document_id: doc.id,
                tldr: tldrs[i],
                title: titles[i],
                tags: &tags_list[i],
                entities: &[],
                topics: &[],
                relationships: &[],
                word_count: 100,
            },
        )
        .await
        .unwrap();
        doc_ids.push(doc.id);
    }

    // Build app with /search and /reindex
    let (state, _pg) = create_test_app_state(indexer).await;
    let app = Router::new()
        .route("/search", get(handle_search))
        .route("/reindex", post(handle_reindex))
        .with_state(state);

    // 1. Search should return nothing (index is empty)
    let (status, body) = get_request(app.clone(), "/search?q=OAuth2").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(results.is_empty(), "Expected empty index before reindex");

    // 2. POST /reindex — should rebuild from Postgres
    let (status, body) = post_json(app.clone(), "/reindex", json!({})).await;
    assert_eq!(status, StatusCode::OK);
    let resp: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(
        resp["indexed"].as_u64().unwrap() >= 3,
        "Expected at least 3 indexed documents"
    );

    // 3. Search should now find documents
    let (status, body) = get_request(app.clone(), "/search?q=OAuth2").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(!results.is_empty(), "Expected search results after reindex");
    assert_eq!(results[0]["file"], "auth.md");

    let (status, body) = get_request(app.clone(), "/search?q=kubernetes").await;
    assert_eq!(status, StatusCode::OK);
    let results: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert!(!results.is_empty(), "Expected search results for kubernetes");
    assert_eq!(results[0]["file"], "deploy.md");

    // Clean up test data
    for doc_id in &doc_ids {
        sqlx::query("DELETE FROM summaries WHERE document_id = $1")
            .bind(doc_id)
            .execute(&pg_pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM documents WHERE id = $1")
            .bind(doc_id)
            .execute(&pg_pool)
            .await
            .unwrap();
    }
}
