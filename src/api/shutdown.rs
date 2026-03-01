use crate::api::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ShutdownParams {
    #[serde(default)]
    pub docker: bool,
}

#[derive(Serialize)]
pub struct ShutdownResponse {
    pub message: String,
}

pub async fn handle_shutdown(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ShutdownParams>,
) -> Json<ShutdownResponse> {
    if params.docker {
        tracing::info!("shutdown requested via API (with docker)");
        state.shutdown_docker.store(true, Ordering::SeqCst);
    } else {
        tracing::info!("shutdown requested via API");
    }
    state.shutdown.send(true).ok();
    Json(ShutdownResponse {
        message: if params.docker {
            "Shutdown initiated (docker containers will be stopped)".to_string()
        } else {
            "Shutdown initiated".to_string()
        },
    })
}
