use crate::scanner::files::ScannedFile;
use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

const GROUP_NAME: &str = "hawkeye-workers";

#[derive(Debug, Clone, Serialize)]
pub struct QueueStatus {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<FileError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CancelResult {
    pub cancelled: usize,
    pub already_completed: usize,
    pub already_failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileError {
    pub file: String,
    pub error: String,
    pub attempts: usize,
}

#[derive(Clone)]
pub struct RedisQueue {
    pool: Pool,
    stream_key: String,
    total_key: String,
    completed_key: String,
    failed_key: String,
    errors_key: String,
}

impl RedisQueue {
    pub fn new(pool: Pool, workspace_id: Uuid) -> Self {
        let ws = workspace_id.to_string();
        Self {
            pool,
            stream_key: format!("hawkeye:jobs:{}", ws),
            total_key: format!("hawkeye:stats:{}:total", ws),
            completed_key: format!("hawkeye:stats:{}:completed", ws),
            failed_key: format!("hawkeye:stats:{}:failed", ws),
            errors_key: format!("hawkeye:errors:{}", ws),
        }
    }

    pub async fn ensure_group(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let result: Result<(), redis::RedisError> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(&self.stream_key)
            .arg(GROUP_NAME)
            .arg("$")
            .arg("MKSTREAM")
            .query_async(&mut *conn)
            .await;

        match result {
            Ok(()) => Ok(()),
            Err(e) if e.to_string().contains("BUSYGROUP") => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn publish_files(
        &self,
        files: &[ScannedFile],
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let count = files.len();

        for file in files {
            let path_str = file.path.display().to_string();
            redis::cmd("XADD")
                .arg(&self.stream_key)
                .arg("*")
                .arg("path")
                .arg(&path_str)
                .arg("hash")
                .arg(&file.hash)
                .arg("size")
                .arg(file.size)
                .query_async::<redis::Value>(&mut *conn)
                .await?;
        }

        let _: () = conn.incr(&self.total_key, count).await?;

        Ok(count)
    }

    pub async fn read_next(
        &self,
        consumer: &str,
    ) -> Result<Option<(String, ScannedFile)>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;

        let reply: redis::Value = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(GROUP_NAME)
            .arg(consumer)
            .arg("COUNT")
            .arg(1)
            .arg("BLOCK")
            .arg(2000)
            .arg("STREAMS")
            .arg(&self.stream_key)
            .arg(">")
            .query_async(&mut *conn)
            .await?;

        parse_stream_message(reply)
    }

    pub async fn ack_completed(
        &self,
        msg_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        redis::cmd("XACK")
            .arg(&self.stream_key)
            .arg(GROUP_NAME)
            .arg(msg_id)
            .query_async::<redis::Value>(&mut *conn)
            .await?;
        let _: () = conn.incr(&self.completed_key, 1).await?;
        Ok(())
    }

    pub async fn ack_failed(
        &self,
        msg_id: &str,
        file: &str,
        error: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        redis::cmd("XACK")
            .arg(&self.stream_key)
            .arg(GROUP_NAME)
            .arg(msg_id)
            .query_async::<redis::Value>(&mut *conn)
            .await?;
        let _: () = conn.incr(&self.failed_key, 1).await?;

        let err_json = serde_json::to_string(&FileError {
            file: file.to_string(),
            error: error.to_string(),
            attempts: 3,
        })?;
        let _: () = conn.rpush(&self.errors_key, &err_json).await?;
        let _: () = conn.ltrim(&self.errors_key, -100, -1).await?;

        Ok(())
    }

    pub async fn read_status(
        &self,
    ) -> Result<QueueStatus, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;

        let total: usize = conn.get(&self.total_key).await.unwrap_or(0);
        let completed: usize = conn.get(&self.completed_key).await.unwrap_or(0);
        let failed: usize = conn.get(&self.failed_key).await.unwrap_or(0);
        let in_progress = total.saturating_sub(completed).saturating_sub(failed);

        let error_strs: Vec<String> = conn.lrange(&self.errors_key, 0, -1).await.unwrap_or_default();
        let errors: Vec<FileError> = error_strs
            .iter()
            .filter_map(|s| serde_json::from_str(s).ok())
            .collect();

        Ok(QueueStatus {
            total,
            completed,
            failed,
            in_progress,
            errors,
        })
    }

