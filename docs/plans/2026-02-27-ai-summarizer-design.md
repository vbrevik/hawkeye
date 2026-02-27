# Eagle3: Local AI Summarizer — Design Document

**Date:** 2026-02-27
**Status:** Approved

## Problem

10,000+ markdown files containing notes and documentation. Too much information to find anything. Need automated summarization with both quick TL;DR and structured metadata extraction, plus a search API to query results.

## Constraints

- Mac M2, 64GB unified RAM, local only
- Backend must be Rust
- Must handle 10,000 files via queue/batch system

## Approach: MLX-native

**Inference:** GPT-OSS 20B in MLX 4-bit format (~11GB) via `mlx-lm serve`, exposed as OpenAI-compatible API on localhost:8100. MLX-native speculative decoding for speed. ~40 tok/s on M2.

**Why not vLLM + Eagle-3:** vLLM requires CUDA. Eagle-3 speculator is a vLLM/CUDA feature. MLX's own speculative decoding is the best option on Apple Silicon.

**Models:**
- Main: `mlx-community/gpt-oss-20b-mlx-q4` or `InferenceIllusionist/gpt-oss-20b-MLX-4bit`
- Speculative draft: MLX-native draft model (TBD based on available drafts)

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Rust Server (Axum)                 │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌───────────────────┐  │
│  │ REST API │  │  Queue   │  │  Tantivy Search   │  │
│  │ /ingest  │─▶│ Manager  │  │  Index            │  │
│  │ /search  │  │ (tokio)  │  └───────────────────┘  │
│  │ /status  │  └────┬─────┘           ▲              │
│  └──────────┘       │                 │              │
│                     ▼                 │              │
│            ┌────────────────┐   ┌─────┴─────┐       │
│            │  Batch Worker  │──▶│ Summary   │       │
│            │  Pool (N=4)   │   │ Store     │       │
│            └───────┬────────┘   │ (.json)   │       │
│                    │            └───────────┘       │
└────────────────────┼────────────────────────────────┘
                     │ HTTP (localhost)
                     ▼
┌────────────────────────────────────────┐
│  Python Sidecar (mlx-lm serve)        │
│  OpenAI-compatible API on :8100       │
│  Model: gpt-oss-20b-MLX-4bit         │
└────────────────────────────────────────┘
```

Three processes:
1. **Rust server** — API, queue, search index, summary storage
2. **Python sidecar** — `mlx-lm serve` running GPT-OSS 20B, stateless inference
3. **Static JSON files** — one `.summary.json` per input `.md` file

## Queue & Batch System

### Flow

1. Directory scan (glob `*.md`)
2. Priority queue — skip files with up-to-date `.summary.json` (SHA-256 match), sort small files first
3. 4 concurrent tokio worker tasks (configurable via CLI)
4. Rate-limited: max 4 in-flight requests to mlx-lm
5. Each file gets a single chat completion call producing both TL;DR and structured metadata

### Skip Logic

If `foo.md` has `foo.summary.json` and the json's `source_hash` matches the md's SHA-256, skip. Re-runs are fast.

### Failure Handling

Failed files go to retry queue (max 3 attempts), then written to `errors.log`.

### Prompt Strategy

Single prompt per file extracts both:
- TL;DR (2-3 sentence summary)
- Structured data (title, tags, entities, topics)

## API

```
POST /ingest          Start processing a directory
  body: { "path": "/path/to/md/files" }

GET  /status          Queue progress
  returns: { total, completed, failed, in_progress, errors[] }

GET  /search          Full-text search across summaries
  params: ?q=kubernetes&tags=devops&limit=20
  returns: [{ file, tldr, tags, entities, score }]

GET  /summary/:file   Single file summary
  returns: { tldr, title, tags, entities, topics, source_hash }

POST /reindex         Rebuild Tantivy index from .summary.json files
```

## Search

Tantivy (Rust-native full-text search, like Lucene without JVM).

Index fields: `tldr`, `title`, `tags`, `entities`, `topics`, `source_path`

Indexed automatically after each file completes. Supports ranked results, phrase queries, field-specific search (`tags:devops`).

Index stored in `.eagle3_index/` directory.

## Summary JSON Format

```json
{
  "source": "meeting-notes-jan-15.md",
  "source_hash": "sha256:ab3f...",
  "created_at": "2026-02-27T18:30:00Z",
  "tldr": "Discussion about migrating auth service to OAuth2...",
  "title": "Auth Migration Meeting Notes",
  "tags": ["auth", "oauth2", "migration"],
  "entities": ["Auth Service", "OAuth2", "Team Backend"],
  "topics": ["authentication", "infrastructure"],
  "word_count": 847
}
```

## Project Structure

```
eagle3/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry + Axum server startup
│   ├── config.rs            # CLI args, env config
│   ├── api/
│   │   ├── mod.rs
│   │   ├── ingest.rs        # POST /ingest
│   │   ├── search.rs        # GET /search
│   │   ├── status.rs        # GET /status
│   │   └── summary.rs       # GET /summary/:file
│   ├── queue/
│   │   ├── mod.rs
│   │   ├── manager.rs       # Queue state, enqueue/dequeue
│   │   └── worker.rs        # Worker loop, calls inference
│   ├── inference/
│   │   ├── mod.rs
│   │   └── client.rs        # HTTP client to mlx-lm
│   ├── search/
│   │   ├── mod.rs
│   │   └── indexer.rs       # Tantivy index + query
│   ├── summary/
│   │   ├── mod.rs
│   │   └── store.rs         # Read/write .summary.json
│   └── scanner/
│       ├── mod.rs
│       └── files.rs         # Directory scan, hash, skip
├── scripts/
│   └── start_mlx.sh         # Launch mlx-lm serve
├── tests/
│   ├── integration/
│   │   ├── api_test.rs
│   │   └── queue_test.rs
│   └── unit/
│       ├── scanner_test.rs
│       └── summary_test.rs
└── docs/
    └── plans/
```

## Key Dependencies

- `axum` + `tokio` — async server & runtime
- `tantivy` — search index
- `reqwest` — HTTP client to mlx-lm
- `serde` / `serde_json` — JSON
- `sha2` — file hashing
- `clap` — CLI args
- `tracing` — structured logging

## Testing Strategy

- **Unit tests** — scanner hashing, summary serialization, queue ordering
- **Integration tests** — mock mlx-lm endpoint, test full flow: ingest → queue → inference → summary → index → search
- **Manual E2E** — test against live mlx-lm (too slow for CI)
