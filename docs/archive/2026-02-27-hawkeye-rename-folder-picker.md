# Hawkeye Rename + Filesystem Folder Picker Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rename the project from `eagle3` to `hawkeye`, then replace the sidebar text-input ingest section with an inline linear-drill-down filesystem browser.

**Architecture:** A new `GET /browse?path=...` Axum endpoint serves directory listings from the server filesystem. The embedded HTML frontend replaces the ingest text input with a scrollable directory list, breadcrumb, and "Ingest here" button that navigate the local filesystem and trigger ingestion.

**Tech Stack:** Rust/Axum (backend), vanilla JS/HTML/CSS embedded in `src/api/ui.rs` (frontend), `std::fs::read_dir` for directory listing. All JS DOM manipulation uses `createElement`/`textContent` — no `innerHTML` with dynamic data.

---

## Task 1: Rename eagle3 → hawkeye (Cargo + config)

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/config.rs`
- Modify: `src/main.rs`

**Step 1: Update Cargo.toml**

Change the package name and add an explicit binary entry:

```toml
[package]
name = "hawkeye"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "hawkeye"
path = "src/main.rs"
```

**Step 2: Update src/config.rs**

```rust
// line 4 — change command name and index default:
#[command(name = "hawkeye", about = "Local AI-powered markdown summarizer")]
// ...
    #[arg(long, default_value = ".hawkeye_index")]
    pub index_path: String,
```

**Step 3: Update src/main.rs**

```rust
// tracing filter line:
.with_env_filter("hawkeye=info")
// startup log line:
tracing::info!("hawkeye listening on http://{}", addr);
```

**Step 4: Verify build**

```bash
cargo build 2>&1 | tail -5
ls target/debug/hawkeye
```

Expected: `target/debug/hawkeye` binary exists, no errors.

**Step 5: Commit**

```bash
git add Cargo.toml src/config.rs src/main.rs
git commit -m "chore: rename package and binary to hawkeye"
```

---

## Task 2: Rename remaining eagle3 references (UI + scripts + README)

**Files:**
- Modify: `src/api/ui.rs`
- Modify: `scripts/start_mlx.sh`
- Modify: `README.md`

**Step 1: Update src/api/ui.rs**

Find all occurrences:
```bash
grep -n "eagle3\|Eagle3" src/api/ui.rs
```

Change `<title>Eagle3</title>` → `<title>Hawkeye</title>` and any other branding strings in the HTML.

**Step 2: Update scripts/start_mlx.sh**

```
# Line 4: Eagle3 MLX Inference Sidecar → Hawkeye MLX Inference Sidecar
# Line 16: === Eagle3 MLX Sidecar === → === Hawkeye MLX Sidecar ===
```

**Step 3: Update README.md**

```bash
sed -i '' 's/eagle3/hawkeye/g; s/Eagle3/Hawkeye/g' README.md
grep -n "eagle3\|Eagle3" README.md   # verify none remain
```

**Step 4: Verify zero remaining references**

```bash
grep -rn "eagle3\|Eagle3" src/ scripts/ README.md Cargo.toml
```

Expected: zero matches.

**Step 5: Build**

```bash
cargo build 2>&1 | tail -3
```

**Step 6: Commit**

```bash
git add src/api/ui.rs scripts/start_mlx.sh README.md
git commit -m "chore: rename remaining eagle3 references to hawkeye"
```

---

## Task 3: Add /browse API endpoint

**Files:**
- Create: `src/api/browse.rs`
- Modify: `src/api/mod.rs`
- Modify: `src/main.rs`

**Step 1: Write the failing tests**

Create `src/api/browse.rs` with a test module only (no implementation yet):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_list_dir_returns_only_dirs() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        assert!(result.parent.is_some());
        let home_path = std::path::PathBuf::from(&home);
        for entry in &result.entries {
            assert!(home_path.join(entry).is_dir());
        }
    }

    #[test]
    fn test_list_dir_root_has_no_parent() {
        let result = list_dir(&std::path::PathBuf::from("/")).unwrap();
        assert_eq!(result.parent, None);
    }

    #[test]
    fn test_list_dir_hides_dotfiles() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        for entry in &result.entries {
            assert!(!entry.starts_with('.'), "dotfile should be hidden: {entry}");
        }
    }
}
```

**Step 2: Add module stub so tests compile**

Add to `src/api/mod.rs`:
```rust
pub mod browse;
```

**Step 3: Run tests to verify they fail**

