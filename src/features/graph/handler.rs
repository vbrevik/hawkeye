use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use super::neo4j::GraphResponse;

pub async fn handle_entity_graph(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<GraphResponse>, AppError> {
    let neo4j = state
        .neo4j
        .as_ref()
        .ok_or_else(|| AppError::internal("Knowledge graph not available"))?;

    let response = neo4j.query_entity(&name).await.map_err(AppError::internal)?;
    tracing::info!(entity = %name, nodes = response.nodes.len(), edges = response.edges.len(), "entity graph");
    Ok(Json(response))
}

pub async fn handle_document_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<GraphResponse>, AppError> {
    let neo4j = state
        .neo4j
        .as_ref()
        .ok_or_else(|| AppError::internal("Knowledge graph not available"))?;

    let response = neo4j.query_document(&id).await.map_err(AppError::internal)?;
    tracing::info!(doc_id = %id, nodes = response.nodes.len(), edges = response.edges.len(), "document graph");
    Ok(Json(response))
}
