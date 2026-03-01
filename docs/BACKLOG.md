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
- `POST /ingest` — scan directory, queue files for summarization via Redis Streams
- `POST /cancel` — discard unprocessed queued jobs, keep server running
- `POST /shutdown` — graceful server shutdown (drain requests, stop consumers, exit)
- `POST /shutdown?docker=true` — graceful shutdown + `docker compose down`
- `GET /status` — queue progress from Redis counters (total, completed, failed, in_progress)
- `GET /mlx-status` — MLX sidecar health
- `GET /health` — per-service TCP/HTTP health checks with latency classification
- `GET /search` — Tantivy full-text search with field-specific queries
- `GET /facets` — tag, topic, entity frequency counts
- `GET /browse` — filesystem directory listing with md_file_count
- `GET /summary/{file}` — single file summary with relationships from Postgres

### Infrastructure (v2 Task 1) ✅
- Docker Compose with Redis, Postgres, etcd, MinIO, Milvus, Neo4j
- All services use named volumes, health checks configured
- Ports offset from defaults to avoid conflicts (Postgres 5433, Neo4j 7475/7688)
- Service URL fields added to AppConfig with defaults

### Health Monitoring ✅
- `GET /health` endpoint with per-service TCP/HTTP checks
- ServiceHealth types with latency classification (fast/ok/slow/timeout)
- Checks Redis, Postgres, etcd, MinIO, Milvus, Neo4j, MLX via tokio::join!

### Ingestion Control ✅
- `POST /cancel` — clears unprocessed Redis Stream jobs, adjusts counters, recreates consumer group
- Returns `{ cancelled, already_completed, already_failed }`
- In-flight jobs finish gracefully but don't affect status

### Graceful Shutdown ✅
- Signal handling: SIGINT (Ctrl+C), SIGTERM (`kill`), and `POST /shutdown` all trigger the same path
- `with_graceful_shutdown()` drains in-flight HTTP requests
- Consumer workers use `tokio::select!` with `watch` channel to break cleanly
- 3-second grace period for consumers, then abort remaining
- `POST /shutdown?docker=true` runs `docker compose down` after server stops
- `AppState` carries `shutdown: watch::Sender<bool>` and `shutdown_docker: AtomicBool`

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

### Postgres Replaces .summary.json Sidecars (v2 Task 3) ✅
- Worker writes documents + summaries to Postgres instead of `.summary.json` files
- Scanner accepts pre-fetched hash map from Postgres for skip logic
- `GET /summary/:file` reads from Postgres via joined query
- `summary/store.rs` deleted, `Summary` struct moved to `summary/types.rs`
- `get_summary_by_source_path` and `get_source_hashes` added to db layer
- All unit tests and integration tests updated for Postgres-backed flow

---

## What's Built So Far

Hawkeye is a fully functional local knowledge platform:

| Layer | Status | Components |
|-------|--------|------------|
| **Ingest** | ✅ | Filesystem scanner → Redis Streams queue → MLX LLM sidecar → Postgres + Tantivy |
| **Search** | ✅ | Full-text (Tantivy), faceted (tags/topics/entities), filesystem browse |
| **Intelligence** | ✅ | TL;DR, title, tags, entities, topics, relationship extraction (single LLM call) |
| **Queue** | ✅ | Durable Redis Streams with consumer groups, survives restarts |
| **Storage** | ✅ | Postgres (documents, summaries, relationships), Tantivy index |
| **Ops** | ✅ | Health checks, cancel in-flight jobs, graceful shutdown, optional docker teardown |
| **UI** | ✅ | Inline HTML (search, browse, ingest, facets, detail panel, drawer) |
| **Semantic Search** | 🟡 | Embedding sidecar ✅ + Milvus vector store (Task 7) |
| **Frontend** | 🔜 | SvelteKit migration with multi-page routing (Task 11) |
| **Knowledge Graph** | 🔜 | Neo4j with extracted relationships + graph viz page (Task 8) |
| **Real-Time** | 🔜 | SSE events for live ingest progress in SvelteKit UI (Task 9) |
| **Multi-Tenant** | 🔜 | Workspaces + API keys (Task 10) |

---

