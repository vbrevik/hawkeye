pub mod browse;
pub mod health;
pub mod ingest;
pub mod search;
pub mod status;
pub mod summary;
pub mod tags;
pub mod ui;

use crate::config::AppConfig;
use crate::search::indexer::SearchIndexer;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub config: AppConfig,
    pub redis_pool: RedisPool,
    pub indexer: Arc<Mutex<SearchIndexer>>,
    pub pg_pool: PgPool,
}
