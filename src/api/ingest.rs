use crate::api::AppState;
use crate::db::documents::get_source_hashes;
use crate::queue::manager::QueueManager;
use crate::scanner::files::scan_directory;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct IngestRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    pub message: String,
    pub files_queued: usize,
    pub files_skipped: usize,
}

pub async fn handle_ingest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<IngestRequest>,
) -> Result<Json<IngestResponse>, (StatusCode, String)> {
    let dir = PathBuf::from(&req.path);
    if !dir.is_dir() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("{} is not a directory", req.path),
        ));
    }

    let hashes = get_source_hashes(&state.pg_pool, Uuid::nil())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let known_hashes: HashMap<String, String> = hashes
        .into_iter()
        .map(|h| (h.source_path, h.source_hash))
        .collect();

    let scan_result = scan_directory(&dir, &known_hashes)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queued = scan_result.to_process.len();
    let skipped = scan_result.skipped;

    let client = state.inference.clone();
    let indexer = state.indexer.clone();
    let queue_state = state.queue.state.clone();
    let concurrency = state.config.workers;
    let pool = state.pg_pool.clone();

    tokio::spawn(async move {
        let manager = QueueManager::from_state(queue_state, concurrency);
        manager.process_files(scan_result.to_process, client, indexer, pool).await;
    });

    Ok(Json(IngestResponse {
        message: "Ingestion started".to_string(),
        files_queued: queued,
        files_skipped: skipped,
    }))
}
