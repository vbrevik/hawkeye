use crate::features::graph::Neo4jClient;
use crate::features::queue::QueueManager;
use crate::features::search::indexer::SearchIndexer;
use crate::features::semantic::EmbedClient;
use crate::features::semantic::MilvusClient;
use crate::shared::config::AppConfig;
use crate::shared::inference::client::InferenceClient;
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
    pub neo4j: Option<Arc<Neo4jClient>>,
    pub inference: Arc<InferenceClient>,
    pub queue: Arc<QueueManager>,
    pub shutdown: watch::Sender<bool>,
    pub shutdown_docker: AtomicBool,
}
