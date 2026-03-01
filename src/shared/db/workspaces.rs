use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct WorkspaceRow {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)] // fields populated by sqlx::FromRow
pub struct ApiKeyRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub key_hash: String,
    pub key_prefix: String,
    pub label: String,
    pub created_by: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct WorkspaceDocRow {
    pub id: Uuid,
    pub source_path: String,
    pub source_hash: String,
    pub created_at: DateTime<Utc>,
    pub title: Option<String>,
}

pub async fn create_workspace(
    pool: &PgPool,
    name: &str,
) -> Result<WorkspaceRow, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        INSERT INTO workspaces (name)
        VALUES ($1)
        RETURNING id, name, created_at, updated_at
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn find_workspace(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<WorkspaceRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        SELECT id, name, created_at, updated_at
        FROM workspaces
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create_api_key(
    pool: &PgPool,
    workspace_id: Uuid,
    key_hash: &str,
    key_prefix: &str,
    label: &str,
) -> Result<ApiKeyRow, sqlx::Error> {
    sqlx::query_as::<_, ApiKeyRow>(
        r#"
        INSERT INTO api_keys (workspace_id, key_hash, key_prefix, label)
        VALUES ($1, $2, $3, $4)
        RETURNING id, workspace_id, key_hash, key_prefix, label, created_by, expires_at, revoked_at, created_at
        "#,
    )
    .bind(workspace_id)
    .bind(key_hash)
    .bind(key_prefix)
    .bind(label)
    .fetch_one(pool)
    .await
}

pub async fn find_api_key_by_hash(
    pool: &PgPool,
    key_hash: &str,
) -> Result<Option<ApiKeyRow>, sqlx::Error> {
    sqlx::query_as::<_, ApiKeyRow>(
        r#"
        SELECT id, workspace_id, key_hash, key_prefix, label, created_by, expires_at, revoked_at, created_at
        FROM api_keys
        WHERE key_hash = $1
        "#,
    )
    .bind(key_hash)
    .fetch_optional(pool)
    .await
}

pub async fn list_workspace_docs(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<WorkspaceDocRow>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceDocRow>(
        r#"
        SELECT d.id, d.source_path, d.source_hash, d.created_at,
               s.title
        FROM documents d
        LEFT JOIN summaries s ON s.document_id = d.id
        WHERE d.workspace_id = $1
        ORDER BY d.created_at DESC
        "#,
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}
