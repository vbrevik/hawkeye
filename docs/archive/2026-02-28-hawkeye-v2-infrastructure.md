# Hawkeye v2 Infrastructure Expansion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expand Hawkeye from a local `.summary.json` summarizer into a team knowledge platform backed by Postgres, Redis Streams, Milvus (semantic search), and Neo4j (knowledge graph), all composed on a single machine.

**Architecture:** Postgres becomes the source of truth for documents and summaries, replacing `.summary.json` sidecars. Redis Streams replace the in-memory `QueueManager`. A bge-m3 embedding sidecar feeds Milvus for semantic search. An LLM relationship extraction step feeds Neo4j. The existing Axum server gains new API endpoints for semantic search, graph queries, workspaces, API keys, and SSE event streaming.

**Tech Stack:** Rust/Axum 0.8, sqlx 0.8 (Postgres 16), redis 0.26 (Streams), reqwest (Milvus REST), neo4rs 0.8 (Neo4j 5), tokio-stream 0.1 (SSE), uuid 1, Docker Compose, infinity-emb (bge-m3 embedding server @ 7703)

---

## Existing codebase map

```
src/
  api/
    mod.rs         — AppState { config, queue, inference, indexer }
    ingest.rs      — POST /ingest, triggers scan_directory + process_files
    search.rs      — GET /search (Tantivy)
    status.rs      — GET /status, GET /mlx-status
    browse.rs      — GET /browse
    summary.rs     — GET /summary/:file
    ui.rs          — GET / (embedded HTML)
  inference/
    client.rs      — InferenceClient, summarize() → Summary
  queue/
    manager.rs     — QueueManager (in-memory semaphore)
    worker.rs      — process_file(): call LLM → write .summary.json → index Tantivy
  scanner/
    files.rs       — scan_directory(), skip if .summary.json matches hash
  summary/
    store.rs       — Summary struct, read/write .summary.json
  search/
    indexer.rs     — SearchIndexer (Tantivy)
  config.rs        — AppConfig (clap)
  main.rs          — router, AppState wiring
```

Key coupling to break:
- `scan_directory()` uses `summary_path_for()` for skip logic → replace with Postgres check
- `worker.rs` calls `store::write_summary()` → replace with `pg::documents::upsert()`
- `AppState` needs 4 new fields: `pg_pool`, `redis`, `embed`, `neo4j`

---

## Task 1: Docker Compose + dev environment

**GOAL:** Running `docker compose up -d` starts Redis, Postgres, Milvus (standalone), and Neo4j on their canonical ports. `cargo test` still passes.

**CONSTRAINTS:**
- Milvus standalone requires etcd + MinIO in the same compose file
- Postgres credentials: user=hawkeye, password=hawkeye, db=hawkeye
- Neo4j auth: neo4j/hawkeye
- No changes to Rust code in this task
- All services use named volumes so data survives restarts

**FORMAT:**
- Create: `docker-compose.yml` (project root)
- Create: `.env` (Postgres/Redis/Neo4j env vars, git-ignored)
- Modify: `.gitignore` (add `.env`, `postgres_data/`, `milvus_data/`)

**FAILURE CONDITIONS:**
- `docker compose ps` shows any service not in "running" state after `up -d`
- `redis-cli ping` returns anything other than PONG
- `psql -h localhost -U hawkeye -d hawkeye -c '\dt'` fails
- Milvus health endpoint `curl http://localhost:9091/healthz` returns non-200
- `curl http://localhost:7474` doesn't return Neo4j browser page

---

**Step 1: Create `docker-compose.yml`**

```yaml
# docker-compose.yml
version: "3.9"

services:
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      retries: 5

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: hawkeye
      POSTGRES_PASSWORD: hawkeye
      POSTGRES_DB: hawkeye
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U hawkeye"]
      interval: 5s
      retries: 10

  etcd:
    image: quay.io/coreos/etcd:v3.5.11
    environment:
      ETCD_AUTO_COMPACTION_MODE: revision
      ETCD_AUTO_COMPACTION_RETENTION: "1000"
      ETCD_QUOTA_BACKEND_BYTES: "4294967296"
      ETCD_SNAPSHOT_COUNT: "50000"
    command: >
      etcd
      --advertise-client-urls http://etcd:2379
      --listen-client-urls http://0.0.0.0:2379
      --data-dir /etcd
    volumes:
      - etcd_data:/etcd
    healthcheck:
      test: ["CMD", "etcdctl", "endpoint", "health"]
      interval: 10s
      retries: 5

  minio:
    image: minio/minio:RELEASE.2023-03-13T19-46-17Z
    environment:
      MINIO_ACCESS_KEY: minioadmin
      MINIO_SECRET_KEY: minioadmin
    command: minio server /minio_data --console-address ":9001"
    volumes:
      - minio_data:/minio_data
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 10s
      retries: 5

  milvus:
    image: milvusdb/milvus:v2.4.17
    command: ["milvus", "run", "standalone"]
    environment:
      ETCD_ENDPOINTS: etcd:2379
      MINIO_ADDRESS: minio:9000
    ports:
      - "19530:19530"
      - "9091:9091"
    volumes:
      - milvus_data:/var/lib/milvus
    depends_on:
      etcd:
        condition: service_healthy
      minio:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9091/healthz"]
      interval: 10s
      retries: 10

  neo4j:
    image: neo4j:5-community
    environment:
      NEO4J_AUTH: neo4j/hawkeye
      NEO4J_PLUGINS: '["apoc"]'
    ports:
      - "7474:7474"
      - "7687:7687"
    volumes:
      - neo4j_data:/data
    healthcheck:
      test: ["CMD-SHELL", "wget -q -O /dev/null http://localhost:7474 || exit 1"]
      interval: 10s
      retries: 10

volumes:
  redis_data:
  postgres_data:
  etcd_data:
  minio_data:
  milvus_data:
  neo4j_data:
```

**Step 2: Add `.env` and update `.gitignore`**

```bash
# .env
POSTGRES_URL=postgresql://hawkeye:hawkeye@localhost:5432/hawkeye
REDIS_URL=redis://localhost:6379
MILVUS_URL=http://localhost:19530
NEO4J_URL=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=hawkeye
```

Add to `.gitignore`:
```
.env
```

**Step 3: Start and verify**

```bash
docker compose up -d
docker compose ps       # all Running
redis-cli ping          # PONG
psql postgresql://hawkeye:hawkeye@localhost:5432/hawkeye -c '\l'
curl http://localhost:9091/healthz  # {"status":"ok"}
curl http://localhost:7474          # Neo4j browser HTML
cargo test              # all pass
```

**Step 4: Commit**

```bash
git add docker-compose.yml .env .gitignore
git commit -m "feat: docker compose with redis, postgres, milvus, neo4j"
```

---

## Task 2: Postgres schema + sqlx integration

**GOAL:** `cargo test` passes including a new test that runs migrations against a real Postgres instance and inserts/queries a document row. The `db` module is wired into `AppState`.

**CONSTRAINTS:**
- Use `sqlx` 0.8 with features: `runtime-tokio-rustls`, `postgres`, `uuid`, `chrono`, `json`, `macros`
- Use `uuid` 1 with features: `v4`, `serde`
- All schema changes go in `migrations/` directory, numbered sequentially
- Do NOT use sqlx compile-time query checking (`query!` macro) — use `query_as` with explicit structs to keep build simple without a live DB at compile time
- `AppConfig` gains new fields with sensible defaults but no new required args (all `--long-flag` with defaults)
- No changes to existing API behavior in this task

