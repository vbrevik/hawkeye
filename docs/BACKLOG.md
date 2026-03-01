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
| **Semantic Search** | ✅ | Embedding sidecar (bge-m3) + Milvus vector store + `/search/semantic` endpoint |
| **Frontend** | ✅ | SvelteKit app (search, browse, ingest, facets, detail panel, mobile drawer) |
| **Knowledge Graph** | ✅ | Neo4j with extracted relationships + graph viz page (Task 8) |
| **Real-Time** | ✅ | SSE events for live ingest progress via Redis pub/sub (Task 9) |
| **Multi-Tenant** | ✅ | Workspaces + API keys + revoke (Task 10) |

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

#### Task 7 — Milvus vector store + semantic search ✅
**Priority:** Medium | **Effort:** Medium

- `src/milvus/client.rs` — `MilvusClient` with `ensure_collection()`, `insert_chunks()`, `delete_by_doc_id()`, `search_similar()` (dedup by doc_id) + 5 mock-server unit tests
- `src/api/semantic.rs` — `GET /search/semantic?q=...&limit=N` handler (embed → Milvus search → Postgres enrichment)
- `src/queue/worker.rs` — embed content + store vectors in Milvus after Postgres write (non-critical: warns on failure, doesn't block ingest)
- `src/queue/consumer.rs` — passes embed + milvus clients through to workers
- `src/api/mod.rs` — `AppState` gains `embed: Arc<EmbedClient>` + `milvus: Arc<MilvusClient>`
- `src/db/documents.rs` — `DocSummaryBrief` + `get_doc_summaries_by_ids()` batch query
- 39 unit tests pass, clippy clean

---

#### 🔧 Task R1 — Backend refactor to feature-based architecture ✅
**Priority:** Medium | **Effort:** Small | **When:** After Task 7, before Task 11

- Migrated flat `src/api/*.rs` + `src/db/*.rs` + `src/queue/*.rs` + `src/scanner/*.rs` + `src/search/*.rs` + `src/summary/*.rs` + `src/inference/*.rs` + `src/embedding/*.rs` + `src/milvus/*.rs` + `src/config.rs` into `src/features/` + `src/shared/` structure
- `features/`: ingest/ (handler, scanner, worker), queue/ (stream, consumer), search/ (handler, indexer, facets), semantic/ (handler, embed_client, milvus), summary/ (handler, types), browse/ (handler)
- `shared/`: config.rs, state.rs, db/documents.rs, inference/client.rs, health.rs, shutdown.rs, status.rs, ui.rs
- Rewrote `lib.rs` (`pub mod features; pub mod shared;`) and `main.rs` with all new import paths
- Updated `tests/integration_test.rs` imports
- Deleted all 9 old directories/files
- 39 unit tests pass, clippy clean, no HTTP route changes

---

#### 🔍 Tech Debt Audit 1 — Post-backend stabilization ✅
**When:** After Task R1 (before SvelteKit migration)

Completed. Key findings and actions:

- [x] **Error handling consistency** — unified `AppError` enum (BadRequest/NotFound/Internal) implementing `IntoResponse`; migrated all 7 handlers
- [x] **Test coverage gaps** — added `test_facets_handler` and `test_status_handler` integration tests (10 total, up from 8)
- [x] **Logging consistency** — added tracing to 6 handlers: `handle_search`, `handle_browse`, `handle_summary`, `handle_ingest`, `handle_health`, `handle_mlx_status`
- [x] **`#[allow(dead_code)]` audit** — reviewed; remaining allows are justified (test-only code, Postgres row structs)
- [x] **Duplicate logic** — `DEFAULT_WORKSPACE_ID` constant exists in `shared/config.rs`; `Uuid::nil()` usage is consistent
- [x] **SQL query review** — no N+1 queries found; all queries are bounded

Deferred to later audits: dependency audit, config splitting, full documentation pass

---

### Phase 3: Frontend Foundation

> SvelteKit is done **before** Tasks 8-9 so graph visualization and SSE live updates are built in the proper component framework from day one — no throwaway UI work.

#### Task 11 — SvelteKit frontend migration ✅
**Priority:** Medium | **Effort:** Medium

- SvelteKit app in `web/` with `@sveltejs/adapter-static`, builds to `static/`
- Axum serves `static/` via `tower-http::services::ServeDir` with `index.html` SPA fallback
- 26 Svelte/TypeScript files: 8 API clients, 8 feature components, 2 stores, layout + page + app shell
- Three-column layout: sidebar (logo, inference, browser, queue, facets), center (search + results), right (detail panel)
- Mobile drawer for summary details, ⌘K keyboard shortcut, cancel confirmation, toast notifications
- Dark theme design system (indigo accent, Outfit + JetBrains Mono fonts)
- Vite proxy to `:7700` for dev workflow (`cd web && npm run dev`)
- `src/shared/ui.rs` retained as reference (suppressed with `#[allow(dead_code)]`)
- Unified `AppError` enum created as part of this task (P1 from Tech Debt Audit 1)

---

#### 🔍 Tech Debt Audit 2 — Post-frontend migration ✅
**When:** After Task 11 (combined with Audit 3 into a single pass)

- [x] **Component size** — audited: `+page.svelte` (446 lines) is over threshold but functional as a page shell; GraphCanvas (340) and graph page (278) are acceptable single-responsibility components. DetailPanel (218) and SummaryDrawer (185) are borderline — defer splitting.
- [x] **API type drift** — **FIXED:** `SearchResult` was missing `topics` field in Rust (Tantivy indexed it but didn't return it). `QueueStatus.errors` was `string[]` in TS but `Vec<FileError>` in Rust — added `FileError` interface with `{file, error, attempts}`.
- [x] **Accessibility** — added `aria-label` to SearchBar and graph page search input. ResultCard already had `role="button"` + `tabindex`. Full a11y pass deferred.
- [x] **Bundle size** — checked: largest client chunk 25.69 kB gzip (Svelte runtime), total ~85 kB gzip. No bloat.
- [x] **Backend cleanup** — `src/shared/ui.rs` already deleted in prior task. No orphans found.
- [x] **Shared component extraction** — **FIXED:** extracted `apiFetch<T>()` wrapper in `web/src/lib/api/client.ts` — centralized `res.ok` check, `ApiError` class, auth header hook ready for Task 10. All 8 API modules migrated.
- [x] **Error boundaries** — **FIXED:** `health.ts`, `status.ts`, `search.ts` (fetchFacets) were missing `res.ok` checks — now all use `apiFetch` which throws `ApiError` on non-OK responses. Components that poll (`status.ts` store) catch errors gracefully.

---

### Phase 4: Features Built in SvelteKit

> These tasks add complex UI (graph viz page, live SSE updates) — built directly in SvelteKit components.

#### Task 8 — Neo4j knowledge graph + graph visualization ✅
**Priority:** Medium | **Effort:** Medium

- `neo4rs` 0.8 client: `write_document_graph`, `query_entity` (2-hop traversal, 200 edge limit), `query_document`, `ensure_indexes`
- `GET /graph/entity/{name}` and `GET /graph/document/{id}` endpoints with tracing
- Neo4j config: `neo4j_bolt_url`, `neo4j_user`, `neo4j_password` (dev-only defaults matching docker-compose)
- `neo4j: Option<Arc<Neo4jClient>>` in AppState — graceful degradation when Neo4j unavailable
- Worker writes to Neo4j after Postgres (non-critical: warns on failure, doesn't block ingest)
- Node labels: Document, Entity, Tag, Topic with MERGE (no duplicates)
- Edges: MENTIONS (doc→entity), HAS_TAG (doc→tag), ABOUT (doc→topic), RELATED_TO (entity→entity)
- SvelteKit `/graph` page with force-directed Canvas visualization (zoom, pan, click-to-explore)
- `GraphCanvas.svelte` renders nodes as colored circles by type with labels
- Sidebar nav link "Knowledge Graph →" added to main page
- `test_graph_handler_without_neo4j` integration test (verifies 500 when neo4j is None)
- 11 integration tests pass, clippy clean

#### Task 9 — SSE /events endpoint + live UI updates ✅
**Priority:** Medium | **Effort:** Small

- `features/events/publisher.rs` — `DocumentEvent` enum (Done/Failed) with serde tagged union, `publish_document_event()` via Redis PUBLISH on channel `hawkeye:events:{workspace_id}`
- `features/events/handler.rs` — `GET /events` SSE handler: per-client Redis pub/sub connection, relays messages via mpsc channel, 15-second keepalive comments
- Consumer publishes `document_done`/`document_failed` events after `ack_completed`/`ack_failed`
- `web/src/lib/features/events/useEvents.ts` — `connectEvents()`/`disconnectEvents()` using browser EventSource API; updates queue status store in real-time; shows toast on completion/failure
- Added `tokio-stream` and `async-stream` dependencies
- Live-tested: 32 events streamed correctly during 100-doc ingest, keepalive working
- 39 unit tests pass, clippy clean

---

#### 🔍 Tech Debt Audit 3 — Pre-auth hardening ✅
**When:** Combined with Audit 2 into a single pass

- [x] **Handler signatures** — all handlers use `State(state): State<Arc<AppState>>` as first extractor, then `Path`/`Query`/`Json`. Consistent ✅
- [x] **Workspace ID propagation** — `Uuid::nil()` only in `DEFAULT_WORKSPACE_ID` constant (`shared/config.rs`). All code references the constant. Ready for real workspace IDs ✅
- [x] **Redis key isolation** — all keys workspace-scoped: `hawkeye:jobs:{ws}`, `hawkeye:stats:{ws}:*`, `hawkeye:errors:{ws}`, `hawkeye:events:{ws}` ✅
- [x] **Neo4j data isolation** — ⚠️ **GAP FOUND:** No `workspace_id` in Neo4j queries — graph data is shared across all workspaces. **Deferred to Task 10** — add workspace property to Document nodes and filter in Cypher queries.
- [x] **Milvus data isolation** — `workspace_id` field in collection schema, used in `insert_chunks()` and `search_similar()` filter ✅
- [x] **Integration test isolation** — tests use `Uuid::new_v4()` for unique workspace IDs ✅
- [x] **Frontend API layer** — **FIXED:** extracted `apiFetch()` wrapper with auth header hook (commented placeholder for Task 10). All 8 API modules migrated.
- [x] **Error response consistency** — **FIXED:** `handle_semantic_search` changed from `Json<Vec<...>>` (empty on error) to `Result<Json<...>, AppError>`. All handlers now return `Result<..., AppError>` with `{ error: string }` JSON shape. Only exception: `handle_mlx_status` and `handle_facets` use graceful degradation (return default values on error) — acceptable.

---

### Phase 5: Multi-Tenant

#### Task 10 — Workspaces + API key management ✅
**Priority:** Low | **Effort:** Medium

**GOAL:** `POST /workspaces` creates a workspace. `POST /api-keys` issues a new API key (returns plaintext once, stores SHA-256 hash). All `/workspaces/*` endpoints require `Authorization: Bearer <key>`. Existing public endpoints (`/search`, `/ingest`, `/status`, `/browse`) remain unauthenticated.

**CONSTRAINTS:**
- API key format: `hke_` prefix + 32 random alphanumeric chars
- Hash with `sha2::Sha256` (already in project) — NOT bcrypt
- `POST /workspaces` and `POST /api-keys` are bootstrap endpoints — no auth required
- Auth middleware: extract Bearer token → hash → query `api_keys` table → reject 401 if not found
- `GET /workspaces/:id/docs` returns document list with status from Postgres
- `DELETE /api-keys/:id` revokes a key (idempotent via `COALESCE(revoked_at, now())`)
- Add `rand` crate for key generation

**FAILURE CONDITIONS:**
- API key stored in plaintext in database
- Auth middleware returns 500 instead of 401 for invalid key
- `GET /workspaces/:id/docs` with wrong workspace_id leaks another workspace's docs
- Bootstrap endpoints accidentally protected by auth
- Existing public endpoints break (require auth)

**Completed:**
- `src/features/auth/` — handler (create workspace, create key, revoke key, list docs), middleware (Bearer token extraction + SHA-256 hash lookup), keys (generation with `hke_` prefix + 32 alphanumeric chars)
- `src/shared/db/workspaces.rs` — CRUD for workspaces, API keys (insert, find by hash/id, revoke), workspace doc listing
- `DELETE /api-keys/:id` — revoke endpoint with ownership validation (404 if not found, 401 if wrong workspace)
- Auth middleware rejects revoked keys with "API key has been revoked" message
- `web/src/routes/settings/+page.svelte` — 3-step wizard UI (create workspace → generate API key → view docs)
- `web/src/lib/api/workspaces.ts` — frontend API client for workspace/key management
- `web/src/lib/api/client.ts` — `setAuthToken()` + `apiFetch()` includes Bearer header when token set
- Integration test: `test_workspace_and_auth` with 16 assertions (create, empty name rejection, key generation, auth middleware, cross-workspace isolation, revoke, revoked key rejection)

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
Task 7 (Milvus semantic search)        ✅
  │
  ▼
🔧 Task R1 (Backend refactor)           ✅
  │
  ▼
🔍 Tech Debt Audit 1                    ✅
  │
  ▼
Task 11 (SvelteKit frontend)            ✅
  │
  ▼
🔍 Tech Debt Audit 2                    ✅ (combined with Audit 3)
  │
  ├──► Task 8 (Neo4j + graph viz)          ✅
  ├──► Task 9 (SSE + live UI)              ✅
  │
  ▼
🔍 Tech Debt Audit 3                    ✅ (combined with Audit 2)
  │
  ▼
Task 10 (Workspaces + API keys)        ✅
```

**Completed:** Tasks 1–9, R1, Tech Debt Audits 1–3, Task 11 + cancel/shutdown/docker teardown
**In progress:** —
**Key additions:** 3 tech debt audits at phase boundaries. Recurring hygiene practices applied during every task.

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

**Current Task:** Improvements & Polish
**Next Task:** —
**Recently Completed:** Task 10 (Workspaces + API keys + revoke), Tech Debt Audits 2+3 (combined), Task 9 (SSE /events)
**Blockers:** None
**Dependencies:** Docker Compose stack must be running for integration tests (Postgres 5433, Redis 6379)

> Historical design docs archived in `docs/archive/`.
