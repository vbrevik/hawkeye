use crate::summary::store;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use std::path::PathBuf;

pub async fn handle_summary(
    Path(file): Path<String>,
) -> Result<Json<store::Summary>, (StatusCode, String)> {
    let md_path = PathBuf::from(&file);
    let summary_path = store::summary_path_for(&md_path);

    if !summary_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("No summary found for {}", file),
        ));
    }

    let summary = store::read_summary(&summary_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summary))
}
