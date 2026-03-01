# Development Backlog

## Vision

Expand Hawkeye from a local `.summary.json` summarizer into a team knowledge platform backed by Postgres, Redis Streams, Milvus (semantic search), and Neo4j (knowledge graph), all composed on a single machine.

## ✅ Completed Work

### Core Summarizer (v1)
- Axum 0.8 server with Tantivy full-text search index
- MLX inference sidecar integration (Qwen3-Next-80B-A3B-Instruct-4bit)
- Concurrent work queue with configurable workers
- SHA-256 content hashing to skip unchanged files on re-ingest
- `.summary.json` sidecar files with tldr, title, tags, entities, topics
- CLI config via clap with sensible defaults

### Web UI
- Embedded single-page HTML UI at `GET /`
- Split-pane knowledge explorer with detail panel
- Keyboard navigation, auto-select, related docs computation
- Inline filesystem browser for ingestion

### API Endpoints (v1)
- `POST /ingest` — scan directory, queue files for summarization
- `GET /status` — queue progress (total, completed, failed, in_progress)
- `GET /mlx-status` — MLX sidecar health
- `GET /search` — Tantivy full-text search with field-specific queries
- `GET /facets` — tag, topic, entity frequency counts
- `GET /browse` — filesystem directory listing with md_file_count
- `GET /summary/{file}` — single file summary from .summary.json

### Infrastructure (v2 Task 1) ✅
- Docker Compose with Redis, Postgres, etcd, MinIO, Milvus, Neo4j
- All services use named volumes, health checks configured
- Ports offset from defaults to avoid conflicts (Postgres 5433, Neo4j 7475/7688)
- Service URL fields added to AppConfig with defaults

### Health Monitoring ✅
- `GET /health` endpoint with per-service TCP/HTTP checks
- ServiceHealth types with latency classification (fast/ok/slow/timeout)
- Checks Redis, Postgres, etcd, MinIO, Milvus, Neo4j, MLX via tokio::join!

### Developer Tooling ✅
- knowledge.md with project context
- Claude Code skills (catch-up, wrap-up, spec-feature, prompt-contracts, systems-thinking, frontend-design)
- Clippy clean (`-D warnings`)
- Integration test suite

### Postgres Schema + sqlx Integration (v2 Task 2) ✅
- sqlx 0.8 with runtime `query_as` (no compile-time `query!` macro)
- Initial migration: 6 tables (workspaces, users, api_keys, documents, summaries, audit_log)
- Default workspace seeded (UUID nil) for single-user mode
- `PgPool` wired into `AppState`, connects and runs migrations on startup
- `src/db/documents.rs` with upsert_document, get_document_by_path, get_source_hashes, insert_summary, get_summary_by_document
- Integration test: migrations + document CRUD against real Postgres

---

## Remaining Backlog

### Phase 1: Postgres Migration

#### Task 3 — Postgres replaces .summary.json sidecars
**Priority:** High | **Effort:** Large

**GOAL:** After `POST /ingest`, document records and summaries are written to Postgres (not `.summary.json` files). Skip logic reads SHA-256 hashes from Postgres instead of checking the filesystem. `GET /summary/:file` reads from Postgres. The `store.rs` sidecar functions are deleted.

**CONSTRAINTS:**
- Scanner becomes async or accepts a pre-fetched `HashSet<String>` of known SHA-256 hashes from Postgres
- Use default workspace UUID (`Uuid::nil()`) for all operations — multi-workspace comes in Task 10
- `Summary` struct moves from `store.rs` to a new `summary/types.rs`
- Tantivy indexing stays unchanged — `search/indexer.rs` is not modified
- All existing scanner tests must be updated (no `.summary.json` references)

**FAILURE CONDITIONS:**
- `.summary.json` files are still written anywhere in the codebase
- `summary/store.rs` still contains `write_summary` or `read_summary`
- `cargo test` fails
- Skip logic still checks the filesystem instead of Postgres
- `GET /summary/:file` returns 404 after a successful ingest

---

### Phase 2: Intelligence Layer

#### Task 4 — Merged LLM prompt with relationship extraction
**Priority:** High | **Effort:** Small

**GOAL:** After ingest, `summaries.relationships` column in Postgres contains extracted entity relationships. The existing LLM system prompt is extended (one call, not two) to also return a `relationships` array. `GET /summary/:file` includes relationships in its JSON response.

