use crate::features::summary::Summary;
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
            tracing::info!(file = %file, title = %r.title, "summary served");
            Ok(Json(r.into_summary()))
        }
        None => Err(AppError::not_found(format!("No summary found for {}", file))),
    }
}
