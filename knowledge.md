# Project knowledge

Local AI-powered markdown summarizer. Point it at a directory of `.md` files → get TL;DR summaries, structured metadata, and full-text search. Summaries are stored in Postgres.

**Stack:** Rust (Axum 0.8, Tantivy 0.25, sqlx 0.8, deadpool-redis 0.18, neo4rs 0.8, Tokio) + Python sidecars (mlx-lm for LLM inference, infinity-emb for embeddings) — optimised for Apple Silicon. Default LLM: `mlx-community/Qwen3.5-35B-A3B-4bit` (Qwen3 MoE, ~20GB RAM, 4-bit quantised).

## Quickstart

- Build: `cargo build --release`
- Run: `cargo run --release` (default port 7700)
- Start MLX sidecar: `./scripts/start_mlx.sh` (port 7701)
- Start vllm-mlx sidecar (alternative): `./scripts/start_vllm_mlx.sh` (port 7701)
- Start embedding sidecar: `./scripts/start_embed.sh` (port 7703)
- Infrastructure: `docker compose up -d` (Redis, Postgres, etcd, MinIO, Milvus, Neo4j)
- Test: `cargo test`
- Integration tests only: `cargo test --test integration_test`
- Lint: `cargo clippy -- -D warnings`
- Benchmark (inference): `uv run scripts/benchmark.py`
- Benchmark (ingest pipeline): `uv run scripts/benchmark_ingest.py --limit 20 --clean`

## Architecture

Organized into **feature-based modules** (`src/features/`) and **shared infrastructure** (`src/shared/`).

- `src/main.rs` — Axum server entrypoint, route definitions, graceful shutdown
- `src/lib.rs` — `pub mod features; pub mod shared;` for integration tests

### Feature modules (`src/features/`)

- `features/ingest/handler.rs` — `POST /ingest` (scan + queue, optional `limit` param) and `POST /cancel` (discard queued jobs)
- `features/ingest/scanner.rs` — Filesystem `.md` file discovery with SHA-256 hashing
- `features/ingest/worker.rs` — Process single file (LLM → Postgres → Tantivy → embeddings → Milvus)
- `features/queue/manager.rs` — In-memory QueueManager with tokio Semaphore-based concurrency, cancel support, and status tracking
- `features/search/handler.rs` — `GET /search` full-text search via Tantivy, `POST /reindex` rebuild Tantivy index from Postgres
- `features/search/indexer.rs` — Tantivy full-text index read/write (`clear_all`, `index_summaries` batch, `index_summary` single)
- `features/search/facets.rs` — `GET /facets` tag/topic/entity frequency counts
- `features/semantic/handler.rs` — `GET /search/semantic` vector search via Milvus + bge-m3 embeddings
- `features/semantic/embed_client.rs` — bge-m3 embedding sidecar HTTP client (EmbedClient) + text chunking
- `features/semantic/milvus.rs` — Milvus vector DB REST API client (collection mgmt, upsert, search)
- `features/summary/handler.rs` — `GET /summary/{file}` single file summary from Postgres
- `shared/db/documents.rs` also exports `get_all_summaries()` — cross-workspace joined query for reindex
- `features/summary/types.rs` — `Summary`, `Relationship`, `RelationType` structs
- `features/browse/handler.rs` — `GET /browse` filesystem directory listing with md_file_count
- `features/graph/neo4j.rs` — Neo4jClient (connect, write_document_graph, query_entity 2-hop, query_document, ensure_indexes)
- `features/graph/handler.rs` — `GET /graph/entity/{name}` entity graph traversal, `GET /graph/document/{id}` document graph
- `features/events/handler.rs` — `GET /events` SSE stream for real-time ingest progress (per-client Redis pub/sub, 15s keepalive)
- `features/events/publisher.rs` — `DocumentEvent` (Done/Failed) pub/sub for worker → SSE bridge via Redis PUBLISH
- `features/auth/handler.rs` — `POST /workspaces`, `POST /api-keys`, `DELETE /api-keys/{id}`, `GET /workspaces/{id}/docs`
- `features/auth/middleware.rs` — `require_auth` layer: Bearer token extraction → SHA-256 hash → `api_keys` table lookup → reject 401 if not found/revoked
- `features/auth/keys.rs` — `generate_api_key()` (`hke_` prefix + 32 alphanumeric), `hash_api_key()` (SHA-256)

### Shared modules (`src/shared/`)

