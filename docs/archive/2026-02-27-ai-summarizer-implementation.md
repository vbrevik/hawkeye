# Eagle3 AI Summarizer — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a local AI-powered markdown summarizer with Rust backend, MLX inference, queue processing for 10K files, and full-text search.

**Architecture:** Rust Axum server manages a queue of markdown files, sends each to a Python mlx-lm sidecar for summarization, writes `.summary.json` files, and indexes them in Tantivy for search.

**Tech Stack:** Rust (Axum, Tokio, Tantivy, Reqwest, Serde, SHA2, Clap, Tracing), Python (mlx-lm serve)

---

## Task 1: Project Scaffold & Core Types

### Prompt Contract

**GOAL:** Cargo project compiles with all module stubs and the `Summary` data type serializes/deserializes correctly. Success = `cargo build` passes and a unit test round-trips a Summary through JSON.

**CONSTRAINTS:**
- Axum 0.8.x, Tokio 1.x, Tantivy 0.25.x, Reqwest 0.12.x
- Path syntax uses `/{param}` (Axum 0.8 style), not `/:param`
- No `#[async_trait]` — Axum 0.8 uses native async traits
- Edition 2021

**FORMAT:**
- `Cargo.toml` at project root
- `src/main.rs` — minimal main with placeholder
- `src/config.rs` — `AppConfig` struct with Clap
- `src/summary/mod.rs` + `src/summary/store.rs` — `Summary` struct + JSON read/write
- All other modules as empty `mod.rs` stubs
- Test in `src/summary/store.rs`

**FAILURE CONDITIONS:**
- `cargo build` fails
- Missing any module from the design (api, queue, inference, search, summary, scanner)
- Summary struct fields don't match design JSON format exactly
- No unit test for Summary serialization

### Step 1: Create Cargo.toml

**Files:** Create: `Cargo.toml`

```toml
[package]
name = "eagle3"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tantivy = "0.25"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
glob = "0.3"
```

### Step 2: Create module stubs

**Files:** Create all source files with minimal module declarations.

`src/main.rs`:
```rust
mod api;
mod config;
mod inference;
mod queue;
mod scanner;
mod search;
mod summary;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("eagle3=info")
        .init();
    tracing::info!("eagle3 starting");
}
```

`src/config.rs`:
```rust
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "eagle3", about = "Local AI-powered markdown summarizer")]
pub struct AppConfig {
    /// Port for the Axum server
    #[arg(long, default_value = "3000")]
    pub port: u16,

    /// MLX inference server URL
    #[arg(long, default_value = "http://localhost:8100")]
    pub mlx_url: String,

    /// Number of concurrent workers
    #[arg(long, default_value = "4")]
    pub workers: usize,

    /// Path to Tantivy index directory
    #[arg(long, default_value = ".eagle3_index")]
    pub index_path: String,
}
```

`src/api/mod.rs`:
```rust
pub mod ingest;
pub mod search;
pub mod status;
pub mod summary;
```

`src/api/ingest.rs`, `src/api/search.rs`, `src/api/status.rs`, `src/api/summary.rs`:
```rust
// Placeholder
```

`src/queue/mod.rs`:
```rust
pub mod manager;
pub mod worker;
```

`src/queue/manager.rs`, `src/queue/worker.rs`:
```rust
// Placeholder
```

`src/inference/mod.rs`:
```rust
pub mod client;
```

`src/inference/client.rs`:
```rust
// Placeholder
```

`src/search/mod.rs`:
```rust
pub mod indexer;
```

`src/search/indexer.rs`:
```rust
// Placeholder
```

`src/summary/mod.rs`:
```rust
pub mod store;
```

`src/scanner/mod.rs`:
```rust
pub mod files;
```

`src/scanner/files.rs`:
```rust
// Placeholder
```

### Step 3: Write Summary type and test

**Files:** Create: `src/summary/store.rs`

```rust
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
```

### Step 4: Verify build and tests

Run: `cargo build`
Expected: Compiles with no errors.

Run: `cargo test`
Expected: 3 tests pass.

