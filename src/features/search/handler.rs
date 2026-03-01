use crate::features::search::indexer::SearchResult;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
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
