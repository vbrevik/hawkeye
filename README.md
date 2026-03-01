# Hawkeye

Local AI-powered markdown summarizer. Point it at a directory of `.md` files, get TL;DR summaries and structured metadata, then search across everything.

**Stack:** Rust (Axum, Tantivy, Tokio) + Python (mlx-lm, GPT-OSS 20B 4-bit) — optimised for Apple Silicon.

## How it works

```
.md files → Rust queue → mlx-lm (GPT-OSS 20B) → .summary.json files + Tantivy index → search API + web UI
```

Each file gets a `.summary.json` alongside it:

```json
{
  "source": "meeting-notes.md",
  "source_hash": "sha256:abc123...",
  "tldr": "Discussion about migrating auth to OAuth2...",
  "title": "Auth Migration Meeting",
  "tags": ["auth", "oauth2", "migration"],
  "entities": ["OAuth2", "Auth Service"],
  "topics": ["authentication"],
  "word_count": 847
}
```

Re-runs skip files whose content hasn't changed (SHA-256 comparison).

## Prerequisites

- Rust 1.75+
- Python 3.10+
- ~22GB free RAM (for 8-bit model) or ~13GB (for 4-bit model)
- Apple Silicon Mac (M1/M2/M3/M4)

## Quick Start

**1. Build hawkeye**

```bash
cargo build --release
```

**2. Start the MLX inference sidecar**

```bash
# 8-bit (better quality, ~22GB RAM)
./scripts/start_mlx.sh

# 4-bit (faster start, ~13GB RAM)
./scripts/start_mlx.sh InferenceIllusionist/gpt-oss-20b-MLX-4bit
```

First run downloads the model (~11–22GB). Wait for:
```
INFO:     Started server process
INFO:     Uvicorn running on http://0.0.0.0:7701
```

**3. Start the hawkeye server**

```bash
# Default: port 7700, 4 workers
cargo run --release

# Custom options
cargo run --release -- --port 8080 --workers 8 --mlx-url http://localhost:7701
```

**4. Open the web UI**

Visit [http://localhost:7700](http://localhost:7700)

The status bar shows whether the MLX sidecar is online and live queue progress.

## API

### Start ingestion

```bash
curl -X POST http://localhost:7700/ingest \
  -H 'Content-Type: application/json' \
  -d '{"path": "/Users/you/notes"}'
```

Response:
```json
{"message": "Ingestion started", "files_queued": 1000, "files_skipped": 0}
```

### Check progress

```bash
curl http://localhost:7700/status
```

```json
{"total": 1000, "completed": 342, "failed": 0, "in_progress": 4, "errors": []}
```

### Search

```bash
# Full-text search
curl "http://localhost:7700/search?q=kubernetes+deployment&limit=10"

# Field-specific search
curl "http://localhost:7700/search?q=tags:devops"
curl "http://localhost:7700/search?q=title:incident"
```

Results ranked by relevance score:
```json
[
  {
    "file": "k8s-runbook.md",
    "title": "Kubernetes Runbook",
    "tldr": "Operational guide for the Kubernetes cluster...",
    "tags": "kubernetes devops infrastructure",
    "entities": "Kubernetes Helm ArgoCD",
    "score": 4.213
  }
]
```

### Get facets (tags, topics, entities)

Returns the most frequent tags, topics, and entities across all indexed files:

```bash
curl "http://localhost:7700/facets"
```

```json
{
  "tags": [
    {"name": "kubernetes", "count": 42},
    {"name": "devops", "count": 31}
  ],
  "topics": [
    {"name": "infrastructure", "count": 18}
  ],
  "entities": [
    {"name": "ArgoCD", "count": 9}
  ]
}
```

Returns up to 20 tags, 10 topics, and 15 entities ranked by frequency.

### Browse a directory

```bash
curl "http://localhost:7700/browse?path=/Users/you/notes"
```

```json
{
  "path": "/Users/you/notes",
  "parent": "/Users/you",
  "entries": ["projects", "archive"],
  "md_file_count": 47
}
```

`md_file_count` shows how many `.md` files are in the directory (not recursive).

### Get single file summary

```bash
curl "http://localhost:7700/summary/path/to/notes.md"
```

### Rebuild search index

If you have existing `.summary.json` files but a fresh index:

```bash
curl -X POST http://localhost:7700/reindex
```

## Test data

Generate 1000 test markdown files:

```bash
# Download Wikipedia articles
python3 scripts/generate_test_data.py test_data/

# Generate synthetic notes (fast, no internet needed)
python3 scripts/generate_synthetic_notes.py test_data/ 1000
```

Then ingest:

```bash
curl -X POST http://localhost:7700/ingest \
  -d '{"path": "'$(pwd)'/test_data"}'
```

## Development

```bash
# Run tests
cargo test

# Run integration tests only
cargo test --test integration_test

# Lint
cargo clippy -- -D warnings
```

## Configuration

| Flag | Default | Description |
|------|---------|-------------|
| `--port` | 7700 | Server port |
| `--mlx-url` | http://localhost:7701 | MLX sidecar URL |
| `--workers` | 4 | Concurrent summarization workers |
| `--index-path` | .hawkeye_index | Tantivy index directory |