### Phase 2: Intelligence Layer

#### Task 4 — Merged LLM prompt with relationship extraction ✅
- SYSTEM_PROMPT extended to extract relationships in single LLM call
- `Relationship` struct with typed `RelationType` enum (serde other fallback)
- `relationships JSONB` column added to summaries (migration 0002)
- `LlmOutput.relationships` defaults to `[]` when LLM omits field
- `GET /summary/:file` includes relationships in response
- 23 unit tests + 3 integration tests pass

#### Task 5 — Redis Streams replace in-memory queue ✅
- `RedisQueue` (src/queue/stream.rs) with XADD, XREADGROUP, XACK, status counters
- Consumer loop (src/queue/consumer.rs) with spawn_consumers() for N workers
- `POST /ingest` publishes to Redis Stream; `GET /status` reads Redis counters
- In-memory `QueueManager` deleted, `AppState.redis_pool` replaces it
- Consumer group created idempotently on startup; jobs survive restarts
- Integration tests use real Redis with unique workspace IDs for isolation

---

#### Task 6 — Embedding sidecar (bge-m3) ✅
**Priority:** Medium | **Effort:** Small

- `scripts/start_embed.sh` + `scripts/embed_server.py` — FastAPI server using sentence-transformers (OpenAI-compatible `/v1/embeddings`)
- `src/embedding/client.rs` — `EmbedClient` with `make_chunks()` (2048 char windows, 50% overlap) and `embed_chunks()` calling `/v1/embeddings`
- `src/config.rs` — `embed_url` (default `http://localhost:7703`) and `embed_model` (default `BAAI/bge-m3`)
- 8 unit tests (5 chunking + 3 mock HTTP server), all passing
- Tested live: single + batch embeddings return 1024-dim vectors correctly
- Note: switched from `infinity-emb` to `sentence-transformers` + FastAPI due to Python 3.13 compatibility issues

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

#### 🔧 Task R1 — Backend refactor to feature-based architecture
**Priority:** Medium | **Effort:** Small | **When:** After Task 7, before Task 11

**GOAL:** Migrate existing flat `src/api/*.rs` + `src/db/*.rs` layout into the `src/features/` + `src/shared/` structure defined in the Feature-Based Architecture section. All existing tests and clippy must still pass.

**SCOPE:**
- Move `api/ingest.rs` + `scanner/files.rs` + `queue/worker.rs` → `features/ingest/`
- Move `api/search.rs` + `search/indexer.rs` → `features/search/`
- Move `api/summary.rs` + `summary/types.rs` → `features/summary/`
- Move `api/browse.rs` → `features/browse/`
- Move `api/tags.rs` → `features/search/` (facets are part of search)
- Move `queue/stream.rs` + `queue/consumer.rs` → `features/queue/`
- Tasks 6-7 code already in `features/semantic/` (created during those tasks)
- Move `api/mod.rs` (AppState) → `shared/state.rs`
- Move `api/health.rs` → `shared/health.rs`
- Move `api/shutdown.rs` → `shared/shutdown.rs`
- Move `db/documents.rs` → `shared/db/documents.rs`
- Move `inference/client.rs` → `shared/inference/client.rs`
- Update all `mod.rs`, `lib.rs`, `main.rs` re-exports
- Delete empty old directories

**FAILURE CONDITIONS:**
- Any test fails after refactor
- Clippy warnings introduced
- Public API (HTTP routes) changes
- Circular module dependencies

---

#### 🔍 Tech Debt Audit 1 — Post-backend stabilization
**When:** After Task R1 (before SvelteKit migration)

Systematic review before building the frontend on top of the backend:

- [ ] **Error handling consistency** — are all handlers returning proper `(StatusCode, String)` or a shared error type? Should we introduce a unified `AppError` enum?
- [ ] **`#[allow(dead_code)]` audit** — remove any that are no longer needed; delete truly dead code
- [ ] **Duplicate logic** — check for repeated patterns (e.g., `Uuid::nil()` hardcoded workspace ID scattered everywhere)
- [ ] **Test coverage gaps** — are there handlers with no integration test? Any untested error paths?
- [ ] **Dependency audit** — `cargo outdated`, check for unused deps (`cargo udeps` if available)
- [ ] **Config sprawl** — is `AppConfig` getting too large? Should it be split by feature?
- [ ] **Documentation** — are all public functions documented? Is README API section current?
- [ ] **Logging consistency** — are all important operations logged with appropriate levels?
- [ ] **SQL query review** — any N+1 queries, missing indexes, or unbounded SELECTs?

