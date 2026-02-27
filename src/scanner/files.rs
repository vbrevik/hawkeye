use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub size: u64,
    pub hash: String,
}

pub struct ScanResult {
    pub to_process: Vec<ScannedFile>,
    pub skipped: usize,
}

/// Compute SHA-256 hash of file contents, prefixed with "sha256:"
pub fn hash_file(path: &Path) -> Result<String, std::io::Error> {
    let content = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(format!("sha256:{:x}", result))
}

/// Scan a directory for .md files, skip those with up-to-date summaries
pub fn scan_directory(dir: &Path) -> Result<ScanResult, Box<dyn std::error::Error>> {
    let pattern = dir.join("*.md");
    let pattern_str = pattern.to_str().ok_or("Invalid path")?;

    let mut files: Vec<ScannedFile> = Vec::new();
    let mut skipped = 0;

    for entry in glob::glob(pattern_str)? {
        let path = entry?;
        let metadata = std::fs::metadata(&path)?;
        let hash = hash_file(&path)?;

        let summary_path = crate::summary::store::summary_path_for(&path);
        if summary_path.exists() {
            if let Ok(existing) = crate::summary::store::read_summary(&summary_path) {
                if existing.source_hash == hash {
                    skipped += 1;
                    continue;
                }
            }
        }

        files.push(ScannedFile {
            path,
            size: metadata.len(),
            hash,
        });
    }

    // Sort by size ascending — small files first for fast progress
    files.sort_by_key(|f| f.size);

    Ok(ScanResult {
        to_process: files,
        skipped,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::store::{summary_path_for, write_summary, Summary};
    use chrono::Utc;

    #[test]
    fn test_hash_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        std::fs::write(&path, "hello world").unwrap();

        let hash = hash_file(&path).unwrap();
        assert!(hash.starts_with("sha256:"));
        assert!(hash.len() > 10);
    }

    #[test]
    fn test_scan_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        let result = scan_directory(dir.path()).unwrap();
        assert_eq!(result.to_process.len(), 0);
        assert_eq!(result.skipped, 0);
    }

    #[test]
    fn test_scan_finds_md_files_sorted_by_size() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("big.md"), "a".repeat(1000)).unwrap();
        std::fs::write(dir.path().join("small.md"), "tiny").unwrap();
        std::fs::write(dir.path().join("medium.md"), "a".repeat(100)).unwrap();
        std::fs::write(dir.path().join("not-md.txt"), "ignored").unwrap();

        let result = scan_directory(dir.path()).unwrap();
        assert_eq!(result.to_process.len(), 3);
        assert!(result.to_process[0].size <= result.to_process[1].size);
        assert!(result.to_process[1].size <= result.to_process[2].size);
    }

    #[test]
    fn test_scan_skips_up_to_date_summaries() {
        let dir = tempfile::tempdir().unwrap();
        let md_path = dir.path().join("notes.md");
        std::fs::write(&md_path, "some content").unwrap();

        let hash = hash_file(&md_path).unwrap();
        let summary = Summary {
            source: "notes.md".to_string(),
            source_hash: hash,
            created_at: Utc::now(),
            tldr: "Existing summary.".to_string(),
            title: "Notes".to_string(),
            tags: vec![],
            entities: vec![],
            topics: vec![],
            word_count: 2,
        };
        write_summary(&summary_path_for(&md_path), &summary).unwrap();

        let result = scan_directory(dir.path()).unwrap();
        assert_eq!(result.to_process.len(), 0);
        assert_eq!(result.skipped, 1);
    }

    #[test]
    fn test_scan_reprocesses_changed_files() {
        let dir = tempfile::tempdir().unwrap();
        let md_path = dir.path().join("notes.md");
        std::fs::write(&md_path, "original content").unwrap();

        let summary = Summary {
            source: "notes.md".to_string(),
            source_hash: "sha256:stale_hash".to_string(),
            created_at: Utc::now(),
            tldr: "Old summary.".to_string(),
            title: "Notes".to_string(),
            tags: vec![],
            entities: vec![],
            topics: vec![],
            word_count: 2,
        };
        write_summary(&summary_path_for(&md_path), &summary).unwrap();

        let result = scan_directory(dir.path()).unwrap();
        assert_eq!(result.to_process.len(), 1);
        assert_eq!(result.skipped, 0);
    }
}
