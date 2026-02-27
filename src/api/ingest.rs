use crate::api::AppState;
use crate::queue::manager::QueueManager;
use crate::scanner::files::scan_directory;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

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

    let scan_result = scan_directory(&dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queued = scan_result.to_process.len();
    let skipped = scan_result.skipped;

    let client = state.inference.clone();
    let indexer = state.indexer.clone();
    let queue_state = state.queue.state.clone();
    let concurrency = state.config.workers;

    tokio::spawn(async move {
        let manager = QueueManager::from_state(queue_state, concurrency);
        manager.process_files(scan_result.to_process, client, indexer).await;
    });

    Ok(Json(IngestResponse {
        message: "Ingestion started".to_string(),
        files_queued: queued,
        files_skipped: skipped,
    }))
}