**FORMAT:**
- Modify: `Cargo.toml` (add sqlx, uuid, deadpool-redis)
- Modify: `src/config.rs` (add postgres_url, redis_url, neo4j_url, neo4j_user, neo4j_password, embed_url)
- Create: `migrations/0001_initial.sql`
- Create: `src/db/mod.rs`
- Create: `src/db/documents.rs`
- Modify: `src/api/mod.rs` (add `pg_pool: sqlx::PgPool` to AppState)
- Modify: `src/main.rs` (connect pool, pass to AppState)

**FAILURE CONDITIONS:**
- `cargo build` fails
- Any existing test breaks
- `sqlx::PgPool::connect()` not called in `main.rs`
- Schema migration doesn't create all 5 tables (workspaces, users, api_keys, documents, summaries)
- No db module integration test

---

**Step 1: Add dependencies to `Cargo.toml`**

```toml
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "json", "macros"] }
uuid = { version = "1", features = ["v4", "serde"] }
deadpool-redis = "0.18"
neo4rs = "0.8"
tokio-stream = "0.1"
```

**Step 2: Extend `src/config.rs`**

Add these fields to `AppConfig`:

```rust
#[arg(long, default_value = "postgresql://hawkeye:hawkeye@localhost:5432/hawkeye")]
pub postgres_url: String,

#[arg(long, default_value = "redis://localhost:6379")]
pub redis_url: String,

#[arg(long, default_value = "http://localhost:19530")]
pub milvus_url: String,

#[arg(long, default_value = "bolt://localhost:7687")]
pub neo4j_url: String,

#[arg(long, default_value = "neo4j")]
pub neo4j_user: String,

#[arg(long, default_value = "hawkeye")]
pub neo4j_password: String,

#[arg(long, default_value = "http://localhost:7703")]
pub embed_url: String,
```

