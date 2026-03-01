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

    let summaries: Vec<Summary> = rows
        .iter()
        .map(|row| {
            let source = std::path::Path::new(&row.source_path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| row.source_path.clone());
            let tags: Vec<String> = serde_json::from_value(row.tags.clone()).unwrap_or_default();
            let entities: Vec<String> =
                serde_json::from_value(row.entities.clone()).unwrap_or_default();
            let topics: Vec<String> =
                serde_json::from_value(row.topics.clone()).unwrap_or_default();

            Summary {
                source,
                source_hash: row.source_hash.clone(),
                created_at: row.created_at,
                tldr: row.tldr.clone(),
                title: row.title.clone(),
                tags,
                entities,
                topics,
                relationships: vec![],
                word_count: row.word_count as u64,
            }
        })
        .collect();

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
