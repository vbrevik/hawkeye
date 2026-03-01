use crate::features::auth::middleware::ValidatedApiKey;
use crate::shared::db::workspaces;
use crate::shared::error::AppError;
use crate::shared::state::AppState;
use axum::extract::{Extension, Path, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::keys;

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateWorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

pub async fn handle_create_workspace(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateWorkspaceRequest>,
) -> Result<Json<CreateWorkspaceResponse>, AppError> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(AppError::bad_request("Workspace name cannot be empty"));
    }

    tracing::info!(name = %name, "creating workspace");

    let row = workspaces::create_workspace(&state.pg_pool, name)
        .await
        .map_err(AppError::internal)?;

    Ok(Json(CreateWorkspaceResponse {
        id: row.id,
        name: row.name,
        created_at: row.created_at,
    }))
}

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub workspace_id: Uuid,
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub key: String,
    pub key_prefix: String,
    pub workspace_id: Uuid,
    pub label: String,
    pub created_at: DateTime<Utc>,
}

pub async fn handle_create_api_key(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, AppError> {
    workspaces::find_workspace(&state.pg_pool, input.workspace_id)
        .await
        .map_err(AppError::internal)?
        .ok_or_else(|| AppError::not_found("Workspace not found"))?;

    let plaintext = keys::generate_api_key();
    let hash = keys::hash_api_key(&plaintext);
    let prefix = keys::key_prefix(&plaintext);
    let label = input.label.unwrap_or_default();

    tracing::info!(
        workspace_id = %input.workspace_id,
        key_prefix = %prefix,
        "creating API key"
    );

    let row = workspaces::create_api_key(
        &state.pg_pool,
        input.workspace_id,
        &hash,
        &prefix,
        &label,
    )
    .await
    .map_err(AppError::internal)?;

    Ok(Json(CreateApiKeyResponse {
        id: row.id,
        key: plaintext,
        key_prefix: row.key_prefix,
        workspace_id: row.workspace_id,
        label: row.label,
        created_at: row.created_at,
    }))
}

#[derive(Serialize)]
pub struct WorkspaceDocsResponse {
    pub workspace_id: Uuid,
    pub documents: Vec<workspaces::WorkspaceDocRow>,
}

pub async fn handle_list_workspace_docs(
    State(state): State<Arc<AppState>>,
    Extension(api_key): Extension<ValidatedApiKey>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<WorkspaceDocsResponse>, AppError> {
    if api_key.workspace_id != workspace_id {
        return Err(AppError::unauthorized(
            "API key does not belong to this workspace",
        ));
    }

    tracing::info!(workspace_id = %workspace_id, "listing workspace documents");

    let docs = workspaces::list_workspace_docs(&state.pg_pool, workspace_id)
        .await
        .map_err(AppError::internal)?;

    Ok(Json(WorkspaceDocsResponse {
        workspace_id,
        documents: docs,
    }))
}