### Step 5: Commit

```bash
git add -A
git commit -m "feat: project scaffold with module stubs and Summary type"
```

---

## Task 2: File Scanner with Hash & Skip Logic

### Prompt Contract

**GOAL:** Scanner discovers all `.md` files in a directory, computes SHA-256 hashes, and skips files with up-to-date summaries. Success = unit test confirms skip logic works correctly for changed vs unchanged files.

**CONSTRAINTS:**
- Use `glob` crate for file discovery, not walkdir
- SHA-256 via `sha2` crate
- Sort results by file size ascending (small first)
- No async needed — scanner runs once at ingest time

**FORMAT:**
- Implementation in `src/scanner/files.rs`
- Types: `ScannedFile { path, size, hash }` and `ScanResult { to_process, skipped }`
- Tests inline in same file

**FAILURE CONDITIONS:**
- Scanner panics on empty directory
- Skip logic doesn't check both file existence AND hash match
- Files not sorted by size ascending
- No test for the skip-when-hash-matches case

### Step 1: Write failing tests

**Files:** Modify: `src/scanner/files.rs`

```rust
use sha2::{Sha256, Digest};
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
    use crate::summary::store::{Summary, write_summary, summary_path_for};
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
```

### Step 2: Run tests

Run: `cargo test scanner`
Expected: All 5 tests pass.

### Step 3: Commit

```bash
git add src/scanner/files.rs
git commit -m "feat: file scanner with SHA-256 hashing and skip logic"
```

---

## Task 3: Inference Client

### Prompt Contract

**GOAL:** HTTP client sends markdown content to mlx-lm's OpenAI-compatible API and parses the response into a Summary struct. Success = unit test with a mock server returns a valid Summary.

**CONSTRAINTS:**
- Use `reqwest` async client
- Single prompt that extracts both TL;DR and structured JSON
- Timeout of 120 seconds per request (large files may be slow at 40 tok/s)
- Parse LLM JSON output, handle malformed responses gracefully

**FORMAT:**
- `src/inference/client.rs` — `InferenceClient` struct with `summarize(&self, filename: &str, content: &str) -> Result<Summary>`
- Prompt template embedded as a const string
- Test with mock Axum server

**FAILURE CONDITIONS:**
- No timeout configured on reqwest client
- Prompt doesn't request JSON output format
- Panics on malformed LLM response instead of returning Err
- No test

### Step 1: Implement inference client

**Files:** Modify: `src/inference/client.rs`

```rust
use crate::summary::store::Summary;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const SYSTEM_PROMPT: &str = r#"You are a document summarizer. Given a markdown document, extract:
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
}

impl InferenceClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
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
            model: "default".to_string(),
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

        let client = InferenceClient::new(&format!("http://{}", addr));
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
```

### Step 2: Run tests

Run: `cargo test inference`
Expected: 1 test passes.

### Step 3: Commit

```bash
git add src/inference/client.rs
git commit -m "feat: inference client with OpenAI-compatible API and mock test"
```

---

## Task 4: Queue Manager

### Prompt Contract

**GOAL:** Queue accepts scanned files, dispatches to N concurrent workers, tracks progress (total/completed/failed/in_progress), and retries failures up to 3 times. Success = integration test processes 10 files through mock inference with correct status counts.

**CONSTRAINTS:**
- Use `tokio::sync::Semaphore` for concurrency limiting
- Shared state via `Arc<Mutex<QueueState>>` for status tracking
- Workers are spawned tokio tasks, not threads
- Max 3 retries per file

**FORMAT:**
- `src/queue/manager.rs` — `QueueManager` struct, `QueueState`, `QueueStatus`
- `src/queue/worker.rs` — `process_file` function
- Tests inline

**FAILURE CONDITIONS:**
- Unbounded concurrency (no semaphore)
- Status counts don't add up (total != completed + failed + in_progress + pending)
- Retry count not enforced
- Panics if inference returns error

### Step 1: Implement queue manager

**Files:** Modify: `src/queue/manager.rs`

