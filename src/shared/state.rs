use crate::features::search::indexer::SearchIndexer;
use crate::features::semantic::EmbedClient;
use crate::features::semantic::MilvusClient;
use crate::shared::config::AppConfig;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};

pub struct AppState {
    pub config: AppConfig,
    pub redis_pool: RedisPool,
    pub indexer: Arc<Mutex<SearchIndexer>>,
    pub pg_pool: PgPool,
    pub embed: Arc<EmbedClient>,
    pub milvus: Arc<MilvusClient>,
    pub shutdown: watch::Sender<bool>,
    pub shutdown_docker: AtomicBool,
}
