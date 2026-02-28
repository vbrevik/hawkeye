# Infrastructure Health Panel Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a real-time `GET /health` endpoint that concurrently checks all 7 services (Redis, Postgres, etcd, MinIO, Milvus, Neo4j, Qwen3) and expose the results in a collapsible drawer in the Hawkeye UI sidebar.

**Architecture:** A new `src/api/health.rs` runs 7 checks concurrently via `tokio::join!`, each with a 2s timeout. Status is `up` (<500ms), `degraded` (500–2000ms), or `down` (error/timeout). A separate JS `pollHealth()` function polls every 5s independently of the existing `pollStatus()`. The drawer slides in from the sidebar using CSS transitions only.

**Tech Stack:** Rust/Axum, `tokio::net::TcpStream` (Redis, Postgres), `reqwest` (HTTP checks), vanilla JS, CSS transforms. Zero new Cargo dependencies.

---

### Task 1: Expose etcd and MinIO ports in docker-compose.yml

**Files:**
- Modify: `docker-compose.yml`

**Step 1: Add host port mappings**

In `docker-compose.yml`, find the `etcd` service and add a `ports` block. Find the `minio` service and add its port:

```yaml
etcd:
  # ... existing config ...
  ports:
    - "2379:2379"

minio:
  # ... existing config ...
  ports:
    - "9000:9000"
```

**Step 2: Restart and verify**

```bash
docker compose down && docker compose up -d
```

Then verify both are reachable from host:

```bash
curl -sf http://localhost:2379/health && echo "etcd ok"
curl -sf http://localhost:9000/minio/health/live && echo "minio ok"
```

Expected output:
```
{"health":"true"}
etcd ok
minio ok
```

**Step 3: Commit**

```bash
git add docker-compose.yml
git commit -m "feat: expose etcd (2379) and minio (9000) host ports for health checks"
```

---

### Task 2: Add service URL fields to AppConfig

**Files:**
- Modify: `src/config.rs`

**Step 1: Write the failing test**

Add to `src/config.rs` at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_service_urls() {
        let cfg = AppConfig::parse_from(["hawkeye"]);
        assert_eq!(cfg.redis_url, "redis://localhost:6379");
        assert_eq!(cfg.postgres_url, "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye");
        assert_eq!(cfg.etcd_url, "http://localhost:2379");
        assert_eq!(cfg.minio_url, "http://localhost:9000");
        assert_eq!(cfg.milvus_url, "http://localhost:19530");
        assert_eq!(cfg.neo4j_url, "http://localhost:7475");
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test test_default_service_urls 2>&1 | tail -5
```

Expected: compile error — fields don't exist yet.

**Step 3: Add the six URL fields to AppConfig**

```rust
/// Redis connection URL
#[arg(long, default_value = "redis://localhost:6379")]
pub redis_url: String,

/// Postgres connection URL
#[arg(long, default_value = "postgresql://hawkeye:hawkeye@localhost:5433/hawkeye")]
pub postgres_url: String,

/// etcd base URL
#[arg(long, default_value = "http://localhost:2379")]
pub etcd_url: String,

/// MinIO base URL
#[arg(long, default_value = "http://localhost:9000")]
pub minio_url: String,

/// Milvus base URL
#[arg(long, default_value = "http://localhost:19530")]
pub milvus_url: String,

/// Neo4j HTTP URL
#[arg(long, default_value = "http://localhost:7475")]
pub neo4j_url: String,
```

**Step 4: Run test to verify it passes**

```bash
cargo test test_default_service_urls 2>&1 | tail -5
```

Expected: `test test_default_service_urls ... ok`

**Step 5: Commit**

```bash
git add src/config.rs
git commit -m "feat: add service URL fields to AppConfig with defaults"
```

---

### Task 3: Create `src/api/health.rs` with status types and latency helper

**Files:**
- Create: `src/api/health.rs`

**Step 1: Write the failing test for status classification**

Create `src/api/health.rs` with only the types and the test block:

```rust
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Up,
    Degraded,
    Down,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: ServiceStatus,
    pub latency_ms: Option<u64>,
}

