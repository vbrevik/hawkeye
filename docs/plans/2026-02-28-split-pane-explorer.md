# Split-Pane Knowledge Explorer Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the slide-out drawer with a persistent three-column layout (sidebar | results | detail panel) so users can explore ingested documents without overlays.

**Architecture:** All changes are in `src/api/ui.rs`, which serves a single HTML string. No new server endpoints needed. Four independent tasks: CSS layout → HTML structure → JS rendering → JS wiring. Each task ends with `cargo build` to verify the Rust string is valid.

**Tech Stack:** Vanilla JS + CSS embedded in Rust `const HTML: &str`. Existing design tokens (`--bg`, `--surface`, `--border`, `--accent`, etc.) are reused throughout.

---

### Task 1: CSS — Three-Column Layout

**Files:**
- Modify: `src/api/ui.rs` (CSS section, lines ~13–201)

**Context:** The current layout is a two-column CSS grid. We're switching to a three-column flexbox. The sidebar stays the same. The `.main` class becomes `.results-col`. A new `.detail-panel` column is added. The existing drawer stays in the HTML for the mobile fallback (<900px).

**Step 1: Replace the layout CSS**

Find this block (around line 39–45):
```css
body { font-family: var(--font); background: var(--bg); color: var(--text); min-height: 100vh; }

.layout { display: grid; grid-template-columns: var(--sidebar-w) 1fr; min-height: 100vh; }

.sidebar {
  background: var(--surface); border-right: 1px solid var(--border);
  padding: 20px 16px; display: flex; flex-direction: column; gap: 24px;
  position: sticky; top: 0; height: 100vh; overflow-y: auto;
}
```

Replace with:
```css
body { font-family: var(--font); background: var(--bg); color: var(--text); height: 100vh; overflow: hidden; }

.layout { display: flex; height: 100vh; overflow: hidden; }

.sidebar {
  width: var(--sidebar-w); flex-shrink: 0;
  background: var(--surface); border-right: 1px solid var(--border);
  padding: 20px 16px; display: flex; flex-direction: column; gap: 24px;
  height: 100vh; overflow-y: auto;
}
```

**Step 2: Replace `.main` with `.results-col`**

Find (around line 94):
```css
.main { display: flex; flex-direction: column; min-height: 100vh; overflow: hidden; }
```

Replace with:
```css
.results-col { flex: 1; min-width: 280px; display: flex; flex-direction: column; overflow: hidden; }
```

**Step 3: Make the results area scroll**

Find (around line 116):
```css
.results-area { padding: 20px 28px; flex: 1; }
```

Replace with:
```css
.results-area { padding: 20px 28px; flex: 1; overflow-y: auto; }
```

**Step 4: Add `.detail-panel` and its content styles**

After the `.skeleton` / `.sk-card` block (after line ~143), add:
```css
.detail-panel {
  width: 42%; flex-shrink: 0;
  border-left: 1px solid var(--border);
  background: var(--surface);
  overflow-y: auto;
  display: flex; flex-direction: column;
}

@media (max-width: 900px) {
  .detail-panel { display: none; }
}

.detail-placeholder {
  flex: 1; display: flex; flex-direction: column;
  align-items: center; justify-content: center;
  gap: 10px; color: var(--text-3); padding: 40px;
}
.detail-placeholder p { font-size: 13px; }

.detail-body { padding: 20px; display: flex; flex-direction: column; gap: 20px; }
.detail-title { font-size: 17px; font-weight: 700; color: var(--text); line-height: 1.3; }
.detail-tldr { font-size: 14px; color: var(--text-2); line-height: 1.6; }
.detail-meta { font-size: 11px; color: var(--text-3); padding-top: 4px; border-top: 1px solid var(--border); display: flex; gap: 16px; }
.detail-meta strong { color: var(--text-2); }
.detail-slabel { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: var(--text-3); margin-bottom: 6px; }
.detail-chips { display: flex; flex-wrap: wrap; gap: 4px; }
.detail-chip { background: var(--surface-2); border-radius: 4px; color: var(--text-2); font-size: 12px; padding: 3px 8px; cursor: pointer; transition: background 0.15s, color 0.15s; }
.detail-chip:hover { background: var(--accent-dim); color: var(--accent-hover); }
.detail-chip.plain { cursor: default; }
.detail-chip.plain:hover { background: var(--surface-2); color: var(--text-2); }
.detail-related-divider { font-size: 11px; color: var(--text-3); border-top: 1px solid var(--border); padding-top: 12px; margin-top: 4px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.06em; }
.detail-related-list { display: flex; flex-direction: column; gap: 6px; }
.related-card { background: var(--surface-2); border: 1px solid var(--border); border-radius: 8px; padding: 10px 12px; cursor: pointer; transition: border-color 0.15s; }
.related-card:hover { border-color: rgba(99,102,241,0.5); }
.related-card-title { font-size: 13px; font-weight: 600; color: var(--text); line-height: 1.3; margin-bottom: 3px; }
.related-card-tldr { font-size: 12px; color: var(--text-3); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
```

