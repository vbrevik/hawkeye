use crate::features::summary::{Relationship, Summary};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)] // fields populated by sqlx::FromRow
pub struct DocumentRow {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub source_path: String,
    pub source_hash: String,
    pub file_size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)] // fields populated by sqlx::FromRow
pub struct SummaryRow {
    pub id: Uuid,
    pub document_id: Uuid,
    pub tldr: String,
    pub title: String,
    pub tags: serde_json::Value,
    pub entities: serde_json::Value,
    pub topics: serde_json::Value,
    pub relationships: serde_json::Value,
    pub word_count: i64,
    pub created_at: DateTime<Utc>,
}

pub async fn upsert_document(
    pool: &PgPool,
    workspace_id: Uuid,
    source_path: &str,
    source_hash: &str,
    file_size_bytes: Option<i64>,
) -> Result<DocumentRow, sqlx::Error> {
    sqlx::query_as::<_, DocumentRow>(
        r#"
        INSERT INTO documents (workspace_id, source_path, source_hash, file_size_bytes)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (workspace_id, source_path)
        DO UPDATE SET source_hash = EXCLUDED.source_hash,
                      file_size_bytes = EXCLUDED.file_size_bytes,
                      updated_at = now()
        RETURNING id, workspace_id, source_path, source_hash, file_size_bytes, created_at, updated_at
        "#,
    )
    .bind(workspace_id)
    .bind(source_path)
    .bind(source_hash)
    .bind(file_size_bytes)
    .fetch_one(pool)
    .await
}

