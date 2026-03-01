use crate::summary::{Relationship, Summary};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const MAX_CONTENT_BYTES: usize = 100_000;

const SYSTEM_PROMPT_BASE: &str = r#"You are a document summarizer. Given a markdown document, extract:
1. A 2-3 sentence TL;DR summary
2. A short title
3. Relevant tags (lowercase, max 5)
4. Named entities mentioned (people, tools, services, max 10)
5. High-level topics (max 3)
6. Relationships between entities (max 10). Each relationship has a "from" entity, a "rel" type, a "to" entity, and a brief "context" string. Use rel types: owns, depends_on, manages, uses, created_by, part_of, related_to, located_in, member_of, produces.

Respond ONLY with valid JSON in this exact format:
{
  "tldr": "...",
  "title": "...",
  "tags": ["..."],
  "entities": ["..."],
  "topics": ["..."],
  "relationships": [{"from": "...", "rel": "...", "to": "...", "context": "..."}]
}
No markdown fences. No explanation. Just the JSON object."#;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct LlmOutput {
    tldr: String,
    title: String,
    tags: Vec<String>,
    entities: Vec<String>,
    topics: Vec<String>,
    #[serde(default)]
    relationships: Vec<Relationship>,
}

pub struct InferenceClient {
    client: Client,
    base_url: String,
    model: String,
    temperature: f32,
}

