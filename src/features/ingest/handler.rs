use crate::features::ingest::scanner::scan_directory;
use crate::features::queue::manager::CancelResult;
use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::db::documents::get_source_hashes;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct IngestRequest {
    pub path: String,
    pub limit: Option<usize>,
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
) -> Result<Json<IngestResponse>, AppError> {
    let dir = PathBuf::from(&req.path);
    if !dir.is_dir() {
        return Err(AppError::bad_request(format!("{} is not a directory", req.path)));
    }

    let hashes = get_source_hashes(&state.pg_pool, DEFAULT_WORKSPACE_ID)
        .await
        .map_err(AppError::internal)?;
    let known_hashes: HashMap<String, String> = hashes
        .into_iter()
        .map(|h| (h.source_path, h.source_hash))
        .collect();

    let mut scan_result = scan_directory(&dir, &known_hashes)
        .map_err(AppError::internal)?;

    if let Some(limit) = req.limit {
        scan_result.to_process.truncate(limit);
    }

    let queued = scan_result.to_process.len();
    let skipped = scan_result.skipped;

    let queue = state.queue.clone();
    let inference = state.inference.clone();
    let indexer = state.indexer.clone();
    let pool = state.pg_pool.clone();
    let embed = state.embed.clone();
    let milvus = state.milvus.clone();
    let neo4j = state.neo4j.clone();
    let redis_pool = state.redis_pool.clone();

    tokio::spawn(async move {
        queue
            .process_files(
                scan_result.to_process,
                inference,
                indexer,
                pool,
                embed,
                milvus,
                neo4j,
                redis_pool,
            )
            .await;
    });

    tracing::info!(path = %req.path, queued, skipped, "ingest started");

    Ok(Json(IngestResponse {
        message: "Ingestion started".to_string(),
        files_queued: queued,
        files_skipped: skipped,
    }))
}

pub async fn handle_cancel(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CancelResult>, AppError> {
    let result = state.queue.cancel().await;

    tracing::info!(
        cancelled = result.cancelled,
        completed = result.already_completed,
        failed = result.already_failed,
        "ingestion cancelled"
    );

    Ok(Json(result))
}
