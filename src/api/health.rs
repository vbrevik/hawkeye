use serde::Serialize;
use std::time::Duration;

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
