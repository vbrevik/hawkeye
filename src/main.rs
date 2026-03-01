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
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use uuid::Uuid;

async fn shutdown_signal(mut shutdown_rx: watch::Receiver<bool>) {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("received SIGINT");
        }
        _ = sigterm => {
            tracing::info!("received SIGTERM");
        }
        _ = shutdown_rx.changed() => {}
    }
}

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

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let consumer_handles = queue::consumer::spawn_consumers(
        queue,
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        config.workers,
        shutdown_rx.clone(),
    );

    tracing::info!(workers = config.workers, "consumers started");

    let state = Arc::new(AppState {
        redis_pool,
        indexer,
        config: config.clone(),
        pg_pool,
        shutdown: shutdown_tx,
        shutdown_docker: AtomicBool::new(false),
    });

    let app = Router::new()
        .route("/", get(api::ui::handle_ui))
        .route("/ingest", post(api::ingest::handle_ingest))
        .route("/cancel", post(api::ingest::handle_cancel))
        .route("/shutdown", post(api::shutdown::handle_shutdown))
        .route("/status", get(api::status::handle_status))
        .route("/mlx-status", get(api::status::handle_mlx_status))
        .route("/search", get(api::search::handle_search))
        .route("/facets", get(api::tags::handle_facets))
        .route("/summary/{file}", get(api::summary::handle_summary))
        .route("/browse", get(api::browse::handle_browse))
        .route("/health", get(api::health::handle_health))
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("hawkeye listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_rx))
        .await
        .unwrap();

    tracing::info!("server stopped, waiting for consumers");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    for h in consumer_handles {
        h.abort();
    }

    if state.shutdown_docker.load(std::sync::atomic::Ordering::SeqCst) {
        tracing::info!("stopping docker containers");
        let status = tokio::process::Command::new("docker")
            .args(["compose", "down"])
            .status()
            .await;
        match status {
            Ok(s) if s.success() => tracing::info!("docker containers stopped"),
            Ok(s) => tracing::warn!(code = ?s.code(), "docker compose down exited with error"),
            Err(e) => tracing::error!(error = %e, "failed to run docker compose down"),
        }
    }

    tracing::info!("shutdown complete");
}
