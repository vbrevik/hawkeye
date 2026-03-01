use crate::shared::state::AppState;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct FacetEntry {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct Facets {
    pub tags: Vec<FacetEntry>,
    pub topics: Vec<FacetEntry>,
    pub entities: Vec<FacetEntry>,
}

fn to_entries(pairs: Vec<(String, usize)>) -> Vec<FacetEntry> {
    pairs.into_iter().map(|(name, count)| FacetEntry { name, count }).collect()
}

pub async fn handle_facets(State(state): State<Arc<AppState>>) -> Json<Facets> {
    let indexer = state.indexer.lock().await;
    let tags = indexer.top_tags(20).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top tags");
        vec![]
    });
    let topics = indexer.top_topics(10).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top topics");
        vec![]
    });
    let entities = indexer.top_entities(15).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top entities");
        vec![]
    });
    Json(Facets {
        tags: to_entries(tags),
        topics: to_entries(topics),
        entities: to_entries(entities),
    })
}