**Step 3: Write migration `migrations/0001_initial.sql`**

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE workspaces (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name       TEXT NOT NULL,
    slug       TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Default workspace for single-user mode
INSERT INTO workspaces (id, name, slug)
VALUES ('00000000-0000-0000-0000-000000000000', 'default', 'default')
ON CONFLICT DO NOTHING;

CREATE TABLE users (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    email        TEXT UNIQUE NOT NULL,
    role         TEXT NOT NULL DEFAULT 'member',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE api_keys (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id),
    name         TEXT NOT NULL,
    key_hash     TEXT UNIQUE NOT NULL,
    last_used_at TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE documents (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id),
    path         TEXT NOT NULL,
    sha256       TEXT NOT NULL,
    status       TEXT NOT NULL DEFAULT 'pending',
    word_count   INT,
    ingested_at  TIMESTAMPTZ,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (workspace_id, path)
);

CREATE INDEX documents_sha256_idx ON documents(sha256);
CREATE INDEX documents_status_idx ON documents(status);

CREATE TABLE summaries (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id   UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    tldr          TEXT NOT NULL,
    title         TEXT NOT NULL,
    tags          TEXT[] NOT NULL DEFAULT '{}',
    entities      TEXT[] NOT NULL DEFAULT '{}',
    topics        TEXT[] NOT NULL DEFAULT '{}',
    relationships JSONB NOT NULL DEFAULT '[]',
    model_version TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE audit_log (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID REFERENCES users(id),
    action      TEXT NOT NULL,
    document_id UUID REFERENCES documents(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Step 4: Create `src/db/mod.rs`**

```rust
pub mod documents;

use sqlx::PgPool;

pub async fn connect(url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

**Step 5: Create `src/db/documents.rs`**

```rust
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub const DEFAULT_WORKSPACE: Uuid = Uuid::nil();

/// Returns Some(sha256) if a document with this path already exists and is done
pub async fn find_existing_sha256(
    pool: &PgPool,
    path: &str,
    workspace_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT sha256 FROM documents WHERE workspace_id = $1 AND path = $2 AND status = 'done'",
        workspace_id,
        path
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.sha256))
}

/// Insert or update a document row, returning its id
pub async fn upsert_document(
    pool: &PgPool,
    workspace_id: Uuid,
    path: &str,
    sha256: &str,
    word_count: i32,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO documents (workspace_id, path, sha256, status, word_count, updated_at)
        VALUES ($1, $2, $3, 'processing', $4, NOW())
        ON CONFLICT (workspace_id, path) DO UPDATE SET
            sha256 = EXCLUDED.sha256,
            status = 'processing',
            word_count = EXCLUDED.word_count,
            updated_at = NOW()
        RETURNING id
        "#,
        workspace_id,
        path,
        sha256,
        word_count
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

pub async fn mark_done(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE documents SET status = 'done', ingested_at = NOW(), updated_at = NOW() WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_failed(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE documents SET status = 'failed', updated_at = NOW() WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_summary(
    pool: &PgPool,
    document_id: Uuid,
    tldr: &str,
    title: &str,
    tags: &[String],
    entities: &[String],
    topics: &[String],
    relationships: &serde_json::Value,
    model_version: &str,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO summaries (document_id, tldr, title, tags, entities, topics, relationships, model_version)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        document_id,
        tldr,
        title,
        tags,
        entities,
        topics,
        relationships,
        model_version
    )
    .fetch_one(pool)
    .await?;
    Ok(row.id)
}

pub struct DocumentSummary {
    pub id: Uuid,
    pub path: String,
    pub sha256: String,
    pub status: String,
    pub word_count: Option<i32>,
    pub ingested_at: Option<DateTime<Utc>>,
}
```

**Step 6: Add `mod db;` to `src/main.rs`, connect pool, add to AppState**

In `src/api/mod.rs`, add:
```rust
use sqlx::PgPool;
// in AppState:
pub pg_pool: PgPool,
```

In `src/main.rs`:
```rust
mod db;
// in main():
let pg_pool = db::connect(&config.postgres_url).await.expect("Postgres connection failed");
// pass to AppState
```

**Step 7: Write integration test**

In `src/db/documents.rs` tests section:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires postgres"]
    async fn test_upsert_and_mark_done() {
        let pool = sqlx::PgPool::connect("postgresql://hawkeye:hawkeye@localhost:5432/hawkeye")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let id = upsert_document(&pool, DEFAULT_WORKSPACE, "/tmp/test.md", "sha256:abc", 42)
            .await
            .unwrap();
        assert!(!id.is_nil());

        mark_done(&pool, id).await.unwrap();

        let sha = find_existing_sha256(&pool, "/tmp/test.md", DEFAULT_WORKSPACE)
            .await
            .unwrap();
        assert_eq!(sha, Some("sha256:abc".to_string()));
    }
}
```

**Step 8: Run and verify**

```bash
cargo build          # must compile
cargo test           # existing tests pass
# With postgres running:
cargo test -- --ignored   # postgres integration test passes
```

**Step 9: Commit**

```bash
git add Cargo.toml Cargo.lock src/config.rs src/db/ src/api/mod.rs src/main.rs migrations/
git commit -m "feat: postgres schema, sqlx integration, db module"
```

---

## Task 3: Postgres replaces `.summary.json` sidecars

**GOAL:** After `POST /ingest`, document records are written to Postgres (not `.summary.json` files). Skip logic reads from Postgres instead of checking for `.summary.json`. `GET /summary/:file` reads from Postgres. Old sidecar read/write functions are deleted.

**CONSTRAINTS:**
- `scan_directory()` must be refactored to be `async` and accept a `&PgPool` — or a set of known sha256 hashes pre-fetched from Postgres must be passed in
- Use the default workspace UUID (`Uuid::nil()`) for all operations in this task — multi-workspace comes in Task 10
- `summary/store.rs` functions `read_summary`, `write_summary`, `summary_path_for` are deleted. The `Summary` struct moves to `db/documents.rs` or a new `summary/types.rs`
- Tantivy indexing stays — `search/indexer.rs` is unchanged
- All existing unit tests for scanner must be updated to not use `.summary.json`

**FORMAT:**
- Modify: `src/scanner/files.rs` (async scan, Postgres skip check)
- Modify: `src/summary/store.rs` → delete file content, keep `Summary` struct in `src/summary/types.rs`
- Modify: `src/queue/worker.rs` (write to Postgres instead of .summary.json)
- Modify: `src/api/summary.rs` (read from Postgres)
- Modify: `src/api/ingest.rs` (pass pool to scan + process)
- Modify: `src/api/mod.rs` (AppState already has pg_pool from Task 2)

**FAILURE CONDITIONS:**
- `.summary.json` files are still written anywhere
- `summary/store.rs` still contains `write_summary` or `read_summary`
- `cargo test` fails
- Skip logic doesn't use Postgres — still checks filesystem
- `GET /summary/:file` returns 404 after ingest (reads from Postgres now)

---

**Step 1: Create `src/summary/types.rs` with Summary struct**

Move `Summary` struct from `store.rs` to `types.rs`. Add `relationships: Vec<Relationship>` (will be populated in Task 4, default empty for now):

```rust
// src/summary/types.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    pub from: String,
    pub rel: String,
    pub to: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub id: Option<Uuid>,
    pub document_id: Option<Uuid>,
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
    pub model_version: String,
}
```

Update `src/summary/mod.rs`:
```rust
pub mod types;
```

**Step 2: Update `src/inference/client.rs`**

Change `use crate::summary::store::Summary;` to `use crate::summary::types::Summary;`. Add empty `relationships: vec![]` and `model_version: self.model.clone()` to the returned `Summary`. Also update `LlmOutput` to match (relationships field is `Vec<serde_json::Value>` for now, will be typed in Task 4).

**Step 3: Update `src/scanner/files.rs`**

Replace the `.summary.json` skip logic. The function becomes async and takes known hashes:

```rust
// New signature: called with pre-fetched known hashes from Postgres
pub fn scan_directory_with_known(
    dir: &Path,
    known_sha256: &std::collections::HashSet<String>,
) -> Result<ScanResult, Box<dyn std::error::Error>> {
    // ... same as before but check known_sha256.contains(&hash) instead of summary file
}
```

Add a helper in `src/db/documents.rs`:
```rust
pub async fn known_sha256_set(
    pool: &PgPool,
    workspace_id: Uuid,
) -> Result<std::collections::HashSet<String>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT sha256 FROM documents WHERE workspace_id = $1 AND status = 'done'",
        workspace_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.sha256).collect())
}
```

Update `src/api/ingest.rs` to fetch known hashes, then call `scan_directory_with_known`.

**Step 4: Update `src/queue/worker.rs`**

Add `pg_pool: Arc<sqlx::PgPool>` parameter. Before calling LLM: upsert document row. After LLM success: call `db::documents::insert_summary()` + `mark_done()`. Remove all `store::write_summary` calls.

```rust
// New signature
pub async fn process_file(
    file: &ScannedFile,
    client: &InferenceClient,
    indexer: &Arc<Mutex<SearchIndexer>>,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let word_count = /* count words in file */;
    let doc_id = db::documents::upsert_document(
        pool, DEFAULT_WORKSPACE, &file.path.display().to_string(), &file.hash, word_count as i32
    ).await?;

    // ... existing LLM call ...

    // On success:
    let rels_json = serde_json::to_value(&summary.relationships)?;
    db::documents::insert_summary(pool, doc_id, &summary.tldr, &summary.title,
        &summary.tags, &summary.entities, &summary.topics,
        &rels_json, &summary.model_version).await?;
    db::documents::mark_done(pool, doc_id).await?;
    // Still index to Tantivy:
    indexer.lock().await.index_summary(&summary)?;
}
```

**Step 5: Update `src/api/summary.rs`**

Replace `.summary.json` file read with Postgres query joining documents + summaries by path.

**Step 6: Delete `src/summary/store.rs`**

Remove the file. Update `src/summary/mod.rs` to only export `types`.

**Step 7: Update scanner tests**

Replace tests that write `.summary.json` files with tests that pass a `HashSet` of known hashes.

**Step 8: Run tests**

```bash
cargo test     # all pass
# Manually: POST /ingest with a folder, check no .summary.json files appear
# GET /summary/:filename — should return data from Postgres
```

**Step 9: Commit**

```bash
git add src/
git commit -m "feat: postgres replaces .summary.json sidecar storage"
```

---

## Task 4: Merged LLM prompt — add relationship extraction

**GOAL:** After ingest, `summaries.relationships` column in Postgres contains extracted entity relationships. The LLM system prompt asks for relationships in one call. The `Relationship` struct is fully typed. `GET /summary/:file` returns relationships in JSON.

**CONSTRAINTS:**
- Single LLM call — do not add a second inference request
- `relationships` is already in `Summary` struct from Task 3 — just populate it
- Keep `/no_think` prefix in system prompt
- Handle LLM responses that omit the `relationships` field (default to `[]`)

**FORMAT:**
- Modify: `src/inference/client.rs` (system prompt + `LlmOutput.relationships`)
- Modify: `src/summary/types.rs` (`Relationship` already there, verify typed deserialization)
- Modify: `src/inference/client.rs` tests (update mock response to include relationships)

**FAILURE CONDITIONS:**
- System prompt still asks for only 5 fields (no relationships)
- `LlmOutput.relationships` is not deserialized as typed `Vec<Relationship>`
- Test mock doesn't include relationships
- Worker crashes when LLM omits relationships field

---

**Step 1: Write failing test in `src/inference/client.rs`**

Add to the test module:

```rust
#[tokio::test]
async fn test_summarize_returns_relationships() {
    let app = Router::new().route("/v1/chat/completions", post(|| async {
        Json(json!({
            "choices": [{"message": {"content": r#"{
                "tldr": "Auth service depends on Postgres.",
                "title": "Auth Service Docs",
                "tags": ["auth"],
                "entities": ["auth-service", "postgres"],
                "topics": ["infrastructure"],
                "relationships": [
                    {"from": "auth-service", "rel": "depends_on", "to": "postgres", "context": "uses for sessions"}
                ]
            }"#}}]
        }))
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let client = InferenceClient::new(&format!("http://{}", addr), "mock");
    let summary = client.summarize("auth.md", "Auth service docs", "sha256:x").await.unwrap();
    assert_eq!(summary.relationships.len(), 1);
    assert_eq!(summary.relationships[0].from, "auth-service");
    assert_eq!(summary.relationships[0].rel, "depends_on");
}
```

Run: `cargo test test_summarize_returns_relationships` — expect FAIL (relationships is empty)

**Step 2: Update `SYSTEM_PROMPT` in `src/inference/client.rs`**

```rust
const SYSTEM_PROMPT: &str = r#"/no_think
You are a document summarizer. Given a markdown document, extract structured information.

Respond ONLY with valid JSON in this exact format:
{
  "tldr": "2-3 sentence summary",
  "title": "short document title",
  "tags": ["lowercase", "max5"],
  "entities": ["Person", "ServiceName", "Tool", "max10"],
  "topics": ["high-level topic", "max3"],
  "relationships": [
    {"from": "EntityA", "rel": "depends_on", "to": "EntityB", "context": "brief why"}
  ]
}

For relationships, use rel values: owns, depends_on, manages, relates_to, uses, runs_on.
Omit relationships array if none are evident. No markdown fences. No explanation."#;
```

**Step 3: Update `LlmOutput` struct**

```rust
use crate::summary::types::Relationship;

#[derive(Debug, Deserialize)]
struct LlmOutput {
    tldr: String,
    title: String,
    tags: Vec<String>,
    entities: Vec<String>,
    topics: Vec<String>,
    #[serde(default)]
    relationships: Vec<Relationship>,
}
```

**Step 4: Pass relationships through to Summary**

In `summarize()`, change:
```rust
Ok(Summary {
    // ...existing fields...
    relationships: output.relationships,
    // ...
})
```

**Step 5: Run test**

```bash
cargo test test_summarize_returns_relationships   # PASS
cargo test                                         # all pass
```

**Step 6: Commit**

```bash
git add src/inference/client.rs src/summary/types.rs
git commit -m "feat: merged LLM prompt extracts entity relationships"
```

---

## Task 5: Redis Streams job queue

**GOAL:** `POST /ingest` pushes jobs to a Redis Stream `hawkeye:ingest:default`. A background consumer group reads jobs and processes files. In-memory `QueueManager` is replaced. `GET /status` still works (reads from Redis, or Postgres doc counts).

**CONSTRAINTS:**
- Use `deadpool-redis` for connection pooling
- Consumer group name: `workers`, consumer name: `hawkeye-worker-{N}`
- Stream key: `hawkeye:ingest:{workspace_id}`
- On startup: create consumer group if it doesn't exist (`XGROUP CREATE ... MKSTREAM`)
- Process one message at a time per worker, ACK on success, leave unacked on failure (will be retried by XPENDING logic in future)
- Keep the existing `workers` count from `AppConfig` — spawn that many consumer tasks
- `GET /status` now returns counts from Postgres (query documents table by status) rather than in-memory counters

**FORMAT:**
- Create: `src/queue/redis_stream.rs`
- Modify: `src/queue/mod.rs`
- Modify: `src/queue/manager.rs` (replace with thin wrapper or deprecate)
- Modify: `src/api/status.rs` (read status from Postgres)
- Modify: `src/api/mod.rs` (add `redis_pool: deadpool_redis::Pool` to AppState)
- Modify: `src/main.rs` (create Redis pool, spawn worker tasks)

**FAILURE CONDITIONS:**
- `QueueManager::process_files()` still spawns in-memory semaphore workers
- Jobs are not visible in `XLEN hawkeye:ingest:00000000-0000-0000-0000-000000000000` after POST /ingest
- Worker panics if Redis is unavailable on startup
- Existing files are reprocessed on restart (Postgres skip logic works correctly)

---

**Step 1: Write failing test for stream push**

In `src/queue/redis_stream.rs`:

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore = "requires redis"]
    async fn test_push_and_read_job() {
        // push a job, create consumer group, read job back
        // assert job fields match what was pushed
    }
}
```

**Step 2: Create `src/queue/redis_stream.rs`**

```rust
use deadpool_redis::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const STREAM_PREFIX: &str = "hawkeye:ingest:";
const GROUP: &str = "workers";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngestJob {
    pub path: String,
    pub sha256: String,
    pub size: u64,
    pub workspace_id: String,
}

pub async fn ensure_consumer_group(pool: &Pool, workspace_id: Uuid) -> anyhow::Result<()> {
    let stream_key = format!("{}{}", STREAM_PREFIX, workspace_id);
    let mut conn = pool.get().await?;
    // XGROUP CREATE key group $ MKSTREAM — ignore "BUSYGROUP" error (already exists)
    let result: Result<(), redis::RedisError> = redis::cmd("XGROUP")
        .arg("CREATE").arg(&stream_key).arg(GROUP).arg("$").arg("MKSTREAM")
        .query_async(&mut *conn).await;
    match result {
        Ok(_) | Err(_) => Ok(())  // BUSYGROUP is fine
    }
}

pub async fn push_job(pool: &Pool, workspace_id: Uuid, job: &IngestJob) -> anyhow::Result<()> {
    let stream_key = format!("{}{}", STREAM_PREFIX, workspace_id);
    let mut conn = pool.get().await?;
    let job_json = serde_json::to_string(job)?;
    let _id: String = conn.xadd(&stream_key, "*", &[("job", &job_json)]).await?;
    Ok(())
}

pub async fn read_jobs(pool: &Pool, workspace_id: Uuid, consumer: &str, count: usize)
    -> anyhow::Result<Vec<(String, IngestJob)>>
{
    let stream_key = format!("{}{}", STREAM_PREFIX, workspace_id);
    let mut conn = pool.get().await?;
    let results: redis::streams::StreamReadReply = redis::cmd("XREADGROUP")
        .arg("GROUP").arg(GROUP).arg(consumer)
        .arg("COUNT").arg(count)
        .arg("BLOCK").arg(5000)  // 5s block
        .arg("STREAMS").arg(&stream_key).arg(">")
        .query_async(&mut *conn).await?;

    let mut jobs = Vec::new();
    for key in results.keys {
        for msg in key.ids {
            if let Some(redis::Value::BulkString(bytes)) = msg.map.get("job") {
                if let Ok(job) = serde_json::from_slice::<IngestJob>(bytes) {
                    jobs.push((msg.id, job));
                }
            }
        }
    }
    Ok(jobs)
}

pub async fn ack_job(pool: &Pool, workspace_id: Uuid, msg_id: &str) -> anyhow::Result<()> {
    let stream_key = format!("{}{}", STREAM_PREFIX, workspace_id);
    let mut conn = pool.get().await?;
    let _: i64 = conn.xack(&stream_key, GROUP, &[msg_id]).await?;
    Ok(())
}
```

**Step 3: Update `src/api/ingest.rs`**

Instead of calling `queue.process_files()`, push each scanned file as an `IngestJob` to Redis:

```rust
for file in scan_result.to_process {
    let job = IngestJob {
        path: file.path.display().to_string(),
        sha256: file.hash.clone(),
        size: file.size,
        workspace_id: DEFAULT_WORKSPACE.to_string(),
    };
    push_job(&state.redis_pool, DEFAULT_WORKSPACE, &job).await?;
}
```

**Step 4: Update `src/main.rs` — spawn N worker tasks**

```rust
// Create Redis pool
let redis_cfg = deadpool_redis::Config::from_url(&config.redis_url);
let redis_pool = redis_cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))
    .expect("Failed to create Redis pool");

