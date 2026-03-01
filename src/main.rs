mod api;
mod config;
mod db;
mod inference;
mod queue;
mod scanner;
mod search;
mod summary;

use api::AppState;
use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use config::AppConfig;
use inference::client::InferenceClient;
use queue::stream::RedisQueue;
use search::indexer::SearchIndexer;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("hawkeye=info")
        .init();

    let config = AppConfig::parse();

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("database migrations applied");

    let redis_cfg = deadpool_redis::Config::from_url(&config.redis_url);
    let redis_pool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    let queue = RedisQueue::new(redis_pool.clone(), Uuid::nil());
    queue
        .ensure_group()
        .await
        .expect("Failed to create Redis consumer group");

    tracing::info!("redis consumer group ready");

    let indexer = SearchIndexer::new_in_dir(std::path::Path::new(&config.index_path))
        .expect("Failed to create search index");

    let inference = Arc::new(InferenceClient::new(&config.mlx_url, &config.mlx_model));
    let indexer = Arc::new(Mutex::new(indexer));

    let _consumer_handles = queue::consumer::spawn_consumers(
        queue,
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        config.workers,
    );

    tracing::info!(workers = config.workers, "consumers started");

    let state = Arc::new(AppState {
        inference,
        redis_pool,
        indexer,
        config: config.clone(),
        pg_pool,
    });

    let app = Router::new()
        .route("/", get(api::ui::handle_ui))
        .route("/ingest", post(api::ingest::handle_ingest))
        .route("/status", get(api::status::handle_status))
        .route("/mlx-status", get(api::status::handle_mlx_status))
        .route("/search", get(api::search::handle_search))
        .route("/facets", get(api::tags::handle_facets))
        .route("/summary/{file}", get(api::summary::handle_summary))
        .route("/browse", get(api::browse::handle_browse))
        .route("/health", get(api::health::handle_health))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("hawkeye listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app).await.unwrap();
}