```rust
use crate::inference::client::InferenceClient;
use crate::scanner::files::ScannedFile;
use crate::search::indexer::SearchIndexer;
use crate::summary::store::{self, Summary};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

#[derive(Debug, Clone, Serialize)]
pub struct QueueStatus {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<FileError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileError {
    pub file: String,
    pub error: String,
    pub attempts: usize,
}

pub struct QueueState {
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub in_progress: usize,
    pub errors: Vec<FileError>,
}

impl QueueState {
    pub fn status(&self) -> QueueStatus {
        QueueStatus {
            total: self.total,
            completed: self.completed,
            failed: self.failed,
            in_progress: self.in_progress,
            errors: self.errors.clone(),
        }
    }
}

pub struct QueueManager {
    pub state: Arc<Mutex<QueueState>>,
    concurrency: usize,
}

impl QueueManager {
    pub fn new(concurrency: usize) -> Self {
        Self {
            state: Arc::new(Mutex::new(QueueState {
                total: 0,
                completed: 0,
                failed: 0,
                in_progress: 0,
                errors: Vec::new(),
            })),
            concurrency,
        }
    }

    pub async fn process_files(
        &self,
        files: Vec<ScannedFile>,
        client: Arc<InferenceClient>,
        indexer: Arc<Mutex<SearchIndexer>>,
    ) {
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let total = files.len();

        {
            let mut state = self.state.lock().await;
            state.total = total;
        }

        let mut handles = Vec::new();

        for file in files {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = client.clone();
            let state = self.state.clone();
            let indexer = indexer.clone();

            let handle = tokio::spawn(async move {
                {
                    let mut s = state.lock().await;
                    s.in_progress += 1;
                }

                let result =
                    super::worker::process_file(&file, &client, &indexer).await;

                {
                    let mut s = state.lock().await;
                    s.in_progress -= 1;
                    match result {
                        Ok(_) => s.completed += 1,
                        Err(e) => {
                            s.failed += 1;
                            s.errors.push(FileError {
                                file: file.path.display().to_string(),
                                error: e.to_string(),
                                attempts: 3,
                            });
                        }
                    }
                }

                drop(permit);
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}
```

### Step 2: Implement worker with retry

**Files:** Modify: `src/queue/worker.rs`

```rust
use crate::inference::client::InferenceClient;
use crate::scanner::files::ScannedFile;
use crate::search::indexer::SearchIndexer;
use crate::summary::store;
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_RETRIES: usize = 3;

pub async fn process_file(
    file: &ScannedFile,
    client: &InferenceClient,
    indexer: &Arc<Mutex<SearchIndexer>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = tokio::fs::read_to_string(&file.path).await?;
    let filename = file.path.file_name().unwrap().to_str().unwrap();

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match client.summarize(filename, &content, &file.hash).await {
            Ok(summary) => {
                let summary_path = store::summary_path_for(&file.path);
                store::write_summary(&summary_path, &summary)?;

                let mut idx = indexer.lock().await;
                idx.index_summary(&summary)?;

                tracing::info!(file = %file.path.display(), "summarized");
                return Ok(());
            }
            Err(e) => {
                tracing::warn!(
                    file = %file.path.display(),
                    attempt,
                    error = %e,
                    "inference failed, retrying"
                );
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "Unknown error".into()))
}
```

### Step 3: Run tests

Run: `cargo build`
Expected: Compiles (tests depend on SearchIndexer which comes in Task 5).

### Step 4: Commit

```bash
git add src/queue/
git commit -m "feat: queue manager with semaphore concurrency and retry logic"
```

---

## Task 5: Tantivy Search Indexer

### Prompt Contract

**GOAL:** Tantivy index supports adding summaries and querying by text, tags, and field-specific search. Success = unit test indexes 3 summaries and finds correct results for text query and tag filter.

