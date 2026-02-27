# Hawkeye Infrastructure Expansion Design

**Date:** 2026-02-27
**Status:** Approved

## Goal

Expand Hawkeye from a single-machine local summarizer into a production-grade team knowledge platform with semantic search, knowledge graph, and agent-friendly API — while staying on a single machine with Docker-composed services.

## Target users

- Small team (5-50 people) sharing a knowledge base
- Agents and other systems consuming the API

---

## Architecture

```
Hawkeye (Rust/Axum) :7700
Redis               :6379   queue, cache, pub/sub
Postgres            :5432   documents, users, workspaces, API keys
Tantivy             embedded full-text search (unchanged)
Milvus              :19530  vector search, RAG
Neo4j               :7474   knowledge graph
MLX :7701           Qwen3-Next-80B-A3B  (summarize + relationship extraction)
MLX :7703           bge-m3              (embeddings)
Filesystem          original .md files (unchanged, folder picker unchanged)
```

**Not added:** Nginx, HAProxy, RabbitMQ, Garage/MinIO — YAGNI for single-machine deployment.

---

## Ingestion Pipeline

```
File Scanner
    │
    ▼
Redis Stream: hawkeye:ingest:{workspace_id}   (replaces in-memory queue)
    │
    ▼
Worker (stateless, N instances)
    │
    ├─▶ MLX :7701  Single call: summarize + extract relationships
    │       └─▶ {
    │             tldr, title, tags[], entities[], topics[],
    │             relationships: [{from, rel, to, context}]
    │           }
    │
    ├─▶ MLX :7703  bge-m3 embed (per 512-token chunk, overlapping)
    │       └─▶ float[1024] per chunk
    │
    └─▶ Fan-out writes (parallel):
            ├─▶ Postgres  — document record + summary JSONB
            ├─▶ Tantivy   — full-text index (unchanged)
            ├─▶ Milvus    — chunk vectors
            └─▶ Neo4j     — entity nodes + relationship edges
                    │
                    ▼
            Redis Pub/Sub → SSE /events → browser / agents
```

**Key changes from current:**
- Redis Streams replaces `QueueManager` — durable, survives restarts, consumer groups
- Workers become stateless — run N of them, no shared state
- Single LLM call produces both summary and relationships (merged prompt)
- `.summary.json` sidecar files removed — Postgres is source of truth
- Skip logic: `sha256` in Postgres replaces hash-in-filename check

---

## Storage Schemas

### Postgres

```sql
workspaces (
  id          uuid primary key,
  name        text not null,
  slug        text unique not null,
  created_at  timestamptz default now()
)

users (
  id            uuid primary key,
  workspace_id  uuid references workspaces,
  email         text unique,
  role          text,         -- admin | member
  created_at    timestamptz default now()
)

api_keys (
  id          uuid primary key,
  user_id     uuid references users,
  name        text,
  key_hash    text unique,
  last_used_at timestamptz
)

documents (
  id            uuid primary key,
  workspace_id  uuid references workspaces,
  path          text not null,
  sha256        text not null,
  status        text,         -- pending | processing | done | failed
  word_count    int,
  ingested_at   timestamptz,
  updated_at    timestamptz
)

summaries (
  id              uuid primary key,
  document_id     uuid references documents,
  tldr            text,
  title           text,
  tags            text[],
  entities        text[],
  topics          text[],
  relationships   jsonb,      -- [{from, rel, to, context}]
  model_version   text,
  created_at      timestamptz default now()
)

audit_log (
  id          uuid primary key,
  user_id     uuid references users,
  action      text,
  document_id uuid references documents,
  created_at  timestamptz default now()
)
```

### Redis

```
Stream:  hawkeye:ingest:{workspace_id}    job queue (consumer groups)
Hash:    hawkeye:doc:{sha256}             hot summary cache, TTL 1h
Pub/Sub: hawkeye:events:{workspace_id}   completion events → SSE
String:  hawkeye:worker:{id}:heartbeat   liveness, TTL 30s
```

### Milvus

```
Collection: doc_chunks
  doc_id        varchar    FK to Postgres documents.id
  chunk_index   int        0..N (512-token overlapping chunks)
  workspace_id  varchar    for per-workspace filtering
  vector        float[1024] bge-m3 dense embedding
```

### Neo4j

```cypher
-- Node types
(:Document  { id, path, workspace_id })
(:Entity    { name, type })    -- Person | Service | Tool | Team | Concept
(:Tag       { name })
(:Topic     { name })

-- Relationships
(:Document)-[:MENTIONS { context }]->(:Entity)
(:Document)-[:TAGGED_WITH]->(:Tag)
(:Document)-[:COVERS]->(:Topic)

-- LLM-extracted entity relationships
(:Entity)-[:OWNS]->(:Entity)
(:Entity)-[:DEPENDS_ON]->(:Entity)
(:Entity)-[:RELATES_TO { label, context }]->(:Entity)
```

Example queries:
```cypher
-- All docs mentioning the same service as a given incident
MATCH (d:Document {id:$id})-[:MENTIONS]->(e)<-[:MENTIONS]-(other)
RETURN other

-- Two-hop neighbourhood of an entity
MATCH (e:Entity {name:$name})-[*1..2]-(connected)
RETURN connected
```

### Tantivy

Unchanged. Full-text keyword search stays embedded in the Rust binary.

---

## API Changes

```
# Existing (unchanged)
GET  /search?q=...           full-text (Tantivy)
POST /ingest                 queue a folder
GET  /status                 queue stats
GET  /browse?path=...        filesystem browser

# New: semantic + hybrid search
GET  /search/semantic?q=...  vector search via Milvus
GET  /search/hybrid?q=...    Tantivy + Milvus merged, re-ranked

# New: graph
GET  /graph/entity/:name     entity connections (Neo4j)
GET  /graph/document/:id     document entity neighbourhood
GET  /similar/:id            nearest neighbour docs (Milvus)

# New: workspaces + auth
POST /workspaces             create workspace
GET  /workspaces/:id/docs    list documents
POST /api-keys               issue API key for agents

# New: live status
GET  /events                 SSE stream (Redis pub/sub → browser/agent)
```

---

## LLM Prompt Changes

### Merged summarize + relationship extraction (MLX :7701)

```
Respond ONLY with valid JSON:
{
  "tldr": "...",
  "title": "...",
  "tags": ["..."],
  "entities": ["..."],
  "topics": ["..."],
  "relationships": [
    {"from": "Alice", "rel": "owns", "to": "auth-service", "context": "..."},
    {"from": "auth-service", "rel": "depends_on", "to": "postgres", "context": "..."}
  ]
}
```

### Embedding (MLX :7703 — bge-m3)

Called via `/v1/embeddings` endpoint. Document chunked into 512-token overlapping windows before embedding.

---

## Docker Compose Services

```yaml
services:
  hawkeye:     # Rust binary
  redis:       # redis:7-alpine
  postgres:    # postgres:16-alpine
  milvus:      # milvusdb/milvus:v2.4
  neo4j:       # neo4j:5-community
  # MLX sidecars started separately (native macOS, not Docker)
```

---

## What Does Not Change

- File scanner and folder picker UI
- Tantivy full-text search
- MLX sidecar startup script
- The Rust/Axum server as the single entry point
- Single machine deployment