```bash
cargo test list_dir 2>&1 | tail -10
```

Expected: compile error — `list_dir` not found.

**Step 4: Implement src/api/browse.rs**

```rust
use crate::api::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct BrowseQuery {
    pub path: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BrowseResponse {
    pub path: String,
    pub parent: Option<String>,
    pub entries: Vec<String>,
}

pub fn list_dir(dir: &PathBuf) -> Result<BrowseResponse, String> {
    let path = dir
        .canonicalize()
        .map_err(|e| format!("Cannot resolve path: {e}"))?;

    let parent = path
        .parent()
        .filter(|p| *p != path.as_path()) // root's parent == itself
        .map(|p| p.to_string_lossy().into_owned());

    let mut entries: Vec<String> = std::fs::read_dir(&path)
        .map_err(|e| format!("Cannot read directory: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') { None } else { Some(name) }
        })
        .collect();
    entries.sort();

    Ok(BrowseResponse {
        path: path.to_string_lossy().into_owned(),
        parent,
        entries,
    })
}

pub async fn handle_browse(
    State(_state): State<Arc<AppState>>,
    Query(q): Query<BrowseQuery>,
) -> Result<Json<BrowseResponse>, (StatusCode, String)> {
    let default_home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let path_str = q.path.unwrap_or(default_home);
    let dir = PathBuf::from(&path_str);

    list_dir(&dir).map(Json).map_err(|e| (StatusCode::BAD_REQUEST, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_list_dir_returns_only_dirs() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        assert!(result.parent.is_some());
        let home_path = std::path::PathBuf::from(&home);
        for entry in &result.entries {
            assert!(home_path.join(entry).is_dir());
        }
    }

    #[test]
    fn test_list_dir_root_has_no_parent() {
        let result = list_dir(&std::path::PathBuf::from("/")).unwrap();
        assert_eq!(result.parent, None);
    }

    #[test]
    fn test_list_dir_hides_dotfiles() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        for entry in &result.entries {
            assert!(!entry.starts_with('.'), "dotfile should be hidden: {entry}");
        }
    }
}
```

**Step 5: Register the route in src/main.rs**

In the `Router::new()` chain, add:
```rust
.route("/browse", get(api::browse::handle_browse))
```

**Step 6: Run tests**

```bash
cargo test list_dir 2>&1 | tail -15
```

Expected: all 3 tests PASS.

**Step 7: Manual smoke test (server must be running)**

```bash
curl "http://localhost:7700/browse" | jq '{path, parent, count: (.entries | length)}'
curl "http://localhost:7700/browse?path=/" | jq '{path, parent}'
curl "http://localhost:7700/browse?path=/nonexistent"
```

Expected: valid JSON for first two; 400 error for the last.

**Step 8: Commit**

```bash
git add src/api/browse.rs src/api/mod.rs src/main.rs
git commit -m "feat: add /browse endpoint for filesystem directory listing"
```

---

## Task 4: Replace sidebar ingest section with folder picker UI

**Files:**
- Modify: `src/api/ui.rs`

All changes are inside the Rust raw string constant (`r##"..."##`).

**Step 1: Replace the ingest HTML section**

Find (around line 203–207):
```html
<div class="section-label">Ingest</div>
<input class="ingest-input" id="ingestPath" type="text" placeholder="/path/to/notes" />
<button class="btn" id="ingestBtn" onclick="startIngest()">Start Ingest</button>
<div id="ingestNotice" class="ingest-notice" style="display:none"></div>
```

Replace with:
```html
<div class="section-label">Ingest</div>
<div class="browser-wrap">
  <div class="browser-crumb" id="browserCrumb"></div>
  <div class="browser-list" id="browserList"></div>
  <button class="btn" id="ingestBtn" onclick="ingestCurrent()">Ingest here</button>
  <div id="ingestNotice" class="ingest-notice" style="display:none"></div>
</div>
```

**Step 2: Remove old `.ingest-input` CSS, add browser CSS**

Remove:
```css
.ingest-input { ... }
.ingest-input:focus { ... }
.ingest-input::placeholder { ... }
```

