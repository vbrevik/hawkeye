use crate::shared::db::workspaces;
use crate::shared::state::AppState;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ValidatedApiKey {
    #[allow(dead_code)]
    pub key_id: Uuid,
    pub workspace_id: Uuid,
}

fn error_response(msg: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": msg }))).into_response()
}

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(t) => t.to_string(),
        None => return error_response("Missing or invalid Authorization header"),
    };

    if !token.starts_with("hke_") {
        return error_response("Invalid API key format");
    }

    let key_hash = super::keys::hash_api_key(&token);

    let api_key = match workspaces::find_api_key_by_hash(&state.pg_pool, &key_hash).await {
        Ok(Some(key)) => key,
        Ok(None) => return error_response("Invalid API key"),
        Err(_) => return error_response("Authentication service unavailable"),
    };

    if api_key.revoked_at.is_some() {
        return error_response("API key has been revoked");
    }

    if let Some(expires) = api_key.expires_at {
        if expires < Utc::now() {
            return error_response("API key has expired");
        }
    }

    request.extensions_mut().insert(ValidatedApiKey {
        key_id: api_key.id,
        workspace_id: api_key.workspace_id,
    });

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::queue::QueueManager;
    use crate::features::search::SearchIndexer;
    use crate::features::semantic::{EmbedClient, MilvusClient};
    use crate::shared::config::AppConfig;
    use crate::shared::db::workspaces;
    use crate::shared::inference::client::InferenceClient;
    use axum::body::Body;
    use axum::{middleware, routing::get, Extension, Router};
    use clap::Parser;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::{watch, Mutex};
    use tower::ServiceExt;

    const TEST_POSTGRES_URL: &str = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye";

    async fn ok_handler() -> StatusCode {
        StatusCode::OK
    }

    async fn echo_workspace(Extension(key): Extension<ValidatedApiKey>) -> String {
        key.workspace_id.to_string()
    }

    fn test_app(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/test", get(ok_handler))
            .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
            .with_state(state)
    }

    fn test_app_with_echo(state: Arc<AppState>) -> Router {
        Router::new()
            .route("/test", get(echo_workspace))
            .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
            .with_state(state)
    }

    /// Create an AppState with lazy connections (no actual DB calls).
    /// Suitable for tests that exercise early-return paths before any DB lookup.
    fn create_lazy_state() -> (Arc<AppState>, tempfile::TempDir) {
        let config = AppConfig::parse_from(["hawkeye"]);
        let pg_pool = PgPool::connect_lazy(TEST_POSTGRES_URL).unwrap();
        let redis_cfg = deadpool_redis::Config::from_url("redis://127.0.0.1:6379");
        let redis_pool = redis_cfg
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap();
        let index_dir = tempfile::tempdir().unwrap();
        let indexer = Arc::new(Mutex::new(
            SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
        ));
        let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock").unwrap());
        let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());
        let (shutdown_tx, _) = watch::channel(false);

        let inference = Arc::new(InferenceClient::new("http://127.0.0.1:1", "mock", 0.1).unwrap());
        let queue = Arc::new(QueueManager::new(1));
        let state = Arc::new(AppState {
            config,
            redis_pool,
            indexer,
            pg_pool,
            embed,
            milvus,
            neo4j: None,
            inference,
            queue,
            shutdown: shutdown_tx,
            shutdown_docker: AtomicBool::new(false),
        });
        (state, index_dir)
    }

    /// Create an AppState with a real Postgres connection + migrations.
    async fn create_db_state() -> (Arc<AppState>, PgPool, tempfile::TempDir) {
        let pg_pool = PgPool::connect(TEST_POSTGRES_URL).await.unwrap();
        sqlx::migrate!().run(&pg_pool).await.unwrap();
        let config = AppConfig::parse_from(["hawkeye"]);
        let redis_cfg = deadpool_redis::Config::from_url("redis://127.0.0.1:6379");
        let redis_pool = redis_cfg
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap();
        let index_dir = tempfile::tempdir().unwrap();
        let indexer = Arc::new(Mutex::new(
            SearchIndexer::new_in_dir(index_dir.path()).unwrap(),
        ));
        let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock").unwrap());
        let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());
        let inference = Arc::new(InferenceClient::new("http://127.0.0.1:1", "mock", 0.1).unwrap());
        let queue = Arc::new(QueueManager::new(1));
        let (shutdown_tx, _) = watch::channel(false);

        let pg_clone = pg_pool.clone();
        let state = Arc::new(AppState {
            config,
            redis_pool,
            indexer,
            pg_pool,
            embed,
            milvus,
            neo4j: None,
            inference,
            queue,
            shutdown: shutdown_tx,
            shutdown_docker: AtomicBool::new(false),
        });
        (state, pg_clone, index_dir)
    }

    async fn response_json(response: axum::response::Response) -> serde_json::Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    // --- error_response helper ---

    #[tokio::test]
    async fn test_error_response_format() {
        let resp = error_response("test error message");
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "test error message");
    }

    // --- Header / format validation (no DB hit) ---

    #[tokio::test]
    async fn test_missing_auth_header() {
        let (state, _dir) = create_lazy_state();
        let app = test_app(state);
        let req = Request::builder().uri("/test").body(Body::empty()).unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "Missing or invalid Authorization header");
    }

    #[tokio::test]
    async fn test_basic_auth_rejected() {
        let (state, _dir) = create_lazy_state();
        let app = test_app(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", "Basic dXNlcjpwYXNz")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "Missing or invalid Authorization header");
    }

    #[tokio::test]
    async fn test_non_hke_prefix_rejected() {
        let (state, _dir) = create_lazy_state();
        let app = test_app(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", "Bearer sk_not_a_valid_key_format")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "Invalid API key format");
    }

    // --- DB-dependent tests (require Docker Postgres on :5433) ---

    #[tokio::test]
    #[ignore = "requires Docker Postgres"]
    async fn test_unknown_key_rejected() {
        let (state, _pg, _dir) = create_db_state().await;
        let app = test_app(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", "Bearer hke_nonexistentkey00000000000000ab")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "Invalid API key");
    }

    #[tokio::test]
    #[ignore = "requires Docker Postgres"]
    async fn test_revoked_key_rejected() {
        let (state, pg, _dir) = create_db_state().await;

        let ws = workspaces::create_workspace(&pg, "test-revoked-mw").await.unwrap();
        let plaintext = super::super::keys::generate_api_key();
        let hash = super::super::keys::hash_api_key(&plaintext);
        let prefix = super::super::keys::key_prefix(&plaintext);
        let key = workspaces::create_api_key(&pg, ws.id, &hash, &prefix, "revoked-test")
            .await
            .unwrap();
        workspaces::revoke_api_key(&pg, key.id).await.unwrap();

        let app = test_app(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", format!("Bearer {}", plaintext))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "API key has been revoked");

        sqlx::query("DELETE FROM api_keys WHERE workspace_id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
        sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore = "requires Docker Postgres"]
    async fn test_expired_key_rejected() {
        let (state, pg, _dir) = create_db_state().await;

        let ws = workspaces::create_workspace(&pg, "test-expired-mw").await.unwrap();
        let plaintext = super::super::keys::generate_api_key();
        let hash = super::super::keys::hash_api_key(&plaintext);
        let prefix = super::super::keys::key_prefix(&plaintext);
        let key = workspaces::create_api_key(&pg, ws.id, &hash, &prefix, "expired-test")
            .await
            .unwrap();
        sqlx::query("UPDATE api_keys SET expires_at = now() - interval '1 day' WHERE id = $1")
            .bind(key.id)
            .execute(&pg)
            .await
            .unwrap();

        let app = test_app(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", format!("Bearer {}", plaintext))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let body = response_json(resp).await;
        assert_eq!(body["error"], "API key has expired");

        sqlx::query("DELETE FROM api_keys WHERE workspace_id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
        sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore = "requires Docker Postgres"]
    async fn test_valid_key_passes_through() {
        let (state, pg, _dir) = create_db_state().await;

        let ws = workspaces::create_workspace(&pg, "test-valid-mw").await.unwrap();
        let plaintext = super::super::keys::generate_api_key();
        let hash = super::super::keys::hash_api_key(&plaintext);
        let prefix = super::super::keys::key_prefix(&plaintext);
        workspaces::create_api_key(&pg, ws.id, &hash, &prefix, "valid-test")
            .await
            .unwrap();

        let app = test_app_with_echo(state);
        let req = Request::builder()
            .uri("/test")
            .header("authorization", format!("Bearer {}", plaintext))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body_bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body_str, ws.id.to_string());

        sqlx::query("DELETE FROM api_keys WHERE workspace_id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
        sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(ws.id)
            .execute(&pg)
            .await
            .unwrap();
    }
}
