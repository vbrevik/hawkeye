# Project knowledge

Local AI-powered markdown summarizer. Point it at a directory of `.md` files → get TL;DR summaries, structured metadata, and full-text search. Summaries are stored in Postgres.

**Stack:** Rust (Axum 0.8, Tantivy 0.25, sqlx 0.8, Tokio) + Python sidecar (mlx-lm, GPT-OSS 20B) — optimised for Apple Silicon.

## Quickstart

- Build: `cargo build --release`
- Run: `cargo run --release` (default port 7700)
- Start MLX sidecar: `./scripts/start_mlx.sh` (port 7701)
- Infrastructure: `docker compose up -d` (Redis, Postgres, etcd, MinIO, Milvus, Neo4j)
- Test: `cargo test`
- Integration tests only: `cargo test --test integration_test`
- Lint: `cargo clippy -- -D warnings`

## Architecture

- `src/main.rs` — Axum server entrypoint, route definitions
- `src/config.rs` — CLI args via `clap::Parser` (AppConfig)
- `src/api/` — Route handlers: ui, ingest, search, status, summary, tags, browse, health
- `src/api/mod.rs` — `AppState` struct (config, queue, inference client, indexer, pg_pool)
- `src/inference/` — MLX sidecar HTTP client
- `src/queue/` — Concurrent work queue (manager + workers)
- `src/scanner/` — Filesystem `.md` file discovery
- `src/search/` — Tantivy full-text index
- `src/db/` — Postgres CRUD (documents, summaries) via sqlx `query_as` runtime checking
- `src/summary/` — `Summary` struct (types only; storage is in Postgres via `src/db/`)
- `tests/integration_test.rs` — Integration tests
- `test_data/` — Sample markdown files for testing
- `docker-compose.yml` — Dev infra (Redis 6379, Postgres 5433, etcd 2379, MinIO 9000, Milvus 19530/9091, Neo4j 7475/7688)

- `migrations/` — sqlx Postgres migrations (run automatically on startup)

**Data flow:** `.md` files → Rust queue → mlx-lm sidecar → Postgres (documents + summaries) + Tantivy index → search API + web UI

## Conventions

- Rust 2021 edition, Axum 0.8 with `Arc<AppState>` shared state
- Config via clap derive macros, all flags have defaults
- Handlers are `async fn` with Axum extractors (`State`, `Query`, `Json`, `Path`)
- Each API module is one file per endpoint group
- SHA-256 content hashing to skip unchanged files on re-ingest (hashes checked against Postgres, not filesystem)
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
- When running `cargo test`, the test config uses its own defaults — some tests may hit localhost:7701 MLX which won't be running
- `clap` defaults in `AppConfig` apply only when the binary is run without args — in tests you often need to set them explicitly
- Model variants: 4-bit (~13GB RAM) vs 8-bit (~22GB) — pass model arg to start script: `./scripts/start_mlx.sh InferenceIllusionist/gpt-oss-20b-MLX-4bit`
- Running server holds Tantivy index lock — kill existing process before starting a new instance
- `sqlx::migrate!()` must be called **without arguments** (defaults to `$CARGO_MANIFEST_DIR/migrations`). Passing `"migrations"` as a string fails with "paths relative to the current file's directory are not currently supported"
- Pre-written code in plan docs drifts fast (ports, config, API shapes). Use **prompt contracts** (GOAL/CONSTRAINTS/FAILURE CONDITIONS) in `docs/BACKLOG.md` instead — they stay valid because they describe *what* to build, not *how*. Historical design docs live in `docs/archive/`