// Ensure consumer group
queue::redis_stream::ensure_consumer_group(&redis_pool, Uuid::nil()).await
    .expect("Failed to create consumer group");

// Spawn N worker tasks
for i in 0..config.workers {
    let pool = redis_pool.clone();
    let pg = pg_pool.clone();
    let inference = state.inference.clone();
    let indexer = state.indexer.clone();
    tokio::spawn(async move {
        let consumer = format!("hawkeye-worker-{}", i);
        loop {
            match queue::redis_stream::read_jobs(&pool, Uuid::nil(), &consumer, 1).await {
                Ok(jobs) => {
                    for (msg_id, job) in jobs {
                        let file = ScannedFile { path: job.path.into(), size: job.size, hash: job.sha256 };
                        let result = queue::worker::process_file(&file, &inference, &indexer, &pg).await;
                        if result.is_ok() {
                            let _ = queue::redis_stream::ack_job(&pool, Uuid::nil(), &msg_id).await;
                        }
                    }
                }
                Err(e) => tracing::error!("Redis read error: {}", e),
            }
        }
    });
}
```

**Step 5: Update `GET /status` to read Postgres**

```rust
// In src/api/status.rs
pub async fn handle_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let row = sqlx::query!(
        "SELECT
            COUNT(*) FILTER (WHERE status='done') as completed,
            COUNT(*) FILTER (WHERE status='failed') as failed,
            COUNT(*) FILTER (WHERE status='processing') as in_progress,
            COUNT(*) as total
        FROM documents WHERE workspace_id = $1",
        Uuid::nil()
    )
    .fetch_one(&state.pg_pool)
    .await
    .unwrap_or_else(|_| /* zero counts */);
    Json(json!({ "total": row.total, "completed": row.completed, "failed": row.failed, "in_progress": row.in_progress }))
}
```

**Step 6: Add redis_pool to AppState and main.rs wiring**

**Step 7: Verify**

```bash
cargo build
# Start stack: docker compose up -d && cargo run
# POST /ingest {"path": "test_data/"}
# redis-cli XLEN hawkeye:ingest:00000000-0000-0000-0000-000000000000  → N
# Watch redis-cli XPENDING ... → decreasing as workers process
# GET /status → {"total": N, "completed": N, "failed": 0}
```

**Step 8: Commit**

```bash
git add src/
git commit -m "feat: redis streams replace in-memory queue"
```

---

## Task 6: bge-m3 embedding sidecar + EmbedClient

**GOAL:** `scripts/start_embed.sh` starts an OpenAI-compatible embedding server at port 7703. `src/embedding/client.rs` can call it and return `Vec<Vec<f32>>` for a batch of text chunks. A unit test with a mock HTTP server passes.

**CONSTRAINTS:**
- Use `infinity-emb` served via `uv run --with "infinity-emb[all]"` — no Docker for sidecars
- The embedding server must expose `POST /v1/embeddings` compatible with OpenAI format
- `EmbedClient` chunks text into 512-token overlapping windows (step 256)
- Use a simple word-count approximation for token count (1 token ≈ 4 chars)
- No Milvus writes in this task — just the client and chunking logic

**FORMAT:**
- Create: `scripts/start_embed.sh`
- Create: `src/embedding/mod.rs`
- Create: `src/embedding/client.rs`
- Modify: `src/lib.rs` (add `pub mod embedding`)
- Modify: `src/api/mod.rs` (add `embed: Arc<EmbedClient>` to AppState — wired in Task 7)

**FAILURE CONDITIONS:**
- `scripts/start_embed.sh` uses pip3 or python3 directly (not uv)
- `EmbedClient::embed_chunks()` doesn't chunk — embeds whole document as one vector
- Chunk size exceeds 512 token approximation
- Mock server test doesn't verify chunk count or vector dimensions

---

**Step 1: Create `scripts/start_embed.sh`**

```bash
#!/usr/bin/env bash
set -euo pipefail