**CONSTRAINTS:**
- Tantivy 0.25.x API (use `TantivyDocument`, not deprecated `Document`)
- Schema: `source_path` (STRING|STORED), `tldr` (TEXT|STORED), `title` (TEXT|STORED), `tags` (TEXT|STORED), `entities` (TEXT|STORED), `topics` (TEXT|STORED)
- In-memory index for tests, directory-based for production
- Commit after each document (auto-commit on add)

**FORMAT:**
- `src/search/indexer.rs` — `SearchIndexer` struct with `new`, `index_summary`, `search`, `reindex_from_dir`
- Tests inline

**FAILURE CONDITIONS:**
- Uses deprecated Tantivy API
- No STORED on fields that need to be returned in search results
- Search panics on empty index
- No test for field-specific search (e.g. `tags:auth`)

### Step 1: Implement search indexer

**Files:** Modify: `src/search/indexer.rs`

```rust
use crate::summary::store::Summary;
use serde::Serialize;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy, TantivyDocument};

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub file: String,
    pub tldr: String,
    pub title: String,
    pub tags: String,
    pub entities: String,
    pub score: f32,
}

pub struct SearchIndexer {
    index: Index,
    writer: IndexWriter,
    schema: Schema,
    source_path: Field,
    tldr: Field,
    title: Field,
    tags: Field,
    entities: Field,
    topics: Field,
}

impl SearchIndexer {
    pub fn new_in_dir(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let (schema, fields) = Self::build_schema();
        std::fs::create_dir_all(path)?;
        let index = Index::create_in_dir(path, schema.clone())
            .or_else(|_| Index::open_in_dir(path))?;
        let writer = index.writer(50_000_000)?;
        Ok(Self {
            index,
            writer,
            schema,
            source_path: fields.0,
            tldr: fields.1,
            title: fields.2,
            tags: fields.3,
            entities: fields.4,
            topics: fields.5,
        })
    }

    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let (schema, fields) = Self::build_schema();
        let index = Index::create_in_ram(schema.clone());
        let writer = index.writer(50_000_000)?;
        Ok(Self {
            index,
            writer,
            schema,
            source_path: fields.0,
            tldr: fields.1,
            title: fields.2,
            tags: fields.3,
            entities: fields.4,
            topics: fields.5,
        })
    }

    fn build_schema() -> (Schema, (Field, Field, Field, Field, Field, Field)) {
        let mut builder = Schema::builder();
        let source_path = builder.add_text_field("source_path", STRING | STORED);
        let tldr = builder.add_text_field("tldr", TEXT | STORED);
        let title = builder.add_text_field("title", TEXT | STORED);
        let tags = builder.add_text_field("tags", TEXT | STORED);
        let entities = builder.add_text_field("entities", TEXT | STORED);
        let topics = builder.add_text_field("topics", TEXT | STORED);
        (builder.build(), (source_path, tldr, title, tags, entities, topics))
    }

    pub fn index_summary(&mut self, summary: &Summary) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.add_document(doc!(
            self.source_path => summary.source.clone(),
            self.tldr => summary.tldr.clone(),
            self.title => summary.title.clone(),
            self.tags => summary.tags.join(" "),
            self.entities => summary.entities.join(" "),
            self.topics => summary.topics.join(" ")
        ))?;
        self.writer.commit()?;
        Ok(())
    }

    pub fn search(
        &self,
        query_str: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.tldr, self.title, self.tags, self.entities, self.topics],
        );
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            let get_field = |field: Field| -> String {
                doc.get_first(field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };

            results.push(SearchResult {
                file: get_field(self.source_path),
                tldr: get_field(self.tldr),
                title: get_field(self.title),
                tags: get_field(self.tags),
                entities: get_field(self.entities),
                score,
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::summary::store::Summary;
    use chrono::Utc;

    fn make_summary(source: &str, tldr: &str, title: &str, tags: Vec<&str>) -> Summary {
        Summary {
            source: source.to_string(),
            source_hash: "sha256:test".to_string(),
            created_at: Utc::now(),
            tldr: tldr.to_string(),
            title: title.to_string(),
            tags: tags.into_iter().map(String::from).collect(),
            entities: vec![],
            topics: vec![],
            word_count: 100,
        }
    }

    #[test]
    fn test_index_and_search() {
        let mut indexer = SearchIndexer::new_in_memory().unwrap();

        indexer
            .index_summary(&make_summary(
                "auth.md",
                "Guide to setting up OAuth2 authentication",
                "OAuth2 Setup Guide",
                vec!["auth", "oauth2"],
            ))
            .unwrap();

        indexer
            .index_summary(&make_summary(
                "deploy.md",
                "Steps for deploying to Kubernetes cluster",
                "K8s Deployment",
                vec!["kubernetes", "devops"],
            ))
            .unwrap();

        indexer
            .index_summary(&make_summary(
                "rust.md",
                "Introduction to Rust ownership and borrowing",
                "Rust Ownership",
                vec!["rust", "programming"],
            ))
            .unwrap();

        let results = indexer.search("OAuth2", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].file, "auth.md");

        let results = indexer.search("kubernetes", 10).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].file, "deploy.md");
    }

    #[test]
    fn test_search_empty_index() {
        let indexer = SearchIndexer::new_in_memory().unwrap();
        let results = indexer.search("anything", 10).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_field_specific_search() {
        let mut indexer = SearchIndexer::new_in_memory().unwrap();

        indexer
            .index_summary(&make_summary(
                "auth.md",
                "Authentication guide",
                "Auth Guide",
                vec!["auth", "security"],
            ))
            .unwrap();

        indexer
            .index_summary(&make_summary(
                "other.md",
                "Something about auth mentioned in passing",
                "Other Doc",
                vec!["unrelated"],
            ))
            .unwrap();

        let results = indexer.search("tags:auth", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file, "auth.md");
    }
}
```

