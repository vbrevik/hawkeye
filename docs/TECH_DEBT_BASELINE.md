# Hawkeye Tech Debt Baseline

> Captured: Post-Task R1 + Tech Debt Audit 1 (after AppError refactor, integration tests, SvelteKit frontend)
> Commit: `ab6fe30` on `feat/facets-browse-md-count`

## Summary Dashboard

| Metric | Value |
|--------|-------|
| Rust files | 35 |
| Rust LOC | 4,697 |
| TypeScript/Svelte files | 24 |
| TypeScript/Svelte LOC | 1,643 |
| Svelte components | 11 |
| **Total project LOC** | **6,340** |
| Unit tests | 39 passing ✅ |
| Integration tests | 10 passing ✅ |
| Clippy | ✅ Clean (`-D warnings`) |
| unwrap() in prod | 0 ✅ |
| expect() in prod | 0 ✅ |
| TODO/FIXME/HACK | 0 ✅ |
| unsafe blocks | 0 ✅ |
| console.log (frontend) | 0 ✅ |
| `any` type (TS) | 0 ✅ |
| Handler test coverage | 8/12 (67%) |

## Handler Test Coverage

| Handler | Integration Test |
|---------|-----------------|
| handle_ingest | ✅ test_full_pipeline |
| handle_cancel | ✅ test_cancel_ingestion |
| handle_search | ✅ test_search_handler |
| handle_facets | ✅ test_facets_handler |
| handle_browse | ✅ test_browse_handler |
| handle_summary | ✅ test_summary_handler |
| handle_health | ✅ test_health_handler |
| handle_status | ✅ test_status_handler |
| handle_semantic_search | ❌ (needs embedding sidecar) |
| handle_shutdown | ❌ (destructive — kills server) |
| handle_mlx_status | ❌ (needs MLX sidecar) |

## 🟡 MEDIUM — Remaining Issues

### 1. `#[allow(dead_code)]` — 6 instances across 3 files

| Location | Reason | Action |
|----------|--------|--------|
| `shared/db/documents.rs:7,19` | `DocumentRow`, `SummaryRow` — fields populated by sqlx::FromRow | Legitimate — keep |
| `shared/db/documents.rs:59,148` | `get_document_by_path`, `get_summary_by_document` — used only in integration tests | Legitimate — `#[cfg(test)]` won't work for integration tests (external crate) |
| `features/queue/stream.rs:218` | `cleanup()` — used only in integration tests | Same as above |
| `features/search/indexer.rs:23` | `schema` field — stored for potential future use | Low risk, keep |

### 2. Unbounded SELECTs — 2 genuine concerns

| Query | Line | Risk |
|-------|------|------|
| `get_source_hashes` | documents.rs:90 | Medium — returns ALL hashes for workspace |
| `get_all_summaries_brief` | documents.rs:179 | Medium — returns ALL summaries for workspace |

Other SELECTs without LIMIT return single rows by unique constraint — not a concern.

**Action:** Add pagination during Task 10 (multi-tenant).

### 3. Untested handlers — 3 remaining

- `handle_semantic_search` — needs embedding sidecar + Milvus mock
- `handle_shutdown` — destructive (kills server process)
- `handle_mlx_status` — needs MLX sidecar running

**Action:** Mock-based tests after Tasks 8-9.

### 4. `Uuid::nil()` default workspace ID

Single instance in `shared/config.rs:6`. Intentional pre-auth placeholder.

**Action:** Replace during Task 10 (workspaces + API keys).

## 🟢 LOW — Minor Items

### 5. Large files (>300 LOC)

| File | LOC | Status |
|------|-----|--------|
| `features/semantic/milvus.rs` | 416 | REST client, reasonable for scope |
| `shared/inference/client.rs` | 345 | LLM client with tests, reasonable |
| `features/queue/stream.rs` | 301 | Redis Streams, at threshold |

### 6. Missing tooling

`cargo-outdated` and `cargo-udeps` not installed — can't audit dependency freshness or unused deps.

## ✅ Clean Areas (Verified Healthy)

- Zero unwrap/expect in production code
- Zero TODO/FIXME/HACK markers
- Zero unsafe blocks
- Zero console.log in frontend code
- Zero `any` types in TypeScript
- All SQL columns properly indexed (workspace_id, source_path, source_hash, document_id)
- Consistent error handling via `AppError` enum with JSON responses
- Feature-based architecture — clean separation, no circular dependencies
- No duplicate code — single source of truth for every module
- Clippy clean with `-D warnings`

## Priority Actions

| Priority | Action | Effort | When |
|----------|--------|--------|------|
| P3 | Move test-only DB functions behind a cargo feature flag | Small | Next refactor |
| P3 | Add pagination to `get_source_hashes` and `get_all_summaries_brief` | Small | Task 10 |
| Defer | Mock-based tests for semantic_search and mlx_status | Medium | After Tasks 8-9 |
| Defer | Install `cargo-outdated` + `cargo-udeps` | Tiny | CI setup |

## Historical Trend

| Scan | 🔴 Critical | 🟠 High | 🟡 Medium | 🟢 Low |
|------|------------|---------|-----------|--------|
| Pre-fix (initial) | 1 (duplicate modules) | 2 (clippy, low test coverage) | 3 | 2 |
| **Post-fix (this baseline)** | **0** | **0** | **4** | **2** |

**Overall health: GOOD — no blockers, no high-priority issues. Ready for next phase.**
