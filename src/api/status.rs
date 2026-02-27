use crate::api::AppState;
use crate::queue::manager::QueueStatus;
use axum::extract::State;
use axum::Json;
use std::sync::Arc;

pub async fn handle_status(State(state): State<Arc<AppState>>) -> Json<QueueStatus> {
    let s = state.queue.state.lock().await;
    Json(s.status())
}
