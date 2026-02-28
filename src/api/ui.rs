use axum::response::Html;

pub async fn handle_ui() -> Html<&'static str> {
    Html(HTML)
}

const HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Hawkeye</title>
  <style>
    *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }

    :root {
      --bg: #09090b;
      --surface: #18181b;
      --surface-2: #27272a;
      --border: #3f3f46;
      --text: #fafafa;
      --text-2: #a1a1aa;
      --text-3: #52525b;
      --accent: #6366f1;
      --accent-dim: rgba(99,102,241,0.12);
      --accent-hover: #818cf8;
      --green: #22c55e;
      --yellow: #eab308;
      --red: #ef4444;
      --font: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      --mono: ui-monospace, "SF Mono", "Fira Code", monospace;
      --r: 8px;
      --sidebar-w: 220px;
      --drawer-w: 340px;
    }

    body { font-family: var(--font); background: var(--bg); color: var(--text); height: 100vh; overflow: hidden; }

    .layout { display: flex; height: 100vh; overflow: hidden; }

    .sidebar {
      width: var(--sidebar-w); flex-shrink: 0;
      background: var(--surface); border-right: 1px solid var(--border);
      padding: 20px 16px; display: flex; flex-direction: column; gap: 24px;
      height: 100vh; overflow-y: auto;
    }

    .logo { font-size: 17px; font-weight: 700; letter-spacing: -0.03em; color: var(--text); }
    .logo span { color: var(--accent); }

    .section-label {
      font-size: 10px; font-weight: 700; text-transform: uppercase;
      letter-spacing: 0.08em; color: var(--text-3); margin-bottom: 8px;
    }

    .mlx-block { background: var(--surface-2); border: 1px solid var(--border); border-radius: var(--r); padding: 10px 12px; }
    .mlx-row { display: flex; align-items: center; gap: 8px; margin-bottom: 3px; }
    .dot { width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0; background: var(--text-3); }
    .dot.online { background: var(--green); animation: pulse 2.5s infinite; }
    .dot.warn { background: var(--yellow); }
    @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }
    .mlx-label { font-size: 12px; font-weight: 600; color: var(--text); }
    .mlx-model { font-size: 10px; color: var(--text-3); font-family: var(--mono); line-height: 1.4; word-break: break-all; }

    .btn {
      width: 100%; background: var(--accent); border: none; border-radius: var(--r);
      color: #fff; font-size: 12px; font-weight: 600; padding: 7px;
      cursor: pointer; transition: background 0.15s, opacity 0.15s;
    }
    .btn:hover { background: var(--accent-hover); }
    .btn:disabled { opacity: 0.4; cursor: not-allowed; }

    .progress-bar { background: var(--surface-2); border-radius: 4px; height: 4px; overflow: hidden; margin-top: 8px; }
    .progress-fill { height: 100%; background: linear-gradient(90deg, var(--accent), var(--accent-hover)); border-radius: 4px; transition: width 0.4s ease; }
    .ingest-notice { font-size: 11px; color: var(--text-2); margin-top: 6px; line-height: 1.4; }

    .stats-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }
    .stat-cell { background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; padding: 8px; text-align: center; }
    .stat-cell .val { font-size: 18px; font-weight: 700; color: var(--text); line-height: 1; margin-bottom: 2px; }
    .stat-cell .lbl { font-size: 10px; color: var(--text-3); text-transform: uppercase; letter-spacing: 0.05em; }
    .stat-cell.prog .val { color: var(--accent); }
    .stat-cell.fail .val { color: var(--red); }

    .filter-chips { display: flex; flex-wrap: wrap; gap: 4px; }
    .chip {
      display: inline-flex; align-items: center; gap: 4px;
      background: var(--accent-dim); border: 1px solid rgba(99,102,241,0.3);
      border-radius: 4px; color: var(--accent-hover); font-size: 11px;
      padding: 2px 7px; cursor: pointer; transition: background 0.15s;
    }
    .chip:hover { background: rgba(99,102,241,0.22); }
    .chip .x { color: var(--text-3); font-size: 9px; margin-left: 1px; }
    .no-filters { font-size: 11px; color: var(--text-3); font-style: italic; }

    .results-col { flex: 1; min-width: 280px; display: flex; flex-direction: column; overflow: hidden; }

    .search-bar-wrap {
      padding: 20px 28px 16px; position: sticky; top: 0;
      background: var(--bg); z-index: 10; border-bottom: 1px solid var(--border);
    }
    .search-bar {
      display: flex; align-items: center; gap: 10px;
      background: var(--surface); border: 1px solid var(--border);
      border-radius: 10px; padding: 10px 14px;
      transition: border-color 0.15s, box-shadow 0.15s;
    }
    .search-bar:focus-within { border-color: var(--accent); box-shadow: 0 0 0 3px var(--accent-dim); }
    .search-icon { color: var(--text-3); font-size: 15px; flex-shrink: 0; }
    .search-input { flex: 1; background: none; border: none; color: var(--text); font-size: 15px; outline: none; }
    .search-input::placeholder { color: var(--text-3); }
    .search-kbd {
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: 4px; color: var(--text-3); font-size: 10px;
      font-family: var(--mono); padding: 2px 6px; flex-shrink: 0;
    }

    .results-area { padding: 20px 28px; flex: 1; overflow-y: auto; }
    .results-count { font-size: 12px; color: var(--text-3); margin-bottom: 14px; }
    .results-list { display: flex; flex-direction: column; gap: 8px; }

    .card {
      background: var(--surface); border: 1px solid var(--border);
      border-radius: 10px; padding: 14px 16px; cursor: pointer;
      transition: border-color 0.15s, background 0.15s;
    }
    .card:hover { border-color: rgba(99,102,241,0.5); background: rgba(99,102,241,0.03); }
    .card.active { border-color: var(--accent); background: var(--accent-dim); }

    .card-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 8px; margin-bottom: 4px; }
    .card-title { font-size: 14px; font-weight: 600; color: var(--text); line-height: 1.3; }
    .card-score { font-size: 11px; font-weight: 600; color: var(--accent); background: var(--accent-dim); border-radius: 4px; padding: 2px 6px; flex-shrink: 0; }
    .card-file { font-size: 11px; font-family: var(--mono); color: var(--text-3); margin-bottom: 6px; }
    .card-tldr { font-size: 13px; color: var(--text-2); line-height: 1.5; margin-bottom: 8px; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
    .card-tags { display: flex; flex-wrap: wrap; gap: 4px; }
    .tag { background: var(--surface-2); border-radius: 4px; color: var(--text-3); font-size: 11px; padding: 2px 7px; cursor: pointer; transition: color 0.15s, background 0.15s; }
    .tag:hover { background: var(--accent-dim); color: var(--accent-hover); }

    .placeholder { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 80px 0; gap: 12px; color: var(--text-3); }
    .placeholder p { font-size: 14px; }

    .skeleton { border-radius: var(--r); background: var(--surface); animation: shimmer 1.4s infinite; }
    @keyframes shimmer { 0%, 100% { opacity: 0.4; } 50% { opacity: 0.9; } }
    .sk-card { height: 90px; border-radius: 10px; margin-bottom: 8px; }

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

    .drawer-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.55); opacity: 0; pointer-events: none; transition: opacity 0.25s; z-index: 40; }
    .drawer-overlay.open { opacity: 1; pointer-events: all; }

    .drawer {
      position: fixed; top: 0; right: 0; bottom: 0; width: var(--drawer-w);
      background: var(--surface); border-left: 1px solid var(--border);
      transform: translateX(100%);
      transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
      z-index: 50; display: flex; flex-direction: column; overflow: hidden;
    }
    .drawer.open { transform: translateX(0); }

    .drawer-header { padding: 16px 20px; border-bottom: 1px solid var(--border); display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; }
    .drawer-title { font-size: 15px; font-weight: 600; color: var(--text); line-height: 1.3; }
    .drawer-close { background: var(--surface-2); border: 1px solid var(--border); border-radius: 6px; color: var(--text-2); font-size: 13px; width: 28px; height: 28px; display: flex; align-items: center; justify-content: center; cursor: pointer; flex-shrink: 0; transition: background 0.15s; }
    .drawer-close:hover { background: var(--border); }

    .drawer-body { flex: 1; overflow-y: auto; padding: 16px 20px; display: flex; flex-direction: column; gap: 20px; }
    .drawer-slabel { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: var(--text-3); margin-bottom: 6px; }
    .drawer-text { font-size: 13px; color: var(--text-2); line-height: 1.6; }
    .drawer-file { font-size: 11px; font-family: var(--mono); color: var(--text-3); word-break: break-all; }
    .drawer-chips { display: flex; flex-wrap: wrap; gap: 4px; }
    .drawer-chip { background: var(--surface-2); border-radius: 4px; color: var(--text-2); font-size: 12px; padding: 3px 8px; cursor: pointer; transition: background 0.15s, color 0.15s; }
    .drawer-chip:hover { background: var(--accent-dim); color: var(--accent-hover); }
    .drawer-chip.static { cursor: default; }
    .drawer-chip.static:hover { background: var(--surface-2); color: var(--text-2); }

    .drawer-meta { display: flex; gap: 16px; padding-top: 4px; border-top: 1px solid var(--border); }
    .drawer-meta-item { font-size: 12px; color: var(--text-3); }
    .drawer-meta-item strong { color: var(--text-2); }

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
    .browser-md-count {
      font-size: 11px; color: var(--text-2); min-height: 16px;
    }
    .browser-md-count.none { color: var(--text-3); }
  </style>
