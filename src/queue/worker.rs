use crate::config::DEFAULT_WORKSPACE_ID;
use crate::db::documents::{self, InsertSummary};
use crate::embedding::client::EmbedClient;
use crate::inference::client::InferenceClient;
use crate::milvus::client::MilvusClient;
use crate::scanner::files::ScannedFile;
use crate::search::indexer::SearchIndexer;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

const MAX_RETRIES: usize = 3;

pub async fn process_file(
    file: &ScannedFile,
    client: &InferenceClient,
    indexer: &Arc<Mutex<SearchIndexer>>,
    pool: &PgPool,
    embed: &EmbedClient,
    milvus: &MilvusClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = tokio::fs::read_to_string(&file.path).await?;
    let filename = file
        .path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("unknown");

    let mut last_error: Option<Box<dyn std::error::Error + Send + Sync>> = None;

    for attempt in 1..=MAX_RETRIES {
        match client.summarize(filename, &content, &file.hash).await {
            Ok(summary) => {
                let source_path = file.path.display().to_string();

                let doc = documents::upsert_document(
                    pool,
                    DEFAULT_WORKSPACE_ID,
                    &source_path,
                    &file.hash,
                    Some(file.size as i64),
                )
                .await
                .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                    e.to_string().into()
                })?;

                documents::insert_summary(
                    pool,
                    &InsertSummary {
                        document_id: doc.id,
                        tldr: &summary.tldr,
                        title: &summary.title,
                        tags: &summary.tags,
                        entities: &summary.entities,
                        topics: &summary.topics,
                        relationships: &summary.relationships,
                        word_count: summary.word_count as i64,
                    },
                )
                .await
                .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                    e.to_string().into()
                })?;

                let mut idx = indexer.lock().await;
                idx.index_summary(&summary)
                    .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                        e.to_string().into()
                    })?;

                tracing::info!(file = %file.path.display(), "summarized");

                // Embed and store vectors (non-critical — log errors, don't fail ingest)
                let ws_id = DEFAULT_WORKSPACE_ID.to_string();
                let doc_id_str = doc.id.to_string();
                match embed.embed_chunks(&content).await {
                    Ok(vectors) => {
                        if let Err(e) = milvus.delete_by_doc_id(&doc_id_str).await {
                            tracing::warn!(file = %file.path.display(), error = %e, "milvus delete failed");
                        }
                        match milvus.insert_chunks(&doc_id_str, &ws_id, &vectors).await {
                            Ok(n) => tracing::info!(file = %file.path.display(), chunks = n, "vectors stored in milvus"),
                            Err(e) => tracing::warn!(file = %file.path.display(), error = %e, "milvus insert failed"),
                        }
                    }
                    Err(e) => {
                        tracing::warn!(file = %file.path.display(), error = %e, "embedding failed — doc searchable via full-text only");
                    }
                }

                return Ok(());
            }
            Err(e) => {
                tracing::warn!(
                    file = %file.path.display(),
                    attempt,
                    error = %e,
                    "inference failed, retrying"
                );
                last_error = Some(e);
                if attempt < MAX_RETRIES {
                    let backoff = Duration::from_secs(1 << (attempt - 1));
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error after retries".into()))
}
