use axum::{routing::post, Json, Router};
use hawkeye::{
    config::DEFAULT_WORKSPACE_ID,
    db::documents,
    embedding::client::EmbedClient,
    inference::client::InferenceClient,
    milvus::client::MilvusClient,
    queue::{
        consumer::spawn_consumers,
        stream::RedisQueue,
    },
    scanner::files::scan_directory,
    search::indexer::SearchIndexer,
};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{watch, Mutex};
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
