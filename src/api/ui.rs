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
      --bg: #08080a;
      --surface: #141418;
      --surface-2: #1c1c22;
      --border: #2a2a35;
      --border-subtle: #1e1e28;
      --text: #f0f0f5;
      --text-2: #9d9db5;
      --text-3: #55556a;
      --accent: #6366f1;
      --accent-dim: rgba(99,102,241,0.10);
      --accent-hover: #818cf8;
      --accent-glow: rgba(99,102,241,0.18);
      --gradient-accent: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
      --green: #34d399;
      --yellow: #fbbf24;
      --red: #f87171;
      --font: "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      --mono: "JetBrains Mono", ui-monospace, "SF Mono", "Fira Code", monospace;
      --r: 10px;
      --r-sm: 6px;
      --sidebar-w: 240px;
      --drawer-w: 360px;
    }

    body {
      font-family: var(--font); background: var(--bg); color: var(--text);
      height: 100vh; overflow: hidden;
      -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale;
    }

    .layout { display: flex; height: 100vh; overflow: hidden; }

    /* ── Sidebar ── */
    .sidebar {
      width: var(--sidebar-w); flex-shrink: 0;
      background: var(--surface); border-right: 1px solid var(--border-subtle);
      padding: 20px 16px; display: flex; flex-direction: column; gap: 20px;
      height: 100vh; overflow-y: auto; overflow-x: hidden;
    }
    .sidebar::-webkit-scrollbar { width: 3px; }
    .sidebar::-webkit-scrollbar-track { background: transparent; }
    .sidebar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
    .sidebar::-webkit-scrollbar-thumb:hover { background: var(--text-3); }

    .logo {
      font-size: 18px; font-weight: 800; letter-spacing: -0.04em;
      color: var(--text); display: flex; align-items: center; gap: 8px;
      padding-bottom: 4px;
    }
    .logo-icon {
      width: 26px; height: 26px; border-radius: 8px;
      background: var(--gradient-accent);
      display: flex; align-items: center; justify-content: center;
      font-size: 14px; color: #fff; flex-shrink: 0;
    }
    .logo span {
      background: var(--gradient-accent);
      -webkit-background-clip: text; -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .section-label {
      font-size: 10px; font-weight: 600; text-transform: uppercase;
      letter-spacing: 0.1em; color: var(--text-3); margin-bottom: 8px;
      display: flex; align-items: center; gap: 6px;
    }
    .section-label-icon { font-size: 11px; opacity: 0.7; }

    .sidebar-divider {
      height: 1px; background: var(--border-subtle); margin: 2px 0;
    }

    /* ── Inference Block ── */
    .mlx-block {
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: var(--r); padding: 12px 14px;
      transition: border-color 0.2s;
    }
    .mlx-block.online { border-color: rgba(52,211,153,0.25); background: linear-gradient(135deg, var(--surface-2), rgba(52,211,153,0.04)); }
    .mlx-block.warn { border-color: rgba(251,191,36,0.25); background: linear-gradient(135deg, var(--surface-2), rgba(251,191,36,0.04)); }
    .mlx-row { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
    .dot { width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0; background: var(--text-3); transition: background 0.3s; }
    .dot.online { background: var(--green); box-shadow: 0 0 8px rgba(52,211,153,0.5); animation: pulse 2.5s ease-in-out infinite; }
    .dot.warn { background: var(--yellow); box-shadow: 0 0 8px rgba(251,191,36,0.4); }
    @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.35; } }
    .mlx-label { font-size: 12px; font-weight: 600; color: var(--text); }
    .mlx-model { font-size: 10px; color: var(--text-3); font-family: var(--mono); line-height: 1.5; word-break: break-all; }

    /* ── Buttons ── */
    .btn {
      width: 100%; background: var(--gradient-accent); border: none; border-radius: var(--r-sm);
      color: #fff; font-size: 12px; font-weight: 600; padding: 8px 12px;
      cursor: pointer; transition: transform 0.15s, box-shadow 0.15s, opacity 0.15s;
      letter-spacing: 0.01em;
    }
    .btn:hover { transform: translateY(-1px); box-shadow: 0 4px 16px rgba(99,102,241,0.3); }
    .btn:active { transform: translateY(0); }
    .btn:disabled { opacity: 0.35; cursor: not-allowed; transform: none; box-shadow: none; }

    .btn--ghost {
      background: transparent; border: 1px solid var(--border);
      color: var(--text-2); font-size: 11px; padding: 5px 10px; margin-top: 6px;
    }
    .btn--ghost:hover {
      border-color: rgba(248,113,113,0.4); color: var(--red);
      background: rgba(248,113,113,0.08);
      transform: none; box-shadow: none;
    }
    .btn--ghost:disabled { opacity: 0.35; cursor: not-allowed; border-color: var(--border); color: var(--text-3); background: transparent; }

    /* ── Progress ── */
    .progress-bar { background: var(--surface-2); border-radius: 4px; height: 4px; overflow: hidden; margin-top: 8px; }
    .progress-fill {
      height: 100%; border-radius: 4px;
      background: linear-gradient(90deg, var(--accent), var(--accent-hover), var(--accent));
      background-size: 200% 100%;
      animation: progress-shimmer 2s ease infinite;
      transition: width 0.4s ease;
    }
    @keyframes progress-shimmer { 0% { background-position: -200% 0; } 100% { background-position: 200% 0; } }
    .ingest-notice { font-size: 11px; color: var(--text-2); margin-top: 6px; line-height: 1.4; }

    /* ── Stats ── */
    .stats-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }
    .stat-cell {
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: var(--r-sm); padding: 10px 8px; text-align: center;
      transition: border-color 0.2s, background 0.2s;
    }
    .stat-cell:hover { border-color: var(--text-3); }
    .stat-cell .val { font-size: 20px; font-weight: 800; color: var(--text); line-height: 1; margin-bottom: 2px; letter-spacing: -0.03em; }
    .stat-cell .lbl { font-size: 9px; color: var(--text-3); text-transform: uppercase; letter-spacing: 0.08em; font-weight: 500; }
    .stat-cell.prog .val { color: var(--accent); }
    .stat-cell.prog { border-color: rgba(99,102,241,0.2); }
    .stat-cell.fail .val { color: var(--red); }
    .stat-cell.fail { border-color: rgba(248,113,113,0.15); }

    /* ── Filter Chips ── */
    .filter-chips { display: flex; flex-wrap: wrap; gap: 4px; }

    .chip {
      display: inline-flex; align-items: center; gap: 3px;
      border-radius: 4px; font-size: 11px;
      padding: 2px 8px; cursor: pointer;
      transition: transform 0.1s, background 0.15s;
      font-weight: 500;
    }
    .chip:hover { transform: scale(1.03); }
    .chip:active { transform: scale(0.97); }
    .chip .x { font-size: 9px; margin-left: 2px; opacity: 0.5; }

    .chip--tag {
      background: var(--accent-dim); border: 1px solid rgba(99,102,241,0.2);
      color: var(--accent-hover);
    }
    .chip--tag:hover { background: rgba(99,102,241,0.2); }

    .chip--topic {
      background: rgba(56,189,248,0.08); border: 1px solid rgba(56,189,248,0.18);
      color: #7dd3fc;
    }
    .chip--topic:hover { background: rgba(56,189,248,0.15); }

    .chip--entity {
      background: rgba(251,191,36,0.08); border: 1px solid rgba(251,191,36,0.18);
      color: #fcd34d;
    }
    .chip--entity:hover { background: rgba(251,191,36,0.15); }

    .chip--active {
      background: var(--accent-dim); border: 1px solid rgba(99,102,241,0.3);
      color: var(--accent-hover);
    }
    .chip--active:hover { background: rgba(99,102,241,0.2); }

    .no-filters { font-size: 11px; color: var(--text-3); font-style: italic; }

    /* ── Results Column ── */
    .results-col { flex: 1; min-width: 280px; display: flex; flex-direction: column; overflow: hidden; }

    .search-bar-wrap {
      padding: 24px 32px 20px; position: sticky; top: 0;
      background: linear-gradient(180deg, var(--bg) 80%, transparent 100%);
      z-index: 10;
    }
    .search-bar {
      display: flex; align-items: center; gap: 12px;
      background: var(--surface); border: 1px solid var(--border);
      border-radius: 14px; padding: 12px 18px;
      transition: border-color 0.2s, box-shadow 0.3s, background 0.2s;
    }
    .search-bar:focus-within {
      border-color: var(--accent);
      box-shadow: 0 0 0 3px var(--accent-dim), 0 8px 32px rgba(99,102,241,0.12);
      background: var(--surface-2);
    }
    .search-icon { color: var(--text-3); font-size: 16px; flex-shrink: 0; transition: color 0.2s; }
    .search-bar:focus-within .search-icon { color: var(--accent); }
    .search-input {
      flex: 1; background: none; border: none; color: var(--text);
      font-size: 16px; font-weight: 400; outline: none;
      font-family: var(--font);
    }
    .search-input::placeholder { color: var(--text-3); font-weight: 400; }
    .search-kbd {
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: 5px; color: var(--text-3); font-size: 10px;
      font-family: var(--mono); padding: 3px 7px; flex-shrink: 0;
      font-weight: 500; letter-spacing: 0.02em;
    }

    .results-area { padding: 4px 32px 32px; flex: 1; overflow-y: auto; }
    .results-area::-webkit-scrollbar { width: 4px; }
    .results-area::-webkit-scrollbar-track { background: transparent; }
    .results-area::-webkit-scrollbar-thumb { background: var(--border); border-radius: 4px; }

    .results-count { font-size: 12px; color: var(--text-3); margin-bottom: 16px; font-weight: 500; }
    .results-list { display: flex; flex-direction: column; gap: 6px; }

    /* ── Cards ── */
    .card {
      background: var(--surface); border: 1px solid var(--border-subtle);
      border-radius: var(--r); padding: 14px 16px; cursor: pointer;
      transition: transform 0.2s, border-color 0.2s, box-shadow 0.2s, background 0.2s;
      position: relative;
      animation: fadeInUp 0.35s ease both;
    }
    .card:hover {
      transform: translateY(-1px);
      border-color: rgba(99,102,241,0.3);
      box-shadow: 0 4px 16px rgba(0,0,0,0.25);
      background: var(--surface-2);
    }
    .card.active {
      border-color: var(--accent);
      background: var(--accent-dim);
      box-shadow: inset 3px 0 0 var(--accent), 0 0 16px var(--accent-glow);
    }

    .card--featured {
      padding: 18px 20px;
      border-left: 3px solid var(--accent);
      background: linear-gradient(135deg, var(--surface) 0%, rgba(99,102,241,0.03) 100%);
    }
    .card--featured .card-title { font-size: 16px; }
    .card--featured .card-tldr { -webkit-line-clamp: 3; }
    .card--featured.active { box-shadow: inset 3px 0 0 var(--accent-hover), 0 0 20px var(--accent-glow); }

    @keyframes fadeInUp {
      from { opacity: 0; transform: translateY(6px); }
      to { opacity: 1; transform: translateY(0); }
    }

    .card-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 10px; margin-bottom: 4px; }
    .card-title { font-size: 14px; font-weight: 700; color: var(--text); line-height: 1.3; letter-spacing: -0.01em; }
    .card-score {
      font-size: 10px; font-weight: 700; color: var(--accent-hover);
      background: var(--accent-dim); border: 1px solid rgba(99,102,241,0.2);
      border-radius: 5px; padding: 2px 7px; flex-shrink: 0;
      font-family: var(--mono); letter-spacing: -0.02em;
    }
    .card-file { font-size: 11px; font-family: var(--mono); color: var(--text-3); margin-bottom: 6px; letter-spacing: -0.01em; }
    .card-tldr {
      font-size: 13px; color: var(--text-2); line-height: 1.6; margin-bottom: 8px;
      display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;
    }
    .card-tags { display: flex; flex-wrap: wrap; gap: 4px; }
    .tag {
      background: var(--surface-2); border-radius: 4px; color: var(--text-3);
      font-size: 10px; padding: 2px 7px; cursor: pointer;
      transition: color 0.15s, background 0.15s, transform 0.1s;
      font-weight: 500;
    }
    .tag:hover { background: var(--accent-dim); color: var(--accent-hover); transform: scale(1.04); }

    /* ── Empty / Placeholder ── */
    .placeholder {
      display: flex; flex-direction: column; align-items: center;
      justify-content: center; padding: 100px 0; gap: 16px; color: var(--text-3);
    }
    .placeholder-icon {
      width: 64px; height: 64px; border-radius: 20px;
      background: var(--surface); border: 1px solid var(--border);
      display: flex; align-items: center; justify-content: center;
      position: relative; overflow: hidden;
    }
    .placeholder-icon::before {
      content: ""; position: absolute; inset: -20px;
      background: radial-gradient(circle at 50% 50%, var(--accent-glow), transparent 70%);
      animation: orbDrift 6s ease-in-out infinite;
    }
    @keyframes orbDrift {
      0%, 100% { transform: translate(0, 0) scale(1); }
      33% { transform: translate(4px, -6px) scale(1.1); }
      66% { transform: translate(-4px, 4px) scale(0.95); }
    }
    .placeholder-title { font-size: 15px; font-weight: 600; color: var(--text-2); letter-spacing: -0.01em; }
    .placeholder-hint {
      font-size: 11px; color: var(--text-3); font-family: var(--mono);
      display: flex; gap: 12px; margin-top: 4px;
    }
    .placeholder-hint span { opacity: 0.6; }

    /* ── Skeleton ── */
    .skeleton {
      border-radius: var(--r);
      background: linear-gradient(90deg, var(--surface) 25%, var(--surface-2) 50%, var(--surface) 75%);
      background-size: 200% 100%;
      animation: skeleton-sweep 1.6s ease infinite;
    }
    @keyframes skeleton-sweep { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }
    .sk-card { height: 92px; border-radius: var(--r); margin-bottom: 6px; }

    /* ── Detail Panel ── */
    .detail-panel {
      width: 42%; flex-shrink: 0;
      border-left: 1px solid var(--border-subtle);
      background: var(--surface);
      overflow-y: auto;
      display: flex; flex-direction: column;
    }
    .detail-panel::-webkit-scrollbar { width: 4px; }
    .detail-panel::-webkit-scrollbar-track { background: transparent; }
    .detail-panel::-webkit-scrollbar-thumb { background: var(--border); border-radius: 4px; }

    @media (max-width: 900px) {
      .detail-panel { display: none; }
    }

    .detail-placeholder {
      flex: 1; display: flex; flex-direction: column;
      align-items: center; justify-content: center;
      gap: 12px; color: var(--text-3); padding: 40px;
    }
    .detail-placeholder p { font-size: 13px; color: var(--text-3); }
    .detail-placeholder-hint { font-size: 11px; font-family: var(--mono); opacity: 0.5; }

    .detail-body {
      padding: 24px; display: flex; flex-direction: column; gap: 20px;
      animation: detailSlideIn 0.3s ease both;
    }
    @keyframes detailSlideIn {
      from { opacity: 0; transform: translateY(8px); }
      to { opacity: 1; transform: translateY(0); }
    }
    .detail-title {
      font-size: 19px; font-weight: 800; color: var(--text);
      line-height: 1.3; letter-spacing: -0.025em;
    }
    .detail-tldr { font-size: 14px; color: var(--text-2); line-height: 1.7; }
    .detail-meta {
      font-size: 11px; color: var(--text-3); padding-top: 8px;
      border-top: 1px solid var(--border); display: flex; gap: 16px;
    }
    .detail-meta strong { color: var(--text-2); font-weight: 600; }
    .detail-slabel {
      font-size: 10px; font-weight: 700; text-transform: uppercase;
      letter-spacing: 0.08em; color: var(--text-3); margin-bottom: 6px;
    }
    .detail-chips { display: flex; flex-wrap: wrap; gap: 4px; }
    .detail-chip {
      background: var(--surface-2); border-radius: 5px; color: var(--text-2);
      font-size: 12px; padding: 3px 9px; cursor: pointer;
      transition: background 0.15s, color 0.15s, transform 0.1s;
      font-weight: 500;
    }
    .detail-chip:hover { background: var(--accent-dim); color: var(--accent-hover); transform: scale(1.03); }
    .detail-chip.plain { cursor: default; }
    .detail-chip.plain:hover { background: var(--surface-2); color: var(--text-2); transform: none; }
    .detail-related-divider {
      font-size: 10px; color: var(--text-3); border-top: 1px solid var(--border);
      padding-top: 14px; margin-top: 4px; font-weight: 700;
      text-transform: uppercase; letter-spacing: 0.08em;
    }
    .detail-related-list { display: flex; flex-direction: column; gap: 6px; }
    .related-card {
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: 8px; padding: 12px 14px; cursor: pointer;
      transition: border-color 0.2s, transform 0.15s, box-shadow 0.2s;
    }
    .related-card:hover { border-color: rgba(99,102,241,0.4); transform: translateY(-1px); box-shadow: 0 2px 8px rgba(0,0,0,0.2); }
    .related-card-title { font-size: 13px; font-weight: 600; color: var(--text); line-height: 1.3; margin-bottom: 3px; }
    .related-card-tldr { font-size: 12px; color: var(--text-3); display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; line-height: 1.5; }

    /* ── Drawer (Mobile) ── */
    .drawer-overlay {
      position: fixed; inset: 0; background: rgba(0,0,0,0.6);
      backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px);
      opacity: 0; pointer-events: none; transition: opacity 0.3s; z-index: 40;
    }
    .drawer-overlay.open { opacity: 1; pointer-events: all; }

    .drawer {
      position: fixed; top: 0; right: 0; bottom: 0; width: var(--drawer-w);
      background: var(--surface); border-left: 1px solid var(--border);
      transform: translateX(100%);
      transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      z-index: 50; display: flex; flex-direction: column; overflow: hidden;
    }
    .drawer.open { transform: translateX(0); }

    .drawer-header {
      padding: 18px 20px; border-bottom: 1px solid var(--border);
      display: flex; align-items: flex-start; justify-content: space-between; gap: 12px;
    }
    .drawer-title { font-size: 16px; font-weight: 700; color: var(--text); line-height: 1.3; letter-spacing: -0.01em; }
    .drawer-close {
      background: var(--surface-2); border: 1px solid var(--border); border-radius: var(--r-sm);
      color: var(--text-2); font-size: 13px; width: 30px; height: 30px;
      display: flex; align-items: center; justify-content: center;
      cursor: pointer; flex-shrink: 0; transition: background 0.15s, transform 0.1s;
    }
    .drawer-close:hover { background: var(--border); transform: scale(1.05); }

    .drawer-body { flex: 1; overflow-y: auto; padding: 18px 20px; display: flex; flex-direction: column; gap: 20px; }
    .drawer-slabel { font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em; color: var(--text-3); margin-bottom: 6px; }
    .drawer-text { font-size: 13px; color: var(--text-2); line-height: 1.7; }
    .drawer-file { font-size: 11px; font-family: var(--mono); color: var(--text-3); word-break: break-all; }
    .drawer-chips { display: flex; flex-wrap: wrap; gap: 4px; }
    .drawer-chip {
      background: var(--surface-2); border-radius: 4px; color: var(--text-2);
      font-size: 12px; padding: 3px 8px; cursor: pointer;
      transition: background 0.15s, color 0.15s;
    }
    .drawer-chip:hover { background: var(--accent-dim); color: var(--accent-hover); }
    .drawer-chip.static { cursor: default; }
    .drawer-chip.static:hover { background: var(--surface-2); color: var(--text-2); }

    .drawer-meta { display: flex; gap: 16px; padding-top: 8px; border-top: 1px solid var(--border); }
    .drawer-meta-item { font-size: 12px; color: var(--text-3); }
    .drawer-meta-item strong { color: var(--text-2); font-weight: 600; }

    /* ── Browser ── */
    .browser-wrap { display: flex; flex-direction: column; gap: 6px; }
    .browser-crumb {
      font-size: 11px; color: var(--text-2); font-family: var(--mono);
      word-break: break-all; line-height: 1.4; min-height: 14px;
    }
    .browser-list {
      max-height: 180px; overflow-y: auto;
      border: 1px solid var(--border); border-radius: var(--r-sm);
      background: var(--bg);
    }
    .browser-list::-webkit-scrollbar { width: 3px; }
    .browser-list::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
    .browser-row {
      display: flex; align-items: center; gap: 8px;
      padding: 6px 10px; font-size: 12px; cursor: pointer;
      color: var(--text-2); user-select: none;
      transition: background 0.12s, color 0.12s;
    }
    .browser-row:hover { background: var(--accent-dim); color: var(--text); }
    .browser-row.parent { color: var(--text-3); font-style: italic; }
    .browser-row.parent:hover { color: var(--text-2); }
    .browser-icon { opacity: 0.5; flex-shrink: 0; font-size: 13px; }
    .browser-empty {
      padding: 12px 10px; font-size: 12px;
      color: var(--text-3); text-align: center;
    }
    .browser-md-count {
      font-size: 11px; color: var(--text-2); min-height: 16px; font-weight: 500;
    }
    .browser-md-count.none { color: var(--text-3); font-weight: 400; }

    /* ── Toasts ── */
    .toast-container {
      position: fixed; bottom: 20px; right: 20px; z-index: 100;
      display: flex; flex-direction: column-reverse; gap: 8px;
      pointer-events: none;
    }
    .toast {
      pointer-events: auto;
      background: var(--surface-2); border: 1px solid var(--border);
      border-radius: var(--r); padding: 10px 16px;
      font-size: 13px; color: var(--text-2); line-height: 1.4;
      display: flex; align-items: center; gap: 10px;
      box-shadow: 0 8px 24px rgba(0,0,0,0.4);
      animation: toastIn 0.3s ease both;
      max-width: 340px; cursor: pointer;
    }
    .toast.removing { animation: toastOut 0.25s ease both; }
    .toast-icon { flex-shrink: 0; font-size: 14px; }
    .toast--success { border-color: rgba(52,211,153,0.3); }
    .toast--success .toast-icon { color: var(--green); }
    .toast--error { border-color: rgba(248,113,113,0.3); }
    .toast--error .toast-icon { color: var(--red); }
    .toast--info { border-color: rgba(99,102,241,0.3); }
    .toast--info .toast-icon { color: var(--accent-hover); }
    @keyframes toastIn {
      from { opacity: 0; transform: translateY(12px) scale(0.96); }
      to { opacity: 1; transform: translateY(0) scale(1); }
    }
    @keyframes toastOut {
      from { opacity: 1; transform: translateY(0) scale(1); }
      to { opacity: 0; transform: translateY(8px) scale(0.96); }
    }
  </style>
