# Infrastructure Health Panel — Design

**Date:** 2026-02-28
**Status:** Approved

## Summary

Add a real-time infrastructure health panel to the Hawkeye UI. A new `GET /health` backend endpoint checks all 7 services concurrently and returns connectivity status with latency. The UI adds a collapsible drawer triggered from the sidebar showing per-service status rows.

## Decisions

| Topic | Decision |
|-------|----------|
| Health depth | Connectivity + latency only (`up` / `degraded` / `down`) |
| UI placement | Collapsible drawer (slides over content from left) |
| Qwen3 sidecar | Stays external on `localhost:7701`, not containerised |
| Log aggregation | Skip for now — `docker compose logs -f` is sufficient |
| Architecture | Separate `/health` endpoint + separate JS `pollHealth()` loop |
| New dependencies | None — raw `tokio::net::TcpStream` for Redis/Postgres, `reqwest` for the rest |

---

## Section 1: Backend

### Endpoint

```
GET /health
```

Always returns HTTP 200. The payload carries per-service status.

### Response shape

```json
{
  "services": [
    { "name": "redis",    "status": "up",   "latency_ms": 1  },
    { "name": "postgres", "status": "up",   "latency_ms": 3  },
    { "name": "etcd",     "status": "up",   "latency_ms": 5  },
    { "name": "minio",    "status": "up",   "latency_ms": 8  },
    { "name": "milvus",   "status": "up",   "latency_ms": 12 },
    { "name": "neo4j",    "status": "up",   "latency_ms": 20 },
    { "name": "qwen3",    "status": "down", "latency_ms": null }
  ],
  "checked_at": "2026-02-28T21:00:00Z"
}
```

### Status thresholds

| Status | Condition |
|--------|-----------|
| `up` | Responds in < 500ms |
| `degraded` | Responds in 500ms – 2000ms |
| `down` | Error, connection refused, or timeout (> 2s) |

### Per-service check methods

| Service | Method | Address |
|---------|---------|---------|
| Redis | Raw TCP `PING` → expect `+PONG\r\n` | `localhost:6379` |
| Postgres | TCP connect (banner presence = up) | `localhost:5433` |
| etcd | `GET /health` → `{"health":"true"}` | `localhost:2379` |
| MinIO | `GET /minio/health/live` → 200 | `localhost:9000` |
| Milvus | `GET /healthz` → 200 | `localhost:9091` |
| Neo4j | `GET /` → 200 | `localhost:7475` |
| Qwen3 | `GET /v1/models` → 200 | `localhost:7701` |

All 7 checks run concurrently via `tokio::join!`. Each has a hard 2s timeout.

### New file: `src/api/health.rs`

~120 lines. Contains `handle_health` async handler and per-service check functions.

---

## Section 2: UI

### Sidebar trigger

Sits below the existing MLX block in the sidebar. Always visible — shows aggregate status without opening the drawer:

```
● 6/7 services up
[Services ↗]
```

The aggregate dot reflects the worst status across all services:
- Green → all `up`
- Yellow → any `degraded`
- Red → any `down`

### Drawer layout

340px wide panel, slides in from the left over the main content using CSS `transform: translateX` transition. No JS animation library.

```
┌─────────────────────────────────────┐
│ Infrastructure Health        [✕]    │
│ Last checked 2s ago                 │
├─────────────────────────────────────┤
│ ● redis       up       1ms          │
│ ● postgres    up       3ms          │
│ ● etcd        up       5ms          │
│ ● minio       up       8ms          │
│ ● milvus      up      12ms          │
│ ● neo4j       up      20ms          │
│ ● qwen3       down     —            │
└─────────────────────────────────────┘
```

Status colour mapping reuses existing CSS variables:
- `up` → `--green`
- `degraded` → `--yellow`
- `down` → `--red`, row label dimmed

### Polling

`pollHealth()` is a separate JS function from the existing `pollStatus()`. It runs every 5 seconds, starting immediately on page load. It updates the drawer data and aggregate sidebar count whether the drawer is open or closed.

---

## Section 3: Config & Wiring

### AppConfig additions (`src/config.rs`)

Six new URL fields with defaults matching `.env`:

```
--redis-url        redis://localhost:6379
--postgres-url     postgresql://hawkeye:hawkeye@localhost:5433/hawkeye
--etcd-url         http://localhost:2379
--minio-url        http://localhost:9000
--milvus-url       http://localhost:19530
--neo4j-url        http://localhost:7475
```

Qwen3 already uses `--mlx-url`. All URLs stored in `AppConfig`, passed through `AppState`.

### docker-compose.yml changes

Expose two services that currently have no host port mapping:

```yaml
etcd:
  ports:
    - "2379:2379"

minio:
  ports:
    - "9000:9000"
```

### File change summary

| File | Change |
|------|--------|
| `src/config.rs` | 6 new URL fields |
| `src/api/health.rs` | New file, ~120 lines |
| `src/api/mod.rs` | `pub mod health;` |
| `src/main.rs` | 1 new route: `.route("/health", get(api::health::handle_health))` |
| `src/api/ui.rs` | Drawer HTML/CSS/JS (~80 lines) |
| `docker-compose.yml` | 2 host port mappings (etcd, minio) |

No new Cargo dependencies.