### Step 2: Run tests

Run: `cargo test search`
Expected: 3 tests pass.

### Step 3: Commit

```bash
git add src/search/indexer.rs
git commit -m "feat: Tantivy search indexer with full-text and field-specific search"
```

---

## Task 6: API Layer

### Prompt Contract

**GOAL:** All 5 REST endpoints working: POST /ingest, GET /status, GET /search, GET /summary/{file}, POST /reindex. Success = integration test calls /ingest, waits, then /search returns results.

**CONSTRAINTS:**
- Axum 0.8 path syntax: `/{param}` not `/:param`
- Shared state via `axum::extract::State` with `Arc<AppState>`
- All handlers return `Json<T>` or proper error status codes
- /ingest spawns processing in background, returns immediately

**FORMAT:**
- `src/api/ingest.rs` — POST handler, spawns queue processing
- `src/api/status.rs` — GET handler, returns QueueStatus
- `src/api/search.rs` — GET handler with query params
- `src/api/summary.rs` — GET handler for single file
- `src/main.rs` — wires everything together with Router
- Shared `AppState` struct in `src/main.rs` or `src/api/mod.rs`

**FAILURE CONDITIONS:**
- /ingest blocks until all files processed (must be async background)
- Uses `:param` path syntax instead of `{param}`
- Missing error handling (404 for missing summary, 400 for bad input)
- No shared state between handlers

### Step 1: Define AppState and wire router

**Files:** Modify: `src/api/mod.rs`

```rust
pub mod ingest;
pub mod search;
pub mod status;
pub mod summary;

use crate::config::AppConfig;
use crate::inference::client::InferenceClient;
use crate::queue::manager::QueueManager;
use crate::search::indexer::SearchIndexer;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub config: AppConfig,
    pub queue: QueueManager,
    pub inference: Arc<InferenceClient>,
    pub indexer: Arc<Mutex<SearchIndexer>>,
}
```

### Step 2: Implement ingest handler

**Files:** Modify: `src/api/ingest.rs`

