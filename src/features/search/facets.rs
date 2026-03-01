use crate::shared::state::AppState;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct FacetEntry {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct Facets {
    pub tags: Vec<FacetEntry>,
    pub topics: Vec<FacetEntry>,
    pub entities: Vec<FacetEntry>,
}

fn to_entries(pairs: Vec<(String, usize)>) -> Vec<FacetEntry> {
    pairs.into_iter().map(|(name, count)| FacetEntry { name, count }).collect()
}

pub async fn handle_facets(State(state): State<Arc<AppState>>) -> Json<Facets> {
    let indexer = state.indexer.lock().await;
    let tags = indexer.top_tags(20).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top tags");
        vec![]
    });
    let topics = indexer.top_topics(10).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top topics");
        vec![]
    });
    let entities = indexer.top_entities(15).unwrap_or_else(|e| {
        tracing::error!(error = %e, "failed to read top entities");
        vec![]
    });
    Json(Facets {
        tags: to_entries(tags),
        topics: to_entries(topics),
        entities: to_entries(entities),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::search::SearchIndexer;
    use crate::features::semantic::{EmbedClient, MilvusClient};
    use crate::features::summary::Summary;
    use crate::shared::config::AppConfig;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::get;
    use axum::Router;
    use chrono::Utc;
    use clap::Parser;
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use std::sync::atomic::AtomicBool;
    use tokio::sync::{watch, Mutex};
    use tower::ServiceExt;

    // --- to_entries ---

    #[test]
    fn test_to_entries_empty() {
        let entries = to_entries(vec![]);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_to_entries_single() {
        let entries = to_entries(vec![("rust".to_string(), 5)]);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "rust");
        assert_eq!(entries[0].count, 5);
    }

    #[test]
    fn test_to_entries_preserves_order() {
        let entries = to_entries(vec![
            ("auth".to_string(), 10),
            ("devops".to_string(), 3),
            ("rust".to_string(), 7),
        ]);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, "auth");
        assert_eq!(entries[0].count, 10);
        assert_eq!(entries[1].name, "devops");
        assert_eq!(entries[1].count, 3);
        assert_eq!(entries[2].name, "rust");
        assert_eq!(entries[2].count, 7);
    }

    // --- FacetEntry serialization ---

    #[test]
    fn test_facet_entry_json() {
        let entry = FacetEntry {
            name: "kubernetes".to_string(),
            count: 42,
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert_eq!(json["name"], "kubernetes");
        assert_eq!(json["count"], 42);
    }

    // --- Facets serialization ---

    #[test]
    fn test_facets_json_empty() {
        let facets = Facets {
            tags: vec![],
            topics: vec![],
            entities: vec![],
        };
        let json = serde_json::to_value(&facets).unwrap();
        assert!(json["tags"].as_array().unwrap().is_empty());
        assert!(json["topics"].as_array().unwrap().is_empty());
        assert!(json["entities"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_facets_json_populated() {
        let facets = Facets {
            tags: vec![
                FacetEntry { name: "auth".to_string(), count: 5 },
                FacetEntry { name: "rust".to_string(), count: 3 },
            ],
            topics: vec![FacetEntry { name: "security".to_string(), count: 2 }],
            entities: vec![FacetEntry { name: "OAuth2".to_string(), count: 4 }],
        };
        let json = serde_json::to_value(&facets).unwrap();
        assert_eq!(json["tags"].as_array().unwrap().len(), 2);
        assert_eq!(json["tags"][0]["name"], "auth");
        assert_eq!(json["tags"][0]["count"], 5);
        assert_eq!(json["topics"].as_array().unwrap().len(), 1);
        assert_eq!(json["entities"][0]["name"], "OAuth2");
    }

    // --- handle_facets handler ---

    fn make_summary(
        source: &str,
        title: &str,
        tags: Vec<&str>,
        entities: Vec<&str>,
        topics: Vec<&str>,
    ) -> Summary {
        Summary {
            source: source.to_string(),
            source_hash: "sha256:test".to_string(),
            created_at: Utc::now(),
            tldr: "test".to_string(),
            title: title.to_string(),
            tags: tags.into_iter().map(String::from).collect(),
            entities: entities.into_iter().map(String::from).collect(),
            topics: topics.into_iter().map(String::from).collect(),
            relationships: vec![],
            word_count: 100,
        }
    }

    fn create_test_state(indexer: Arc<Mutex<SearchIndexer>>) -> Arc<AppState> {
        let config = AppConfig::parse_from(["hawkeye"]);
        let pg_pool =
            PgPool::connect_lazy("postgresql://hawkeye:hawkeye@localhost:5433/hawkeye").unwrap();
        let redis_cfg = deadpool_redis::Config::from_url("redis://127.0.0.1:6379");
        let redis_pool = redis_cfg
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap();
        let embed = Arc::new(EmbedClient::new("http://127.0.0.1:1", "mock").unwrap());
        let milvus = Arc::new(MilvusClient::new("http://127.0.0.1:1").unwrap());
        let (shutdown_tx, _) = watch::channel(false);

        Arc::new(AppState {
            config,
            redis_pool,
            indexer,
            pg_pool,
            embed,
            milvus,
            neo4j: None,
            shutdown: shutdown_tx,
            shutdown_docker: AtomicBool::new(false),
        })
    }

    #[tokio::test]
    async fn test_handle_facets_empty_index() {
        let indexer = Arc::new(Mutex::new(SearchIndexer::new_in_memory().unwrap()));
        let state = create_test_state(indexer);
        let app = Router::new()
            .route("/facets", get(handle_facets))
            .with_state(state);

        let req = Request::builder()
            .uri("/facets")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["tags"].as_array().unwrap().is_empty());
        assert!(json["topics"].as_array().unwrap().is_empty());
        assert!(json["entities"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_handle_facets_with_data() {
        let indexer = Arc::new(Mutex::new(SearchIndexer::new_in_memory().unwrap()));

        {
            let mut idx = indexer.lock().await;
            idx.index_summary(&make_summary(
                "auth.md",
                "Auth Guide",
                vec!["auth", "security"],
                vec!["OAuth2"],
                vec!["authentication"],
            ))
            .unwrap();
            idx.index_summary(&make_summary(
                "deploy.md",
                "Deploy Guide",
                vec!["devops", "auth"],
                vec!["Kubernetes", "OAuth2"],
                vec!["deployment"],
            ))
            .unwrap();
            idx.index_summary(&make_summary(
                "rust.md",
                "Rust Guide",
                vec!["rust", "auth"],
                vec!["Rust"],
                vec!["programming"],
            ))
            .unwrap();
        }

        let state = create_test_state(indexer);
        let app = Router::new()
            .route("/facets", get(handle_facets))
            .with_state(state);

        let req = Request::builder()
            .uri("/facets")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let tags = json["tags"].as_array().unwrap();
        assert!(!tags.is_empty());
        assert_eq!(tags[0]["name"], "auth");
        assert_eq!(tags[0]["count"], 3);

        let entities = json["entities"].as_array().unwrap();
        assert!(!entities.is_empty());
        let oauth = entities.iter().find(|e| e["name"] == "OAuth2").unwrap();
        assert_eq!(oauth["count"], 2);

        let topics = json["topics"].as_array().unwrap();
        assert!(!topics.is_empty());
        for topic in topics {
            assert_eq!(topic["count"], 1);
        }
    }
}