</head>
<body>

<div class="layout">
  <aside class="sidebar">
    <div class="logo">hawk<span>eye</span></div>

    <div>
      <div class="section-label">Inference</div>
      <div class="mlx-block">
        <div class="mlx-row">
          <div class="dot" id="mlxDot"></div>
          <span class="mlx-label" id="mlxStatus">Checkingâ¦</span>
        </div>
        <div class="mlx-model" id="mlxModel">â</div>
      </div>
    </div>

    <div>
      <div class="section-label">Ingest</div>
      <div class="browser-wrap">
        <div class="browser-crumb" id="browserCrumb"></div>
        <div class="browser-list" id="browserList"></div>
        <div class="browser-md-count" id="browserMdCount"></div>
        <button class="btn" id="ingestBtn" onclick="ingestCurrent()">Ingest here</button>
        <div id="ingestNotice" class="ingest-notice" style="display:none"></div>
      </div>
      <div id="progressWrap" style="display:none">
        <div class="progress-bar"><div class="progress-fill" id="progressFill" style="width:0%"></div></div>
        <div class="ingest-notice" id="progressInfo"></div>
      </div>
    </div>

    <div>
      <div class="section-label">Queue</div>
      <div class="stats-grid">
        <div class="stat-cell"><div class="val" id="statTotal">0</div><div class="lbl">Total</div></div>
        <div class="stat-cell"><div class="val" id="statCompleted">0</div><div class="lbl">Done</div></div>
        <div class="stat-cell prog"><div class="val" id="statInProgress">0</div><div class="lbl">Active</div></div>
        <div class="stat-cell fail"><div class="val" id="statFailed">0</div><div class="lbl">Failed</div></div>
      </div>
    </div>

    <div>
      <div class="section-label">Tags</div>
      <div class="filter-chips" id="tagCloud">
        <span class="no-filters">Ingest documents to see tags</span>
      </div>
    </div>

    <div>
      <div class="section-label">Topics</div>
      <div class="filter-chips" id="topicCloud">
        <span class="no-filters">—</span>
      </div>
    </div>

    <div>
      <div class="section-label">Entities</div>
      <div class="filter-chips" id="entityCloud">
        <span class="no-filters">—</span>
      </div>
    </div>

    <div>
      <div class="section-label">Active Filters</div>
      <div class="filter-chips" id="filterChips">
        <span class="no-filters">No filters active</span>
      </div>
    </div>
  </aside>

  <main class="results-col">
    <div class="search-bar-wrap">
      <div class="search-bar">
        <span class="search-icon">â</span>
        <input class="search-input" id="searchInput" placeholder="Search summariesâ¦" autocomplete="off" spellcheck="false" />
        <span class="search-kbd">âK</span>
      </div>
    </div>
    <div class="results-area">
      <div id="resultsContainer">
        <div class="placeholder">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.3">
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.35-4.35"/>
          </svg>
          <p>Search your indexed summaries</p>
        </div>
      </div>
    </div>
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

