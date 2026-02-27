use axum::response::Html;

pub async fn handle_ui() -> Html<&'static str> {
    Html(HTML)
}

const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Eagle3 — AI Summarizer</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      background: #0f1117; color: #e2e8f0; min-height: 100vh;
    }
    header {
      background: #1a1d27; border-bottom: 1px solid #2d3148;
      padding: 16px 24px; display: flex; align-items: center; gap: 16px;
    }
    header h1 { font-size: 18px; font-weight: 600; color: #fff; }
    header span { color: #6c7aad; font-size: 13px; }
    .status-bar {
      background: #1a1d27; border-bottom: 1px solid #2d3148;
      padding: 10px 24px; display: flex; gap: 24px; align-items: center;
      font-size: 13px;
    }
    .status-item { display: flex; align-items: center; gap: 6px; }
    .dot { width: 8px; height: 8px; border-radius: 50%; background: #4ade80; }
    .dot.warn { background: #facc15; }
    .dot.idle { background: #6c7aad; }
    .stat-value { font-weight: 600; color: #fff; }
    .stat-label { color: #6c7aad; }
    main { max-width: 860px; margin: 0 auto; padding: 32px 24px; }
    .section { margin-bottom: 40px; }
    h2 {
      font-size: 14px; font-weight: 600; color: #6c7aad;
      text-transform: uppercase; letter-spacing: 0.06em; margin-bottom: 16px;
    }
    .row { display: flex; gap: 8px; }
    input[type="text"] {
      flex: 1; background: #1a1d27; border: 1px solid #2d3148;
      border-radius: 8px; color: #e2e8f0; padding: 10px 14px;
      font-size: 14px; outline: none; transition: border-color 0.15s;
    }
    input[type="text"]:focus { border-color: #6c7aad; }
    input[type="text"]::placeholder { color: #3d4468; }
    button {
      background: #4f5fd4; border: none; border-radius: 8px; color: #fff;
      cursor: pointer; font-size: 14px; font-weight: 500; padding: 10px 20px;
      transition: background 0.15s; white-space: nowrap;
    }
    button:hover { background: #6370e0; }
    button:disabled { background: #2d3148; color: #4a5280; cursor: not-allowed; }
    .progress-bar {
      background: #1a1d27; border: 1px solid #2d3148; border-radius: 8px;
      overflow: hidden; height: 8px; margin: 12px 0;
    }
    .progress-fill {
      height: 100%;
      background: linear-gradient(90deg, #4f5fd4, #818cf8);
      transition: width 0.3s ease;
    }
    .progress-info { font-size: 13px; color: #6c7aad; margin-top: 6px; }
    .results { display: flex; flex-direction: column; gap: 12px; }
    .result-card {
      background: #1a1d27; border: 1px solid #2d3148;
      border-radius: 10px; padding: 16px; transition: border-color 0.15s;
    }
    .result-card:hover { border-color: #4f5fd4; }
    .result-title { font-size: 15px; font-weight: 600; color: #fff; margin-bottom: 6px; }
    .result-file { font-size: 12px; color: #4a5280; font-family: monospace; margin-bottom: 8px; }
    .result-tldr { font-size: 14px; color: #a0aec0; line-height: 1.5; }
    .tags { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 10px; }
    .tag { background: #2d3148; border-radius: 4px; color: #818cf8; font-size: 12px; padding: 2px 8px; }
    .score { font-size: 12px; color: #4a5280; margin-top: 8px; }
    .notice {
      background: #1e2035; border: 1px solid #2d3148; border-radius: 8px;
      color: #818cf8; font-size: 13px; padding: 10px 14px; margin-top: 8px;
    }
    .empty { color: #4a5280; font-size: 14px; padding: 24px 0; text-align: center; }
  </style>
</head>
<body>

<header>
  <h1>Eagle3</h1>
  <span>Local AI Summarizer</span>
</header>

<div class="status-bar">
  <div class="status-item">
    <div class="dot idle" id="mlxDot"></div>
    <span class="stat-label">MLX Sidecar</span>
    <span class="stat-value" id="mlxStatus">Checking...</span>
  </div>
  <div class="status-item">
    <span class="stat-label">Total</span>
    <span class="stat-value" id="statTotal">0</span>
  </div>
  <div class="status-item">
    <span class="stat-label">Done</span>
    <span class="stat-value" id="statCompleted">0</span>
  </div>
  <div class="status-item">
    <span class="stat-label">In Progress</span>
    <span class="stat-value" id="statInProgress">0</span>
  </div>
  <div class="status-item">
    <span class="stat-label">Failed</span>
    <span class="stat-value" id="statFailed">0</span>
  </div>
</div>

<main>
  <div class="section">
    <h2>Ingest Directory</h2>
    <div class="row">
      <input type="text" id="ingestPath" placeholder="/path/to/your/markdown/files" />
      <button id="ingestBtn" onclick="startIngest()">Start Ingest</button>
    </div>
    <div id="ingestNotice" style="display:none" class="notice"></div>
    <div id="progressContainer" style="display:none">
      <div class="progress-bar">
        <div class="progress-fill" id="progressFill" style="width:0%"></div>
      </div>
      <p class="progress-info" id="progressInfo"></p>
    </div>
  </div>

  <div class="section">
    <h2>Search Summaries</h2>
    <div class="row">
      <input type="text" id="searchQuery"
             placeholder="kubernetes auth migration..."
             onkeydown="if(event.key==='Enter') doSearch()" />
      <button onclick="doSearch()">Search</button>
    </div>
  </div>

  <div id="results" class="results"></div>
</main>

<script>
  function setText(id, text) {
    document.getElementById(id).textContent = text;
  }

  async function pollStatus() {
    try {
      const res = await fetch('/status');
      const d = await res.json();
      setText('statTotal', d.total);
      setText('statCompleted', d.completed);
      setText('statInProgress', d.in_progress);
      setText('statFailed', d.failed);

      if (d.total > 0) {
        const pct = Math.round((d.completed + d.failed) / d.total * 100);
        document.getElementById('progressContainer').style.display = 'block';
        document.getElementById('progressFill').style.width = pct + '%';
        setText('progressInfo', `${d.completed + d.failed} / ${d.total} files (${pct}%)`);
      }
    } catch (_) {}

    try {
      await fetch('http://localhost:8100/health',
        { signal: AbortSignal.timeout(1000) });
      document.getElementById('mlxDot').className = 'dot';
      setText('mlxStatus', 'Online');
    } catch (_) {
      document.getElementById('mlxDot').className = 'dot warn';
      setText('mlxStatus', 'Offline');
    }
  }

  async function startIngest() {
    const path = document.getElementById('ingestPath').value.trim();
    if (!path) return;
    const btn = document.getElementById('ingestBtn');
    btn.disabled = true;
    btn.textContent = 'Starting...';
    const notice = document.getElementById('ingestNotice');
    notice.style.display = 'block';
    try {
      const res = await fetch('/ingest', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ path }),
      });
      const d = await res.json();
      notice.textContent = res.ok
        ? `Started — ${d.files_queued} queued, ${d.files_skipped} skipped`
        : `Error: ${JSON.stringify(d)}`;
    } catch (e) {
      notice.textContent = 'Error: ' + e.message;
    }
    btn.disabled = false;
    btn.textContent = 'Start Ingest';
  }

  async function doSearch() {
    const q = document.getElementById('searchQuery').value.trim();
    if (!q) return;
    const container = document.getElementById('results');
    container.textContent = '';

    const loading = document.createElement('p');
    loading.className = 'empty';
    loading.textContent = 'Searching...';
    container.appendChild(loading);

    try {
      const res = await fetch('/search?q=' + encodeURIComponent(q) + '&limit=20');
      const data = await res.json();
      container.textContent = '';

      if (!data.length) {
        const empty = document.createElement('p');
        empty.className = 'empty';
        empty.textContent = 'No results found.';
        container.appendChild(empty);
        return;
      }

      for (const r of data) {
        const card = document.createElement('div');
        card.className = 'result-card';

        const title = document.createElement('div');
        title.className = 'result-title';
        title.textContent = r.title || r.file;
        card.appendChild(title);

        const file = document.createElement('div');
        file.className = 'result-file';
        file.textContent = r.file;
        card.appendChild(file);

        const tldr = document.createElement('div');
        tldr.className = 'result-tldr';
        tldr.textContent = r.tldr;
        card.appendChild(tldr);

        if (r.tags) {
          const tagsDiv = document.createElement('div');
          tagsDiv.className = 'tags';
          for (const t of r.tags.split(' ').filter(Boolean)) {
            const tag = document.createElement('span');
            tag.className = 'tag';
            tag.textContent = t;
            tagsDiv.appendChild(tag);
          }
          card.appendChild(tagsDiv);
        }

        const score = document.createElement('div');
        score.className = 'score';
        score.textContent = 'Score: ' + r.score.toFixed(3);
        card.appendChild(score);

        container.appendChild(card);
      }
    } catch (e) {
      container.textContent = '';
      const err = document.createElement('p');
      err.className = 'empty';
      err.textContent = 'Error: ' + e.message;
      container.appendChild(err);
    }
  }

  pollStatus();
  setInterval(pollStatus, 3000);
</script>
</body>
</html>"#;
