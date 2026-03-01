# 🔍 Tech Debt Baseline — Hawkeye

**Date:** 2026-03-01
**Commit:** c1323f1 → updated efc3302
**Scanned:** 27 files, 3,224 LOC

## Summary

| Metric | Value | Assessment |
|--------|-------|------------|
| **Rust source files** | 27 | Manageable |
| **Total LOC (src/)** | 3,224 | Small project |
| **Integration test LOC** | 433 | Good coverage |
| **Unit tests** | 26 passing | ✅ |
| **Integration tests** | 4 passing | ✅ |
| **Clippy** | Clean (`-D warnings`) | ✅ |
| **TODOs/FIXMEs/HACKs** | 0 | ✅ |
| **Unsafe blocks** | 0 | ✅ |

## Largest Files

| File | LOC | Notes |
|------|-----|-------|
| `src/api/ui.rs` | 972 | Inline HTML monolith — deleted by Task 11 (SvelteKit) |
| `src/queue/stream.rs` | 301 | RedisQueue — reasonable for its scope |
| `src/search/indexer.rs` | 276 | Tantivy indexer — includes tests |
| `src/inference/client.rs` | 211 | MLX client — includes tests |
| `src/db/documents.rs` | 197 | Postgres CRUD — will grow with features |
| `src/api/health.rs` | 181 | Health checks — includes tests |

## 🔴 Blockers

None found.

## 🟡 Issues

### ~~1. `Uuid::nil()` hardcoded everywhere (9 occurrences)~~ ✅ RESOLVED (efc3302)

- **Fix applied:** Extracted `DEFAULT_WORKSPACE_ID` constant in `src/config.rs`, replaced all 9 occurrences across 7 files

### ~~2. `#[allow(dead_code)]` — 6 occurrences~~ ✅ RESOLVED (efc3302)

- **Fix applied:** Audited all 6 annotations — all are legitimate (structs populated by `sqlx::FromRow`, functions used in integration tests, field used internally). Added explanatory comments to each so intent is clear

### 3. No unified error type

- **Where:** `src/api/ingest.rs`, `src/api/summary.rs`, `src/api/search.rs`, `src/api/browse.rs`
- **Pattern:** Raw `(StatusCode, String)` tuples with repeated `.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?`
- **Risk:** Inconsistent error responses for frontend; boilerplate accumulation
- **Fix effort:** 30 min — create `AppError` enum implementing `IntoResponse`

### ~~4. `expect()` in production paths~~ ✅ RESOLVED (efc3302)

- **Fix applied:** `InferenceClient::new()` now returns `Result<Self, reqwest::Error>` instead of panicking. All 10 call sites updated. `main.rs` startup expects kept (appropriate for startup panics)

### 5. Error handling inconsistency

- **What:** Some handlers return `Result<Json<T>, (StatusCode, String)>` (ingest, summary, search, browse); others return `Json<T>` directly with no error path (status, health, facets, ui, shutdown)
- **Where:** `src/api/status.rs` silently returns zeros on Redis error (logged, but caller doesn't know)
- **Risk:** Frontend can't distinguish "no jobs" from "Redis is down"
- **Fix effort:** Defer to Task R1 (unified AppError)

### 6. Missing integration test coverage

- **What:** Only 4 integration tests. Not tested via HTTP: `GET /search`, `GET /facets`, `GET /browse`, `GET /health`, `POST /shutdown`, `GET /summary/:file` (tested indirectly via DB, not via HTTP handler)
- **Risk:** Regressions in handler wiring won't be caught
- **Fix effort:** Fill incrementally with each task

### ~~7. Missing logging in handlers~~ ⚠️ PARTIALLY RESOLVED (efc3302)

- **Fix applied:** Added `tracing::error!` to `handle_facets` in `src/api/tags.rs` (was silently swallowing Redis errors via `unwrap_or_default()`)
- **Remaining:** `handle_search`, `handle_browse`, `handle_summary`, `handle_health` still lack `tracing` calls — defer to Task R1 (unified AppError will add structured logging to all handlers)

## ⚪ Minor

### 8. `clone()` proliferation in main.rs

- **Where:** `src/main.rs` — 8 clones during startup wiring
- **Risk:** None (startup only, not a hot path)
- **Fix effort:** Could be cleaner but not worth refactoring now

### 9. Large file: `src/api/ui.rs` — 972 lines

- **What:** Inline HTML/CSS/JS monolith
- **Risk:** Unmaintainable for adding new UI features
- **Fix effort:** Deleted entirely by Task 11 (SvelteKit migration)

## ✅ Clean Areas

- **No TODOs/FIXMEs/HACKs** — codebase is intentional
- **No unsafe code** — all safe Rust
- **Clippy clean** — `-D warnings` enforced
- **All tests pass** — 23 unit + 4 integration
- **Good test structure** — unit tests co-located in modules, integration tests in separate file
- **Consistent Axum patterns** — extractors, handler signatures follow conventions
- **No circular dependencies** — clean module graph
- **Config via clap** — all service URLs configurable with sensible defaults

## Recommended Priority Actions

| Priority | Action | Effort | Status |
|----------|--------|--------|--------|
| ~~**P1**~~ | ~~Extract `DEFAULT_WORKSPACE_ID` constant~~ | ~~5 min~~ | ✅ Done (efc3302) |
| ~~**P2**~~ | ~~Audit and clean up `#[allow(dead_code)]`~~ | ~~10 min~~ | ✅ Done (efc3302) |
| ~~**P3**~~ | ~~Add `tracing::error!` to handlers~~ | ~~15 min~~ | ⚠️ Partial (efc3302) |
| ~~**P3**~~ | ~~Fix `expect()` in `InferenceClient::new()`~~ | ~~5 min~~ | ✅ Done (efc3302) |
| **Defer** | Unified `AppError` type + remaining handler logging | 30 min | Task R1 refactor |
| **Defer** | Integration test coverage gaps | Incremental | Each task |
| **Defer** | `ui.rs` (972 LOC) | Deleted | Task 11 |
