use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

/// Read a summary from a .summary.json file
pub fn read_summary(path: &Path) -> Result<Summary, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let summary: Summary = serde_json::from_str(&content)?;
    Ok(summary)
}

/// Write a summary to a .summary.json file
pub fn write_summary(path: &Path, summary: &Summary) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(summary)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Given a markdown file path, return the expected summary file path
pub fn summary_path_for(md_path: &Path) -> std::path::PathBuf {
    let stem = md_path.file_stem().unwrap().to_str().unwrap();
    md_path.with_file_name(format!("{}.summary.json", stem))
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

    #[test]
    fn test_summary_file_read_write() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.summary.json");

        let summary = Summary {
            source: "notes.md".to_string(),
            source_hash: "sha256:def456".to_string(),
            created_at: Utc::now(),
            tldr: "Meeting about auth.".to_string(),
            title: "Auth Meeting".to_string(),
            tags: vec!["auth".to_string()],
            entities: vec!["OAuth2".to_string()],
            topics: vec!["authentication".to_string()],
            word_count: 120,
        };

        write_summary(&path, &summary).unwrap();
        let loaded = read_summary(&path).unwrap();
        assert_eq!(summary, loaded);
    }

    #[test]
    fn test_summary_path_for() {
        let md = Path::new("/docs/notes.md");
        let expected = Path::new("/docs/notes.summary.json");
        assert_eq!(summary_path_for(md), expected);
    }
}
