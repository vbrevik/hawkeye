use crate::inference::client::InferenceClient;
use crate::scanner::files::ScannedFile;
use crate::search::indexer::SearchIndexer;
use crate::summary::store;
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_RETRIES: usize = 3;

pub async fn process_file(
    file: &ScannedFile,
    client: &InferenceClient,
    indexer: &Arc<Mutex<SearchIndexer>>,
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
                let summary_path = store::summary_path_for(&file.path);
                store::write_summary(&summary_path, &summary)
                    .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                        e.to_string().into()
                    })?;

                let mut idx = indexer.lock().await;
                idx.index_summary(&summary)
                    .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                        e.to_string().into()
                    })?;

                tracing::info!(file = %file.path.display(), "summarized");
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
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error after retries".into()))
}
