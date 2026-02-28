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
