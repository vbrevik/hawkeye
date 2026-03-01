use crate::features::summary::{Relationship, Summary};
use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::db::documents;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

pub async fn handle_summary(
    State(state): State<Arc<AppState>>,
    Path(file): Path<String>,
) -> Result<Json<Summary>, AppError> {
    let row = documents::get_summary_by_source_path(&state.pg_pool, DEFAULT_WORKSPACE_ID, &file)
        .await
        .map_err(AppError::internal)?;

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

            tracing::info!(file = %file, title = %r.title, "summary served");
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
        None => Err(AppError::not_found(format!("No summary found for {}", file))),
    }
}
