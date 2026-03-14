use crate::features::events::{publish_document_event, DocumentEvent};
use crate::features::graph::Neo4jClient;
use crate::features::ingest::scanner::ScannedFile;
use crate::features::ingest::worker;
use crate::features::search::indexer::SearchIndexer;
use crate::features::semantic::embed_client::EmbedClient;
use crate::features::semantic::milvus::MilvusClient;
use crate::shared::config::DEFAULT_WORKSPACE_ID;
use crate::shared::inference::client::InferenceClient;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

#[derive(Debug, Clone, Serialize)]
pub struct QueueStatus {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<FileError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileError {
    pub file: String,
    pub error: String,
    pub attempts: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct CancelResult {
    pub cancelled: usize,
    pub already_completed: usize,
    pub already_failed: usize,
}

struct QueueState {
    total: usize,
    completed: usize,
    failed: usize,
    in_progress: usize,
    errors: Vec<FileError>,
}

pub struct QueueManager {
    state: Arc<Mutex<QueueState>>,
    concurrency: usize,
    cancelled: Arc<AtomicBool>,
}

impl QueueManager {
    pub fn new(concurrency: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(QueueState {
                total: 0,
                completed: 0,
                failed: 0,
                in_progress: 0,
                errors: Vec::new(),
            })),
            concurrency,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn status(&self) -> QueueStatus {
        let s = self.state.lock().await;
        QueueStatus {
            total: s.total,
            completed: s.completed,
            failed: s.failed,
            in_progress: s.in_progress,
            errors: s.errors.clone(),
        }
    }

    pub async fn cancel(&self) -> CancelResult {
        self.cancelled.store(true, Ordering::SeqCst);
        let s = self.state.lock().await;
        let pending = s
            .total
            .saturating_sub(s.completed)
            .saturating_sub(s.failed)
            .saturating_sub(s.in_progress);
        CancelResult {
            cancelled: pending,
            already_completed: s.completed,
            already_failed: s.failed,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn process_files(
        &self,
        files: Vec<ScannedFile>,
        client: Arc<InferenceClient>,
        indexer: Arc<Mutex<SearchIndexer>>,
        pool: PgPool,
        embed: Arc<EmbedClient>,
        milvus: Arc<MilvusClient>,
        neo4j: Option<Arc<Neo4jClient>>,
        redis_pool: deadpool_redis::Pool,
    ) {
        self.cancelled.store(false, Ordering::SeqCst);
        let total = files.len();

        {
            let mut s = self.state.lock().await;
            s.total += total;
        }

        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::new();

        for file in files {
            let sem = semaphore.clone();
            let client = client.clone();
            let state = self.state.clone();
            let indexer = indexer.clone();
            let pool = pool.clone();
            let embed = embed.clone();
            let milvus = milvus.clone();
            let neo4j = neo4j.clone();
            let redis_pool = redis_pool.clone();
            let cancelled = self.cancelled.clone();

            let handle = tokio::spawn(async move {
                let _permit = sem.acquire_owned().await.unwrap();

                if cancelled.load(Ordering::SeqCst) {
                    let mut s = state.lock().await;
                    s.total = s.total.saturating_sub(1);
                    return;
                }

                {
                    let mut s = state.lock().await;
                    s.in_progress += 1;
                }

                let file_path = file.path.display().to_string();
                let result = worker::process_file(
                    &file,
                    &client,
                    &indexer,
                    &pool,
                    &embed,
                    &milvus,
                    neo4j.as_deref(),
                )
                .await;

                {
                    let mut s = state.lock().await;
                    s.in_progress -= 1;
                    match &result {
                        Ok(()) => s.completed += 1,
                        Err(e) => {
                            s.failed += 1;
                            s.errors.push(FileError {
                                file: file_path.clone(),
                                error: e.to_string(),
                                attempts: 3,
                            });
                        }
                    }
                }

                let event = match &result {
                    Ok(()) => DocumentEvent::Done {
                        file: file_path,
                    },
                    Err(e) => DocumentEvent::Failed {
                        file: file_path,
                        error: e.to_string(),
                    },
                };
                if let Err(e) =
                    publish_document_event(&redis_pool, DEFAULT_WORKSPACE_ID, &event).await
                {
                    tracing::warn!(error = %e, "failed to publish SSE event");
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}