- `shared/config.rs` — CLI args via `clap::Parser` (AppConfig), `DEFAULT_WORKSPACE_ID`
- `shared/state.rs` — `AppState` struct (config, redis_pool, inference, queue, indexer, pg_pool, embed, milvus, neo4j, shutdown)
- `shared/db/documents.rs` — Postgres CRUD (documents, summaries, relationships) via sqlx
- `shared/inference/client.rs` — MLX sidecar HTTP client (summarization LLM calls)
- `shared/health.rs` — `GET /health` per-service TCP/HTTP health checks with latency
- `shared/shutdown.rs` — `POST /shutdown` graceful server shutdown (optional `?docker=true`)
- `shared/status.rs` — `GET /status` queue progress, `GET /mlx-status` sidecar health
- `shared/db/workspaces.rs` — Workspace + API key CRUD (create, find by hash/id, revoke, list docs)
- `shared/error.rs` — Unified `AppError` enum (BadRequest/NotFound/Internal/Unauthorized/Forbidden) implementing `IntoResponse`

### Scripts

- `scripts/start_mlx.sh` — Start mlx-lm inference sidecar (port 7701)
- `scripts/start_vllm_mlx.sh` — Start vllm-mlx inference sidecar (port 7701, continuous batching)
- `scripts/start_embed.sh` — Start bge-m3 embedding sidecar (port 7703)
- `scripts/embed_server.py` — Python embedding server
- `scripts/benchmark.py` — Inference-level latency/throughput benchmark (direct sidecar calls)
- `scripts/benchmark_ingest.py` — Full pipeline ingest benchmark (POST /ingest → poll → results)
- `scripts/generate_test_data.py` — Generate test markdown files
- `scripts/generate_synthetic_notes.py` — Generate synthetic meeting notes

### Other files

- `tests/integration_test.rs` — Integration tests (require Docker Postgres + Redis)
- `test_data/` — 1000 sample markdown files for testing
- `migrations/` — sqlx Postgres migrations (run automatically on startup)
- `docker-compose.yml` — Dev infra (Redis 6379, Postgres 5433, etcd 2379, MinIO 9000, Milvus 19530/9091, Neo4j 7475/7688)

**Data flow:** `.md` files → in-memory semaphore queue (4 concurrent workers) → mlx-lm sidecar → Postgres (documents + summaries + relationships) + Tantivy index + embeddings → Milvus + Neo4j knowledge graph → Redis pub/sub (document events) → SSE `/events` → SvelteKit frontend (live updates)

### Frontend (`web/`)

- `web/` — SvelteKit SPA (adapter-static → `static/` dir served by Axum via tower-http ServeDir)
- `web/src/app.css` — Global dark theme CSS (design system variables)
- `web/svelte.config.js` — adapter-static outputs to `../static/`
- `web/vite.config.ts` — Vite proxy → localhost:7700 for dev mode
- Build: `cd web && npm run build` → outputs to `static/`
- Dev: `cd web && npm run dev` (port 5173, proxies API to :7700)

#### Routes

- `web/src/routes/+page.svelte` — Main app page (three-column layout: sidebar, results, detail panel)
- `web/src/routes/+layout.svelte` — App shell (CSS import, ToastContainer)
- `web/src/routes/+layout.ts` — SvelteKit layout config (SSR disabled for static adapter)
- `web/src/routes/graph/+page.svelte` — Knowledge graph exploration page with interactive force-directed Canvas visualization
- `web/src/routes/settings/+page.svelte` — Settings page (workspace creation, API key management)

#### API clients (`web/src/lib/api/`)

- `client.ts` — Shared `apiFetch<T>()` wrapper with `ApiError` class, centralized `res.ok` check, auth header hook
- `types.ts` — Shared TypeScript types matching Rust API shapes (includes `FileError` for queue errors)
- `search.ts`, `browse.ts`, `ingest.ts`, `summary.ts`, `status.ts`, `health.ts`, `shutdown.ts`, `graph.ts`, `workspaces.ts` — Per-endpoint API client modules, all use `apiFetch`

#### Feature components (`web/src/lib/features/`)

