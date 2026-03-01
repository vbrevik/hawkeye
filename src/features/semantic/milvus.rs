use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

const COLLECTION_NAME: &str = "doc_chunks";
const VECTOR_DIM: usize = 1024;

pub struct MilvusClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub doc_id: String,
    pub chunk_index: i32,
    pub distance: f32,
}

#[derive(Debug, Deserialize)]
struct MilvusResponse {
    code: i32,
    #[serde(default)]
    message: String,
    #[serde(default)]
    data: serde_json::Value,
}

impl MilvusClient {
    pub fn new(base_url: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    /// Ensure the `doc_chunks` collection exists in Milvus, creating it if needed.
    /// Also loads the collection into memory (idempotent).
    pub async fn ensure_collection(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let resp: MilvusResponse = self
            .client
            .post(format!("{}/v2/vectordb/collections/has", self.base_url))
            .json(&json!({"collectionName": COLLECTION_NAME}))
            .send()
            .await?
            .json()
            .await?;

        if resp.code != 0 {
            return Err(format!("Milvus has_collection error: {}", resp.message).into());
        }

        let has = resp
            .data
            .get("has")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !has {
            let create_body = json!({
                "collectionName": COLLECTION_NAME,
                "schema": {
                    "autoId": true,
                    "enableDynamicField": false,
                    "fields": [
                        {
                            "fieldName": "id",
                            "dataType": "Int64",
                            "isPrimary": true
                        },
                        {
                            "fieldName": "doc_id",
                            "dataType": "VarChar",
                            "elementTypeParams": {"max_length": "128"}
                        },
                        {
                            "fieldName": "chunk_index",
                            "dataType": "Int32"
                        },
                        {
                            "fieldName": "workspace_id",
                            "dataType": "VarChar",
                            "elementTypeParams": {"max_length": "128"}
                        },
                        {
                            "fieldName": "vector",
                            "dataType": "FloatVector",
                            "elementTypeParams": {"dim": VECTOR_DIM.to_string()}
                        }
                    ]
                },
                "indexParams": [
                    {
                        "fieldName": "vector",
                        "metricType": "L2",
                        "indexName": "vector_idx",
                        "params": {"index_type": "AUTOINDEX"}
                    }
                ]
            });

            let resp: MilvusResponse = self
                .client
                .post(format!(
                    "{}/v2/vectordb/collections/create",
                    self.base_url
                ))
                .json(&create_body)
                .send()
                .await?
                .json()
                .await?;

            if resp.code != 0 {
                return Err(
                    format!("Milvus create_collection error: {}", resp.message).into(),
                );
            }
        }

        // Load collection into memory (idempotent — safe even if already loaded)
        let resp: MilvusResponse = self
            .client
            .post(format!("{}/v2/vectordb/collections/load", self.base_url))
            .json(&json!({"collectionName": COLLECTION_NAME}))
            .send()
            .await?
            .json()
            .await?;

        if resp.code != 0 {
            return Err(format!("Milvus load_collection error: {}", resp.message).into());
        }

        Ok(())
    }

    /// Delete all chunks for a document (used before re-inserting on re-ingest).
    pub async fn delete_by_doc_id(
        &self,
        doc_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let body = json!({
            "collectionName": COLLECTION_NAME,
            "filter": format!("doc_id == \"{}\"", doc_id)
        });

        let resp: MilvusResponse = self
            .client
            .post(format!("{}/v2/vectordb/entities/delete", self.base_url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if resp.code != 0 {
            return Err(format!("Milvus delete error: {}", resp.message).into());
        }

        Ok(())
    }

    /// Insert embedding vectors for a document's chunks.
    pub async fn insert_chunks(
        &self,
        doc_id: &str,
        workspace_id: &str,
        vectors: &[Vec<f32>],
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        if vectors.is_empty() {
            return Ok(0);
        }

        let data: Vec<serde_json::Value> = vectors
            .iter()
            .enumerate()
            .map(|(i, vec)| {
                json!({
                    "doc_id": doc_id,
                    "chunk_index": i as i32,
                    "workspace_id": workspace_id,
                    "vector": vec
                })
            })
            .collect();

        let body = json!({
            "collectionName": COLLECTION_NAME,
            "data": data
        });

        let resp: MilvusResponse = self
            .client
            .post(format!("{}/v2/vectordb/entities/insert", self.base_url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if resp.code != 0 {
            return Err(format!("Milvus insert error: {}", resp.message).into());
        }

        let count = resp
            .data
            .get("insertCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;
        Ok(count)
    }

    /// Search for similar vectors, returning deduplicated doc_ids with best (lowest) distances.
    pub async fn search_similar(
        &self,
        query_vector: &[f32],
        workspace_id: &str,
        limit: usize,
    ) -> Result<Vec<SearchHit>, Box<dyn std::error::Error + Send + Sync>> {
        let fetch_limit = limit * 3;

        let body = json!({
            "collectionName": COLLECTION_NAME,
            "data": [query_vector],
            "annsField": "vector",
            "limit": fetch_limit,
            "outputFields": ["doc_id", "chunk_index", "workspace_id"],
            "filter": format!("workspace_id == \"{}\"", workspace_id)
        });

        let resp: MilvusResponse = self
            .client
            .post(format!("{}/v2/vectordb/entities/search", self.base_url))
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        if resp.code != 0 {
            return Err(format!("Milvus search error: {}", resp.message).into());
        }

        let results = resp.data.as_array().cloned().unwrap_or_default();

        let mut best: HashMap<String, SearchHit> = HashMap::new();
        for item in results {
            let doc_id = item
                .get("doc_id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let chunk_index = item
                .get("chunk_index")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;
            let distance = item
                .get("distance")
                .and_then(|v| v.as_f64())
                .unwrap_or(f64::MAX) as f32;

            if doc_id.is_empty() {
                continue;
            }

            let hit = SearchHit {
                doc_id: doc_id.clone(),
                chunk_index,
                distance,
            };
            match best.get(&doc_id) {
                Some(existing) if existing.distance <= distance => {}
                _ => {
                    best.insert(doc_id, hit);
                }
            }
        }

        let mut hits: Vec<SearchHit> = best.into_values().collect();
        hits.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(limit);

        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::post;
    use axum::{Json, Router};
    use serde_json::json;

    async fn mock_has_exists() -> Json<serde_json::Value> {
        Json(json!({"code": 0, "data": {"has": true}}))
    }

    async fn mock_load_ok() -> Json<serde_json::Value> {
        Json(json!({"code": 0, "data": {}}))
    }

    async fn mock_insert_ok(Json(body): Json<serde_json::Value>) -> Json<serde_json::Value> {
        let count = body
            .get("data")
            .and_then(|d| d.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        Json(json!({"code": 0, "data": {"insertCount": count}}))
    }

    async fn mock_delete_ok() -> Json<serde_json::Value> {
        Json(json!({"code": 0, "data": {}}))
    }

    async fn mock_search_with_duplicates() -> Json<serde_json::Value> {
        Json(json!({
            "code": 0,
            "data": [
                {"id": 1, "distance": 0.5, "doc_id": "aaa", "chunk_index": 0, "workspace_id": "ws"},
                {"id": 2, "distance": 0.2, "doc_id": "aaa", "chunk_index": 1, "workspace_id": "ws"},
                {"id": 3, "distance": 0.3, "doc_id": "bbb", "chunk_index": 0, "workspace_id": "ws"},
                {"id": 4, "distance": 0.1, "doc_id": "ccc", "chunk_index": 0, "workspace_id": "ws"},
                {"id": 5, "distance": 0.8, "doc_id": "bbb", "chunk_index": 1, "workspace_id": "ws"}
            ]
        }))
    }

    #[tokio::test]
    async fn test_ensure_collection_already_exists() {
        let app = Router::new()
            .route("/v2/vectordb/collections/has", post(mock_has_exists))
            .route("/v2/vectordb/collections/load", post(mock_load_ok));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = MilvusClient::new(&format!("http://{}", addr)).unwrap();
        let result = client.ensure_collection().await;
        assert!(result.is_ok(), "ensure_collection should succeed when collection exists");
    }

    #[tokio::test]
    async fn test_insert_chunks_with_mock() {
        let app = Router::new()
            .route("/v2/vectordb/entities/insert", post(mock_insert_ok));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = MilvusClient::new(&format!("http://{}", addr)).unwrap();
        let vectors = vec![vec![0.1_f32; 1024], vec![0.2_f32; 1024]];
        let count = client
            .insert_chunks("doc-abc", "ws-123", &vectors)
            .await
            .unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_insert_empty_vectors() {
        let client = MilvusClient::new("http://127.0.0.1:1").unwrap();
        let count = client.insert_chunks("doc", "ws", &[]).await.unwrap();
        assert_eq!(count, 0, "inserting empty vectors should return 0");
    }

    #[tokio::test]
    async fn test_search_deduplicates_by_doc_id() {
        let app = Router::new()
            .route("/v2/vectordb/entities/search", post(mock_search_with_duplicates));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = MilvusClient::new(&format!("http://{}", addr)).unwrap();
        let query = vec![0.1_f32; 1024];
        let hits = client.search_similar(&query, "ws", 10).await.unwrap();

        assert_eq!(hits.len(), 3, "Should deduplicate to 3 unique doc_ids");
        assert_eq!(hits[0].doc_id, "ccc", "ccc has lowest distance 0.1");
        assert_eq!(hits[1].doc_id, "aaa", "aaa best chunk has distance 0.2");
        assert_eq!(hits[2].doc_id, "bbb", "bbb best chunk has distance 0.3");
    }

    #[tokio::test]
    async fn test_delete_by_doc_id() {
        let app = Router::new()
            .route("/v2/vectordb/entities/delete", post(mock_delete_ok));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = MilvusClient::new(&format!("http://{}", addr)).unwrap();
        let result = client.delete_by_doc_id("doc-abc").await;
        assert!(result.is_ok());
    }
}
