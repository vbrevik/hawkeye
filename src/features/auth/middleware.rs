use crate::shared::db::workspaces;
use crate::shared::state::AppState;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ValidatedApiKey {
    pub key_id: Uuid,
    pub workspace_id: Uuid,
}

fn error_response(msg: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": msg }))).into_response()
}

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(t) => t.to_string(),
        None => return error_response("Missing or invalid Authorization header"),
    };

    if !token.starts_with("hke_") {
        return error_response("Invalid API key format");
    }

    let key_hash = super::keys::hash_api_key(&token);

    let api_key = match workspaces::find_api_key_by_hash(&state.pg_pool, &key_hash).await {
        Ok(Some(key)) => key,
        Ok(None) => return error_response("Invalid API key"),
        Err(_) => return error_response("Authentication service unavailable"),
    };

    if api_key.revoked_at.is_some() {
        return error_response("API key has been revoked");
    }

    if let Some(expires) = api_key.expires_at {
        if expires < Utc::now() {
            return error_response("API key has expired");
        }
    }

    request.extensions_mut().insert(ValidatedApiKey {
        key_id: api_key.id,
        workspace_id: api_key.workspace_id,
    });

    next.run(request).await
}