```rust
use crate::api::AppState;
use crate::scanner::files::scan_directory;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct IngestRequest {
    pub path: String,
}

#[derive(Serialize)]
pub struct IngestResponse {
    pub message: String,
    pub files_queued: usize,
    pub files_skipped: usize,
}

pub async fn handle_ingest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<IngestRequest>,
) -> Result<Json<IngestResponse>, (StatusCode, String)> {
    let dir = PathBuf::from(&req.path);
    if !dir.is_dir() {
        return Err((StatusCode::BAD_REQUEST, format!("{} is not a directory", req.path)));
    }

    let scan_result = scan_directory(&dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let queued = scan_result.to_process.len();
    let skipped = scan_result.skipped;

    let client = state.inference.clone();
    let indexer = state.indexer.clone();
    let queue_state = state.queue.state.clone();
    let concurrency = state.config.workers;

    // Process in background
    tokio::spawn(async move {
        let manager = QueueManager::from_state(queue_state, concurrency);
        manager.process_files(scan_result.to_process, client, indexer).await;
    });

    Ok(Json(IngestResponse {
        message: "Ingestion started".to_string(),
        files_queued: queued,
        files_skipped: skipped,
    }))
}
```

### Step 3: Implement status handler

**Files:** Modify: `src/api/status.rs`

```rust
use crate::api::AppState;
use crate::queue::manager::QueueStatus;
use axum::extract::State;
use axum::Json;
use std::sync::Arc;

pub async fn handle_status(
    State(state): State<Arc<AppState>>,
) -> Json<QueueStatus> {
    let s = state.queue.state.lock().await;
    Json(s.status())
}
```

### Step 4: Implement search handler

**Files:** Modify: `src/api/search.rs`

```rust
use crate::api::AppState;
use crate::search::indexer::SearchResult;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

pub async fn handle_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, String)> {
    let limit = params.limit.unwrap_or(20);
    let indexer = state.indexer.lock().await;
    let results = indexer
        .search(&params.q, limit)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(results))
}
```

### Step 5: Implement summary handler

**Files:** Modify: `src/api/summary.rs`

```rust
use crate::summary::store;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use std::path::PathBuf;

pub async fn handle_summary(
    Path(file): Path<String>,
) -> Result<Json<store::Summary>, (StatusCode, String)> {
    // Derive summary path from the md filename
    let md_path = PathBuf::from(&file);
    let summary_path = store::summary_path_for(&md_path);

    if !summary_path.exists() {
        return Err((StatusCode::NOT_FOUND, format!("No summary found for {}", file)));
    }

    let summary = store::read_summary(&summary_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(summary))
}
```

### Step 6: Update QueueManager with from_state constructor

**Files:** Modify: `src/queue/manager.rs` — add method:

```rust
impl QueueManager {
    // ... existing new() ...

    pub fn from_state(
        state: Arc<Mutex<QueueState>>,
        concurrency: usize,
    ) -> Self {
        Self { state, concurrency }
    }
}
```

### Step 7: Wire main.rs

**Files:** Modify: `src/main.rs`

```rust
mod api;
mod config;
mod inference;
mod queue;
mod scanner;
mod search;
mod summary;

use api::AppState;
use axum::routing::{get, post};
use axum::Router;
use clap::Parser;
use config::AppConfig;
use inference::client::InferenceClient;
use queue::manager::QueueManager;
use search::indexer::SearchIndexer;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("eagle3=info")
        .init();

    let config = AppConfig::parse();

    let indexer = SearchIndexer::new_in_dir(std::path::Path::new(&config.index_path))
        .expect("Failed to create search index");

    let state = Arc::new(AppState {
        inference: Arc::new(InferenceClient::new(&config.mlx_url)),
        queue: QueueManager::new(config.workers),
        indexer: Arc::new(Mutex::new(indexer)),
        config: config.clone(),
    });

    let app = Router::new()
        .route("/ingest", post(api::ingest::handle_ingest))
        .route("/status", get(api::status::handle_status))
        .route("/search", get(api::search::handle_search))
        .route("/summary/{file}", get(api::summary::handle_summary))
        .with_state(state.clone());

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("eagle3 listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Step 8: Verify build

Run: `cargo build`
Expected: Compiles.

### Step 9: Commit

```bash
git add src/
git commit -m "feat: REST API layer with ingest, status, search, and summary endpoints"
```

---

## Task 7: MLX Sidecar Launch Script

### Prompt Contract

**GOAL:** Shell script that installs mlx-lm (if needed) and starts the inference server with the correct model. Success = script starts mlx-lm serve and responds to a curl health check.

**CONSTRAINTS:**
- Python 3.10+ via system or venv
- Uses `mlx-lm` package >= 0.26.3 (required for gpt_oss architecture)
- Binds to port 8100
- Logs to stdout

**FORMAT:**
- `scripts/start_mlx.sh` — single bash script
- Documented usage at top

**FAILURE CONDITIONS:**
- No check for Python availability
- No check for mlx-lm version
- Hardcoded model path instead of configurable

### Step 1: Create script

**Files:** Create: `scripts/start_mlx.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