<div class="drawer-overlay" id="drawerOverlay" onclick="closeDrawer()"></div>
<div class="drawer" id="drawer">
  <div class="drawer-header">
    <div class="drawer-title" id="drawerTitle">â</div>
    <button class="drawer-close" onclick="closeDrawer()">â</button>
  </div>
  <div class="drawer-body" id="drawerBody"></div>
</div>

<script>
  // All server data is set via textContent or built with DOM APIs - no innerHTML with server data.
  let activeFilters = new Set();
  let activeCardFile = null;
  let searchTimer = null;
  let currentResults = [];   // latest search results for related-doc computation

  function setText(id, t) { document.getElementById(id).textContent = t; }
  function show(id) { document.getElementById(id).style.display = ""; }

  const searchInput = document.getElementById("searchInput");
  searchInput.focus();

  searchInput.addEventListener("input", () => {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(doSearch, 300);
  });

  searchInput.addEventListener("keydown", e => {
    if (e.key === "Escape") {
      if (searchInput.value) { searchInput.value = ""; doSearch(); }
      else if (window.innerWidth >= 900) clearDetailPanel();
      else closeDrawer();
      e.stopPropagation();
    }
  });

  document.addEventListener("keydown", e => {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault(); searchInput.focus(); searchInput.select();
    }
    if (e.key === "Escape") {
      if (window.innerWidth >= 900) clearDetailPanel();
      else closeDrawer();
    }
  });

  async function doSearch() {
    const raw = searchInput.value.trim();
    const tagQ = [...activeFilters].join(" ");
    const q = [raw, tagQ].filter(Boolean).join(" ");

    if (!q) { showEmpty(); return; }

    const container = document.getElementById("resultsContainer");
    container.replaceChildren();
    for (let i = 0; i < 5; i++) {
      const sk = document.createElement("div");
      sk.className = "skeleton sk-card";
      container.appendChild(sk);
    }

    try {
      const res = await fetch("/search?q=" + encodeURIComponent(q) + "&limit=25");
      const data = await res.json();
      renderResults(data, raw);
    } catch (e) {
      showEmpty("Error: " + e.message);
    }
  }

  function showEmpty(msg) {
    currentResults = [];
    if (window.innerWidth >= 900) clearDetailPanel();
    const container = document.getElementById("resultsContainer");
    const wrap = document.createElement("div");
    wrap.className = "placeholder";
    if (!msg) {
      const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svg.setAttribute("width", "40"); svg.setAttribute("height", "40");
      svg.setAttribute("viewBox", "0 0 24 24"); svg.setAttribute("fill", "none");
      svg.setAttribute("stroke", "currentColor"); svg.setAttribute("stroke-width", "1.5");
      svg.setAttribute("opacity", "0.3");
      const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
      circle.setAttribute("cx", "11"); circle.setAttribute("cy", "11"); circle.setAttribute("r", "8");
      const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
      path.setAttribute("d", "m21 21-4.35-4.35");
      svg.append(circle, path);
      wrap.appendChild(svg);
    }
    const p = document.createElement("p");
    p.textContent = msg || "Search your indexed summaries";
    wrap.appendChild(p);
    container.replaceChildren(wrap);
  }

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

  function makeCard(r) {
    const card = document.createElement("div");
    card.className = "card" + (r.file === activeCardFile ? " active" : "");
    card.dataset.file = r.file;
    card.addEventListener("click", () => selectResult(r));

    const header = document.createElement("div");
    header.className = "card-header";

    const title = document.createElement("div");
    title.className = "card-title";
    title.textContent = r.title || r.file;

    const score = document.createElement("div");
    score.className = "card-score";
    score.textContent = r.score.toFixed(2);
    header.append(title, score);

    const file = document.createElement("div");
    file.className = "card-file";
    file.textContent = r.file;

    const tldr = document.createElement("div");
    tldr.className = "card-tldr";
    tldr.textContent = r.tldr;

    const tags = document.createElement("div");
    tags.className = "card-tags";
    (r.tags || "").split(" ").filter(Boolean).forEach(t => {
      const tag = document.createElement("span");
      tag.className = "tag";
      tag.textContent = "#" + t;
      tag.addEventListener("click", e => { e.stopPropagation(); addFilter(t); });
      tags.appendChild(tag);
    });

    card.append(header, file, tldr, tags);
    return card;
  }

  function addFilter(tag) {
    if (activeFilters.has(tag)) return;
    activeFilters.add(tag); renderFilters(); doSearch();
  }

  function removeFilter(tag) {
    activeFilters.delete(tag); renderFilters(); doSearch();
  }

  function renderFilters() {
    const el = document.getElementById("filterChips");
    el.replaceChildren();
    if (!activeFilters.size) {
      const s = document.createElement("span");
      s.className = "no-filters";
      s.textContent = "No filters active";
      el.appendChild(s);
      return;
    }
    for (const t of activeFilters) {
      const chip = document.createElement("span");
      chip.className = "chip";
      chip.textContent = "#" + t;
      const x = document.createElement("span");
      x.className = "x";
      x.textContent = "\u2715";
      chip.appendChild(x);
      chip.addEventListener("click", () => removeFilter(t));
      el.appendChild(chip);
    }
  }

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

  function computeRelated(source, allResults) {
    const myTags = new Set(Array.isArray(source.tags) ? source.tags : (source.tags || "").split(" ").filter(Boolean));
    const myEntities = new Set(Array.isArray(source.entities) ? source.entities : (source.entities || "").split(" ").filter(Boolean));
    return allResults.filter(r => {
      if (r.file === source.file) return false;
      const rTags = Array.isArray(r.tags) ? r.tags : (r.tags || "").split(" ").filter(Boolean);
      const rEntities = Array.isArray(r.entities) ? r.entities : (r.entities || "").split(" ").filter(Boolean);
      return rTags.some(t => myTags.has(t)) || rEntities.some(e => myEntities.has(e));
    }).slice(0, 5);
  }

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

    const tags = Array.isArray(s.tags) ? s.tags : (s.tags || "").split(" ").filter(Boolean);
    const topics = Array.isArray(s.topics) ? s.topics : (s.topics || "").split(" ").filter(Boolean);
    const entities = Array.isArray(s.entities) ? s.entities : (s.entities || "").split(" ").filter(Boolean);

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
        rl.textContent = rel.tldr || "";
        rc.append(rt, rl);
        rc.addEventListener("click", () => selectResult(rel));
        relList.appendChild(rc);
      }
      body.appendChild(relList);
    }

    panel.replaceChildren(body);
  }

  async function openDetailPanel(result) {
    activeCardFile = result.file;
    document.querySelectorAll(".card").forEach(c =>
      c.classList.toggle("active", c.dataset.file === result.file)
    );
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

  function selectResult(r) {
    if (window.innerWidth >= 900) {
      openDetailPanel(r);
    } else {
      openDrawer(r);
    }
  }

  async function openDrawer(result) {
    activeCardFile = result.file;
    document.querySelectorAll(".card").forEach(c =>
      c.classList.toggle("active", c.dataset.file === result.file)
    );
    setText("drawerTitle", result.title || result.file);

    const body = document.getElementById("drawerBody");
    const sk = document.createElement("div");
    sk.className = "skeleton";
    sk.style.cssText = "height:140px;border-radius:8px;";
    body.replaceChildren(sk);

    document.getElementById("drawer").classList.add("open");
    document.getElementById("drawerOverlay").classList.add("open");

    try {
      const res = await fetch("/summary/" + encodeURIComponent(result.file));
      if (!res.ok) throw new Error("not found");
      renderDrawer(await res.json());
    } catch (_) {
      renderDrawerFallback(result);
    }
  }

  function closeDrawer() {
    activeCardFile = null;
    document.querySelectorAll(".card").forEach(c => c.classList.remove("active"));
    document.getElementById("drawer").classList.remove("open");
    document.getElementById("drawerOverlay").classList.remove("open");
  }

  function makeSection(label, contentNode) {
    const wrap = document.createElement("div");
    const lbl = document.createElement("div");
    lbl.className = "drawer-slabel";
    lbl.textContent = label;
    wrap.append(lbl, contentNode);
    return wrap;
  }

  function makeChips(arr, clickable) {
    const wrap = document.createElement("div");
    wrap.className = "drawer-chips";
    if (!arr || !arr.length) {
      const em = document.createElement("span");
      em.style.cssText = "color:var(--text-3);font-size:12px;";
      em.textContent = "\u2014";
      wrap.appendChild(em);
      return wrap;
    }
    for (const v of arr) {
      const chip = document.createElement("span");
      chip.className = "drawer-chip" + (clickable ? "" : " static");
      chip.textContent = clickable ? "#" + v : v;
      if (clickable) chip.addEventListener("click", () => addFilter(v));
      wrap.appendChild(chip);
    }
    return wrap;
  }

  function renderDrawer(s) {
    setText("drawerTitle", s.title || s.source);

    const fileEl = document.createElement("div");
    fileEl.className = "drawer-file";
    fileEl.textContent = s.source;

    const tldrEl = document.createElement("div");
    tldrEl.className = "drawer-text";
    tldrEl.textContent = s.tldr;

    const meta = document.createElement("div");
    meta.className = "drawer-meta";
    const wEl = document.createElement("div");
    wEl.className = "drawer-meta-item";
    const wStrong = document.createElement("strong");
    wStrong.textContent = (s.word_count || 0).toLocaleString();
    wEl.append(wStrong, " words");
    const dEl = document.createElement("div");
    dEl.className = "drawer-meta-item";
    dEl.textContent = "Indexed " + (s.created_at ? new Date(s.created_at).toLocaleDateString() : "\u2014");
    meta.append(wEl, dEl);

    document.getElementById("drawerBody").replaceChildren(
      makeSection("File", fileEl),
      makeSection("TL;DR", tldrEl),
      makeSection("Tags", makeChips(s.tags, true)),
      makeSection("Entities", makeChips(s.entities, false)),
      makeSection("Topics", makeChips(s.topics, false)),
      meta
    );
  }

  function renderDrawerFallback(r) {
    const fileEl = document.createElement("div");
    fileEl.className = "drawer-file";
    fileEl.textContent = r.file;

    const tldrEl = document.createElement("div");
    tldrEl.className = "drawer-text";
    tldrEl.textContent = r.tldr;

    const tags = (r.tags || "").split(" ").filter(Boolean);
    const nodes = [makeSection("File", fileEl), makeSection("TL;DR", tldrEl)];
    if (tags.length) nodes.push(makeSection("Tags", makeChips(tags, true)));
    document.getElementById("drawerBody").replaceChildren(...nodes);
  }

  let browserPath = null;

  function makeBrowserRow(text, isParent, onClick) {
    const row = document.createElement("div");
    row.className = isParent ? "browser-row parent" : "browser-row";
    const icon = document.createElement("span");
    icon.className = "browser-icon";
    icon.textContent = isParent ? "\u2191" : "\uD83D\uDCC1";
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
    while (list.firstChild) list.removeChild(list.firstChild);
    list.appendChild(makeBrowserEmpty("Loading\u2026"));
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
    const parts = data.path.split("/").filter(Boolean);
    const short = parts.length > 2 ? "\u2026/" + parts.slice(-2).join("/") : data.path;
    crumb.textContent = short;

    const mdCount = document.getElementById("browserMdCount");
    const n = data.md_file_count || 0;
    mdCount.textContent = n === 0 ? "No .md files here" : n + " .md file" + (n !== 1 ? "s" : "");
    mdCount.className = "browser-md-count" + (n === 0 ? " none" : "");

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
    btn.textContent = "Ingesting\u2026";
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
        if (data.files_queued > 0) setTimeout(refreshFacets, 5000);
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

  async function pollStatus() {
    try {
      const d = await fetch("/status").then(r => r.json());
      setText("statTotal", d.total);
      setText("statCompleted", d.completed);
      setText("statInProgress", d.in_progress);
      setText("statFailed", d.failed);
      if (d.total > 0) {
        const pct = Math.round((d.completed + d.failed) / d.total * 100);
        show("progressWrap");
        document.getElementById("progressFill").style.width = pct + "%";
        setText("progressInfo", (d.completed + d.failed) + " / " + d.total + " (" + pct + "%)");
      }
    } catch (_) {}

    try {
      const mlx = await fetch("/mlx-status").then(r => r.json());
      const dot = document.getElementById("mlxDot");
      if (mlx.online) {
        dot.className = "dot online";
        setText("mlxStatus", "Online");
        setText("mlxModel", mlx.model || "unknown model");
      } else {
        dot.className = "dot warn";
        setText("mlxStatus", mlx.message || "Offline");
        setText("mlxModel", "\u2014");
      }
    } catch (_) {}
  }

  function makeChipCloud(container, entries, prefix, emptyText) {
    container.replaceChildren();
    if (!entries || !entries.length) {
      const s = document.createElement("span");
      s.className = "no-filters";
      s.textContent = emptyText;
      container.appendChild(s);
      return;
    }
    for (const { name, count } of entries) {
      const chip = document.createElement("span");
      chip.className = "chip";
      const label = document.createTextNode(prefix + name + "\u00a0");
      const badge = document.createElement("span");
      badge.style.cssText = "opacity:0.55;font-size:9px;";
      badge.textContent = count;
      chip.append(label, badge);
      chip.addEventListener("click", () => addFilter(name));
      container.appendChild(chip);
    }
  }

  async function refreshFacets() {
    try {
      const f = await fetch("/facets").then(r => r.json());
      makeChipCloud(document.getElementById("tagCloud"), f.tags, "#", "Ingest documents to see tags");
      makeChipCloud(document.getElementById("topicCloud"), f.topics, "", "—");
      makeChipCloud(document.getElementById("entityCloud"), f.entities, "", "—");
    } catch (_) {}
  }

  pollStatus();
  setInterval(pollStatus, 3000);
  refreshFacets();
  browseDir(null);
</script>
</body>
</html>"##;