#[allow(dead_code)] // used in integration tests
pub async fn get_document_by_path(
    pool: &PgPool,
    workspace_id: Uuid,
    source_path: &str,
) -> Result<Option<DocumentRow>, sqlx::Error> {
    sqlx::query_as::<_, DocumentRow>(
        r#"
        SELECT id, workspace_id, source_path, source_hash, file_size_bytes, created_at, updated_at
        FROM documents
        WHERE workspace_id = $1 AND source_path = $2
        "#,
    )
    .bind(workspace_id)
    .bind(source_path)
    .fetch_optional(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SourceHashRow {
    pub source_path: String,
    pub source_hash: String,
}

pub async fn get_source_hashes(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<Vec<SourceHashRow>, sqlx::Error> {
    sqlx::query_as::<_, SourceHashRow>(
        r#"
        SELECT source_path, source_hash
        FROM documents
        WHERE workspace_id = $1
        "#,
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
}

#[derive(Debug)]
pub struct InsertSummary<'a> {
    pub document_id: Uuid,
    pub tldr: &'a str,
    pub title: &'a str,
    pub tags: &'a [String],
    pub entities: &'a [String],
    pub topics: &'a [String],
    pub relationships: &'a [Relationship],
    pub word_count: i64,
}

pub async fn insert_summary(
    pool: &PgPool,
    input: &InsertSummary<'_>,
) -> Result<SummaryRow, sqlx::Error> {
    let tags_json = serde_json::to_value(input.tags)
        .map_err(|e| sqlx::Error::Protocol(format!("failed to serialize tags: {e}")))?;
    let entities_json = serde_json::to_value(input.entities)
        .map_err(|e| sqlx::Error::Protocol(format!("failed to serialize entities: {e}")))?;
    let topics_json = serde_json::to_value(input.topics)
        .map_err(|e| sqlx::Error::Protocol(format!("failed to serialize topics: {e}")))?;
    let relationships_json = serde_json::to_value(input.relationships)
        .map_err(|e| sqlx::Error::Protocol(format!("failed to serialize relationships: {e}")))?;

    sqlx::query_as::<_, SummaryRow>(
        r#"
        INSERT INTO summaries (document_id, tldr, title, tags, entities, topics, relationships, word_count)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (document_id)
        DO UPDATE SET tldr = EXCLUDED.tldr,
                      title = EXCLUDED.title,
                      tags = EXCLUDED.tags,
                      entities = EXCLUDED.entities,
                      topics = EXCLUDED.topics,
                      relationships = EXCLUDED.relationships,
                      word_count = EXCLUDED.word_count
        RETURNING id, document_id, tldr, title, tags, entities, topics, relationships, word_count, created_at
        "#,
    )
    .bind(input.document_id)
    .bind(input.tldr)
    .bind(input.title)
    .bind(tags_json)
    .bind(entities_json)
    .bind(topics_json)
    .bind(relationships_json)
    .bind(input.word_count)
    .fetch_one(pool)
    .await
}

#[allow(dead_code)] // used in integration tests
pub async fn get_summary_by_document(
    pool: &PgPool,
    document_id: Uuid,
) -> Result<Option<SummaryRow>, sqlx::Error> {
    sqlx::query_as::<_, SummaryRow>(
        r#"
        SELECT id, document_id, tldr, title, tags, entities, topics, relationships, word_count, created_at
        FROM summaries
        WHERE document_id = $1
        "#,
    )
    .bind(document_id)
    .fetch_optional(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DocSummaryBrief {
    pub id: Uuid,
    pub source_path: String,
    pub title: String,
    pub tldr: String,
}

pub async fn get_doc_summaries_by_ids(
    pool: &PgPool,
    doc_ids: &[Uuid],
) -> Result<Vec<DocSummaryBrief>, sqlx::Error> {
    sqlx::query_as::<_, DocSummaryBrief>(
        r#"
        SELECT d.id, d.source_path, s.title, s.tldr
        FROM documents d
        JOIN summaries s ON s.document_id = d.id
        WHERE d.id = ANY($1)
        "#,
    )
    .bind(doc_ids)
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FullSummaryRow {
    pub source_path: String,
    pub source_hash: String,
    pub tldr: String,
    pub title: String,
    pub tags: serde_json::Value,
    pub entities: serde_json::Value,
    pub topics: serde_json::Value,
    pub relationships: serde_json::Value,
    pub word_count: i64,
    pub created_at: DateTime<Utc>,
}

impl FullSummaryRow {
    pub fn into_summary(self) -> Summary {
        let source = std::path::Path::new(&self.source_path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or(self.source_path.clone());

        Summary {
            source,
            source_hash: self.source_hash,
            created_at: self.created_at,
            tldr: self.tldr,
            title: self.title,
            tags: serde_json::from_value(self.tags).unwrap_or_default(),
            entities: serde_json::from_value(self.entities).unwrap_or_default(),
            topics: serde_json::from_value(self.topics).unwrap_or_default(),
            relationships: serde_json::from_value(self.relationships).unwrap_or_default(),
            word_count: self.word_count as u64,
        }
    }
}

pub async fn get_all_summaries(
    pool: &PgPool,
) -> Result<Vec<FullSummaryRow>, sqlx::Error> {
    sqlx::query_as::<_, FullSummaryRow>(
        r#"
        SELECT d.source_path, d.source_hash,
               s.tldr, s.title, s.tags, s.entities, s.topics, s.relationships, s.word_count, s.created_at
        FROM documents d
        JOIN summaries s ON s.document_id = d.id
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_summary_by_source_path(
    pool: &PgPool,
    workspace_id: Uuid,
    source_path: &str,
) -> Result<Option<FullSummaryRow>, sqlx::Error> {
    sqlx::query_as::<_, FullSummaryRow>(
        r#"
        SELECT d.source_path, d.source_hash,
               s.tldr, s.title, s.tags, s.entities, s.topics, s.relationships, s.word_count, s.created_at
        FROM documents d
        JOIN summaries s ON s.document_id = d.id
        WHERE d.workspace_id = $1 AND d.source_path = $2
        "#,
    )
    .bind(workspace_id)
    .bind(source_path)
    .fetch_optional(pool)
    .await
}