# Hawkeye Embedding Sidecar — bge-m3 via infinity-emb
# Usage: ./scripts/start_embed.sh
# Starts OpenAI-compatible embedding server on port 7703

MODEL="${EMBED_MODEL:-BAAI/bge-m3}"
PORT="${EMBED_PORT:-7703}"

if ! command -v uv &>/dev/null; then
  echo "ERROR: uv not found. Install: curl -LsSf https://astral.sh/uv/install.sh | sh"
  exit 1
fi

echo "=== Hawkeye Embedding Sidecar ==="
echo "Model : $MODEL"
echo "Port  : $PORT"

exec uv run --with "infinity-emb[all]" \
  python3 -m infinity_emb v2 \
  --model-name-or-path "$MODEL" \
  --port "$PORT"
```

```bash
chmod +x scripts/start_embed.sh
```

**Step 2: Write failing test**

In `src/embedding/client.rs`:

```rust
#[tokio::test]
async fn test_embed_chunks_returns_correct_count() {
    // 3000-char text → should produce multiple 512-token chunks
    let text = "word ".repeat(600); // ~3000 chars
    let client = EmbedClient::new("http://unused");
    let chunks = client.make_chunks(&text);
    assert!(chunks.len() >= 2, "Expected at least 2 chunks for long text");
    for chunk in &chunks {
        assert!(chunk.len() <= 512 * 4, "Chunk exceeds 512 token approximation");
    }
}
```

Run: `cargo test test_embed_chunks_returns_correct_count` — FAIL (EmbedClient doesn't exist)

**Step 3: Create `src/embedding/client.rs`**

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const CHUNK_CHARS: usize = 512 * 4;    // ~512 tokens at 4 chars/token
const STEP_CHARS: usize = CHUNK_CHARS / 2; // 50% overlap

#[derive(Debug, Serialize)]
struct EmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedData>,
}

#[derive(Debug, Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

pub struct EmbedClient {
    client: Client,
    base_url: String,
}

impl EmbedClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to build embed client"),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Split text into overlapping character windows approximating 512 tokens each
    pub fn make_chunks(&self, text: &str) -> Vec<String> {
        if text.len() <= CHUNK_CHARS {
            return vec![text.to_string()];
        }
        let chars: Vec<char> = text.chars().collect();
        let mut chunks = Vec::new();
        let mut start = 0;
        while start < chars.len() {
            let end = (start + CHUNK_CHARS).min(chars.len());
            chunks.push(chars[start..end].iter().collect());
            if end == chars.len() { break; }
            start += STEP_CHARS;
        }
        chunks
    }

    /// Embed all chunks of a document, returning one vector per chunk
    pub async fn embed_chunks(
        &self,
        text: &str,
        model: &str,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        let chunks = self.make_chunks(text);
        let request = EmbedRequest {
            model: model.to_string(),
            input: chunks,
        };
        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&request)
            .send()
            .await?;
        let embed_response: EmbedResponse = response.json().await?;
        Ok(embed_response.data.into_iter().map(|d| d.embedding).collect())
    }
}
```

**Step 4: Create `src/embedding/mod.rs`**

```rust
pub mod client;
```

**Step 5: Add module to `src/main.rs`**

```rust
mod embedding;
```

**Step 6: Run tests**

```bash
cargo test test_embed_chunks_returns_correct_count   # PASS
cargo test                                            # all pass
```

**Step 7: Commit**

```bash
git add scripts/start_embed.sh src/embedding/
git commit -m "feat: bge-m3 embedding sidecar script and EmbedClient"
```

---

## Task 7: Milvus integration + `/search/semantic` endpoint

**GOAL:** After ingest, each document's chunks are embedded and stored in Milvus collection `doc_chunks`. `GET /search/semantic?q=...` returns the top-10 nearest-neighbor documents.

