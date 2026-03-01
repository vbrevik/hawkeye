use axum::{routing::post, Json, Router};
use hawkeye::{
    api::AppState,
    config::AppConfig,
    db::documents,
    inference::client::InferenceClient,
    queue::manager::QueueManager,
    scanner::files::scan_directory,
    search::indexer::SearchIndexer,
};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const TEST_POSTGRES_URL: &str = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye";

async fn mock_llm_handler() -> Json<serde_json::Value> {
    Json(json!({
        "choices": [{
            "message": {
                "content": "{\"tldr\": \"A test document about testing.\", \"title\": \"Test Doc\", \"tags\": [\"test\", \"docs\"], \"entities\": [\"TestEntity\"], \"topics\": [\"testing\"], \"relationships\": [{\"from\": \"TestEntity\", \"rel\": \"related_to\", \"to\": \"testing\", \"context\": \"test relationship\"}]}"
            }
        }]
    }))
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

    // 3. Create app state
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = SearchIndexer::new_in_dir(index_dir.path()).unwrap();
    let mlx_url = format!("http://{}", mock_addr);
    let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pg_pool).await.unwrap();

    let state = Arc::new(AppState {
        config: AppConfig {
            port: 0,
            mlx_url: mlx_url.clone(),
            mlx_model: "mock-model".to_string(),
            workers: 2,
            index_path: index_dir.path().display().to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            postgres_url: TEST_POSTGRES_URL.to_string(),
            etcd_url: "http://localhost:2379".to_string(),
            minio_url: "http://localhost:9000".to_string(),
            milvus_url: "http://localhost:19530".to_string(),
            neo4j_url: "http://localhost:7475".to_string(),
        },
        inference: Arc::new(InferenceClient::new(&mlx_url, "mock-model")),
        queue: QueueManager::new(2),
        indexer: Arc::new(Mutex::new(indexer)),
        pg_pool: pg_pool.clone(),
    });

    // 4. Scan and process
    let scan = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan.to_process.len(), 5, "Expected 5 files to process");
    assert_eq!(scan.skipped, 0);

    state
        .queue
        .process_files(
            scan.to_process,
            state.inference.clone(),
            state.indexer.clone(),
            state.pg_pool.clone(),
        )
        .await;

    // 5. Verify queue status
    let status = state.queue.state.lock().await;
    assert_eq!(status.completed, 5, "All 5 files should be completed");
    assert_eq!(status.failed, 0, "No failures expected");
    assert_eq!(status.in_progress, 0);
    drop(status);

    // 6. Verify summaries in Postgres
    for i in 0..5 {
        let md_path = dir.path().join(format!("doc{}.md", i));
        let source_path = md_path.display().to_string();

        let doc = documents::get_document_by_path(&pg_pool, Uuid::nil(), &source_path)
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

    // 7. Verify search works
    let indexer = state.indexer.lock().await;
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

    // First run — process all 3
    let indexer = SearchIndexer::new_in_dir(index_dir.path()).unwrap();
    let queue = QueueManager::new(2);
    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model"));
    let indexer = Arc::new(Mutex::new(indexer));

    let scan1 = scan_directory(dir.path(), &HashMap::new()).unwrap();
    assert_eq!(scan1.to_process.len(), 3);
    queue
        .process_files(scan1.to_process, inference.clone(), indexer.clone(), pg_pool.clone())
        .await;

    let status = queue.state.lock().await;
    assert_eq!(status.completed, 3);
    drop(status);

    // Second run — fetch known hashes from Postgres, all 3 should be skipped
    let hashes = documents::get_source_hashes(&pg_pool, Uuid::nil())
        .await
        .unwrap();
    let known: HashMap<String, String> = hashes
        .into_iter()
        .map(|h| (h.source_path, h.source_hash))
        .collect();

    let scan2 = scan_directory(dir.path(), &known).unwrap();
    assert_eq!(scan2.to_process.len(), 0, "All files should be skipped on rerun");
    assert_eq!(scan2.skipped, 3);
}

#[tokio::test]
async fn test_db_migrations_and_document_crud() {
    let pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let workspace_id = Uuid::nil();

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