---

### Phase 3: Frontend Foundation

> SvelteKit is done **before** Tasks 8-9 so graph visualization and SSE live updates are built in the proper component framework from day one — no throwaway UI work.

#### Task 11 — SvelteKit frontend migration
**Priority:** Medium | **Effort:** Medium

**GOAL:** Replace the inline HTML string in `src/api/ui.rs` with a SvelteKit app in `web/`. Build outputs static files to `static/` which Axum serves via `tower-http::services::ServeDir`. The UI is split into reusable Svelte components with file-based routing for multiple pages.

**CONSTRAINTS:**
- SvelteKit with static adapter (`@sveltejs/adapter-static`) — no SSR, no Node.js at runtime
- `web/` directory for the SvelteKit project; `static/` for build output (gitignored)
- Axum serves `static/` via `ServeDir`, falling back to `index.html` for SPA routing
- Port existing UI (search, browse, ingest, status, facets, detail panel) into Svelte components
- Add Cancel button (visible during active ingestion) and Shutdown button (with docker checkbox + confirmation)
- File-based routing: `/` (search + browse), `/admin` (cancel, shutdown, status dashboard)
- Dev workflow: `cd web && npm run dev` with Vite proxy to `localhost:7700` for API calls
- Delete `src/api/ui.rs` after migration; `GET /` serves `static/index.html`
- TypeScript for type-safe API calls to Rust endpoints

**FAILURE CONDITIONS:**
- Node.js required at runtime (must be build-time only)
- Existing UI features missing after migration (search, browse, ingest, facets, detail panel, drawer)
- API calls break due to CORS or proxy misconfiguration
- No dev hot-reload workflow
- `cargo build` alone doesn't produce a working UI (need documented build step)

---

#### 🔍 Tech Debt Audit 2 — Post-frontend migration
**When:** After Task 11 (before building new features in SvelteKit)

Ensure the SvelteKit foundation is solid before building on it:

- [ ] **Component size** — any component over ~150 lines should be split
- [ ] **API type drift** — do TypeScript types in `features/*/types.ts` match actual Rust API responses?
- [ ] **Accessibility** — keyboard navigation, ARIA labels, focus management preserved from inline HTML
- [ ] **Bundle size** — check Vite build output, ensure no unnecessary dependencies
- [ ] **Backend cleanup** — delete `src/api/ui.rs` and any orphaned inline HTML helpers
- [ ] **Shared component extraction** — identify repeated patterns across features and extract to `shared/components/`
- [ ] **Error boundaries** — do API failures show user-friendly messages, not raw JSON?

---

### Phase 4: Features Built in SvelteKit

> These tasks add complex UI (graph viz page, live SSE updates) — built directly in SvelteKit components.

#### Task 8 — Neo4j knowledge graph + graph visualization
**Priority:** Medium | **Effort:** Medium

**GOAL:** After ingest, entity nodes and relationship edges from `summary.relationships` are written to Neo4j. `GET /graph/entity/:name` returns connected entities (2-hop). `GET /graph/document/:id` returns the document's entity neighbourhood. SvelteKit `/graph` page renders an interactive graph visualization.

**CONSTRAINTS:**
- Use `neo4rs` 0.8 — Bolt protocol, async
- Node labels: `Document`, `Entity`, `Tag`, `Topic`
- Entity nodes use `MERGE` (not `CREATE`) to avoid duplicates
- All Neo4j writes happen in the worker after Postgres write
- If Neo4j is unavailable, log warning and continue — do NOT fail ingest
- Connection created once on startup via `neo4rs::Graph::new()`
- Graph viz page in SvelteKit at `/graph` with interactive node exploration

