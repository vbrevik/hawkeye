# Design: Rename to Hawkeye + Filesystem Folder Picker

**Date:** 2026-02-27
**Status:** Approved

## Goals

1. Rename the project from `eagle3` to `hawkeye` throughout the codebase.
2. Replace the manual path text input in the sidebar with an inline filesystem browser (linear drill-down style) so users can navigate and select a folder to ingest without typing paths.

## Rename: eagle3 → hawkeye

### Scope

| File | Change |
|------|--------|
| `Cargo.toml` | Package `name` and `[[bin]]` name → `hawkeye` |
| `src/api/ui.rs` | Page `<title>`, header branding strings |
| `scripts/start_mlx.sh` | Comment header references |
| `README.md` | All occurrences of "eagle3" |

Module/directory structure is unchanged. The binary will be `target/.../hawkeye`.

## Folder Picker Design

### Approach

**Linear drill-down** — replaces the sidebar ingest section (text input + button) with a compact inline filesystem browser. One directory is "current"; clicking a subdirectory navigates into it. An "Ingest here" button triggers ingestion of the current directory.

### API: `GET /browse`

New endpoint `src/api/browse.rs`, registered as `GET /browse` in `main.rs`.

**Query param:** `path` (optional, defaults to `$HOME`)

**Response:**
```json
{
  "path": "/Users/vidar/notes",
  "parent": "/Users/vidar",
  "entries": ["Documents", "Projects", "notes"]
}
```

- `entries`: directory names only (no files), sorted alphabetically
- `parent`: absolute path of parent directory, or `null` at filesystem root

### UI: Sidebar Ingest Section Replacement

The current text input + "Start Ingest" button is replaced with:

1. **Breadcrumb** — displays last 2–3 path segments, truncated with `…` prefix if path is deep; clicking a segment navigates there
2. **Scrollable directory list** — max ~180px tall, overflow-y scroll; each row has a folder icon and directory name; clicking navigates into it
3. **"↑ Parent" row** — pinned at top of list; hidden when at filesystem root
4. **"Ingest here" button** — calls `POST /ingest` with the current browsed path

**States:**
- Loading spinner while fetching `/browse`
- "No subdirectories" empty state when a directory has no children
- Error message if `/browse` returns an error

**Initialization:** fetches `/browse` (defaulting to `$HOME`) on page load.
**No persistence** — starts at `$HOME` on each page reload.

### Data Flow

```
user clicks dir → fetch /browse?path=<new_path> → update current path + entries list
user clicks "Ingest here" → POST /ingest { path: currentPath } → show result notice
user clicks "↑ Parent" → fetch /browse?path=<parent> → update view
```

## Out of Scope

- Multi-folder selection
- File-level selection (ingest always operates on a directory)
- Persisting last browsed path across reloads
- Hidden directory toggle
