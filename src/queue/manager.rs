use crate::inference::client::InferenceClient;
use crate::scanner::files::ScannedFile;
use crate::search::indexer::SearchIndexer;
use serde::Serialize;
use sqlx::PgPool;
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

pub struct QueueState {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<FileError>,
}

impl QueueState {
    pub fn status(&self) -> QueueStatus {
        QueueStatus {
            total: self.total,
            completed: self.completed,
            failed: self.failed,
            in_progress: self.in_progress,
            errors: self.errors.clone(),
        }
    }
}

pub struct QueueManager {
    pub state: Arc<Mutex<QueueState>>,
    concurrency: usize,
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
        }
    }

    pub fn from_state(state: Arc<Mutex<QueueState>>, concurrency: usize) -> Self {
        Self { state, concurrency }
    }

    pub async fn process_files(
        &self,
        files: Vec<ScannedFile>,
        client: Arc<InferenceClient>,
        indexer: Arc<Mutex<SearchIndexer>>,
        pool: PgPool,
    ) {
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let total = files.len();

        {
            let mut state = self.state.lock().await;
            state.total += total;
        }

        let mut handles = Vec::new();

        for file in files {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = client.clone();
            let state = self.state.clone();
            let indexer = indexer.clone();
            let pool = pool.clone();

            let handle = tokio::spawn(async move {
                {
                    let mut s = state.lock().await;
                    s.in_progress += 1;
                }

                let result = super::worker::process_file(&file, &client, &indexer, &pool).await;

                {
                    let mut s = state.lock().await;
                    s.in_progress -= 1;
                    match result {
                        Ok(_) => s.completed += 1,
                        Err(e) => {
                            s.failed += 1;
                            s.errors.push(FileError {
                                file: file.path.display().to_string(),
                                error: e.to_string(),
                                attempts: 3,
                            });
                        }
                    }
                }

                drop(permit);
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}