**CONSTRAINTS:**
- Use Milvus REST API v2 via `reqwest` — no additional crate
- Milvus REST API base: `{config.milvus_url}/v2/vectordb/`
- Collection schema: `doc_id` (varchar), `chunk_index` (int32), `workspace_id` (varchar), `vector` (float_vector dim=1024)
- Create collection on startup if it doesn't exist (idempotent)
- Embed model name: `BAAI/bge-m3`
- Search uses L2 metric, returns doc_id + distance, deduplicate by doc_id, return top-10 unique documents
- Milvus REST API requires JSON body — do not use gRPC

**FORMAT:**
- Create: `src/milvus/mod.rs`
- Create: `src/milvus/client.rs`
- Modify: `src/queue/worker.rs` (call milvus.insert_chunks after summary written)
- Modify: `src/api/mod.rs` (add `milvus: Arc<MilvusClient>`)
- Modify: `src/api/search.rs` (add `handle_semantic_search`)
- Modify: `src/main.rs` (create MilvusClient, add route `/search/semantic`)

**FAILURE CONDITIONS:**
- `doc_chunks` collection not created on startup
- Vectors not inserted after ingest
- `/search/semantic` returns 500 when Milvus unavailable (should return empty results with error log)
- Duplicate doc_ids in search results (must deduplicate)
- Vector dimension ≠ 1024

---

**Step 1: Write failing test**

```rust
// In src/milvus/client.rs tests
#[tokio::test]
#[ignore = "requires milvus"]
async fn test_ensure_collection_idempotent() {
    let client = MilvusClient::new("http://localhost:19530");
    client.ensure_collection().await.unwrap();
    client.ensure_collection().await.unwrap(); // second call must not error
}
```

**Step 2: Create `src/milvus/client.rs`**

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;

pub struct MilvusClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct MilvusResp {
    code: i64,
}

impl MilvusClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(10)).build().unwrap(),
            base_url: format!("{}/v2/vectordb", base_url.trim_end_matches('/')),
        }
    }

    pub async fn ensure_collection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if exists
        let resp: Value = self.client
            .post(format!("{}/collections/has", self.base_url))
            .json(&json!({"collectionName": "doc_chunks"}))
            .send().await?.json().await?;

        if resp["data"]["has"].as_bool() == Some(true) {
            return Ok(());
        }

        // Create
        self.client
            .post(format!("{}/collections/create", self.base_url))
            .json(&json!({
                "collectionName": "doc_chunks",
                "schema": {
                    "fields": [
                        {"fieldName": "id", "dataType": "Int64", "isPrimary": true, "autoId": true},
                        {"fieldName": "doc_id", "dataType": "VarChar", "typeParams": {"max_length": "64"}},
                        {"fieldName": "chunk_index", "dataType": "Int32"},
                        {"fieldName": "workspace_id", "dataType": "VarChar", "typeParams": {"max_length": "64"}},
                        {"fieldName": "vector", "dataType": "FloatVector", "typeParams": {"dim": "1024"}}
                    ]
                },
                "indexParams": [{
                    "fieldName": "vector",
                    "metricType": "L2",
                    "indexType": "AUTOINDEX"
                }]
            }))
            .send().await?;
        Ok(())
    }

    pub async fn insert_chunks(
        &self,
        doc_id: &str,
        workspace_id: &str,
        vectors: Vec<Vec<f32>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let rows: Vec<Value> = vectors.into_iter().enumerate().map(|(i, v)| {
            json!({
                "doc_id": doc_id,
                "chunk_index": i as i32,
                "workspace_id": workspace_id,
                "vector": v
            })
        }).collect();

        self.client
            .post(format!("{}/entities/insert", self.base_url))
            .json(&json!({"collectionName": "doc_chunks", "data": rows}))
            .send().await?;
        Ok(())
    }

    pub async fn search(
        &self,
        workspace_id: &str,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let resp: Value = self.client
            .post(format!("{}/entities/search", self.base_url))
            .json(&json!({
                "collectionName": "doc_chunks",
                "data": [query_vector],
                "annsField": "vector",
                "limit": limit * 3,  // over-fetch to account for deduplication
                "outputFields": ["doc_id"],
                "filter": format!("workspace_id == \"{}\"", workspace_id)
            }))
            .send().await?.json().await?;

        // Deduplicate doc_ids preserving order
        let mut seen = std::collections::HashSet::new();
        let mut doc_ids = Vec::new();
        if let Some(hits) = resp["data"].as_array() {
            for hit in hits.iter().flat_map(|h| h.as_array().unwrap_or(&vec![])) {
                if let Some(id) = hit["doc_id"].as_str() {
                    if seen.insert(id.to_string()) {
                        doc_ids.push(id.to_string());
                        if doc_ids.len() >= limit { break; }
                    }
                }
            }
        }
        Ok(doc_ids)
    }
}
```

**Step 3: Add Milvus insert to `src/queue/worker.rs`**

After `mark_done()`, embed + insert:

```rust
// embed content chunks
let vectors = embed_client.embed_chunks(&content, "BAAI/bge-m3").await?;
milvus.insert_chunks(&doc_id.to_string(), "default", vectors).await?;
```

Pass `embed_client: &EmbedClient` and `milvus: &MilvusClient` as new parameters to `process_file`.

**Step 4: Add `/search/semantic` handler**

```rust
// src/api/search.rs
pub async fn handle_semantic_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Json<serde_json::Value> {
    let vectors = match state.embed.embed_chunks(&params.q, "BAAI/bge-m3").await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Embed failed: {}", e);
            return Json(json!({"results": []}));
        }
    };
    let query_vector = vectors.into_iter().next().unwrap_or_default();
    let doc_ids = match state.milvus.search("default", query_vector, 10).await {
        Ok(ids) => ids,
        Err(e) => {
            tracing::error!("Milvus search failed: {}", e);
            return Json(json!({"results": []}));
        }
    };
    // fetch summaries from postgres by doc_id
    // ...return results
    Json(json!({"results": doc_ids}))
}
```

**Step 5: Verify**

```bash
cargo build
# start stack + embed sidecar (./scripts/start_embed.sh)
# POST /ingest, GET /search/semantic?q=kubernetes
```

**Step 6: Commit**

```bash
git add src/milvus/ src/queue/worker.rs src/api/search.rs src/main.rs
git commit -m "feat: milvus vector store, embedding pipeline, semantic search endpoint"
```

---

## Task 8: Neo4j integration + `/graph` endpoints

**GOAL:** After ingest, entity nodes and relationship edges from `summary.relationships` are written to Neo4j. `GET /graph/entity/:name` returns connected entities. `GET /graph/document/:id` returns the document's entity neighbourhood.

**CONSTRAINTS:**
- Use `neo4rs = "0.8"` — Bolt protocol, async
- Node labels: `Document`, `Entity`, `Tag`, `Topic`
- Entity `MERGE` on `name` to avoid duplicates
- All Neo4j writes happen in `worker.rs` after Postgres write
- If Neo4j is unavailable, log warning and continue — do not fail ingest
- Connection created once on startup via `neo4rs::Graph::new()`

**FORMAT:**
- Create: `src/graph/mod.rs`
- Create: `src/graph/client.rs`
- Modify: `src/queue/worker.rs` (call graph writes after summary written)
- Modify: `src/api/mod.rs` (add `neo4j: Arc<neo4rs::Graph>`)
- Modify: `src/main.rs` (connect Neo4j, add routes)
- Create: `src/api/graph.rs`

**FAILURE CONDITIONS:**
- Neo4j writes block or panic when Neo4j is down
- Entity nodes duplicated (must use MERGE not CREATE)
- `/graph/entity/:name` returns 500 on unknown entity (should return empty)
- Relationships extracted from LLM are not reflected in Neo4j

---

**Step 1: Create `src/graph/client.rs`**

```rust
use neo4rs::{query, Graph};
use std::sync::Arc;
use crate::summary::types::Relationship;