pub fn classify(elapsed: Duration) -> ServiceStatus {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_up() {
        assert_eq!(classify(Duration::from_millis(100)), ServiceStatus::Up);
    }

    #[test]
    fn test_classify_degraded() {
        assert_eq!(classify(Duration::from_millis(700)), ServiceStatus::Degraded);
    }

    #[test]
    fn test_classify_boundary_up() {
        assert_eq!(classify(Duration::from_millis(499)), ServiceStatus::Up);
    }

    #[test]
    fn test_classify_boundary_degraded() {
        assert_eq!(classify(Duration::from_millis(500)), ServiceStatus::Degraded);
    }
}
```

**Step 2: Wire the new module** — add to `src/api/mod.rs`:

```rust
pub mod health;
```

**Step 3: Run tests to verify they fail**

```bash
cargo test test_classify 2>&1 | tail -10
```

Expected: panics with "not yet implemented".

**Step 4: Implement `classify`**

```rust
pub fn classify(elapsed: Duration) -> ServiceStatus {
    let ms = elapsed.as_millis();
    if ms < 500 {
        ServiceStatus::Up
    } else if ms < 2000 {
        ServiceStatus::Degraded
    } else {
        ServiceStatus::Down
    }
}
```

**Step 5: Run tests to verify they pass**

```bash
cargo test test_classify 2>&1 | tail -5
```

Expected: 4 tests pass.

**Step 6: Commit**

```bash
git add src/api/health.rs src/api/mod.rs
git commit -m "feat: add ServiceHealth types and classify() latency helper"
```

---

### Task 4: Implement the 7 service check functions

**Files:**
- Modify: `src/api/health.rs`

**Step 1: Add imports at the top of `src/api/health.rs`**

```rust
use reqwest::Client;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
```

**Step 2: Add the TCP-based check helper (used by Redis and Postgres)**

```rust
async fn tcp_check(addr: &str, name: &str) -> ServiceHealth {
    let deadline = Duration::from_secs(2);
    let start = Instant::now();
    match timeout(deadline, TcpStream::connect(addr)).await {
        Ok(Ok(_)) => ServiceHealth {
            name: name.to_string(),
            status: classify(start.elapsed()),
            latency_ms: Some(start.elapsed().as_millis() as u64),
        },
        _ => ServiceHealth {
            name: name.to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}
```

**Step 3: Add the Redis check (sends PING, expects +PONG)**

```rust
async fn check_redis(addr: &str) -> ServiceHealth {
    let deadline = Duration::from_secs(2);
    let start = Instant::now();
    let result = timeout(deadline, async {
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(b"*1\r\n$4\r\nPING\r\n").await?;
        let mut buf = [0u8; 7];
        tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await?;
        Ok::<_, std::io::Error>(buf.starts_with(b"+PONG"))
    })
    .await;

    let elapsed = start.elapsed();
    match result {
        Ok(Ok(true)) => ServiceHealth {
            name: "redis".to_string(),
            status: classify(elapsed),
            latency_ms: Some(elapsed.as_millis() as u64),
        },
        _ => ServiceHealth {
            name: "redis".to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}
```

**Step 4: Add the HTTP-based check helper (used by etcd, MinIO, Milvus, Neo4j, Qwen3)**

```rust
async fn http_check(client: &Client, url: &str, name: &str) -> ServiceHealth {
    let start = Instant::now();
    match timeout(Duration::from_secs(2), client.get(url).send()).await {
        Ok(Ok(res)) if res.status().is_success() => {
            let elapsed = start.elapsed();
            ServiceHealth {
                name: name.to_string(),
                status: classify(elapsed),
                latency_ms: Some(elapsed.as_millis() as u64),
            }
        }
        _ => ServiceHealth {
            name: name.to_string(),
            status: ServiceStatus::Down,
            latency_ms: None,
        },
    }
}
```

**Step 5: Run all tests to make sure nothing broke**

```bash
cargo test 2>&1 | tail -5
```

Expected: all existing tests still pass.

**Step 6: Commit**

```bash
git add src/api/health.rs
git commit -m "feat: add per-service TCP and HTTP check functions"
```

---

### Task 5: Implement the `handle_health` handler

**Files:**
- Modify: `src/api/health.rs`
- Modify: `src/main.rs`

**Step 1: Add the response types and handler at the bottom of `src/api/health.rs`**

```rust
use crate::api::AppState;
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HealthResponse {
    pub services: Vec<ServiceHealth>,
    pub checked_at: String,
}

pub async fn handle_health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let cfg = &state.config;
    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap_or_default();

    // Extract host:port from redis_url (strip "redis://")
    let redis_addr = cfg.redis_url
        .strip_prefix("redis://")
        .unwrap_or(&cfg.redis_url)
        .to_string();

    // Extract host:port from postgres_url
    let pg_addr = cfg.postgres_url
        .trim_start_matches("postgresql://")
        .trim_start_matches("postgres://")
        .split('@')
        .nth(1)
        .unwrap_or("localhost:5433")
        .to_string();

    let (redis, postgres, etcd, minio, milvus, neo4j, qwen3) = tokio::join!(
        check_redis(&redis_addr),
        tcp_check(&pg_addr, "postgres"),
        http_check(&client, &format!("{}/health", cfg.etcd_url), "etcd"),
        http_check(&client, &format!("{}/minio/health/live", cfg.minio_url), "minio"),
        http_check(&client, &format!("{}/healthz", cfg.milvus_url.replace("19530", "9091")), "milvus"),
        http_check(&client, &cfg.neo4j_url, "neo4j"),
        http_check(&client, &format!("{}/v1/models", cfg.mlx_url), "qwen3"),
    );

    Json(HealthResponse {
        services: vec![redis, postgres, etcd, minio, milvus, neo4j, qwen3],
        checked_at: Utc::now().to_rfc3339(),
    })
}
```

> **Note on Milvus:** The health port (9091) is different from the gRPC port (19530). The handler derives it by replacing 19530 with 9091 in the URL. If `--milvus-url` is customised, also pass `--milvus-health-url` or keep both exposed in compose.

**Step 2: Wire the route in `src/main.rs`**

Add after the existing routes:

```rust
.route("/health", get(api::health::handle_health))
```

**Step 3: Build and smoke-test**

```bash
cargo build 2>&1 | grep -E "^error" | head -10
```

Expected: no errors.

Then start the server and hit the endpoint:

```bash
cargo run -- &
sleep 2
curl -s http://localhost:7700/health | python3 -m json.tool
```

Expected: JSON with 7 service entries, all showing `"up"` if Docker services are running.

**Step 4: Kill the background server**

```bash
kill %1
```

**Step 5: Commit**

```bash
git add src/api/health.rs src/main.rs
git commit -m "feat: add /health endpoint with concurrent 7-service checks"
```

---

### Task 6: Add CSS for the health drawer to `src/api/ui.rs`

**Files:**
- Modify: `src/api/ui.rs`

The CSS lives inside the `<style>` block in the `HTML` const. Find the end of the existing CSS (just before `</style>`) and insert:

**Step 1: Add health-drawer CSS**

```css
/* ── Health drawer ───────────────────────────────────── */
.infra-trigger {
  display: flex; align-items: center; gap: 8px;
  background: var(--surface-2); border: 1px solid var(--border);
  border-radius: var(--r); padding: 8px 10px; cursor: pointer;
  transition: border-color 0.15s;
}
.infra-trigger:hover { border-color: var(--accent); }
.infra-summary { font-size: 11px; color: var(--text-2); flex: 1; }

.health-drawer {
  position: fixed; top: 0; left: var(--sidebar-w);
  width: var(--drawer-w); height: 100vh;
  background: var(--surface); border-right: 1px solid var(--border);
  z-index: 50; transform: translateX(-110%);
  transition: transform 0.22s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex; flex-direction: column;
}
.health-drawer.open { transform: translateX(0); }

.drawer-head {
  display: flex; align-items: center; justify-content: space-between;
  padding: 16px; border-bottom: 1px solid var(--border); flex-shrink: 0;
}
.drawer-title { font-size: 13px; font-weight: 700; color: var(--text); }
.drawer-age { font-size: 10px; color: var(--text-3); margin-top: 2px; }
.drawer-close {
  background: none; border: none; color: var(--text-2);
  cursor: pointer; font-size: 16px; padding: 4px 6px; border-radius: 4px;
}
.drawer-close:hover { background: var(--surface-2); color: var(--text); }

.svc-list { flex: 1; overflow-y: auto; padding: 8px 0; }
.svc-row {
  display: flex; align-items: center; gap: 10px;
  padding: 8px 16px; transition: background 0.1s;
}
.svc-row:hover { background: var(--surface-2); }
.svc-name { font-size: 12px; color: var(--text); flex: 1; font-weight: 500; }
.svc-name.down { color: var(--text-3); }
.svc-status { font-size: 11px; color: var(--text-2); width: 60px; text-align: right; }
.svc-latency { font-size: 11px; color: var(--text-3); font-family: var(--mono); width: 38px; text-align: right; }
```

**Step 2: Verify it compiles**

```bash
cargo build 2>&1 | grep "^error" | head -5
```

Expected: no errors.

**Step 3: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: add health drawer CSS"
```

---

### Task 7: Add HTML for the health drawer to `src/api/ui.rs`

**Files:**
- Modify: `src/api/ui.rs`

**Step 1: Add the sidebar trigger block**

Inside `<aside class="sidebar">`, after the closing `</div>` of the MLX block (around line 256), insert a new section:

```html
    <div>
      <div class="section-label">Infrastructure</div>
      <div class="infra-trigger" onclick="toggleHealthDrawer()">
        <div class="dot" id="infraDot"></div>
        <span class="infra-summary" id="infraSummary">Checking…</span>
        <span style="font-size:10px;color:var(--text-3)">↗</span>
      </div>
    </div>
```

**Step 2: Add the drawer element**

Just before `</body>`, after the closing `</div>` of `.layout`, insert:

```html
<div class="health-drawer" id="healthDrawer">
  <div class="drawer-head">
    <div>
      <div class="drawer-title">Infrastructure Health</div>
      <div class="drawer-age" id="drawerAge">—</div>
    </div>
    <button class="drawer-close" onclick="toggleHealthDrawer()">✕</button>
  </div>
  <div class="svc-list" id="svcList"></div>
</div>
```

**Step 3: Build and verify**

```bash
cargo build 2>&1 | grep "^error" | head -5
```

**Step 4: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: add health drawer HTML and sidebar trigger"
```

---

### Task 8: Add `pollHealth()` JS function to `src/api/ui.rs`

**Files:**
- Modify: `src/api/ui.rs`

**Step 1: Add the JS** — find `pollStatus();` near the bottom of the `<script>` block and insert before it:

```js
  let healthDrawerOpen = false;
  let lastHealthData = null;
  let lastHealthTime = null;

  function toggleHealthDrawer() {
    healthDrawerOpen = !healthDrawerOpen;
    document.getElementById("healthDrawer").classList.toggle("open", healthDrawerOpen);
  }

  function renderHealth(data) {
    lastHealthData = data;
    lastHealthTime = Date.now();

    const services = data.services;
    const worst = services.reduce((acc, s) => {
      if (s.status === "down") return "down";
      if (s.status === "degraded" && acc !== "down") return "degraded";
      return acc;
    }, "up");

    const dot = document.getElementById("infraDot");
    dot.className = "dot" + (worst === "up" ? " online" : worst === "degraded" ? " warn" : "");
    if (worst === "down") dot.style.background = "var(--red)";
    else dot.style.background = "";

    const upCount = services.filter(s => s.status === "up").length;
    setText("infraSummary", upCount + "/" + services.length + " services up");

    const list = document.getElementById("svcList");
    list.replaceChildren();
    for (const svc of services) {
      const row = document.createElement("div");
      row.className = "svc-row";

      const d = document.createElement("div");
      d.className = "dot";
      if (svc.status === "up") { d.className += " online"; }
      else if (svc.status === "degraded") { d.className += " warn"; }
      else { d.style.background = "var(--red)"; }

      const name = document.createElement("div");
      name.className = "svc-name" + (svc.status === "down" ? " down" : "");
      name.textContent = svc.name;

      const status = document.createElement("div");
      status.className = "svc-status";
      status.textContent = svc.status;

      const lat = document.createElement("div");
      lat.className = "svc-latency";
      lat.textContent = svc.latency_ms != null ? svc.latency_ms + "ms" : "—";

      row.append(d, name, status, lat);
      list.appendChild(row);
    }
  }

  function updateDrawerAge() {
    if (!lastHealthTime) return;
    const secs = Math.round((Date.now() - lastHealthTime) / 1000);
    setText("drawerAge", "Last checked " + (secs < 2 ? "just now" : secs + "s ago"));
  }

  async function pollHealth() {
    try {
      const data = await fetch("/health").then(r => r.json());
      renderHealth(data);
    } catch (_) {
      setText("infraSummary", "Health check failed");
    }
  }
```

**Step 2: Add the poll startup calls** — after `pollStatus();`, add:

```js
  pollHealth();
  setInterval(pollHealth, 5000);
  setInterval(updateDrawerAge, 1000);
```

**Step 3: Build**

```bash
cargo build 2>&1 | grep "^error" | head -5
```

**Step 4: End-to-end test**

Start the server and open the UI:

```bash
cargo run &
open http://localhost:7700
```

- Sidebar should show "Infrastructure" section with aggregate dot
- Clicking "↗" opens the drawer
- All running Docker services show green dots with latency
- Qwen3 shows red if MLX isn't running
- Clicking "✕" closes the drawer
- Aggregate dot and count update every 5s without opening drawer

Kill server: `kill %1`

**Step 5: Run all tests**

```bash
cargo test 2>&1 | tail -8
```

Expected: all tests pass.

**Step 6: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: add pollHealth() JS with collapsible drawer rendering"
```

---

### Task 9: Final verification

**Step 1: Run full test suite**

```bash
cargo test 2>&1 | tail -10
```

Expected: all tests pass, 0 failed.

**Step 2: Check all Docker services are healthy**

```bash
docker compose ps
```

Expected: all 6 services `(healthy)`.

**Step 3: Smoke test the health endpoint**

```bash
cargo run &
sleep 2
curl -s http://localhost:7700/health | python3 -m json.tool
kill %1
```

Expected: 7 entries, redis/postgres/etcd/minio/milvus/neo4j all `"up"`, qwen3 `"down"` (unless MLX is running).

**Step 4: Final commit (if any cleanup needed)**

```bash
git add -p
git commit -m "chore: health panel final cleanup"
```