- `search/SearchBar.svelte` — Full-text search input
- `search/ResultCard.svelte` — Search result display card
- `browse/FileBrowser.svelte` — Filesystem directory listing
- `facets/FacetCloud.svelte` — Tag/topic/entity frequency cloud
- `facets/FacetExplorer.svelte` — Expanded facet exploration panel
- `summary/DetailPanel.svelte` — Document summary detail view
- `summary/SummaryDrawer.svelte` — Slide-out summary drawer
- `sidebar/Sidebar.svelte` — App sidebar (navigation, actions, queue stats)
- `dashboard/Dashboard.svelte` — Landing dashboard view
- `dashboard/Onboarding.svelte` — First-run onboarding flow
- `dashboard/StatsRow.svelte` — Summary statistics row
- `status/QueueStats.svelte` — Queue progress display
- `status/InferenceBlock.svelte` — MLX sidecar status indicator
- `status/HealthPanel.svelte` — Service health overview panel
- `status/SystemHealthCard.svelte` — Individual service health card
- `graph/GraphCanvas.svelte` — Interactive force-directed knowledge graph visualization (Canvas)
- `events/useEvents.ts` — SSE event stream hook for real-time ingest updates
- `settings/CreateWorkspace.svelte` — Workspace creation form
- `settings/GenerateApiKey.svelte` — API key generation step
- `settings/WorkspaceDocs.svelte` — Workspace document listing

#### Stores (`web/src/lib/stores/`)

- `status.ts` — Queue status polling store
- `health.ts` — Service health polling store
- `toast.ts` — Toast notification store

#### Shared components (`web/src/lib/components/`)

- `ToastContainer.svelte` — Global toast notification container

## Conventions

- Rust 2021 edition, Axum 0.8 with `Arc<AppState>` shared state
- Config via clap derive macros, all flags have defaults
- Handlers are `async fn` with Axum extractors (`State`, `Query`, `Json`, `Path`)
- Feature-based module layout: `src/features/` for domain logic, `src/shared/` for cross-cutting concerns
- Auth: Bearer token with `hke_` prefix, SHA-256 hashed in DB. Bootstrap endpoints (`POST /workspaces`, `POST /api-keys`) are unauthenticated; protected endpoints use `require_auth` middleware layer. Public endpoints (`/search`, `/ingest`, `/status`, `/browse`) remain unauthenticated
- SHA-256 content hashing to skip unchanged files on re-ingest (hashes checked against Postgres, not filesystem)
- In-memory semaphore queue (tokio `Semaphore`) for job processing — no Redis Streams, no consumer groups. Queue state (total/completed/failed/in_progress) is held in `Arc<Mutex<QueueState>>` inside `QueueManager`
- Redis pub/sub for real-time SSE events — channel `hawkeye:events:{workspace_id}`, published after document processing completes/fails
- Clippy with `-D warnings` (treat warnings as errors)
- No global package installs; use `cargo` for Rust deps
- Docker ports intentionally offset from defaults (Postgres 5433, Neo4j 7475/7688) to avoid conflicts
- Neo4j Bolt URL defaults to `bolt://localhost:7688`, user `neo4j`, password `hawkeye` (dev-only defaults matching docker-compose)
- Python scripts use `uv run` with inline `# /// script` dependency declarations — no virtualenv needed

## Gotchas