Add in the `<style>` block:
```css
.browser-wrap { display: flex; flex-direction: column; gap: 6px; }
.browser-crumb {
  font-size: 11px; color: var(--text-2);
  word-break: break-all; line-height: 1.4; min-height: 14px;
}
.browser-list {
  max-height: 180px; overflow-y: auto;
  border: 1px solid var(--border); border-radius: 6px;
  background: var(--bg-2);
}
.browser-row {
  display: flex; align-items: center; gap: 6px;
  padding: 5px 8px; font-size: 12px; cursor: pointer;
  color: var(--text-1); user-select: none;
}
.browser-row:hover { background: var(--hover); }
.browser-row.parent { color: var(--text-2); font-style: italic; }
.browser-icon { opacity: 0.6; flex-shrink: 0; }
.browser-empty {
  padding: 10px 8px; font-size: 12px;
  color: var(--text-3); text-align: center;
}
```

**Step 3: Replace startIngest JS with browser JS**

Remove the old `startIngest()` function. Add:

```js
let browserPath = null;

function makeBrowserRow(text, isParent, onClick) {
  const row = document.createElement("div");
  row.className = isParent ? "browser-row parent" : "browser-row";

  const icon = document.createElement("span");
  icon.className = "browser-icon";
  icon.textContent = isParent ? "↑" : "📁";

  const label = document.createElement("span");
  label.textContent = text;

  row.appendChild(icon);
  row.appendChild(label);
  row.onclick = onClick;
  return row;
}

function makeBrowserEmpty(text) {
  const el = document.createElement("div");
  el.className = "browser-empty";
  el.textContent = text;
  return el;
}

async function browseDir(path) {
  const list = document.getElementById("browserList");
  const crumb = document.getElementById("browserCrumb");

  // Clear and show loading state
  while (list.firstChild) list.removeChild(list.firstChild);
  list.appendChild(makeBrowserEmpty("Loading…"));

  const url = path ? "/browse?path=" + encodeURIComponent(path) : "/browse";
  let data;
  try {
    const res = await fetch(url);
    if (!res.ok) throw new Error(res.statusText);
    data = await res.json();
  } catch (e) {
    while (list.firstChild) list.removeChild(list.firstChild);
    list.appendChild(makeBrowserEmpty("Error loading directory"));
    return;
  }

  browserPath = data.path;

  // Breadcrumb: last 2 segments
  const parts = data.path.split("/").filter(Boolean);
  const short = parts.length > 2 ? "…/" + parts.slice(-2).join("/") : data.path;
  crumb.textContent = short;

  while (list.firstChild) list.removeChild(list.firstChild);

  if (data.parent) {
    list.appendChild(makeBrowserRow("Parent", true, () => browseDir(data.parent)));
  }

  if (data.entries.length === 0) {
    list.appendChild(makeBrowserEmpty("No subdirectories"));
    return;
  }

  for (const name of data.entries) {
    const childPath = data.path + "/" + name;
    list.appendChild(makeBrowserRow(name, false, () => browseDir(childPath)));
  }
}

async function ingestCurrent() {
  if (!browserPath) return;
  const btn = document.getElementById("ingestBtn");
  const notice = document.getElementById("ingestNotice");
  btn.disabled = true;
  btn.textContent = "Ingesting…";
  notice.style.display = "none";

  try {
    const res = await fetch("/ingest", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ path: browserPath }),
    });
    const data = await res.json();
    notice.style.display = "block";
    if (res.ok) {
      notice.textContent = "Queued " + data.files_queued + " files, skipped " + data.files_skipped;
    } else {
      notice.textContent = "Error: " + (data.message || res.statusText);
    }
  } catch (_) {
    notice.style.display = "block";
    notice.textContent = "Network error";
  } finally {
    btn.disabled = false;
    btn.textContent = "Ingest here";
  }
}
```

**Step 4: Initialize browser on page load**

In the existing `window.addEventListener("load", ...)` block, add:
```js
browseDir(null); // starts at $HOME
```

**Step 5: Build**

```bash
cargo build 2>&1 | tail -5
```

Expected: no errors.

**Step 6: Manual test in browser**

Restart the server (`cargo run`) and visit `http://localhost:7700`:

1. Sidebar shows a directory list starting at `$HOME`
2. Breadcrumb shows last 2 path segments
3. Clicking a directory navigates into it; list and crumb update
4. "↑ Parent" row appears; clicking navigates up
5. At a leaf directory (no subdirs): "No subdirectories" message shown
6. Clicking "Ingest here" sends request and shows queued file count

**Step 7: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: replace ingest text input with inline filesystem browser"
```

---

## Done

Final verification:
```bash
cargo test 2>&1 | tail -10
ls target/debug/hawkeye
curl http://localhost:7700/browse | jq '{path, count: (.entries | length)}'
```

All tests pass. Binary is `hawkeye`. Folder picker navigates and ingests correctly.