impl InferenceClient {
    pub fn new(base_url: &str, model: &str, temperature: f32) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
            temperature,
        })
    }

    pub async fn summarize(
        &self,
        filename: &str,
        content: &str,
        source_hash: &str,
    ) -> Result<Summary, Box<dyn std::error::Error + Send + Sync>> {
        let word_count = content.split_whitespace().count() as u64;

        let truncated = if content.len() > MAX_CONTENT_BYTES {
            tracing::warn!(
                file = filename,
                original_bytes = content.len(),
                limit = MAX_CONTENT_BYTES,
                "content truncated to fit model context window"
            );
            &content[..content.floor_char_boundary(MAX_CONTENT_BYTES)]
        } else {
            content
        };

        let system_prompt = if self.model.to_lowercase().contains("qwen3") {
            format!("/no_think\n{}", SYSTEM_PROMPT_BASE)
        } else {
            SYSTEM_PROMPT_BASE.to_string()
        };

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Filename: {}\n\n{}", filename, truncated),
                },
            ],
            temperature: self.temperature,
            max_tokens: 2048,
        };

        let start = Instant::now();
        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await?;
        let elapsed = start.elapsed();

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(format!("MLX sidecar returned {}: {}", status, body).into());
        }

        let chat_response: ChatResponse = response.json().await?;
        let raw_content = &chat_response.choices[0].message.content;

        // Strip markdown fences if LLM wraps output
        let cleaned = raw_content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let output: LlmOutput = serde_json::from_str(cleaned)?;

        tracing::info!(
            file = filename,
            elapsed_ms = elapsed.as_millis() as u64,
            "inference completed"
        );

        Ok(Summary {
            source: filename.to_string(),
            source_hash: source_hash.to_string(),
            created_at: Utc::now(),
            tldr: output.tldr,
            title: output.title,
            tags: output.tags,
            entities: output.entities,
            topics: output.topics,
            relationships: output.relationships,
            word_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::{Json, Router};
    use serde_json::json;
    use std::sync::{Arc, Mutex};

    async fn mock_chat_handler() -> Json<serde_json::Value> {
        Json(json!({
            "choices": [{
                "message": {
                    "content": "{\"tldr\": \"A meeting about auth migration.\", \"title\": \"Auth Meeting\", \"tags\": [\"auth\", \"oauth2\"], \"entities\": [\"OAuth2\"], \"topics\": [\"authentication\"], \"relationships\": [{\"from\": \"OAuth2\", \"rel\": \"related_to\", \"to\": \"authentication\", \"context\": \"auth migration\"}]}"
                }
            }]
        }))
    }

    #[tokio::test]
    async fn test_summarize_with_mock_server() {
        let app = Router::new().route("/v1/chat/completions", post(mock_chat_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = InferenceClient::new(&format!("http://{}", addr), "mock-model", 0.1).unwrap();
        let result = client
            .summarize("notes.md", "Meeting about OAuth2 migration", "sha256:abc")
            .await;

        let summary = result.unwrap();
        assert_eq!(summary.title, "Auth Meeting");
        assert_eq!(summary.tags, vec!["auth", "oauth2"]);
        assert_eq!(summary.source, "notes.md");
        assert_eq!(summary.source_hash, "sha256:abc");
        assert!(summary.word_count > 0);
        assert_eq!(summary.relationships.len(), 1);
        assert_eq!(summary.relationships[0].from, "OAuth2");
    }

    #[tokio::test]
    async fn test_summarize_missing_relationships_defaults_to_empty() {
        let app = Router::new().route("/v1/chat/completions", post(mock_chat_no_rels));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = InferenceClient::new(&format!("http://{}", addr), "mock-model", 0.1).unwrap();
        let result = client
            .summarize("notes.md", "Some content", "sha256:abc")
            .await;

        let summary = result.unwrap();
        assert!(summary.relationships.is_empty());
    }

    async fn mock_chat_no_rels() -> Json<serde_json::Value> {
        Json(json!({
            "choices": [{
                "message": {
                    "content": "{\"tldr\": \"Summary.\", \"title\": \"Title\", \"tags\": [], \"entities\": [], \"topics\": []}"
                }
            }]
        }))
    }

    async fn mock_500_handler() -> (StatusCode, String) {
        (StatusCode::INTERNAL_SERVER_ERROR, "out of memory".to_string())
    }

    #[tokio::test]
    async fn test_summarize_returns_error_on_500() {
        let app = Router::new().route("/v1/chat/completions", post(mock_500_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client = InferenceClient::new(&format!("http://{}", addr), "mock-model", 0.1).unwrap();
        let result = client
            .summarize("notes.md", "Some content", "sha256:abc")
            .await;

        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("500"), "error should contain status code: {msg}");
        assert!(msg.contains("out of memory"), "error should contain response body: {msg}");
    }

    async fn mock_capture_system_prompt(
        State(captured): State<Arc<Mutex<Vec<String>>>>,
        Json(body): Json<serde_json::Value>,
    ) -> Json<serde_json::Value> {
        if let Some(messages) = body["messages"].as_array() {
            for msg in messages {
                if msg["role"].as_str() == Some("system") {
                    if let Some(content) = msg["content"].as_str() {
                        captured.lock().unwrap().push(content.to_string());
                    }
                }
            }
        }
        Json(json!({
            "choices": [{
                "message": {
                    "content": "{\"tldr\": \"Summary.\", \"title\": \"Title\", \"tags\": [], \"entities\": [], \"topics\": []}"
                }
            }]
        }))
    }

    #[tokio::test]
    async fn test_no_think_prefix_only_for_qwen3() {
        let captured: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let state = captured.clone();
        let app = Router::new()
            .route("/v1/chat/completions", post(mock_capture_system_prompt))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let url = format!("http://{}", addr);

        let non_qwen3 = InferenceClient::new(&url, "mlx-community/Qwen2.5-7B-Instruct-4bit", 0.1).unwrap();
        non_qwen3.summarize("a.md", "hello", "sha256:a").await.unwrap();

        let qwen3 = InferenceClient::new(&url, "mlx-community/Qwen3-8B-Instruct-4bit", 0.1).unwrap();
        qwen3.summarize("b.md", "world", "sha256:b").await.unwrap();

        let prompts = captured.lock().unwrap();
        assert!(!prompts[0].starts_with("/no_think"), "non-Qwen3 model should not have /no_think prefix");
        assert!(prompts[1].starts_with("/no_think"), "Qwen3 model should have /no_think prefix");
    }

    #[tokio::test]
    async fn test_summarize_truncates_large_content() {
        let app = Router::new().route("/v1/chat/completions", post(mock_chat_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let large_content = "word ".repeat(MAX_CONTENT_BYTES);
        assert!(large_content.len() > MAX_CONTENT_BYTES);

        let client = InferenceClient::new(&format!("http://{}", addr), "mock-model", 0.1).unwrap();
        let result = client
            .summarize("big.md", &large_content, "sha256:big")
            .await;

        let summary = result.unwrap();
        assert_eq!(summary.source, "big.md");
        assert_eq!(summary.word_count, large_content.split_whitespace().count() as u64);
    }
}