**Step 5: Verify it compiles**

```bash
cd /Users/vidarbrevik/projects/eagle3 && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors. (Character encoding in the HTML string won't be validated at compile time, but syntax errors in the Rust will.)

**Step 6: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: CSS three-column layout with detail panel styles"
```

---

### Task 2: HTML — Add Detail Panel, Rename Main

**Files:**
- Modify: `src/api/ui.rs` (HTML section, lines ~203–303)

**Context:** We rename `<main class="main">` to `<main class="results-col">` and add a `.detail-panel` div between `</main>` and the drawer. The drawer stays for mobile. The detail panel starts with a placeholder state.

**Step 1: Rename the main element**

Find (around line 274):
```html
  <main class="main">
```

Replace with:
```html
  <main class="results-col">
```

**Step 2: Add the detail panel after `</main>`**

Find:
```html
</main>
</div>

<div class="drawer-overlay"
```

Replace with:
```html
</main>

  <div class="detail-panel" id="detailPanel">
    <div class="detail-placeholder" id="detailPlaceholder">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.3">
        <path d="M9 18l6-6-6-6"/>
      </svg>
      <p>Select a document to explore</p>
    </div>
  </div>
</div>

<div class="drawer-overlay"
```

**Step 3: Verify it compiles**

```bash
cd /Users/vidarbrevik/projects/eagle3 && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

**Step 4: Visual check**

Start the server (`cargo run -- --index-path /tmp/test-idx`) and open `http://localhost:7700`. You should see three columns: sidebar, empty search area, and an empty detail panel on the right with "Select a document to explore".

**Step 5: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: add detail panel HTML structure, rename main to results-col"
```

---

### Task 3: JS — Detail Panel Rendering and Related Docs

**Files:**
- Modify: `src/api/ui.rs` (JS section, lines ~305–745)

**Context:** We add three new JS responsibilities: (1) opening the detail panel and fetching `/summary/{file}`, (2) rendering the detail panel content (title, tldr, meta, chips, related), (3) computing related docs from the current in-memory result set. The existing `openDrawer` / `renderDrawer` functions stay for mobile.

**Step 1: Add state variables**

Find the existing state variables at the top of the `<script>` block:
```js
  let activeFilters = new Set();
  let activeCardFile = null;
  let searchTimer = null;
```

Replace with:
```js
  let activeFilters = new Set();
  let activeCardFile = null;
  let searchTimer = null;
  let currentResults = [];   // latest search results for related-doc computation
```

**Step 2: Add `clearDetailPanel()`**

After the `renderFilters()` function, add:
```js
  function clearDetailPanel() {
    const panel = document.getElementById("detailPanel");
    const placeholder = document.getElementById("detailPlaceholder");
    if (!placeholder) {
      // re-create placeholder
      panel.replaceChildren();
      const ph = document.createElement("div");
      ph.id = "detailPlaceholder";
      ph.className = "detail-placeholder";
      const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svg.setAttribute("width", "32"); svg.setAttribute("height", "32");
      svg.setAttribute("viewBox", "0 0 24 24"); svg.setAttribute("fill", "none");
      svg.setAttribute("stroke", "currentColor"); svg.setAttribute("stroke-width", "1.5");
      svg.setAttribute("opacity", "0.3");
      const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
      path.setAttribute("d", "M9 18l6-6-6-6");
      svg.appendChild(path);
      const p = document.createElement("p");
      p.textContent = "Select a document to explore";
      ph.append(svg, p);
      panel.appendChild(ph);
    }
    activeCardFile = null;
    document.querySelectorAll(".card").forEach(c => c.classList.remove("active"));
  }
```

**Step 3: Add `computeRelated(source, allResults)`**

```js
  function computeRelated(source, allResults) {
    const myTags = new Set((source.tags || "").split(" ").filter(Boolean));
    const myEntities = new Set((source.entities || "").split(" ").filter(Boolean));
    return allResults.filter(r => {
      if (r.file === source.file) return false;
      const rTags = (r.tags || "").split(" ").filter(Boolean);
      const rEntities = (r.entities || "").split(" ").filter(Boolean);
      return rTags.some(t => myTags.has(t)) || rEntities.some(e => myEntities.has(e));
    }).slice(0, 5);
  }
