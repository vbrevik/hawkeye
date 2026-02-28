use axum::{routing::post, Json, Router};
use hawkeye::{
    api::AppState,
    config::AppConfig,
    inference::client::InferenceClient,
    queue::manager::QueueManager,
    scanner::files::scan_directory,
    search::indexer::SearchIndexer,
    summary::store::read_summary,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn mock_llm_handler() -> Json<serde_json::Value> {
    Json(json!({
        "choices": [{
            "message": {
                "content": "{\"tldr\": \"A test document about testing.\", \"title\": \"Test Doc\", \"tags\": [\"test\", \"docs\"], \"entities\": [\"TestEntity\"], \"topics\": [\"testing\"]}"
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

    let state = Arc::new(AppState {
        config: AppConfig {
            port: 0,
            mlx_url: mlx_url.clone(),
            mlx_model: "mock-model".to_string(),
            workers: 2,
            index_path: index_dir.path().display().to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            postgres_url: "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye".to_string(),
            etcd_url: "http://localhost:2379".to_string(),
            minio_url: "http://localhost:9000".to_string(),
            milvus_url: "http://localhost:19530".to_string(),
            neo4j_url: "http://localhost:7475".to_string(),
        },
        inference: Arc::new(InferenceClient::new(&mlx_url, "mock-model")),
        queue: QueueManager::new(2),
        indexer: Arc::new(Mutex::new(indexer)),
    });

    // 4. Scan and process
    let scan = scan_directory(dir.path()).unwrap();
    assert_eq!(scan.to_process.len(), 5, "Expected 5 files to process");
    assert_eq!(scan.skipped, 0);

    state
        .queue
        .process_files(scan.to_process, state.inference.clone(), state.indexer.clone())
        .await;

    // 5. Verify queue status
    let status = state.queue.state.lock().await;
    assert_eq!(status.completed, 5, "All 5 files should be completed");
    assert_eq!(status.failed, 0, "No failures expected");
    assert_eq!(status.in_progress, 0);
    drop(status);

    // 6. Verify .summary.json files on disk
    for i in 0..5 {
        let md_path = dir.path().join(format!("doc{}.md", i));
        let summary_path = hawkeye::summary::store::summary_path_for(&md_path);
        assert!(summary_path.exists(), "Missing summary for doc{}.md", i);

        let summary = read_summary(&summary_path).unwrap();
        assert_eq!(summary.title, "Test Doc");
        assert_eq!(summary.source, format!("doc{}.md", i));
        assert!(!summary.source_hash.is_empty());
        assert!(summary.word_count > 0);
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

    // First run — process all 3
    let indexer = SearchIndexer::new_in_dir(index_dir.path()).unwrap();
    let queue = QueueManager::new(2);
    let inference = Arc::new(InferenceClient::new(&mlx_url, "mock-model"));
    let indexer = Arc::new(Mutex::new(indexer));

    let scan1 = scan_directory(dir.path()).unwrap();
    assert_eq!(scan1.to_process.len(), 3);
    queue.process_files(scan1.to_process, inference.clone(), indexer.clone()).await;

    let status = queue.state.lock().await;
    assert_eq!(status.completed, 3);
    drop(status);

    // Second run — all 3 should be skipped (content unchanged)
    let scan2 = scan_directory(dir.path()).unwrap();
    assert_eq!(scan2.to_process.len(), 0, "All files should be skipped on rerun");
    assert_eq!(scan2.skipped, 3);
}
