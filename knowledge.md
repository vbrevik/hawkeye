# Project knowledge

Local AI-powered markdown summarizer. Point it at a directory of `.md` files → get TL;DR summaries, structured metadata, and full-text search. Summaries are stored in Postgres.

**Stack:** Rust (Axum 0.8, Tantivy 0.25, sqlx 0.8, deadpool-redis 0.18, Tokio) + Python sidecar (mlx-lm, GPT-OSS 20B) — optimised for Apple Silicon.

## Quickstart

- Build: `cargo build --release`
- Run: `cargo run --release` (default port 7700)
- Start MLX sidecar: `./scripts/start_mlx.sh` (port 7701)
- Start embedding sidecar: `./scripts/start_embed.sh` (port 7703)
- Infrastructure: `docker compose up -d` (Redis, Postgres, etcd, MinIO, Milvus, Neo4j)
- Test: `cargo test`
- Integration tests only: `cargo test --test integration_test`
- Lint: `cargo clippy -- -D warnings`

## Architecture

- `src/main.rs` — Axum server entrypoint, route definitions
- `src/config.rs` — CLI args via `clap::Parser` (AppConfig)
- `src/api/` — Route handlers: ui, ingest, cancel, shutdown, search, status, summary, tags, browse, health
- `src/api/mod.rs` — `AppState` struct (config, redis_pool, indexer, pg_pool, shutdown watch channel, shutdown_docker flag)
- `src/inference/` — MLX sidecar HTTP client
- `src/queue/` — Redis Streams job queue (stream.rs: RedisQueue, consumer.rs: consumer loop, worker.rs: file processor)
- `src/scanner/` — Filesystem `.md` file discovery
- `src/search/` — Tantivy full-text index
- `src/db/` — Postgres CRUD (documents, summaries) via sqlx `query_as` runtime checking
- `src/embedding/` — bge-m3 embedding sidecar HTTP client (EmbedClient) + text chunking
- `src/summary/` — `Summary`, `Relationship`, `RelationType` structs (types only; storage is in Postgres via `src/db/`)
- `tests/integration_test.rs` — Integration tests
- `test_data/` — Sample markdown files for testing
- `docker-compose.yml` — Dev infra (Redis 6379, Postgres 5433, etcd 2379, MinIO 9000, Milvus 19530/9091, Neo4j 7475/7688)

- `migrations/` — sqlx Postgres migrations (run automatically on startup)

**Data flow:** `.md` files → Redis Stream (XADD) → consumer workers (XREADGROUP) → mlx-lm sidecar → Postgres (documents + summaries with relationships) + Tantivy index → search API + web UI

## Conventions

- Rust 2021 edition, Axum 0.8 with `Arc<AppState>` shared state
- Config via clap derive macros, all flags have defaults
- Handlers are `async fn` with Axum extractors (`State`, `Query`, `Json`, `Path`)
- Each API module is one file per endpoint group
- SHA-256 content hashing to skip unchanged files on re-ingest (hashes checked against Postgres, not filesystem)
- Redis Streams for durable job queue — consumer group `hawkeye-workers`, stream key `hawkeye:jobs:{workspace_id}`
- Clippy with `-D warnings` (treat warnings as errors)
- No global package installs; use `cargo` for Rust deps
- Docker ports intentionally offset from defaults (Postgres 5433, Neo4j 7475/7688) to avoid conflicts

## Gotchas

- Milvus health check is on port **9091**, not the main 19530 — the code does `.replace(":19530", ":9091")`
- MLX sidecar runs on **7701** by default (not 8100 as older docs may say)
- Default server port is **7700** (not 3000)
- SHA-256 hash is stored in Postgres `documents.source_hash` — skip logic fetches known hashes from Postgres before scanning, so if you manually edit a `.md` file, re-ingest will detect the changed hash and reprocess it
- `Arc<AppState>` is cloned into each handler via `State(state): State<Arc<AppState>>` extractor — the `Arc` means cheap clones, but you still need `.clone()` on the inner fields
- The Tantivy index dir (`.hawkeye_index`) is gitignored — it's created automatically on first run
- Integration tests require **both** Docker Postgres (5433) **and** Redis (6379) running — `docker compose up -d`
- Integration tests use `Uuid::new_v4()` workspace IDs for Redis key isolation between parallel tests
- When running `cargo test`, the test config uses its own defaults — some tests may hit localhost:7701 MLX which won't be running
- `clap` defaults in `AppConfig` apply only when the binary is run without args — in tests you often need to set them explicitly
- Model variants: 4-bit (~13GB RAM) vs 8-bit (~22GB) — pass model arg to start script: `./scripts/start_mlx.sh InferenceIllusionist/gpt-oss-20b-MLX-4bit`
- Embedding sidecar runs on **7703** by default, serves `BAAI/bge-m3` (1024-dim vectors) via `infinity-emb` with OpenAI-compatible `/v1/embeddings` endpoint
- `EmbedClient` chunks text into ~512-token overlapping windows (2048 chars, 50% overlap) before embedding — max chunk size is approximate (1 token ≈ 4 chars)
- Graceful shutdown: server handles SIGINT (Ctrl+C), SIGTERM (`kill`), and `POST /shutdown` — all trigger the same path: stop accepting requests → wait for in-flight responses → signal consumers via `watch` channel → wait 3s for consumer cleanup → abort remaining → exit
- `POST /shutdown` triggers graceful server shutdown; `POST /shutdown?docker=true` also runs `docker compose down` after the server stops
- `POST /cancel` vs `POST /shutdown`: cancel discards **queued jobs** but keeps the server running; shutdown stops the **entire server process**
- Running server holds Tantivy index lock — use `POST /shutdown` or `kill` (SIGTERM) instead of `kill -9` to release it cleanly
- LLM prompt extracts relationships in the same call as summaries — `LlmOutput.relationships` uses `#[serde(default)]` so missing field defaults to `[]`
- `RelationType` enum uses `#[serde(other)]` on `Other` variant to handle unknown relationship types from the LLM gracefully
- Redis consumer group is created idempotently on startup (`BUSYGROUP` error is swallowed) — no manual setup needed
- `POST /cancel` clears unprocessed jobs from the Redis Stream, adjusts counters so `in_progress` becomes 0, and recreates the consumer group for future ingests — in-flight jobs still finish but won't affect status
- Queue status (`GET /status`) is derived from Redis counters (`hawkeye:stats:{ws}:total/completed/failed`), not in-memory state
- `sqlx::migrate!()` must be called **without arguments** (defaults to `$CARGO_MANIFEST_DIR/migrations`). Passing `"migrations"` as a string fails with "paths relative to the current file's directory are not currently supported"
- Pre-written code in plan docs drifts fast (ports, config, API shapes). Use **prompt contracts** (GOAL/CONSTRAINTS/FAILURE CONDITIONS) in `docs/BACKLOG.md` instead — they stay valid because they describe *what* to build, not *how*. Historical design docs live in `docs/archive/`

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

For **TypeScript/SvelteKit** (when `web/` exists): `any` types, `// @ts-ignore`, components >150 LOC, raw `fetch()`, `console.log` in production.

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
6. Extract to `shared/` after 3+ occurrences of same pattern
