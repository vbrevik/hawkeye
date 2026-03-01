use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DocumentEvent {
    #[serde(rename = "document_done")]
    Done { file: String },
    #[serde(rename = "document_failed")]
    Failed { file: String, error: String },
}

pub fn events_channel(workspace_id: Uuid) -> String {
    format!("hawkeye:events:{}", workspace_id)
}

pub async fn publish_document_event(
    pool: &Pool,
    workspace_id: Uuid,
    event: &DocumentEvent,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = pool.get().await?;
    let channel = events_channel(workspace_id);
    let payload = serde_json::to_string(event)?;
    let _: () = conn.publish(&channel, &payload).await?;
    Ok(())
}
