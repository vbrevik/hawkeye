mod api;
mod config;
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
use queue::manager::QueueManager;
use search::indexer::SearchIndexer;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;

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

    let indexer = SearchIndexer::new_in_dir(std::path::Path::new(&config.index_path))
        .expect("Failed to create search index");

    let state = Arc::new(AppState {
        inference: Arc::new(InferenceClient::new(&config.mlx_url, &config.mlx_model)),
        queue: QueueManager::new(config.workers),
        indexer: Arc::new(Mutex::new(indexer)),
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