    pub async fn cancel(&self) -> Result<CancelResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;

        let total: usize = conn.get(&self.total_key).await.unwrap_or(0);
        let completed: usize = conn.get(&self.completed_key).await.unwrap_or(0);
        let failed: usize = conn.get(&self.failed_key).await.unwrap_or(0);
        let cancelled = total.saturating_sub(completed).saturating_sub(failed);

        // Delete the stream to discard all unprocessed messages
        redis::cmd("DEL")
            .arg(&self.stream_key)
            .query_async::<redis::Value>(&mut *conn)
            .await?;

        // Adjust total so in_progress becomes 0
        let _: () = conn.set(&self.total_key, completed + failed).await?;

        // Recreate consumer group for future ingests
        drop(conn);
        self.ensure_group().await?;

        Ok(CancelResult {
            cancelled,
            already_completed: completed,
            already_failed: failed,
        })
    }

    #[allow(dead_code)] // used in integration tests
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        redis::cmd("DEL")
            .arg(&self.stream_key)
            .arg(&self.total_key)
            .arg(&self.completed_key)
            .arg(&self.failed_key)
            .arg(&self.errors_key)
            .query_async::<redis::Value>(&mut *conn)
            .await?;
        Ok(())
    }
}

fn parse_stream_message(
    value: redis::Value,
) -> Result<Option<(String, ScannedFile)>, Box<dyn std::error::Error + Send + Sync>> {
    // XREADGROUP returns Nil on timeout, or nested arrays:
    // [[stream_key, [[msg_id, [field, value, ...]]]]]
    match value {
        redis::Value::Nil => Ok(None),
        redis::Value::Array(streams) if streams.is_empty() => Ok(None),
        redis::Value::Array(streams) => {
            let stream = streams.into_iter().next().ok_or("empty stream reply")?;
            let stream_parts = match stream {
                redis::Value::Array(parts) => parts,
                _ => return Ok(None),
            };
            let messages = match stream_parts.into_iter().nth(1) {
                Some(redis::Value::Array(msgs)) => msgs,
                _ => return Ok(None),
            };
            let msg = match messages.into_iter().next() {
                Some(redis::Value::Array(m)) => m,
                _ => return Ok(None),
            };

            let mut msg_iter = msg.into_iter();
            let id = extract_string(msg_iter.next().ok_or("missing msg id")?)?;
            let fields = match msg_iter.next() {
                Some(redis::Value::Array(f)) => f,
                _ => return Err("missing fields".into()),
            };

            let mut path = String::new();
            let mut hash = String::new();
            let mut size: u64 = 0;

            let mut field_iter = fields.into_iter();
            while let Some(key) = field_iter.next() {
                let key_str = extract_string(key)?;
                let val = field_iter.next().ok_or("missing field value")?;
                match key_str.as_str() {
                    "path" => path = extract_string(val)?,
                    "hash" => hash = extract_string(val)?,
                    "size" => size = extract_string(val)?.parse()?,
                    _ => {}
                }
            }

            Ok(Some((
                id,
                ScannedFile {
                    path: PathBuf::from(path),
                    hash,
                    size,
                },
            )))
        }
        _ => Ok(None),
    }
}

fn extract_string(
    value: redis::Value,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    match value {
        redis::Value::BulkString(bytes) => Ok(String::from_utf8(bytes)?),
        redis::Value::SimpleString(s) => Ok(s),
        redis::Value::Int(i) => Ok(i.to_string()),
        _ => Err(format!("expected string, got {:?}", value).into()),
    }
}
