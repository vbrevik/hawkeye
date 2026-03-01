mod features;
mod shared;

use shared::state::AppState;
use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use tower_http::services::{ServeDir, ServeFile};
use clap::Parser;
use shared::config::{AppConfig, DEFAULT_WORKSPACE_ID};
use features::graph::Neo4jClient;
use features::semantic::EmbedClient;
use shared::inference::client::InferenceClient;
use features::semantic::MilvusClient;
use features::queue::RedisQueue;
use features::search::SearchIndexer;
use sqlx::postgres::PgPoolOptions;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};

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

    let queue = RedisQueue::new(redis_pool.clone(), DEFAULT_WORKSPACE_ID);
    queue
        .ensure_group()
        .await
        .expect("Failed to create Redis consumer group");

    tracing::info!("redis consumer group ready");

    let indexer = SearchIndexer::new_in_dir(std::path::Path::new(&config.index_path))
        .expect("Failed to create search index");

    let inference = Arc::new(
        InferenceClient::new(&config.mlx_url, &config.mlx_model, config.temperature)
            .expect("Failed to build inference client"),
    );
    let indexer = Arc::new(Mutex::new(indexer));

    let embed = Arc::new(
        EmbedClient::new(&config.embed_url, &config.embed_model)
            .expect("Failed to build embedding client"),
    );
    let milvus = Arc::new(
        MilvusClient::new(&config.milvus_url)
            .expect("Failed to build Milvus client"),
    );

    if let Err(e) = milvus.ensure_collection().await {
        tracing::warn!(error = %e, "failed to ensure Milvus collection — semantic search may not work");
    } else {
        tracing::info!("milvus doc_chunks collection ready");
    }

    let neo4j = match Neo4jClient::new(
        &config.neo4j_bolt_url,
        &config.neo4j_user,
        &config.neo4j_password,
    )
    .await
    {
        Ok(client) => {
            if let Err(e) = client.ensure_indexes().await {
                tracing::warn!(error = %e, "failed to create Neo4j indexes");
            } else {
                tracing::info!("neo4j indexes ready");
            }
            Some(Arc::new(client))
        }
        Err(e) => {
            tracing::warn!(error = %e, "neo4j not available — knowledge graph disabled");
            None
        }
    };

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let consumer_handles = features::queue::consumer::spawn_consumers(
        queue,
        inference.clone(),
        indexer.clone(),
        pg_pool.clone(),
        embed.clone(),
        milvus.clone(),
        neo4j.clone(),
        config.workers,
        shutdown_rx.clone(),
    );

    tracing::info!(workers = config.workers, "consumers started");

    let state = Arc::new(AppState {
        redis_pool,
        indexer,
        config: config.clone(),
        pg_pool,
        embed,
        milvus,
        neo4j,
        shutdown: shutdown_tx,
        shutdown_docker: AtomicBool::new(false),
    });

    let protected_workspace_routes = Router::new()
        .route("/workspaces/{id}/docs", get(features::auth::handler::handle_list_workspace_docs))
        .route("/api-keys/{id}", delete(features::auth::handler::handle_revoke_api_key))
        .route_layer(middleware::from_fn_with_state(state.clone(), features::auth::middleware::require_auth));

    let app = Router::new()
        .route("/workspaces", post(features::auth::handler::handle_create_workspace))
        .route("/api-keys", post(features::auth::handler::handle_create_api_key))
        .merge(protected_workspace_routes)
        .route("/ingest", post(features::ingest::handler::handle_ingest))
        .route("/cancel", post(features::ingest::handler::handle_cancel))
        .route("/shutdown", post(shared::shutdown::handle_shutdown))
        .route("/status", get(shared::status::handle_status))
        .route("/mlx-status", get(shared::status::handle_mlx_status))
        .route("/search", get(features::search::handler::handle_search))
        .route("/reindex", post(features::search::handler::handle_reindex))
        .route("/search/semantic", get(features::semantic::handler::handle_semantic_search))
        .route("/facets", get(features::search::facets::handle_facets))
        .route("/summary/{file}", get(features::summary::handler::handle_summary))
        .route("/browse", get(features::browse::handler::handle_browse))
        .route("/graph/entity/{name}", get(features::graph::handler::handle_entity_graph))
        .route("/graph/document/{id}", get(features::graph::handler::handle_document_graph))
        .route("/health", get(shared::health::handle_health))
        .route("/events", get(features::events::handler::handle_events))
        .with_state(state.clone())
        .fallback_service(
            ServeDir::new("static").fallback(ServeFile::new("static/index.html")),
        );

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
