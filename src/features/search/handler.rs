use crate::features::search::indexer::SearchResult;
use crate::features::summary::Summary;
use crate::shared::db::documents;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct ReindexResponse {
    pub indexed: usize,
}

pub async fn handle_reindex(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ReindexResponse>, AppError> {
    let rows = documents::get_all_summaries(&state.pg_pool)
        .await
        .map_err(AppError::internal)?;

    let summaries: Vec<Summary> = rows.into_iter().map(|row| row.into_summary()).collect();

    let mut indexer = state.indexer.lock().await;
    indexer.clear_all().map_err(AppError::internal)?;
    let indexed = indexer
        .index_summaries(&summaries)
        .map_err(AppError::internal)?;

    tracing::info!(indexed, "reindex complete");
    Ok(Json(ReindexResponse { indexed }))
}

pub async fn handle_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResult>>, AppError> {
    let limit = params.limit.unwrap_or(20);
    let indexer = state.indexer.lock().await;
    let results = indexer
        .search(&params.q, limit)
        .map_err(AppError::internal)?;
    tracing::info!(query = %params.q, hits = results.len(), limit, "search");
    Ok(Json(results))
}