# Eagle3 MLX Inference Server
# Usage: ./scripts/start_mlx.sh [MODEL]
# Default model: mlx-community/gpt-oss-20b-mlx-q8

MODEL="${1:-mlx-community/gpt-oss-20b-mlx-q8}"
PORT="${MLX_PORT:-8100}"

echo "=== Eagle3 MLX Sidecar ==="
echo "Model: $MODEL"
echo "Port:  $PORT"

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "ERROR: python3 not found. Install Python 3.10+."
    exit 1
fi

# Check/install mlx-lm
if ! python3 -c "import mlx_lm" 2>/dev/null; then
    echo "Installing mlx-lm..."
    pip3 install "mlx-lm>=0.26.3"
fi

# Check version
MLX_VERSION=$(python3 -c "import mlx_lm; print(mlx_lm.__version__)" 2>/dev/null || echo "unknown")
echo "mlx-lm version: $MLX_VERSION"

# Start server
echo "Starting mlx-lm serve on port $PORT..."
python3 -m mlx_lm.server \
    --model "$MODEL" \
    --port "$PORT"
```

### Step 2: Make executable and commit

```bash
chmod +x scripts/start_mlx.sh
git add scripts/
git commit -m "feat: MLX sidecar launch script"
```

---

## Task 8: Integration Test — Full Pipeline

### Prompt Contract

**GOAL:** End-to-end test that creates md files, starts the server with a mock inference backend, calls /ingest, waits for completion, then verifies /search returns results and .summary.json files exist on disk. Success = `cargo test integration` passes.

**CONSTRAINTS:**
- Mock the mlx-lm endpoint, don't require real model
- Use tempdir for all file I/O
- Timeout the test at 30 seconds
- Clean up after

**FORMAT:**
- `tests/integration_test.rs` — single integration test file
- Uses the same mock server pattern from Task 3

**FAILURE CONDITIONS:**
- Test requires running mlx-lm
- Test leaves files on disk after completion
- No assertion on .summary.json file contents
- No assertion on search results

### Step 1: Create integration test

**Files:** Create: `tests/integration_test.rs`

```rust
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;
use std::time::Duration;

async fn mock_llm_handler() -> Json<serde_json::Value> {
    Json(json!({
        "choices": [{
            "message": {
                "content": "{\"tldr\": \"A test document summary.\", \"title\": \"Test Doc\", \"tags\": [\"test\"], \"entities\": [\"TestEntity\"], \"topics\": [\"testing\"]}"
            }
        }]
    }))
}