**CONSTRAINTS:**
- Single LLM call — do NOT add a second inference request
- Keep `/no_think` prefix in system prompt
- `Relationship` struct: `{ from, rel, to, context }` with typed rel values (owns, depends_on, manages, etc.)
- Handle LLM responses that omit the `relationships` field gracefully (default to `[]`)

**FAILURE CONDITIONS:**
- System prompt still asks for only 5 fields (no relationships)
- `LlmOutput.relationships` is `serde_json::Value` instead of typed `Vec<Relationship>`
- Worker crashes when LLM omits the relationships field
- A second HTTP call is made to the LLM

---

#### Task 5 — Redis Streams replace in-memory queue
**Priority:** Medium | **Effort:** Medium

**GOAL:** `POST /ingest` publishes file paths to a Redis Stream. Workers consume from the stream using consumer groups. Jobs survive server restarts. `GET /status` reads counts from Redis. The in-memory `QueueManager` is deleted.

**CONSTRAINTS:**
- Use `deadpool-redis` for connection pooling (already a planned dependency)
- Redis Stream key: `hawkeye:jobs:{workspace_id}`
- Consumer group: `hawkeye-workers`, created on startup (idempotent)
- Workers ACK messages after successful processing
- Failed jobs stay in the pending entries list (PEL) for retry

**FAILURE CONDITIONS:**
- In-memory `QueueManager` still exists
- Jobs lost on server restart
- `GET /status` returns stale counts (not from Redis)
- Consumer group not created automatically on first run
- No integration test with real Redis

---

#### Task 6 — Embedding sidecar (bge-m3)
**Priority:** Medium | **Effort:** Small

**GOAL:** A startup script runs infinity-emb with bge-m3 on port 7703. `EmbedClient` in Rust sends text to the sidecar and receives `Vec<Vec<f32>>` embeddings (dim 1024). A unit test verifies the client against a mock HTTP server.

**CONSTRAINTS:**
- Use `infinity-emb` Python package (same pattern as MLX sidecar)
- Model: `BAAI/bge-m3` (1024 dimensions)
- Chunk long documents before embedding (max ~512 tokens per chunk)
- `embed_url` added to AppConfig with default `http://localhost:7703`
- No new Rust dependencies beyond reqwest (already in project)

**FAILURE CONDITIONS:**
- Embedding dimension ≠ 1024
- No chunking logic for long documents
- Client panics when sidecar is offline (should return error)
- No unit test

---

#### Task 7 — Milvus vector store + semantic search
**Priority:** Medium | **Effort:** Medium

**GOAL:** After ingest, each document's embedded chunks are stored in Milvus collection `doc_chunks`. `GET /search/semantic?q=...` embeds the query, searches Milvus, and returns the top-10 nearest-neighbor documents (deduplicated by doc_id).

**CONSTRAINTS:**
- Use Milvus REST API v2 via `reqwest` — no additional crate
- Collection schema: `doc_id` (varchar), `chunk_index` (int32), `workspace_id` (varchar), `vector` (float_vector dim=1024)
- Create collection on startup if it doesn't exist (idempotent)
- Search uses L2 metric, over-fetches to account for deduplication
- If Milvus is unavailable, `/search/semantic` returns empty results with an error log (not 500)

**FAILURE CONDITIONS:**
- `doc_chunks` collection not created on startup
- Vectors not inserted after ingest
- Duplicate doc_ids in search results
- `/search/semantic` returns 500 when Milvus is down
- Vector dimension ≠ 1024

---

#### Task 8 — Neo4j knowledge graph
**Priority:** Medium | **Effort:** Medium

**GOAL:** After ingest, entity nodes and relationship edges from `summary.relationships` are written to Neo4j. `GET /graph/entity/:name` returns connected entities (2-hop). `GET /graph/document/:id` returns the document's entity neighbourhood.

**CONSTRAINTS:**
- Use `neo4rs` 0.8 — Bolt protocol, async
- Node labels: `Document`, `Entity`, `Tag`, `Topic`
- Entity nodes use `MERGE` (not `CREATE`) to avoid duplicates
- All Neo4j writes happen in the worker after Postgres write
- If Neo4j is unavailable, log warning and continue — do NOT fail ingest
- Connection created once on startup via `neo4rs::Graph::new()`

