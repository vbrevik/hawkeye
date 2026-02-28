# Split-Pane Knowledge Explorer — Design

**Goal:** Replace the current single-column search + slide-out drawer with a persistent three-column layout that lets users explore ingested documents without overlays or navigation changes.

**Architecture:** Three persistent columns — sidebar (facets/ingest), results list (search + compact cards), detail panel (always-visible document detail). All interaction is client-side state; no new API endpoints required.

**Tech Stack:** Vanilla JS + CSS (matches existing inline UI in `src/api/ui.rs`), served as a single HTML string from Axum.

---

## Layout

```
┌─────────────┬──────────────────────┬────────────────────────┐
│  Sidebar    │   Results Column     │   Detail Panel         │
│  ~260px     │   flex-1 (~35%)      │   ~42% viewport        │
│             │                      │                        │
│ [Ingest]    │ ┌──────────────────┐ │ ┌────────────────────┐ │
│             │ │ Search bar       │ │ │ Title              │ │
│ Tags        │ └──────────────────┘ │ │ TL;DR (prominent)  │ │
│ #rust       │                      │ │                    │ │
│ #auth       │ ● Doc A      0.94    │ │ Tags: #rust #axum  │ │
│             │   tldr preview...    │ │ Topics: web, perf  │ │
│ Topics      │                      │ │ Entities: Tokio    │ │
│ web dev     │ ● Doc B      0.87    │ │                    │ │
│ security    │   tldr preview...    │ │ ── Related ──      │ │
│             │                      │ │ ● Doc with #axum   │ │
│ Entities    │ ● Doc C      0.81    │ │ ● Doc with Tokio   │ │
│ Tokio       │   tldr preview...    │ │                    │ │
└─────────────┴──────────────────────┴────────────────────────┘
```

---

## CSS Architecture

```css
.app { display: flex; height: 100vh; overflow: hidden; }
.sidebar { width: 260px; flex-shrink: 0; overflow-y: auto; border-right: 1px solid var(--border); }
.results-col { flex: 1; min-width: 280px; display: flex; flex-direction: column; overflow: hidden; }
.results-list { flex: 1; overflow-y: auto; }
.detail-panel { width: 42%; flex-shrink: 0; border-left: 1px solid var(--border); overflow-y: auto; }

/* Responsive: hide detail panel below 900px, restore drawer behavior */
@media (max-width: 900px) {
  .detail-panel { display: none; }
}
```

---

## Components

### Result Card (compact, ~60px)
- Thin left border whose color intensity maps to relevance score
- Title line (bold, truncated)
- TL;DR preview (one line, muted, truncated)
- First 3 tag pills + `+N more` overflow indicator
- Selected state: distinct background highlight

### Detail Panel
1. **Title** — large heading
2. **TL;DR** — slightly larger body text, visually separated
3. **Metadata row** — `word_count words · created_at` (dim, small)
4. **Chip rows** — Tags, Topics, Entities; each chip is clickable and inserts its value into the search bar, triggering a new search
5. **Related section** — divider + compact card list of docs from the current result set that share ≥1 entity or tag with the selected doc; clicking a related doc loads it into the detail panel

### Empty States
- No search yet: "Search your knowledge base above" (center column)
- No results: "No results for [query]"
- Detail panel before first selection: "Select a document to explore" with a subtle left-pointing arrow

---

## Data Flow

```
User types query
  → GET /search?q=...
  → Render compact result cards in center column
  → Auto-select first result → populate detail panel

User clicks result card
  → Detail panel re-renders from in-memory result payload (no fetch)
  → Card gets selected highlight
  → URL hash updates: #file=path/to/doc.md

User clicks chip in detail panel
  → Chip value inserted into search input
  → Search fires automatically
  → Results update; detail panel clears until new selection

Related docs
  → Derived client-side: filter current results for docs sharing ≥1 entity or tag with selected doc
  → Clicking a related doc loads it into detail panel without triggering a new search
```

**Existing endpoints used:**
- `GET /search?q=` — returns `{file, tldr, title, tags, entities, score}`
- `GET /facets` — sidebar facets (unchanged)

No new server endpoints needed for this feature.

---

## Error Handling

- Search fetch failure: show inline error in results column, detail panel untouched
- Empty index (no documents ingested): results column shows "Ingest some documents first" prompt
- Chip click with no results: results column shows "No results for [chip value]" — detail panel stays showing previous doc until user selects a new result

---

## Testing

- Visual smoke test: ingest ≥3 docs, search, verify three columns render
- Click a result: verify detail panel populates with correct title, tldr, chips
- Click a chip: verify search bar updates and new query fires
- Related docs: ingest 2 docs with a shared tag, select one, verify the other appears in Related
- Responsive: resize below 900px, verify detail panel hides
- Empty state: fresh index, verify "Ingest some documents first" renders

---

## Out of Scope

- Persistent selected-doc state across page reloads (URL hash is for convenience, not required)
- Fetching richer data from `/summary/{file}` on click (search payload is sufficient)
- Pagination of results or related docs