</head>
<body>

<div class="layout">
  <aside class="sidebar">
    <div class="logo">
      <div class="logo-icon">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M2 12s4-8 10-8 10 8 10 8-4 8-10 8-10-8-10-8z"/>
        </svg>
      </div>
      hawk<span>eye</span>
    </div>

    <div>
      <div class="section-label"><span class="section-label-icon">⚡</span> Inference</div>
      <div class="mlx-block" id="mlxBlock">
        <div class="mlx-row">
          <div class="dot" id="mlxDot"></div>
          <span class="mlx-label" id="mlxStatus">Checking…</span>
        </div>
        <div class="mlx-model" id="mlxModel">—</div>
      </div>
    </div>

    <div class="sidebar-divider"></div>

    <div>
      <div class="section-label"><span class="section-label-icon">📂</span> Ingest</div>
      <div class="browser-wrap">
        <div class="browser-crumb" id="browserCrumb"></div>
        <div class="browser-list" id="browserList"></div>
        <div class="browser-md-count" id="browserMdCount"></div>
        <button class="btn" id="ingestBtn" onclick="ingestCurrent()">Ingest this directory</button>
      </div>
      <div id="progressWrap" style="display:none">
        <div class="progress-bar"><div class="progress-fill" id="progressFill" style="width:0%"></div></div>
        <div class="ingest-notice" id="progressInfo"></div>
      </div>
    </div>

    <div class="sidebar-divider"></div>

    <div>
      <div class="section-label"><span class="section-label-icon">📊</span> Queue</div>
      <div class="stats-grid">
        <div class="stat-cell"><div class="val" id="statTotal">0</div><div class="lbl">Total</div></div>
        <div class="stat-cell"><div class="val" id="statCompleted">0</div><div class="lbl">Done</div></div>
        <div class="stat-cell prog"><div class="val" id="statInProgress">0</div><div class="lbl">Active</div></div>
        <div class="stat-cell fail"><div class="val" id="statFailed">0</div><div class="lbl">Failed</div></div>
      </div>
      <button class="btn btn--ghost" id="cancelBtn" onclick="cancelQueue()">Cancel queued jobs</button>
    </div>

    <div class="sidebar-divider"></div>

    <div>
      <div class="section-label"><span class="section-label-icon">🏷</span> Tags</div>
      <div class="filter-chips" id="tagCloud">
        <span class="no-filters">Ingest documents to see tags</span>
      </div>
    </div>

    <div>
      <div class="section-label"><span class="section-label-icon">💡</span> Topics</div>
      <div class="filter-chips" id="topicCloud">
        <span class="no-filters">—</span>
      </div>
    </div>

    <div>
      <div class="section-label"><span class="section-label-icon">👤</span> Entities</div>
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
        <svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"/>
          <path d="m21 21-4.35-4.35"/>
        </svg>
        <input class="search-input" id="searchInput" placeholder="Search summaries…" autocomplete="off" spellcheck="false" />
        <span class="search-kbd">⌘K</span>
      </div>
    </div>
    <div class="results-area">
      <div id="resultsContainer">
        <div class="placeholder">
          <div class="placeholder-icon">
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="position:relative;z-index:1;opacity:0.5">
              <circle cx="11" cy="11" r="8"/>
              <path d="m21 21-4.35-4.35"/>
            </svg>
          </div>
          <div class="placeholder-title">Index markdown. Surface insights.</div>
          <div class="placeholder-hint">
            <span>⌘K to focus</span>
            <span>·</span>
            <span>ESC to clear</span>
          </div>
        </div>
      </div>
    </div>
  </main>

  <div class="detail-panel" id="detailPanel">
    <div class="detail-placeholder" id="detailPlaceholder">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M9 18l6-6-6-6"/>
      </svg>
      <p>Select a result to explore</p>
      <div class="detail-placeholder-hint">Click any card · ESC to dismiss</div>
    </div>
  </div>
