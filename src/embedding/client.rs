use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const CHUNK_CHARS: usize = 512 * 4; // ~512 tokens at ~4 chars/token
const STEP_CHARS: usize = CHUNK_CHARS / 2; // 50% overlap

#[derive(Debug, Serialize)]
struct EmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedData>,
}

#[derive(Debug, Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

pub struct EmbedClient {
    client: Client,
    base_url: String,
    model: String,
}

impl EmbedClient {
    pub fn new(base_url: &str, model: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
        })
    }

    /// Split text into overlapping character windows approximating 512 tokens each.
    /// Uses 50% overlap between consecutive chunks.
    pub fn make_chunks(text: &str) -> Vec<String> {
        if text.len() <= CHUNK_CHARS {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = text.len().min(start + CHUNK_CHARS);
            let end = text.floor_char_boundary(end);
            chunks.push(text[start..end].to_string());
            if end >= text.len() {
                break;
            }
            start += STEP_CHARS;
            start = text.ceil_char_boundary(start);
        }

        chunks
    }

    /// Embed all chunks of a document, returning one vector per chunk.
    pub async fn embed_chunks(
        &self,
        text: &str,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        let chunks = Self::make_chunks(text);

        let request = EmbedRequest {
            model: self.model.clone(),
            input: chunks,
        };

        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Embedding sidecar returned {}: {}", status, body).into());
        }

        let embed_response: EmbedResponse = response.json().await?;
        Ok(embed_response
            .data
            .into_iter()
            .map(|d| d.embedding)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::{Json, Router};
    use serde_json::json;

    #[test]
    fn test_make_chunks_short_text() {
        let text = "Hello, world!";
        let chunks = EmbedClient::make_chunks(text);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_make_chunks_exact_boundary() {
        let text = "a".repeat(CHUNK_CHARS);
        let chunks = EmbedClient::make_chunks(&text);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), CHUNK_CHARS);
    }

    #[test]
    fn test_make_chunks_long_text() {
        let text = "word ".repeat(1200); // ~6000 chars, well over CHUNK_CHARS (2048)
        let chunks = EmbedClient::make_chunks(&text);
        assert!(chunks.len() >= 2, "Expected at least 2 chunks, got {}", chunks.len());
        for chunk in &chunks {
            assert!(
                chunk.len() <= CHUNK_CHARS,
                "Chunk exceeds {} chars: {}",
                CHUNK_CHARS,
                chunk.len()
            );
        }
    }

    #[test]
    fn test_make_chunks_overlap() {
        let text = "a".repeat(CHUNK_CHARS + STEP_CHARS);
        let chunks = EmbedClient::make_chunks(&text);
        assert_eq!(chunks.len(), 2);
        // First chunk: 0..CHUNK_CHARS, Second chunk: STEP_CHARS..STEP_CHARS+CHUNK_CHARS
        // Overlap region: STEP_CHARS..CHUNK_CHARS
        let overlap_len = CHUNK_CHARS - STEP_CHARS;
        assert_eq!(
            chunks[0][STEP_CHARS..],
            chunks[1][..overlap_len]
        );
    }

    #[test]
    fn test_make_chunks_empty_text() {
        let chunks = EmbedClient::make_chunks("");
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "");
    }

    async fn mock_embed_handler(Json(body): Json<serde_json::Value>) -> Json<serde_json::Value> {
        let input = body["input"].as_array().unwrap();
        let data: Vec<serde_json::Value> = input
            .iter()
            .enumerate()
            .map(|(i, _)| {
                json!({
                    "embedding": vec![0.1_f32; 1024],
                    "index": i
                })
            })
            .collect();

        Json(json!({
            "data": data,
            "model": "BAAI/bge-m3",
            "usage": { "prompt_tokens": 10, "total_tokens": 10 }
        }))
    }

    #[tokio::test]
    async fn test_embed_chunks_with_mock_server() {
        let app = Router::new().route("/v1/embeddings", post(mock_embed_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = EmbedClient::new(&format!("http://{}", addr), "BAAI/bge-m3").unwrap();
        let result = client.embed_chunks("A short document about Rust.").await;

        let vectors = result.unwrap();
        assert_eq!(vectors.len(), 1, "Short text should produce 1 chunk");
        assert_eq!(vectors[0].len(), 1024, "bge-m3 produces 1024-dim vectors");
    }

    #[tokio::test]
    async fn test_embed_chunks_long_text_produces_multiple_vectors() {
        let app = Router::new().route("/v1/embeddings", post(mock_embed_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = EmbedClient::new(&format!("http://{}", addr), "BAAI/bge-m3").unwrap();
        let long_text = "word ".repeat(1200); // >2048 chars
        let result = client.embed_chunks(&long_text).await;

        let vectors = result.unwrap();
        assert!(
            vectors.len() >= 2,
            "Long text should produce multiple chunks, got {}",
            vectors.len()
        );
        for vec in &vectors {
            assert_eq!(vec.len(), 1024, "Each vector should be 1024-dim");
        }
    }

    async fn mock_500_handler() -> (StatusCode, String) {
        (StatusCode::INTERNAL_SERVER_ERROR, "model not loaded".to_string())
    }

    #[tokio::test]
    async fn test_embed_chunks_returns_error_on_500() {
        let app = Router::new().route("/v1/embeddings", post(mock_500_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = EmbedClient::new(&format!("http://{}", addr), "BAAI/bge-m3").unwrap();
        let result = client.embed_chunks("Some text").await;

        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("500"), "error should contain status code: {msg}");
        assert!(msg.contains("model not loaded"), "error should contain body: {msg}");
    }
}