**FAILURE CONDITIONS:**
- Neo4j writes block or panic when Neo4j is down
- Entity nodes duplicated (must use MERGE)
- `/graph/entity/:name` returns 500 on unknown entity (should return empty)
- Relationships extracted from LLM are not reflected in Neo4j

#### Task 9 — SSE /events endpoint + live UI updates
**Priority:** Medium | **Effort:** Small

**GOAL:** `GET /events` returns a Server-Sent Events stream. When a document finishes processing, the worker publishes to Redis pub/sub, and all connected SSE clients receive a `document_done` event within 1 second. Connection stays open indefinitely with keepalive comments. SvelteKit UI consumes events for live ingest progress.

**CONSTRAINTS:**
- Use `axum::response::sse::{Event, Sse}` — no external SSE crate
- Use `tokio-stream` for the SSE stream adapter
- Redis pub/sub channel: `hawkeye:events:{workspace_id}`
- Worker publishes AFTER `mark_done()` is called
- 15-second keepalive comments to prevent proxy timeouts
- Handle client disconnect gracefully (no panic)
- SvelteKit admin page subscribes to SSE for real-time status updates

**FAILURE CONDITIONS:**
- `/events` returns 404 or 500 immediately instead of keeping connection open
- Worker doesn't publish on success
- SSE sends no `event:` field (must have `event: document_done`)
- Server panics on client disconnect

---

#### 🔍 Tech Debt Audit 3 — Pre-auth hardening
**When:** After Tasks 8-9 (before adding auth in Task 10)

Auth is cross-cutting and touches everything — clean house first:

- [ ] **Handler signatures** — consistent extractor ordering across all handlers
- [ ] **Workspace ID propagation** — audit all `Uuid::nil()` usages; prepare for real workspace IDs
- [ ] **Redis key isolation** — verify all Redis keys are workspace-scoped
- [ ] **Neo4j data isolation** — verify graph queries filter by workspace
- [ ] **Milvus data isolation** — verify vector searches filter by workspace_id
- [ ] **Integration test isolation** — all tests use unique workspace IDs (no cross-contamination)
- [ ] **Frontend API layer** — ready to pass `Authorization` header from a central client?
- [ ] **Error response consistency** — all endpoints return same error shape for 4xx/5xx

---

### Phase 5: Multi-Tenant

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
- ~~Graceful shutdown (drain queue, close connections)~~ ✅ Done
- Structured JSON logging option
- CI pipeline (cargo test + clippy + docker compose integration tests)

### Recurring: Tech Debt Hygiene

Apply these practices **during every task**, not just at audit checkpoints:

1. **Boy Scout Rule** — leave every file you touch cleaner than you found it (remove dead code, fix warnings, improve naming)
2. **No new `#[allow(dead_code)]`** — if you're adding one, the code shouldn't exist yet
3. **Run `cargo clippy -- -D warnings`** before every commit — non-negotiable
4. **Test what you build** — every new handler gets at least one integration test; every new client gets a unit test with mock
5. **Update knowledge.md** — after any architectural change (new module, new dependency, new convention)
6. **Check for extracted patterns** — if you write the same code in 3+ places, extract a helper into `shared/`

---

## Implementation Order

```
Task 2 (Postgres schema)               ✅
  │
  ▼
Task 3 (Replace .summary.json)         ✅
  │
  ├──► Task 4 (Relationship extraction)    ✅
  ├──► Task 5 (Redis Streams queue)        ✅
  │
  ▼
Task 6 (Embedding sidecar)             ✅
  │
  ▼
Task 7 (Milvus semantic search)        ← CURRENT
  │
  ▼
🔧 Task R1 (Backend refactor)           ← feature-based architecture
  │
  ▼
🔍 Tech Debt Audit 1                    ← clean backend before frontend
  │
  ▼
Task 11 (SvelteKit frontend)
  │
  ▼
🔍 Tech Debt Audit 2                    ← validate frontend foundation
  │
  ├──► Task 8 (Neo4j + graph viz)          built in SvelteKit
  ├──► Task 9 (SSE + live UI)              built in SvelteKit
  │
  ▼
🔍 Tech Debt Audit 3                    ← harden before auth
  │
  ▼
Task 10 (Workspaces + API keys)
```