- Milvus health check is on port **9091**, not the main 19530 — the code does `.replace(":19530", ":9091")`
- MLX sidecar runs on **7701** by default (not 8100 as older docs may say)
- Embedding sidecar runs on **7703** by default, serves `BAAI/bge-m3` (1024-dim vectors) via OpenAI-compatible `/v1/embeddings` endpoint
- Default server port is **7700** (not 3000)
- SHA-256 hash is stored in Postgres `documents.source_hash` — skip logic fetches known hashes from Postgres before scanning, so if you manually edit a `.md` file, re-ingest will detect the changed hash and reprocess it
- `Arc<AppState>` is cloned into each handler via `State(state): State<Arc<AppState>>` extractor — the `Arc` means cheap clones, but you still need `.clone()` on the inner fields
- The Tantivy index dir (`.hawkeye_index`) is gitignored — it's created automatically on first run
- `static/` directory (SvelteKit build output) is gitignored — rebuild with `cd web && npm run build`
- `web/node_modules` and `web/.svelte-kit` are gitignored
- Neo4j is **optional** (`neo4j: Option<Arc<Neo4jClient>>` in AppState) — if Neo4j is unreachable on startup, graph features are disabled but everything else works. Graph API handlers return 500 "Knowledge graph not available" when disabled
- Neo4j writes happen **after** Postgres in the worker pipeline and are non-critical — failures are logged but don't fail the ingest
- Neo4j stores entities, tags, topics as nodes, with MENTIONS/HAS_TAG/ABOUT edges from Document nodes, plus RELATED_TO edges between entities (from LLM-extracted relationships)
- `query_entity` does a 2-hop graph traversal (up to 200 edges) to show connected entities, documents, tags, and topics
- Integration tests require Docker Postgres (5433) running — `docker compose up -d`. Redis (6379) is needed only for SSE event tests
- Integration tests use `Uuid::new_v4()` workspace IDs for isolation between parallel tests
- When running `cargo test`, the test config uses its own defaults — some tests may hit localhost:7701 MLX which won't be running
- `clap` defaults in `AppConfig` apply only when the binary is run without args — in tests you often need to set them explicitly
- Default model: `mlx-community/Qwen3.5-35B-A3B-4bit` (~20GB RAM, Qwen3 MoE with 3B active params) — override with: `./scripts/start_mlx.sh <model-id>`
- Channel token extraction: some models (e.g. gpt-oss-20b) wrap output in `<|channel|>analysis<|message|>...` and final JSON in `<|channel|>final<|message|>{json}`. The `extract_json_content()` function in the inference client handles this automatically and is a no-op for models that output JSON directly
- Default workers: **4** (in-memory semaphore concurrency) — GPU inference is the bottleneck, but multiple workers keep the pipeline saturated
- `EmbedClient` chunks text into ~512-token overlapping windows (2048 chars, 50% overlap) before embedding — max chunk size is approximate (1 token ≈ 4 chars)
- Graceful shutdown: server handles SIGINT (Ctrl+C), SIGTERM (`kill`), and `POST /shutdown` — all trigger the same path: stop accepting requests → wait for in-flight responses → signal workers via `watch` channel → wait 3s for worker cleanup → abort remaining → exit
- `POST /shutdown` triggers graceful server shutdown; `POST /shutdown?docker=true` also runs `docker compose down` after the server stops
- `POST /cancel` vs `POST /shutdown`: cancel discards **queued jobs** but keeps the server running; shutdown stops the **entire server process**
- API key format: `hke_` + 32 alphanumeric chars (e.g. `hke_a1b2c3...`). Key is shown once on creation, stored as SHA-256 hash in `api_keys` table. Use `setAuthToken()` in the frontend to persist in browser
- `DELETE /api-keys/:id` is idempotent — uses `COALESCE(revoked_at, now())` so re-revoking doesn't change the timestamp
- Auth middleware checks `revoked_at IS NULL` — revoked keys get a clear "API key has been revoked" 401 message
- `POST /workspaces` and `POST /api-keys` are intentionally unauthenticated (bootstrap flow) — don't add auth middleware to these routes
- Settings page (`/settings`) uses a 3-step wizard: create workspace → generate key → view docs. The API key is stored in the browser via `setAuthToken()` and included in subsequent `apiFetch()` calls
- `POST /reindex` clears the entire Tantivy index and rebuilds from all Postgres summaries (cross-workspace). The indexer mutex is held during clear + batch write, so search queries will briefly block. The sidebar "Rebuild search index" button uses two-click confirmation to prevent accidental triggers
- Running server holds Tantivy index lock — use `POST /shutdown` or `kill` (SIGTERM) instead of `kill -9` to release it cleanly
- LLM prompt extracts relationships in the same call as summaries — `LlmOutput.relationships` uses `#[serde(default)]` so missing field defaults to `[]`
- `RelationType` enum uses `#[serde(other)]` on `Other` variant to handle unknown relationship types from the LLM gracefully
- Queue is purely in-memory — no Redis consumer groups, no stream keys. Each `POST /ingest` spawns a background task that processes files through the `QueueManager`
- `POST /cancel` sets an `AtomicBool` flag checked by workers before processing — already-in-flight jobs finish but queued jobs are skipped. Returns count of cancelled/completed/failed
- Queue status (`GET /status`) is derived from in-memory `QueueState` (total/completed/failed/in_progress/errors)
- `sqlx::migrate!()` must be called **without arguments** (defaults to `$CARGO_MANIFEST_DIR/migrations`). Passing `"migrations"` as a string fails with "paths relative to the current file's directory are not currently supported"
- Pre-written code in plan docs drifts fast (ports, config, API shapes). Use **prompt contracts** (GOAL/CONSTRAINTS/FAILURE CONDITIONS) in `docs/BACKLOG.md` instead — they stay valid because they describe *what* to build, not *how*. Historical design docs live in `docs/archive/`
- Inference client checks HTTP status before parsing JSON — non-2xx responses produce clear "MLX sidecar returned {status}: {body}" errors instead of confusing serde failures
- Inference client HTTP timeout is **300 seconds** (5 minutes) to accommodate slow LLM generation on large documents
- Worker retries use exponential backoff (1s → 2s) between attempts, not instant retries
- `max_tokens: 16384` is set on LLM requests — large enough for Qwen3's extended reasoning + structured JSON output
- Content larger than 100KB is truncated before sending to the LLM, with a warning log
- `/no_think` prefix is conditionally prepended to the system prompt only when the model name contains "qwen3" (case-insensitive) — benign on other models but unnecessary
- `temperature` is configurable via `--temperature` CLI flag (default 0.1)
- Inference latency is logged as `elapsed_ms` via `tracing::info!` after each successful LLM call
- vllm-mlx sidecar supports continuous batching (`--continuous-batching` flag) — main advantage over mlx-lm at high concurrency
- **Recommended config:** mlx-lm sidecar + 4 workers for Qwen3 (GPU-bound inference, multiple workers keep the pipeline saturated). Adjust `--workers` based on model throughput and available RAM
- Benchmark results (Qwen2.5-7B-4bit, Apple Silicon):

  | Config | mlx-lm | vllm-mlx | Winner |
  |--------|--------|----------|--------|
  | 4 workers, 20 docs | 48.6s (0.41 f/s) | 50.6s (0.40 f/s) | mlx-lm +4% |
  | 8 workers, 100 docs | 372.4s (0.27 f/s) | 354.6s (0.28 f/s) | vllm-mlx +5% |

  Throughput drops from ~0.4→0.28 f/s at 8 workers — extra concurrency adds overhead without speeding up GPU-bound inference. vllm-mlx continuous batching advantage is marginal (~5%) even at high concurrency