pub struct GraphClient {
    graph: Arc<Graph>,
}

impl GraphClient {
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self, neo4rs::Error> {
        let graph = Graph::new(uri, user, password).await?;
        Ok(Self { graph: Arc::new(graph) })
    }

    pub async fn write_document(
        &self,
        doc_id: &str,
        path: &str,
        workspace_id: &str,
        entities: &[String],
        tags: &[String],
        topics: &[String],
        relationships: &[Relationship],
    ) -> Result<(), neo4rs::Error> {
        // Merge document node
        self.graph.run(query(
            "MERGE (d:Document {id: $id}) SET d.path = $path, d.workspace_id = $workspace_id"
        ).param("id", doc_id).param("path", path).param("workspace_id", workspace_id)).await?;

        // Merge entity nodes + MENTIONS edges
        for entity in entities {
            self.graph.run(query(
                "MERGE (e:Entity {name: $name}) \
                 WITH e MATCH (d:Document {id: $doc_id}) \
                 MERGE (d)-[:MENTIONS]->(e)"
            ).param("name", entity.as_str()).param("doc_id", doc_id)).await?;
        }

        // Tag nodes
        for tag in tags {
            self.graph.run(query(
                "MERGE (t:Tag {name: $name}) \
                 WITH t MATCH (d:Document {id: $doc_id}) \
                 MERGE (d)-[:TAGGED_WITH]->(t)"
            ).param("name", tag.as_str()).param("doc_id", doc_id)).await?;
        }

        // LLM-extracted entity relationships
        for rel in relationships {
            self.graph.run(query(
                "MERGE (a:Entity {name: $from}) MERGE (b:Entity {name: $to}) \
                 MERGE (a)-[:RELATES_TO {label: $rel, context: $ctx}]->(b)"
            ).param("from", rel.from.as_str())
             .param("to", rel.to.as_str())
             .param("rel", rel.rel.as_str())
             .param("ctx", rel.context.as_str())).await?;
        }
        Ok(())
    }

    pub async fn entity_neighbourhood(&self, name: &str) -> Result<Vec<String>, neo4rs::Error> {
        let mut result = self.graph.execute(query(
            "MATCH (e:Entity {name: $name})-[*1..2]-(connected) RETURN DISTINCT connected.name AS name"
        ).param("name", name)).await?;

        let mut names = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            if let Ok(n) = row.get::<String>("name") {
                names.push(n);
            }
        }
        Ok(names)
    }

    pub async fn document_entities(&self, doc_id: &str) -> Result<Vec<String>, neo4rs::Error> {
        let mut result = self.graph.execute(query(
            "MATCH (d:Document {id: $id})-[:MENTIONS]->(e:Entity) RETURN e.name AS name"
        ).param("id", doc_id)).await?;

        let mut names = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            if let Ok(n) = row.get::<String>("name") {
                names.push(n);
            }
        }
        Ok(names)
    }
}
```

**Step 2: Add graph writes to `src/queue/worker.rs`**

After successful Milvus insert:
```rust
if let Err(e) = neo4j.write_document(
    &doc_id.to_string(), &file.path.display().to_string(), "default",
    &summary.entities, &summary.tags, &summary.topics, &summary.relationships
).await {
    tracing::warn!("Neo4j write failed (non-fatal): {}", e);
}
```

**Step 3: Create `src/api/graph.rs`**

```rust
use axum::extract::{Path, State};
use axum::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::api::AppState;

