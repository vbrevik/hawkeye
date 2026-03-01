use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::db::documents;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SemanticQuery {
    pub q: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SemanticResult {
    pub doc_id: String,
    pub source_path: String,
    pub title: String,
    pub tldr: String,
    pub distance: f32,
}

pub async fn handle_semantic_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SemanticQuery>,
) -> Result<Json<Vec<SemanticResult>>, AppError> {
    let limit = params.limit.unwrap_or(10);

    let results = semantic_search_inner(&state, &params.q, limit)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, query = %params.q, "semantic search failed");
            AppError::internal(e)
        })?;

    Ok(Json(results))
}

async fn semantic_search_inner(
    state: &AppState,
    query: &str,
    limit: usize,
) -> Result<Vec<SemanticResult>, Box<dyn std::error::Error + Send + Sync>> {
    let vectors = state.embed.embed_chunks(query).await?;
    let query_vector = vectors
        .into_iter()
        .next()
        .ok_or("embedding returned no vectors")?;

    let workspace_id = DEFAULT_WORKSPACE_ID.to_string();
    let hits = state
        .milvus
        .search_similar(&query_vector, &workspace_id, limit)
        .await?;

    if hits.is_empty() {
        return Ok(vec![]);
    }

    let doc_ids: Vec<Uuid> = hits
        .iter()
        .filter_map(|h| Uuid::parse_str(&h.doc_id).ok())
        .collect();

    let rows = documents::get_doc_summaries_by_ids(&state.pg_pool, &doc_ids).await?;

    let row_map: HashMap<Uuid, documents::DocSummaryBrief> =
        rows.into_iter().map(|r| (r.id, r)).collect();

    let results: Vec<SemanticResult> = hits
        .iter()
        .filter_map(|hit| {
            let doc_uuid = Uuid::parse_str(&hit.doc_id).ok()?;
            let row = row_map.get(&doc_uuid)?;
            Some(SemanticResult {
                doc_id: hit.doc_id.clone(),
                source_path: row.source_path.clone(),
                title: row.title.clone(),
                tldr: row.tldr.clone(),
                distance: hit.distance,
            })
        })
        .collect();

    Ok(results)
}