**Completed:** Tasks 1–6 + cancel/shutdown/docker teardown
**In progress:** Task 7 (Milvus vector store + semantic search)
**Key additions:** Refactor task (R1) and 3 tech debt audits at phase boundaries. Recurring hygiene practices applied during every task.
**Parallelizable:** Tasks 8 and 9 can be done in parallel after SvelteKit migration.

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| ORM | sqlx (no ORM) | Runtime-checked queries, no compile overhead |
| Embedding model | bge-m3 (1024d) | Multilingual, strong retrieval, runs on Apple Silicon |
| Vector DB | Milvus standalone | Handles billions of vectors, REST API, already in compose |
| Graph DB | Neo4j 5 Community | Cypher query language, APOC plugin, mature ecosystem |
| Queue | Redis Streams | Durable, restart-surviving, cancel + graceful shutdown support |
| Frontend | SvelteKit (planned) | Static adapter, no runtime Node.js, file-based routing |
| Auth | SHA-256 hashed bearer tokens | No new dependency (sha2 already in project) |
| Multi-tenant | Workspace UUID on all tables | Clean data isolation, default workspace for single-user |
| Code org | Feature-based modules | Group by domain feature, not technical layer |

---

## Feature-Based Architecture

As the codebase grows (Tasks 6–10), adopt **feature-based module structure** — group code by domain feature rather than technical layer. Each feature owns its handler, DB queries, client, and types. Shared infrastructure lives in a `shared/` layer.

### Rust Backend — Target Structure

Current flat structure (`src/api/*.rs`, `src/db/*.rs`) works for 5 modules but won't scale to 10+. Migrate incrementally: **each new task creates its feature module** rather than adding files to the flat `api/` and `db/` directories.

```
src/
  features/
    ingest/                  ← Task 3/5: ingest pipeline
      mod.rs                 ← pub use, re-exports
      handler.rs             ← POST /ingest, POST /cancel
      scanner.rs             ← filesystem .md file discovery
      worker.rs              ← process single file (LLM → Postgres → Tantivy)
    queue/                   ← Task 5: Redis Streams infrastructure
      mod.rs
      stream.rs              ← RedisQueue (XADD, XREADGROUP, XACK, cancel)
      consumer.rs            ← spawn_consumers(), consumer loop
    search/                  ← v1: full-text search
      mod.rs
      handler.rs             ← GET /search, GET /facets
      indexer.rs             ← Tantivy index read/write
    summary/                 ← v1: summary types + retrieval
      mod.rs
      handler.rs             ← GET /summary/:file
      types.rs               ← Summary, Relationship, RelationType
    browse/                  ← v1: filesystem browsing
      mod.rs
      handler.rs             ← GET /browse
    semantic/                ← Tasks 6-7: embeddings + vector search
      mod.rs
      handler.rs             ← GET /search/semantic
      embed_client.rs        ← HTTP client for bge-m3 sidecar
      chunker.rs             ← document chunking logic
      milvus.rs              ← Milvus REST API client
    graph/                   ← Task 8: knowledge graph
      mod.rs
      handler.rs             ← GET /graph/entity/:name, GET /graph/document/:id
      neo4j.rs               ← Neo4j Bolt client, MERGE queries
    events/                  ← Task 9: SSE real-time
      mod.rs
      handler.rs             ← GET /events (SSE stream)
      publisher.rs           ← Redis pub/sub publish helper
    auth/                    ← Task 10: workspaces + API keys
      mod.rs
      handler.rs             ← POST /workspaces, POST /api-keys
      middleware.rs          ← Bearer token extraction + validation
      keys.rs                ← key generation, hashing
  shared/
    db/                      ← Postgres pool, migrations, common queries
      mod.rs
      documents.rs           ← document + summary CRUD
      workspaces.rs          ← workspace CRUD (Task 10)
    inference/               ← MLX sidecar client (shared by ingest)
      mod.rs
      client.rs
    config.rs                ← AppConfig (clap)
    state.rs                 ← AppState struct
    health.rs                ← GET /health (cross-cutting)
    shutdown.rs              ← POST /shutdown (cross-cutting)
  main.rs                    ← server startup, routing, wiring
  lib.rs                     ← pub mod declarations
```