## Tech Debt Scan

Trigger: user says "scan tech debt", "tech debt audit", "debt scan", "code health check", or "baseline scan".

### Procedure

Run all checks **in parallel** for speed:

1. **Codebase size** — `find src -name '*.rs' -exec wc -l {} + | sort -rn | head -20` (flag files >300 LOC)
2. **Clippy + tests** — `cargo clippy -- -D warnings` and `cargo test --lib | tail -30`
3. **Code smell search** (code-searcher) — `#[allow(dead_code)]`, `unwrap()` in non-test code, `expect(` in runtime paths, `todo!()`, `unimplemented!()`, `FIXME`, `HACK`, `unsafe`
4. **Hardcoded values** — `Uuid::nil()`, hardcoded ports/URLs, magic numbers
5. **Error handling** — `(StatusCode, String)` raw tuples, handlers with no error path, missing `tracing::error!` in error branches
6. **Test coverage** — cross-reference `grep 'pub async fn handle_'` against `grep 'async fn test_'` to find untested handlers
7. **TODOs/unsafe** — `grep -rn 'TODO\|FIXME\|HACK\|XXX' src/` and `grep -rn 'unsafe' src/`

For **TypeScript/SvelteKit** (when `web/` exists): `any` types, `// @ts-ignore`, components >150 LOC, raw `fetch()` (should use `apiFetch` from `client.ts`), `console.log` in production.

### Report Format

Produce a structured report with:
- **Summary table** — file count, LOC, test count, clippy status, TODOs, unsafe blocks
- **🔴 Blockers** — will cause bugs/panics
- **🟡 Issues** — each with: what, where (file:line + count), risk, fix effort
- **⚪ Minor** — style/preference
- **✅ Clean areas** — explicitly verified healthy
- **Priority actions table** — P1 (now), P2 (now), P3 (next task), Defer (milestone)

### After Report

If `docs/TECH_DEBT_BASELINE.md` exists, compare against it (new issues, resolved issues, trends). Then offer:
1. Fix P1-P2 quick wins now
2. Save as baseline to `docs/TECH_DEBT_BASELINE.md`
3. Just note it and continue

### Recurring Hygiene (every task)

1. Boy Scout Rule — leave files cleaner than you found them
2. No new `#[allow(dead_code)]`
3. `cargo clippy -- -D warnings` before every commit
4. Every handler gets integration test; every client gets unit test
5. Update knowledge.md after architectural changes
6. Feature modules import from `crate::shared::*`, never from other features directly
