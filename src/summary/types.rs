use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub source: String,
    pub source_hash: String,
    pub created_at: DateTime<Utc>,
    pub tldr: String,
    pub title: String,
    pub tags: Vec<String>,
    pub entities: Vec<String>,
    pub topics: Vec<String>,
    pub word_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_summary_roundtrip_json() {
        let summary = Summary {
            source: "test-file.md".to_string(),
            source_hash: "sha256:abc123".to_string(),
            created_at: Utc::now(),
            tldr: "A test summary.".to_string(),
            title: "Test File".to_string(),
            tags: vec!["test".to_string(), "example".to_string()],
            entities: vec!["TestEntity".to_string()],
            topics: vec!["testing".to_string()],
            word_count: 42,
        };

        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: Summary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, deserialized);
    }
}