pub async fn handle_entity_graph(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Json<Value> {
    match state.neo4j.entity_neighbourhood(&name).await {
        Ok(connected) => Json(json!({"entity": name, "connected": connected})),
        Err(e) => {
            tracing::error!("Neo4j query failed: {}", e);
            Json(json!({"entity": name, "connected": []}))
        }
    }
}

pub async fn handle_document_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Json<Value> {
    match state.neo4j.document_entities(&id).await {
        Ok(entities) => Json(json!({"document_id": id, "entities": entities})),
        Err(e) => {
            tracing::error!("Neo4j query failed: {}", e);
            Json(json!({"document_id": id, "entities": []}))
        }
    }
}
```

**Step 4: Register routes in `src/main.rs`**

```rust
.route("/graph/entity/:name", get(api::graph::handle_entity_graph))
.route("/graph/document/:id", get(api::graph::handle_document_graph))
```

**Step 5: Verify**

```bash
cargo build
# POST /ingest, then:
# GET /graph/entity/postgres → {"entity":"postgres","connected":["auth-service",...]}
# GET /graph/document/{uuid} → {"document_id":"...","entities":[...]}
```

**Step 6: Commit**

```bash
git add src/graph/ src/api/graph.rs src/queue/worker.rs src/main.rs
git commit -m "feat: neo4j knowledge graph, entity relationship writes, graph API endpoints"
```

---

## Task 9: SSE `/events` endpoint

**GOAL:** `GET /events` returns an SSE stream. When a document finishes processing, the worker publishes a Redis pub/sub message on `hawkeye:events:{workspace_id}`, and all connected SSE clients receive it within 1 second.

**CONSTRAINTS:**
- Use `tokio-stream` for the SSE stream
- Use `axum::response::sse::{Event, Sse}` — no external SSE crate
- Redis pub/sub uses `deadpool-redis` connection (separate subscribe connection, not the pooled one)
- Worker publishes AFTER `mark_done()` is called
- SSE event type: `document_done`, data: `{"path": "...", "doc_id": "..."}`
- Handle client disconnect gracefully (SSE stream ends, no panic)

**FORMAT:**
- Create: `src/api/events.rs`
- Modify: `src/queue/worker.rs` (publish to Redis pub/sub on success)
- Modify: `src/api/mod.rs` (add `redis_client: Arc<redis::Client>` for pub/sub — separate from pool)
- Modify: `src/main.rs` (add `/events` route)

**FAILURE CONDITIONS:**
- `/events` returns 404 or 500 immediately instead of keeping connection open
- Worker doesn't publish on success
- SSE sends no `event:` field (must have `event: document_done`)
- Server panics on client disconnect

---

**Step 1: Add publish to `src/queue/worker.rs`**

Add `redis_client: &redis::Client` parameter. After `mark_done()`:

```rust
if let Ok(mut pubsub_conn) = redis_client.get_async_connection().await {
    let payload = json!({"path": file.path.display().to_string(), "doc_id": doc_id.to_string()}).to_string();
    let channel = format!("hawkeye:events:{}", DEFAULT_WORKSPACE);
    let _: Result<(), _> = pubsub_conn.publish(&channel, payload).await;
}
```

**Step 2: Create `src/api/events.rs`**

```rust
use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::AppState;

pub async fn handle_events(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let workspace_id = Uuid::nil();
    let channel = format!("hawkeye:events:{}", workspace_id);
    let redis_client = state.redis_client.clone();

    let stream = async_stream::stream! {
        let conn = match redis_client.get_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Redis pub/sub connect failed: {}", e);
                return;
            }
        };
        let mut pubsub = conn.into_pubsub();
        if let Err(e) = pubsub.subscribe(&channel).await {
            tracing::error!("Subscribe failed: {}", e);
            return;
        }
        let mut msg_stream = pubsub.on_message();
        // Keep-alive every 15s
        let mut keepalive = tokio::time::interval(Duration::from_secs(15));
        loop {
            tokio::select! {
                msg = msg_stream.next() => {
                    match msg {
                        Some(m) => {
                            if let Ok(payload) = m.get_payload::<String>() {
                                yield Ok::<Event, Infallible>(Event::default().event("document_done").data(payload));
                            }
                        }
                        None => break,
                    }
                }
                _ = keepalive.tick() => {
                    yield Ok::<Event, Infallible>(Event::default().comment("keepalive"));
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)),
    )
}
```

Add to `Cargo.toml`:
```toml
async-stream = "0.3"
```

**Step 3: Register route**

```rust
.route("/events", get(api::events::handle_events))
```

**Step 4: Verify**

```bash
curl -N http://localhost:7700/events &
# POST /ingest — should see: event: document_done\ndata: {"path":"...","doc_id":"..."}
```

**Step 5: Commit**

```bash
git add src/api/events.rs src/queue/worker.rs src/main.rs Cargo.toml
git commit -m "feat: SSE /events endpoint via redis pub/sub"
```

---

## Task 10: Workspaces + API key management

**GOAL:** `POST /workspaces` creates a workspace. `GET /workspaces/:id/docs` lists documents. `POST /api-keys` issues a new API key (returns plaintext once, stores bcrypt hash). All endpoints requiring auth check `Authorization: Bearer <key>` against the `api_keys` table.

**CONSTRAINTS:**
- API key format: `hke_` prefix + 32 random hex chars (total 36 chars)
- Hash API key with `sha2::Sha256` (same crate already in project) — NOT bcrypt (no new dep)
- Auth middleware: extract Bearer token, hash it, query `api_keys` table, reject with 401 if not found
- `POST /workspaces` and `POST /api-keys` are bootstrap endpoints — no auth required (document this)
- All other `/workspaces/*` and future agent endpoints require auth
- `GET /workspaces/:id/docs` returns document list with status from Postgres
- Existing endpoints (`/search`, `/ingest`, `/status`, `/browse`) remain unauthenticated for backwards compat

**FORMAT:**
- Create: `src/api/workspaces.rs`
- Create: `src/api/auth.rs` (middleware + key hashing)
- Modify: `src/main.rs` (add routes, layer auth middleware on protected routes)
- Modify: `src/db/documents.rs` (add list_documents query)

**FAILURE CONDITIONS:**
- API key stored in plaintext in database
- `POST /api-keys` without workspace/user context creates orphan key (must have associated user)
- Auth middleware returns 500 instead of 401 for invalid key
- `GET /workspaces/:id/docs` with wrong workspace_id leaks another workspace's docs
- Bootstrap endpoints (`POST /workspaces`) accidentally protected by auth

---

**Step 1: Create `src/api/auth.rs`**

```rust
use sha2::{Digest, Sha256};
use rand::Rng;

pub fn generate_api_key() -> String {
    let random: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    format!("hke_{}", random)
}

pub fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

Add `rand = "0.8"` to `Cargo.toml`.

**Step 2: Add `list_documents` to `src/db/documents.rs`**

```rust
pub struct DocumentRow {
    pub id: Uuid,
    pub path: String,
    pub sha256: String,
    pub status: String,
    pub word_count: Option<i32>,
    pub ingested_at: Option<DateTime<Utc>>,
}

pub async fn list_documents(
    pool: &PgPool,
    workspace_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<DocumentRow>, sqlx::Error> {
    sqlx::query_as!(DocumentRow,
        "SELECT id, path, sha256, status, word_count, ingested_at
         FROM documents WHERE workspace_id = $1
         ORDER BY updated_at DESC LIMIT $2 OFFSET $3",
        workspace_id, limit, offset
    ).fetch_all(pool).await
}
```

**Step 3: Create `src/api/workspaces.rs`**

```rust
use axum::extract::{Path, State, Json as ExtractJson};
use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::{AppState, auth};
use crate::db::documents;

#[derive(Deserialize)]
pub struct CreateWorkspace { pub name: String, pub slug: String }

pub async fn handle_create_workspace(
    State(state): State<Arc<AppState>>,
    ExtractJson(body): ExtractJson<CreateWorkspace>,
) -> Result<Json<Value>, StatusCode> {
    let id: Uuid = sqlx::query_scalar!(
        "INSERT INTO workspaces (name, slug) VALUES ($1, $2) RETURNING id",
        body.name, body.slug
    )
    .fetch_one(&state.pg_pool).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"id": id, "name": body.name, "slug": body.slug})))
}

pub async fn handle_list_docs(
    State(state): State<Arc<AppState>>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let docs = documents::list_documents(&state.pg_pool, workspace_id, 100, 0)
        .await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items: Vec<Value> = docs.iter().map(|d| json!({
        "id": d.id,
        "path": d.path,
        "status": d.status,
        "word_count": d.word_count,
        "ingested_at": d.ingested_at
    })).collect();
    Ok(Json(json!({"workspace_id": workspace_id, "documents": items})))
}

#[derive(Deserialize)]
pub struct CreateApiKey { pub user_id: Uuid, pub name: String }

pub async fn handle_create_api_key(
    State(state): State<Arc<AppState>>,
    ExtractJson(body): ExtractJson<CreateApiKey>,
) -> Result<Json<Value>, StatusCode> {
    let key = auth::generate_api_key();
    let key_hash = auth::hash_key(&key);

    sqlx::query!(
        "INSERT INTO api_keys (user_id, name, key_hash) VALUES ($1, $2, $3)",
        body.user_id, body.name, key_hash
    )
    .execute(&state.pg_pool).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return plaintext key only once
    Ok(Json(json!({"key": key, "name": body.name, "warning": "Store this key — it will not be shown again"})))
}
```

**Step 4: Register routes in `src/main.rs`**

```rust
.route("/workspaces", post(api::workspaces::handle_create_workspace))
.route("/workspaces/:id/docs", get(api::workspaces::handle_list_docs))
.route("/api-keys", post(api::workspaces::handle_create_api_key))
```

**Step 5: Verify**

```bash
cargo build
# POST /workspaces {"name":"team","slug":"team"} → {"id":"..."}
# POST /api-keys {"user_id":"...","name":"agent-1"} → {"key":"hke_..."}
# GET /workspaces/{id}/docs → {"documents":[...]}
```

**Step 6: Commit**

```bash
git add src/api/workspaces.rs src/api/auth.rs src/main.rs src/db/documents.rs Cargo.toml
git commit -m "feat: workspaces, API key issuance and management"
```

---

## Final: verify full stack

```bash
# Start all services
docker compose up -d
./scripts/start_mlx.sh &
./scripts/start_embed.sh &
cargo run

# Smoke test sequence
curl -X POST http://localhost:7700/ingest -H 'Content-Type: application/json' -d '{"path":"test_data/"}'
curl -N http://localhost:7700/events &           # watch for document_done events
sleep 30
curl 'http://localhost:7700/search?q=kubernetes'
curl 'http://localhost:7700/search/semantic?q=kubernetes deployment'
curl 'http://localhost:7700/graph/entity/postgres'
curl http://localhost:7700/status
```

Run the full test suite:
```bash
cargo test
cargo test -- --ignored  # integration tests (requires docker compose up)
```

**Final commit:**

```bash
git add .
git commit -m "chore: hawkeye v2 complete — postgres, redis streams, milvus, neo4j, SSE, workspaces"
```