</div>

<div id="toastContainer" class="toast-container"></div>

<div class="drawer-overlay" id="drawerOverlay" onclick="closeDrawer()"></div>
<div class="drawer" id="drawer">
  <div class="drawer-header">
    <div class="drawer-title" id="drawerTitle">—</div>
    <button class="drawer-close" onclick="closeDrawer()">✕</button>
  </div>
  <div class="drawer-body" id="drawerBody"></div>
</div>

<script>
  let activeFilters = new Set();
  let activeCardFile = null;
  let searchTimer = null;
  let currentResults = [];

  function setText(id, t) { document.getElementById(id).textContent = t; }
  function show(id) { document.getElementById(id).style.display = ""; }

  function showToast(message, type) {
    const icons = { success: "\u2714", error: "\u2718", info: "\u2139" };
    const container = document.getElementById("toastContainer");
    const toast = document.createElement("div");
    toast.className = "toast toast--" + (type || "info");
    const icon = document.createElement("span");
    icon.className = "toast-icon";
    icon.textContent = icons[type] || icons.info;
    const msg = document.createElement("span");
    msg.textContent = message;
    toast.append(icon, msg);
    container.appendChild(toast);
    function dismiss() {
      if (toast.classList.contains("removing")) return;
      clearTimeout(timer);
      toast.classList.add("removing");
      toast.addEventListener("animationend", () => toast.remove());
    }
    toast.addEventListener("click", dismiss);
    const timer = setTimeout(dismiss, 4000);
  }

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
      sk.style.animationDelay = (i * 80) + "ms";
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
      const iconWrap = document.createElement("div");
      iconWrap.className = "placeholder-icon";
      const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svg.setAttribute("width", "28"); svg.setAttribute("height", "28");
      svg.setAttribute("viewBox", "0 0 24 24"); svg.setAttribute("fill", "none");
      svg.setAttribute("stroke", "currentColor"); svg.setAttribute("stroke-width", "1.5");
      svg.setAttribute("stroke-linecap", "round"); svg.setAttribute("stroke-linejoin", "round");
      svg.style.cssText = "position:relative;z-index:1;opacity:0.5";
      const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
      circle.setAttribute("cx", "11"); circle.setAttribute("cy", "11"); circle.setAttribute("r", "8");
      const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
      path.setAttribute("d", "m21 21-4.35-4.35");
      svg.append(circle, path);
      iconWrap.appendChild(svg);
      wrap.appendChild(iconWrap);

      const title = document.createElement("div");
      title.className = "placeholder-title";
      title.textContent = "Index markdown. Surface insights.";
      wrap.appendChild(title);

      const hint = document.createElement("div");
      hint.className = "placeholder-hint";
      ["\u2318K to focus", "\u00B7", "ESC to clear"].forEach(t => {
        const s = document.createElement("span");
        s.textContent = t;
        hint.appendChild(s);
      });
      wrap.appendChild(hint);
    } else {
      const p = document.createElement("p");
      p.textContent = msg;
      p.style.color = "var(--text-3)";
      wrap.appendChild(p);
    }
    container.replaceChildren(wrap);
  }

  function renderResults(data, query) {
    currentResults = data;
    const container = document.getElementById("resultsContainer");
    if (!data.length) {
      const wrap = document.createElement("div");
      wrap.className = "placeholder";
      const p = document.createElement("div");
      p.className = "placeholder-title";
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
    data.forEach((r, i) => list.appendChild(makeCard(r, i)));

    container.replaceChildren(count, list);

    if (window.innerWidth >= 900 && data.length) {
      selectResult(data[0]);
    }
  }

  function makeCard(r, index) {
    const card = document.createElement("div");
    const isFeatured = index === 0;
    card.className = "card" + (isFeatured ? " card--featured" : "") + (r.file === activeCardFile ? " active" : "");
    card.dataset.file = r.file;
    card.style.animationDelay = (index * 40) + "ms";
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
      chip.className = "chip chip--active";
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
      panel.replaceChildren();
      const ph = document.createElement("div");
      ph.id = "detailPlaceholder";
      ph.className = "detail-placeholder";
      const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svg.setAttribute("width", "32"); svg.setAttribute("height", "32");
      svg.setAttribute("viewBox", "0 0 24 24"); svg.setAttribute("fill", "none");
      svg.setAttribute("stroke", "currentColor"); svg.setAttribute("stroke-width", "1.5");
      svg.setAttribute("opacity", "0.2");
      svg.setAttribute("stroke-linecap", "round"); svg.setAttribute("stroke-linejoin", "round");
      const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
      path.setAttribute("d", "M9 18l6-6-6-6");
      svg.appendChild(path);
      const p = document.createElement("p");
      p.textContent = "Select a result to explore";
      const hint = document.createElement("div");
      hint.className = "detail-placeholder-hint";
      hint.textContent = "Click any card \u00B7 ESC to dismiss";
      ph.append(svg, p, hint);
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
    const panel = document.getElementById("detailPanel");
    const sk = document.createElement("div");
    sk.className = "skeleton";
    sk.style.cssText = "margin:24px;height:200px;border-radius:8px;";
    panel.replaceChildren(sk);

    const related = computeRelated(result, currentResults);

    try {
      const res = await fetch("/summary/" + encodeURIComponent(result.file));
      if (!res.ok) throw new Error("not found");
      const s = await res.json();
      renderDetailPanel(s, related);
    } catch (_) {
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
    btn.disabled = true;
    btn.textContent = "Ingesting\u2026";
    try {
      const res = await fetch("/ingest", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ path: browserPath }),
      });
      const data = await res.json();
      if (res.ok) {
        showToast("Queued " + data.files_queued + " files, skipped " + data.files_skipped, "success");
        if (data.files_queued > 0) setTimeout(refreshFacets, 5000);
      } else {
        showToast("Error: " + (data.message || res.statusText), "error");
      }
    } catch (_) {
      showToast("Network error", "error");
    } finally {
      btn.disabled = false;
      btn.textContent = "Ingest this directory";
    }
  }

  let cancelConfirmTimer = null;

  function cancelQueue() {
    const btn = document.getElementById("cancelBtn");
    if (btn.dataset.confirm !== "1") {
      btn.dataset.confirm = "1";
      btn.textContent = "Confirm cancel?";
      btn.style.borderColor = "rgba(248,113,113,0.5)";
      btn.style.color = "var(--red)";
      btn.style.background = "rgba(248,113,113,0.08)";
      cancelConfirmTimer = setTimeout(() => resetCancelBtn(), 3000);
      return;
    }
    clearTimeout(cancelConfirmTimer);
    doCancelQueue();
  }

  function resetCancelBtn() {
    const btn = document.getElementById("cancelBtn");
    btn.dataset.confirm = "";
    btn.textContent = "Cancel queued jobs";
    btn.style.borderColor = "";
    btn.style.color = "";
    btn.style.background = "";
  }

  async function doCancelQueue() {
    const btn = document.getElementById("cancelBtn");
    btn.disabled = true;
    btn.textContent = "Cancelling\u2026";
    btn.style.borderColor = "";
    btn.style.color = "";
    btn.style.background = "";
    try {
      const res = await fetch("/cancel", { method: "POST" });
      const data = await res.json();
      if (res.ok) {
        showToast("Cancelled " + data.cancelled + " job" + (data.cancelled !== 1 ? "s" : ""), "success");
      } else {
        showToast("Error: " + (data.message || res.statusText), "error");
      }
    } catch (_) {
      showToast("Network error", "error");
    } finally {
      btn.disabled = false;
      btn.dataset.confirm = "";
      btn.textContent = "Cancel queued jobs";
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
      const block = document.getElementById("mlxBlock");
      if (mlx.online) {
        dot.className = "dot online";
        block.className = "mlx-block online";
        setText("mlxStatus", "Online");
        setText("mlxModel", mlx.model || "unknown model");
      } else {
        dot.className = "dot warn";
        block.className = "mlx-block warn";
        setText("mlxStatus", mlx.message || "Offline");
        setText("mlxModel", "\u2014");
      }
    } catch (_) {}
  }

  function makeChipCloud(container, entries, prefix, emptyText, chipClass) {
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
      chip.className = "chip " + chipClass;
      const label = document.createTextNode(prefix + name + "\u00a0");
      const badge = document.createElement("span");
      badge.style.cssText = "opacity:0.5;font-size:9px;font-weight:400;";
      badge.textContent = count;
      chip.append(label, badge);
      chip.addEventListener("click", () => addFilter(name));
      container.appendChild(chip);
    }
  }

  async function refreshFacets() {
    try {
      const f = await fetch("/facets").then(r => r.json());
      makeChipCloud(document.getElementById("tagCloud"), f.tags, "#", "Ingest documents to see tags", "chip--tag");
      makeChipCloud(document.getElementById("topicCloud"), f.topics, "", "\u2014", "chip--topic");
      makeChipCloud(document.getElementById("entityCloud"), f.entities, "", "\u2014", "chip--entity");
    } catch (_) {}
  }

  pollStatus();
  setInterval(pollStatus, 3000);
  refreshFacets();
  browseDir(null);
</script>
</body>
</html>"##;
