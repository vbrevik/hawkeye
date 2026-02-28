use crate::api::AppState;
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
    Json(Facets {
        tags: to_entries(indexer.top_tags(20).unwrap_or_default()),
        topics: to_entries(indexer.top_topics(10).unwrap_or_default()),
        entities: to_entries(indexer.top_entities(15).unwrap_or_default()),
    })
}
