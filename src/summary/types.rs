use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Owns,
    DependsOn,
    Manages,
    Uses,
    CreatedBy,
    PartOf,
    RelatedTo,
    LocatedIn,
    MemberOf,
    Produces,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    pub from: String,
    pub rel: RelationType,
    pub to: String,
    #[serde(default)]
    pub context: String,
}

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
    pub relationships: Vec<Relationship>,
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
            relationships: vec![Relationship {
                from: "TestEntity".to_string(),
                rel: RelationType::RelatedTo,
                to: "Testing".to_string(),
                context: "used for testing".to_string(),
            }],
            word_count: 42,
        };

        let json = serde_json::to_string(&summary).unwrap();
        let deserialized: Summary = serde_json::from_str(&json).unwrap();
        assert_eq!(summary, deserialized);
    }

    #[test]
    fn test_relationship_type_unknown_variant() {
        let json = r#"{"from": "A", "rel": "invented_by", "to": "B", "context": ""}
        "#;
        let rel: Relationship = serde_json::from_str(json).unwrap();
        assert_eq!(rel.rel, RelationType::Other);
    }

    #[test]
    fn test_relationship_missing_context_defaults() {
        let json = r#"{"from": "A", "rel": "uses", "to": "B"}"#;
        let rel: Relationship = serde_json::from_str(json).unwrap();
        assert_eq!(rel.rel, RelationType::Uses);
        assert_eq!(rel.context, "");
    }
}