#[tokio::test]
async fn test_full_pipeline() {
    // 1. Create temp directory with md files
    let dir = tempfile::tempdir().unwrap();
    for i in 0..5 {
        std::fs::write(
            dir.path().join(format!("doc{}.md", i)),
            format!("Document {} content about testing things.", i),
        )
        .unwrap();
    }

    // 2. Start mock LLM server
    let mock_app = Router::new().route("/v1/chat/completions", post(mock_llm_handler));
    let mock_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mock_addr = mock_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(mock_listener, mock_app).await.unwrap();
    });

    // 3. Create app with test config
    let index_dir = tempfile::tempdir().unwrap();
    let indexer = eagle3::search::indexer::SearchIndexer::new_in_dir(index_dir.path()).unwrap();
    let state = std::sync::Arc::new(eagle3::api::AppState {
        config: eagle3::config::AppConfig {
            port: 0,
            mlx_url: format!("http://{}", mock_addr),
            workers: 2,
            index_path: index_dir.path().display().to_string(),
        },
        inference: std::sync::Arc::new(eagle3::inference::client::InferenceClient::new(
            &format!("http://{}", mock_addr),
        )),
        queue: eagle3::queue::manager::QueueManager::new(2),
        indexer: std::sync::Arc::new(tokio::sync::Mutex::new(indexer)),
    });

    // 4. Scan and process
    let scan = eagle3::scanner::files::scan_directory(dir.path()).unwrap();
    assert_eq!(scan.to_process.len(), 5);

    state
        .queue
        .process_files(
            scan.to_process,
            state.inference.clone(),
            state.indexer.clone(),
        )
        .await;

    // 5. Verify status
    let status = state.queue.state.lock().await;
    assert_eq!(status.completed, 5);
    assert_eq!(status.failed, 0);
    drop(status);

    // 6. Verify .summary.json files exist
    for i in 0..5 {
        let summary_path = dir.path().join(format!("doc{}.summary.json", i));
        assert!(summary_path.exists(), "Missing summary for doc{}.md", i);

        let summary = eagle3::summary::store::read_summary(&summary_path).unwrap();
        assert_eq!(summary.title, "Test Doc");
        assert!(!summary.tldr.is_empty());
    }

    // 7. Verify search works
    let indexer = state.indexer.lock().await;
    let results = indexer.search("test", 10).unwrap();
    assert_eq!(results.len(), 5);
}
```

### Step 2: Add lib.rs for integration test access

**Files:** Create: `src/lib.rs`

```rust
pub mod api;
pub mod config;
pub mod inference;
pub mod queue;
pub mod scanner;
pub mod search;
pub mod summary;
```

### Step 3: Run integration test

Run: `cargo test test_full_pipeline`
Expected: PASS

### Step 4: Commit

```bash
git add tests/ src/lib.rs
git commit -m "feat: integration test for full ingest-summarize-search pipeline"
```

---

## Task 9: Polish & Documentation

### Prompt Contract

**GOAL:** Project has a working README with setup instructions, all `cargo clippy` warnings resolved, and `cargo test` passes all tests. Success = clean clippy, all tests green, README covers setup + usage.

**CONSTRAINTS:**
- README covers: prerequisites, MLX setup, building, running, API usage with curl examples
- No clippy warnings (treat as errors)
- Add `.gitignore` for target/, .eagle3_index/

**FORMAT:**
- `README.md` at project root
- `.gitignore` at project root

**FAILURE CONDITIONS:**
- README missing curl examples for each endpoint
- Clippy warnings remain
- Any test fails

### Step 1: Create .gitignore

**Files:** Create: `.gitignore`

```
/target
.eagle3_index/
*.summary.json
```

### Step 2: Fix clippy warnings

Run: `cargo clippy -- -D warnings`
Fix any warnings.

### Step 3: Run all tests

Run: `cargo test`
Expected: All tests pass.

### Step 4: Create README.md

**Files:** Create: `README.md` — cover prerequisites, build, MLX setup, run, API examples with curl.

### Step 5: Final commit

```bash
git add -A
git commit -m "docs: README, gitignore, clippy clean"
```

---

## Summary of Tasks

| Task | Component | Tests | Estimated Steps |
|------|-----------|-------|-----------------|
| 1 | Scaffold + Summary type | 3 unit | 5 |
| 2 | File scanner + skip logic | 5 unit | 3 |
| 3 | Inference client | 1 unit (mock) | 3 |
| 4 | Queue manager + worker | — (builds only) | 4 |
| 5 | Tantivy search indexer | 3 unit | 3 |
| 6 | API layer (5 endpoints) | — (builds only) | 9 |
| 7 | MLX sidecar script | manual | 2 |
| 8 | Integration test | 1 integration | 4 |
| 9 | Polish + docs | clippy + all | 5 |
