use crate::api::AppState;
use crate::db::documents;
use crate::summary::{Relationship, Summary};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_summary(
    State(state): State<Arc<AppState>>,
    Path(file): Path<String>,
) -> Result<Json<Summary>, (StatusCode, String)> {
    let row = documents::get_summary_by_source_path(&state.pg_pool, Uuid::nil(), &file)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match row {
        Some(r) => {
            let source = std::path::Path::new(&r.source_path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or(r.source_path.clone());
            let tags: Vec<String> = serde_json::from_value(r.tags).unwrap_or_default();
            let entities: Vec<String> = serde_json::from_value(r.entities).unwrap_or_default();
            let topics: Vec<String> = serde_json::from_value(r.topics).unwrap_or_default();
            let relationships: Vec<Relationship> =
                serde_json::from_value(r.relationships).unwrap_or_default();

            Ok(Json(Summary {
                source,
                source_hash: r.source_hash,
                created_at: r.created_at,
                tldr: r.tldr,
                title: r.title,
                tags,
                entities,
                topics,
                relationships,
                word_count: r.word_count as u64,
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No summary found for {}", file),
        )),
    }
}
