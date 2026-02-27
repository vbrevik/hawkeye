# Eagle3

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

**1. Build eagle3**

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
INFO:     Uvicorn running on http://0.0.0.0:8100
```

**3. Start the eagle3 server**

```bash
# Default: port 3000, 4 workers
cargo run --release

# Custom options
cargo run --release -- --port 8080 --workers 8 --mlx-url http://localhost:8100
```

**4. Open the web UI**

Visit [http://localhost:3000](http://localhost:3000)

The status bar shows whether the MLX sidecar is online and live queue progress.

## API

### Start ingestion

```bash
curl -X POST http://localhost:3000/ingest \
  -H 'Content-Type: application/json' \
  -d '{"path": "/Users/you/notes"}'
```

Response:
```json
{"message": "Ingestion started", "files_queued": 1000, "files_skipped": 0}
```

### Check progress

```bash
curl http://localhost:3000/status
```

```json
{"total": 1000, "completed": 342, "failed": 0, "in_progress": 4, "errors": []}
```

### Search

```bash
# Full-text search
curl "http://localhost:3000/search?q=kubernetes+deployment&limit=10"

# Field-specific search
curl "http://localhost:3000/search?q=tags:devops"
curl "http://localhost:3000/search?q=title:incident"
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

### Get single file summary

```bash
curl "http://localhost:3000/summary/path/to/notes.md"
```

### Rebuild search index

If you have existing `.summary.json` files but a fresh index:

```bash
curl -X POST http://localhost:3000/reindex
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
curl -X POST http://localhost:3000/ingest \
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
| `--port` | 3000 | Server port |
| `--mlx-url` | http://localhost:8100 | MLX sidecar URL |
| `--workers` | 4 | Concurrent summarization workers |
| `--index-path` | .eagle3_index | Tantivy index directory |
