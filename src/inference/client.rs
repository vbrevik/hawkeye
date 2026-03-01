use crate::summary::Summary;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const SYSTEM_PROMPT: &str = r#"/no_think
You are a document summarizer. Given a markdown document, extract:
1. A 2-3 sentence TL;DR summary
2. A short title
3. Relevant tags (lowercase, max 5)
4. Named entities mentioned (people, tools, services, max 10)
5. High-level topics (max 3)

Respond ONLY with valid JSON in this exact format:
{
  "tldr": "...",
  "title": "...",
  "tags": ["..."],
  "entities": ["..."],
  "topics": ["..."]
}
No markdown fences. No explanation. Just the JSON object."#;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
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
}

pub struct InferenceClient {
    client: Client,
    base_url: String,
    model: String,
}

impl InferenceClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
        }
    }

    pub async fn summarize(
        &self,
        filename: &str,
        content: &str,
        source_hash: &str,
    ) -> Result<Summary, Box<dyn std::error::Error + Send + Sync>> {
        let word_count = content.split_whitespace().count() as u64;

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Filename: {}\n\n{}", filename, content),
                },
            ],
            temperature: 0.1,
        };

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await?;

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

        Ok(Summary {
            source: filename.to_string(),
            source_hash: source_hash.to_string(),
            created_at: Utc::now(),
            tldr: output.tldr,
            title: output.title,
            tags: output.tags,
            entities: output.entities,
            topics: output.topics,
            word_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::post, Json, Router};
    use serde_json::json;

    async fn mock_chat_handler() -> Json<serde_json::Value> {
        Json(json!({
            "choices": [{
                "message": {
                    "content": "{\"tldr\": \"A meeting about auth migration.\", \"title\": \"Auth Meeting\", \"tags\": [\"auth\", \"oauth2\"], \"entities\": [\"OAuth2\"], \"topics\": [\"authentication\"]}"
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

        let client = InferenceClient::new(&format!("http://{}", addr), "mock-model");
        let result = client
            .summarize("notes.md", "Meeting about OAuth2 migration", "sha256:abc")
            .await;

        let summary = result.unwrap();
        assert_eq!(summary.title, "Auth Meeting");
        assert_eq!(summary.tags, vec!["auth", "oauth2"]);
        assert_eq!(summary.source, "notes.md");
        assert_eq!(summary.source_hash, "sha256:abc");
        assert!(summary.word_count > 0);
    }
}