**FAILURE CONDITIONS:**
- Neo4j writes block or panic when Neo4j is down
- Entity nodes duplicated (must use MERGE)
- `/graph/entity/:name` returns 500 on unknown entity (should return empty)
- Relationships extracted from LLM are not reflected in Neo4j

---

### Phase 3: Real-Time & Multi-Tenant

#### Task 9 — SSE /events endpoint
**Priority:** Low | **Effort:** Small

**GOAL:** `GET /events` returns a Server-Sent Events stream. When a document finishes processing, the worker publishes to Redis pub/sub, and all connected SSE clients receive a `document_done` event within 1 second. Connection stays open indefinitely with keepalive comments.

**CONSTRAINTS:**
- Use `axum::response::sse::{Event, Sse}` — no external SSE crate
- Use `tokio-stream` for the SSE stream adapter
- Redis pub/sub channel: `hawkeye:events:{workspace_id}`
- Worker publishes AFTER `mark_done()` is called
- 15-second keepalive comments to prevent proxy timeouts
- Handle client disconnect gracefully (no panic)

**FAILURE CONDITIONS:**
- `/events` returns 404 or 500 immediately instead of keeping connection open
- Worker doesn't publish on success
- SSE sends no `event:` field (must have `event: document_done`)
- Server panics on client disconnect

---

#### Task 10 — Workspaces + API key management
**Priority:** Low | **Effort:** Medium

**GOAL:** `POST /workspaces` creates a workspace. `POST /api-keys` issues a new API key (returns plaintext once, stores SHA-256 hash). All `/workspaces/*` endpoints require `Authorization: Bearer <key>`. Existing public endpoints (`/search`, `/ingest`, `/status`, `/browse`) remain unauthenticated.

**CONSTRAINTS:**
- API key format: `hke_` prefix + 32 random alphanumeric chars
- Hash with `sha2::Sha256` (already in project) — NOT bcrypt
- `POST /workspaces` and `POST /api-keys` are bootstrap endpoints — no auth required
- Auth middleware: extract Bearer token → hash → query `api_keys` table → reject 401 if not found
- `GET /workspaces/:id/docs` returns document list with status from Postgres
- Add `rand` crate for key generation

**FAILURE CONDITIONS:**
- API key stored in plaintext in database
- Auth middleware returns 500 instead of 401 for invalid key
- `GET /workspaces/:id/docs` with wrong workspace_id leaks another workspace's docs
- Bootstrap endpoints accidentally protected by auth
- Existing public endpoints break (require auth)

---

### Improvements & Polish

These are smaller tasks without full prompt contracts:

- Document `/health` endpoint in README API section
- Add health dashboard panel to web UI
- `POST /reindex` endpoint to rebuild Tantivy from Postgres
- Graceful shutdown (drain queue, close connections)
- Structured JSON logging option
- CI pipeline (cargo test + clippy + docker compose integration tests)

---

## Implementation Order

```
Task 2 (Postgres schema)
  │
  ▼
Task 3 (Replace .summary.json)
  │
  ├──► Task 4 (Relationship extraction)
  ├──► Task 5 (Redis Streams queue)
  │
  ▼
Task 6 (Embedding sidecar)
  │
  ▼
Task 7 (Milvus semantic search)
  │
  ▼
Task 8 (Neo4j knowledge graph)
  │
  ├──► Task 9 (SSE /events)
  │
  ▼
Task 10 (Workspaces + API keys)
```

Tasks 4, 5, and 9 can be done in parallel with their predecessors.

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| ORM | sqlx (no ORM) | Runtime-checked queries, no compile overhead |
| Embedding model | bge-m3 (1024d) | Multilingual, strong retrieval, runs on Apple Silicon |
| Vector DB | Milvus standalone | Handles billions of vectors, REST API, already in compose |
| Graph DB | Neo4j 5 Community | Cypher query language, APOC plugin, mature ecosystem |
| Queue | Redis Streams | Durable, restart-surviving, replaces in-memory semaphore |
| Auth | SHA-256 hashed bearer tokens | No new dependency (sha2 already in project) |
| Multi-tenant | Workspace UUID on all tables | Clean data isolation, default workspace for single-user |

## Task Tracking

**Current Task:** Task 3 — Postgres replaces .summary.json sidecars
**Next Task:** Task 4 — Merged LLM prompt with relationship extraction
**Blockers:** None
**Dependencies:** Docker Compose stack must be running for integration tests

> Historical design docs archived in `docs/archive/`.