```

**Step 4: Add `renderDetailPanel(s, related)`**

```js
  function renderDetailPanel(s, related) {
    const panel = document.getElementById("detailPanel");

    function makeDetailChips(values, clickable) {
      const wrap = document.createElement("div");
      wrap.className = "detail-chips";
      if (!values || !values.length) {
        const em = document.createElement("span");
        em.style.cssText = "color:var(--text-3);font-size:12px;";
        em.textContent = "\u2014";
        wrap.appendChild(em);
        return wrap;
      }
      for (const v of values) {
        const chip = document.createElement("span");
        chip.className = "detail-chip" + (clickable ? "" : " plain");
        chip.textContent = clickable ? "#" + v : v;
        if (clickable) chip.addEventListener("click", () => addFilter(v));
        wrap.appendChild(chip);
      }
      return wrap;
    }

    function makeDetailSection(label, contentNode) {
      const wrap = document.createElement("div");
      const lbl = document.createElement("div");
      lbl.className = "detail-slabel";
      lbl.textContent = label;
      wrap.append(lbl, contentNode);
      return wrap;
    }

    const body = document.createElement("div");
    body.className = "detail-body";

    const titleEl = document.createElement("div");
    titleEl.className = "detail-title";
    titleEl.textContent = s.title || s.source || s.file || "";

    const tldrEl = document.createElement("div");
    tldrEl.className = "detail-tldr";
    tldrEl.textContent = s.tldr || "";

    const meta = document.createElement("div");
    meta.className = "detail-meta";
    const wEl = document.createElement("div");
    const wStrong = document.createElement("strong");
    wStrong.textContent = (s.word_count || 0).toLocaleString();
    wEl.append(wStrong, " words");
    const dEl = document.createElement("div");
    dEl.textContent = "Indexed " + (s.created_at ? new Date(s.created_at).toLocaleDateString() : "\u2014");
    meta.append(wEl, dEl);

    const tags = (s.tags || "").split(" ").filter(Boolean);
    const topics = (s.topics || []).filter ? s.topics : (s.topics || "").split(" ").filter(Boolean);
    const entities = (s.entities || []).filter ? s.entities : (s.entities || "").split(" ").filter(Boolean);

    body.append(
      titleEl,
      tldrEl,
      makeDetailSection("Tags", makeDetailChips(tags, true)),
      makeDetailSection("Topics", makeDetailChips(topics, false)),
      makeDetailSection("Entities", makeDetailChips(entities, false)),
      meta
    );

    if (related && related.length) {
      const divider = document.createElement("div");
      divider.className = "detail-related-divider";
      divider.textContent = "Related";
      body.appendChild(divider);

      const relList = document.createElement("div");
      relList.className = "detail-related-list";
      for (const rel of related) {
        const rc = document.createElement("div");
        rc.className = "related-card";
        const rt = document.createElement("div");
        rt.className = "related-card-title";
        rt.textContent = rel.title || rel.file;
        const rl = document.createElement("div");
        rl.className = "related-card-tldr";
        rl.textContent = rel.tldr;
        rc.append(rt, rl);
        rc.addEventListener("click", () => openDetailPanel(rel));
        relList.appendChild(rc);
      }
      body.appendChild(relList);
    }

    panel.replaceChildren(body);
  }
```

**Step 5: Add `openDetailPanel(result)`**

```js
  async function openDetailPanel(result) {
    activeCardFile = result.file;
    document.querySelectorAll(".card").forEach(c =>
      c.classList.toggle("active", c.dataset.file === result.file)
    );
    location.hash = "file=" + encodeURIComponent(result.file);

    // Show skeleton while fetching
    const panel = document.getElementById("detailPanel");
    const sk = document.createElement("div");
    sk.className = "skeleton";
    sk.style.cssText = "margin:20px;height:180px;border-radius:8px;";
    panel.replaceChildren(sk);

    const related = computeRelated(result, currentResults);

    try {
      const res = await fetch("/summary/" + encodeURIComponent(result.file));
      if (!res.ok) throw new Error("not found");
      const s = await res.json();
      renderDetailPanel(s, related);
    } catch (_) {
      // Fallback to search result payload — tags/entities are space-separated strings here
      renderDetailPanel(result, related);
    }
  }
```

**Step 6: Verify it compiles**

```bash
cd /Users/vidarbrevik/projects/eagle3 && cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