**Rules:**
1. Each feature module owns its **handler + business logic + client** — no reaching into another feature's internals
2. Features communicate through **shared types** (re-exported from each feature's `mod.rs`) and **shared infrastructure** (`shared/db`, `shared/config`)
3. `AppState` lives in `shared/state.rs` — features access what they need via Axum extractors
4. Routes are registered in `main.rs` — each feature exposes a `pub fn routes() -> Router<Arc<AppState>>` if complex enough
5. DB queries that serve a single feature live **inside that feature module** (e.g., `graph/neo4j.rs`); cross-feature queries live in `shared/db/`

**Migration strategy:** Incremental, not a big-bang refactor. Each new task creates its feature folder. Existing code gets migrated opportunistically when touched — e.g., when Task 6 adds `semantic/`, move the existing `search/` handler out of `api/search.rs` into `features/search/handler.rs` at the same time.

### SvelteKit Frontend — Target Structure

Apply the same feature-based pattern to the SvelteKit app. Each feature owns its components, API calls, and types. Shared UI primitives live in `lib/shared/`.

```
web/
  src/
    routes/
      +layout.svelte           ← shared shell: sidebar, nav, status bar
      +page.svelte             ← home: search + browse
      admin/
        +page.svelte           ← cancel, shutdown, health dashboard
      graph/
        +page.svelte           ← knowledge graph visualization
    lib/
      features/
        search/                ← full-text + semantic search
          SearchBar.svelte
          SearchResults.svelte
          api.ts               ← GET /search, GET /search/semantic
          types.ts             ← SearchResult, FacetCount
        browse/                ← filesystem browser
          FileBrowser.svelte
          BreadcrumbNav.svelte
          api.ts               ← GET /browse
        ingest/                ← ingestion controls
          IngestButton.svelte
          StatusBar.svelte
          CancelButton.svelte
          api.ts               ← POST /ingest, POST /cancel, GET /status
          types.ts             ← IngestResponse, QueueStatus
        summary/               ← document detail view
          SummaryDrawer.svelte
          RelationshipList.svelte
          api.ts               ← GET /summary/:file
          types.ts             ← Summary, Relationship
        graph/                 ← knowledge graph (Task 8)
          GraphView.svelte
          NodeDetail.svelte
          api.ts               ← GET /graph/entity/:name
          types.ts
        events/                ← SSE live updates (Task 9)
          useEvents.ts         ← EventSource hook/store
          EventToast.svelte
        admin/                 ← server management
          ShutdownButton.svelte
          HealthDashboard.svelte
          api.ts               ← POST /shutdown, GET /health
      shared/
        components/            ← generic UI primitives
          Button.svelte
          Card.svelte
          Badge.svelte
          Spinner.svelte
        api/
          client.ts            ← shared fetch wrapper, base URL, error handling
          types.ts             ← shared API types (ErrorResponse, etc.)
        stores/
          status.ts            ← global queue status store
          theme.ts             ← dark/light mode
        utils/
          format.ts            ← date, number formatting
```

**Rules:**
1. **Routes are thin** — `+page.svelte` files compose feature components, they don't contain business logic
2. Each feature has its own **`api.ts`** — typed fetch calls to the Rust backend, no raw `fetch()` in components
3. **Shared components** are generic UI primitives (Button, Card) — anything domain-specific belongs in its feature folder
4. **Stores** in `shared/stores/` are for truly global state (queue status, theme); feature-local state stays in the feature
5. **Types mirror the Rust API** — TypeScript interfaces in each feature's `types.ts` match the JSON shapes from the backend

## Task Tracking

**Current Task:** Task 7 — Milvus vector store + semantic search
**Next Task:** Task R1 — Backend refactor to feature-based architecture
**Then:** Task 11 — SvelteKit frontend migration (before Tasks 8-9)
**Recently Completed:** Cancel endpoint, graceful shutdown (SIGINT/SIGTERM/API), optional docker teardown
**Blockers:** None
**Dependencies:** Docker Compose stack must be running for integration tests (Postgres 5433, Redis 6379)

> Historical design docs archived in `docs/archive/`.
