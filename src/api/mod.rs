pub mod browse;
pub mod ingest;
pub mod search;
pub mod status;
pub mod summary;
pub mod tags;
pub mod ui;

use crate::config::AppConfig;
use crate::inference::client::InferenceClient;
use crate::queue::manager::QueueManager;
use crate::search::indexer::SearchIndexer;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub config: AppConfig,
    pub queue: QueueManager,
    pub inference: Arc<InferenceClient>,
    pub indexer: Arc<Mutex<SearchIndexer>>,
}
