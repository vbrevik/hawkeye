use crate::features::queue::manager::QueueStatus;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::State;
use axum::Json;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

pub async fn handle_status(State(state): State<Arc<AppState>>) -> Result<Json<QueueStatus>, AppError> {
    let status = state.queue.status().await;
    Ok(Json(status))
}

#[derive(Debug, Serialize)]
pub struct MlxStatus {
    pub online: bool,
    pub model: Option<String>,
    pub message: String,
}

pub async fn handle_mlx_status(State(state): State<Arc<AppState>>) -> Json<MlxStatus> {
    let client = match Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return Json(MlxStatus {
                online: false,
                model: None,
                message: e.to_string(),
            })
        }
    };

    // Try /v1/models to get model name
    let models_url = format!("{}/v1/models", state.config.mlx_url);
    match client.get(&models_url).send().await {
        Ok(res) if res.status().is_success() => {
            let model = res
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|v| v["data"][0]["id"].as_str().map(String::from));
            tracing::info!(model = ?model, "mlx online");
            Json(MlxStatus {
                online: true,
                model,
                message: "Online".to_string(),
            })
        }
        Ok(res) => Json(MlxStatus {
            online: false,
            model: None,
            message: format!("HTTP {}", res.status()),
        }),
        Err(e) => {
            tracing::warn!(error = %e, "mlx offline");
            Json(MlxStatus {
                online: false,
                model: None,
                message: if e.is_timeout() {
                    "Timeout".to_string()
                } else {
                    "Offline".to_string()
                },
            })
        }
    }
}