**Step 7: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: detail panel rendering, related docs computation"
```

---

### Task 4: JS — Wire Up Card Clicks, Auto-Select, Keyboard

**Files:**
- Modify: `src/api/ui.rs` (JS section)

**Context:** We add a `selectResult(r)` dispatcher that decides between the detail panel (desktop ≥900px) and the drawer (mobile). We update `makeCard()` to call it, update `renderResults()` to store results and auto-select the first, and update the Escape handler.

**Step 1: Add `selectResult(r)`**

After `openDetailPanel`, add:
```js
  function selectResult(r) {
    if (window.innerWidth >= 900) {
      openDetailPanel(r);
    } else {
      openDrawer(r);
    }
  }
```

**Step 2: Update `makeCard()` to use `selectResult`**

Find (inside `makeCard`):
```js
    card.addEventListener("click", () => openDrawer(r));
```

Replace with:
```js
    card.addEventListener("click", () => selectResult(r));
```

**Step 3: Store results and auto-select first in `renderResults()`**

Find:
```js
  function renderResults(data, query) {
    const container = document.getElementById("resultsContainer");
    if (!data.length) {
```

Replace the entire `renderResults` function with:
```js
  function renderResults(data, query) {
    currentResults = data;
    const container = document.getElementById("resultsContainer");
    if (!data.length) {
      const wrap = document.createElement("div");
      wrap.className = "placeholder";
      const p = document.createElement("p");
      p.textContent = "No results for \u201c" + query + "\u201d";
      wrap.appendChild(p);
      container.replaceChildren(wrap);
      clearDetailPanel();
      return;
    }

    const count = document.createElement("div");
    count.className = "results-count";
    count.textContent = data.length + " result" + (data.length !== 1 ? "s" : "");

    const list = document.createElement("div");
    list.className = "results-list";
    for (const r of data) list.appendChild(makeCard(r));

    container.replaceChildren(count, list);

    // Auto-select first result on desktop
    if (window.innerWidth >= 900 && data.length) {
      selectResult(data[0]);
    }
  }
```

**Step 4: Update the Escape key handler for the detail panel**

Find:
```js
  searchInput.addEventListener("keydown", e => {
    if (e.key === "Escape") {
      if (searchInput.value) { searchInput.value = ""; doSearch(); }
      else closeDrawer();
    }
  });
```

Replace with:
```js
  searchInput.addEventListener("keydown", e => {
    if (e.key === "Escape") {
      if (searchInput.value) { searchInput.value = ""; doSearch(); }
      else if (window.innerWidth >= 900) clearDetailPanel();
      else closeDrawer();
    }
  });
```

Also update the global Escape handler:
```js
  document.addEventListener("keydown", e => {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault(); searchInput.focus(); searchInput.select();
    }
    if (e.key === "Escape") {
      if (window.innerWidth >= 900) clearDetailPanel();
      else closeDrawer();
    }
  });
```

**Step 5: Clear `currentResults` when search is cleared**

Find in `showEmpty()`:
```js
  function showEmpty(msg) {
    const container = document.getElementById("resultsContainer");
```

Add `currentResults = [];` as the first line inside `showEmpty`:
```js
  function showEmpty(msg) {
    currentResults = [];
    if (window.innerWidth >= 900) clearDetailPanel();
    const container = document.getElementById("resultsContainer");
```

**Step 6: Verify it compiles and run existing tests**

```bash
cd /Users/vidarbrevik/projects/eagle3 && cargo test 2>&1 | tail -10
```

Expected: all tests pass.

**Step 7: Visual smoke test**

Start the server and verify:
1. Search returns results → detail panel auto-populates with first result
2. Click a different card → detail panel updates
3. Click a tag chip in the detail panel → search bar updates and new search fires
4. If two docs share a tag/entity → Related section appears in detail panel
5. Click a related card → detail panel loads that doc
6. Resize window below 900px → detail panel hides, clicking a card opens the drawer instead
7. Press Escape → detail panel clears (desktop) or drawer closes (mobile)

**Step 8: Commit**

```bash
git add src/api/ui.rs
git commit -m "feat: wire up split-pane selection, auto-select, keyboard nav"
```

---

## Execution Handoff

Tasks are independent enough for sequential single-session execution. Each ends with `cargo build` or `cargo test` for fast feedback.

**Note on the embedded HTML string in Rust:** The HTML is a `const HTML: &str = r##"..."##`. All edits are string replacements within this literal. The Rust compiler only validates syntax of the surrounding Rust code, not the HTML/CSS/JS content — so `cargo build` confirms the Rust wrapper is valid but visual testing is required to confirm the frontend behavior.
