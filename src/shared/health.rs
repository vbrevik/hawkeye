use reqwest::Client;
use serde::Serialize;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Up,
    Degraded,
    Down,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: ServiceStatus,
    pub latency_ms: Option<u64>,
}

pub fn classify(elapsed: Duration) -> ServiceStatus {
    let ms = elapsed.as_millis();
    if ms < 500 {
        ServiceStatus::Up
    } else if ms < 2000 {
        ServiceStatus::Degraded
    } else {
        ServiceStatus::Down
    }
}

pub async fn tcp_check(addr: &str, name: &str) -> ServiceHealth {
    let start = Instant::now();
    match timeout(Duration::from_secs(2), TcpStream::connect(addr)).await {
        Ok(Ok(_)) => {
            let elapsed = start.elapsed();
            ServiceHealth {
                name: name.to_string(),
                status: classify(elapsed),
                latency_ms: Some(elapsed.as_millis() as u64),
            }
        }
        _ => ServiceHealth {
            name: name.to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}

pub async fn check_redis(addr: &str) -> ServiceHealth {
    let start = Instant::now();
    let result = timeout(Duration::from_secs(2), async {
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(b"*1\r\n$4\r\nPING\r\n").await?;
        let mut buf = [0u8; 7];
        AsyncReadExt::read(&mut stream, &mut buf).await?;
        Ok::<_, std::io::Error>(buf.starts_with(b"+PONG"))
    })
    .await;

    let elapsed = start.elapsed();
    match result {
        Ok(Ok(true)) => ServiceHealth {
            name: "redis".to_string(),
            status: classify(elapsed),
            latency_ms: Some(elapsed.as_millis() as u64),
        },
        _ => ServiceHealth {
            name: "redis".to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}

pub async fn http_check(client: &Client, url: &str, name: &str) -> ServiceHealth {
    let start = Instant::now();
    match timeout(Duration::from_secs(2), client.get(url).send()).await {
        Ok(Ok(res)) if res.status().is_success() => {
            let elapsed = start.elapsed();
            ServiceHealth {
                name: name.to_string(),
                status: classify(elapsed),
                latency_ms: Some(elapsed.as_millis() as u64),
            }
        }
        _ => ServiceHealth {
            name: name.to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}

use crate::shared::state::AppState;
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub services: Vec<ServiceHealth>,
    pub checked_at: String,
}

pub async fn handle_health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let cfg = &state.config;
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    // Strip scheme from redis_url to get "host:port"
    let redis_addr = cfg.redis_url
        .strip_prefix("redis://")
        .unwrap_or(&cfg.redis_url)
        .to_string();

    // Extract "host:port" from postgres_url (after the @ sign)
    let pg_addr = cfg.postgres_url
        .trim_start_matches("postgresql://")
        .trim_start_matches("postgres://")
        .split('@')
        .nth(1)
        .unwrap_or("localhost:5433")
        .to_string();

    // Milvus health port is 9091, not 19530
    let milvus_health_url = cfg.milvus_url.replace(":19530", ":9091");

    // Pre-build URL strings so they live long enough for tokio::join!
    let etcd_url = format!("{}/health", cfg.etcd_url);
    let minio_url = format!("{}/minio/health/live", cfg.minio_url);
    let milvus_url = format!("{}/healthz", milvus_health_url);
    let neo4j_url = cfg.neo4j_url.clone();
    let qwen3_url = format!("{}/v1/models", cfg.mlx_url);

    let (redis, postgres, etcd, minio, milvus, neo4j, qwen3) = tokio::join!(
        check_redis(&redis_addr),
        tcp_check(&pg_addr, "postgres"),
        http_check(&client, &etcd_url, "etcd"),
        http_check(&client, &minio_url, "minio"),
        http_check(&client, &milvus_url, "milvus"),
        http_check(&client, &neo4j_url, "neo4j"),
        http_check(&client, &qwen3_url, "qwen3"),
    );

    Json(HealthResponse {
        services: vec![redis, postgres, etcd, minio, milvus, neo4j, qwen3],
        checked_at: Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_up() {
        assert_eq!(classify(Duration::from_millis(100)), ServiceStatus::Up);
    }

    #[test]
    fn test_classify_degraded() {
        assert_eq!(classify(Duration::from_millis(700)), ServiceStatus::Degraded);
    }

    #[test]
    fn test_classify_boundary_up() {
        assert_eq!(classify(Duration::from_millis(499)), ServiceStatus::Up);
    }

    #[test]
    fn test_classify_boundary_degraded() {
        assert_eq!(classify(Duration::from_millis(500)), ServiceStatus::Degraded);
    }
}
