<script lang="ts">
	import type { SearchResult, Facets } from '$lib/api/types';
	import { search, fetchFacets } from '$lib/api/search';
	import { cancelJobs } from '$lib/api/ingest';
	import { startPolling, stopPolling } from '$lib/stores/status';
	import { connectEvents, disconnectEvents } from '$lib/features/events/useEvents';
	import { addToast } from '$lib/stores/toast';

	import SearchBar from '$lib/features/search/SearchBar.svelte';
	import ResultCard from '$lib/features/search/ResultCard.svelte';
	import FileBrowser from '$lib/features/browse/FileBrowser.svelte';
	import FacetCloud from '$lib/features/facets/FacetCloud.svelte';
	import InferenceBlock from '$lib/features/status/InferenceBlock.svelte';
	import QueueStats from '$lib/features/status/QueueStats.svelte';
	import DetailPanel from '$lib/features/summary/DetailPanel.svelte';
	import SummaryDrawer from '$lib/features/summary/SummaryDrawer.svelte';

	let searchBar: SearchBar;
	let query = $state('');
	let results = $state<SearchResult[]>([]);
	let selected = $state<SearchResult | null>(null);
	let searching = $state(false);
	let searched = $state(false);

	let activeFilters = $state<string[]>([]);
	let facets = $state<Facets>({ tags: [], topics: [], entities: [] });

	let drawerOpen = $state(false);
	let drawerResult = $state<SearchResult | null>(null);

	let cancelConfirm = $state(false);
	let cancelTimer: ReturnType<typeof setTimeout> | null = null;

	async function doSearch() {
		const raw = query.trim();
		const filterQ = activeFilters.join(' ');
		const q = [raw, filterQ].filter(Boolean).join(' ');

		if (!q) {
			results = [];
			selected = null;
			searched = false;
			return;
		}

		searching = true;
		searched = true;
		try {
			results = await search(q, 25);
			if (results.length > 0 && window.innerWidth >= 900) {
				selected = results[0];
			}
		} catch (e) {
			results = [];
			addToast(`Search failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
		} finally {
			searching = false;
		}
	}

	function addFilter(tag: string) {
		if (activeFilters.includes(tag)) return;
		activeFilters = [...activeFilters, tag];
		doSearch();
	}

	function removeFilter(tag: string) {
		activeFilters = activeFilters.filter((f) => f !== tag);
		doSearch();
	}

	function selectResult(r: SearchResult) {
		if (window.innerWidth >= 900) {
			selected = r;
		} else {
			drawerResult = r;
			drawerOpen = true;
		}
	}

	function closeDrawer() {
		drawerOpen = false;
		drawerResult = null;
	}

	async function refreshFacets() {
		try {
			facets = await fetchFacets();
		} catch { /* ignore */ }
	}

	async function handleCancel() {
		if (!cancelConfirm) {
			cancelConfirm = true;
			cancelTimer = setTimeout(() => { cancelConfirm = false; }, 3000);
			return;
		}
		if (cancelTimer) clearTimeout(cancelTimer);
		cancelConfirm = false;
		try {
			const result = await cancelJobs();
			addToast(`Cancelled ${result.cancelled} job${result.cancelled !== 1 ? 's' : ''}`, 'success');
		} catch (e) {
			addToast(`Cancel failed: ${e instanceof Error ? e.message : 'Unknown'}`, 'error');
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			searchBar?.focus();
		}
		if (e.key === 'Escape') {
			if (drawerOpen) {
				closeDrawer();
			} else if (selected && window.innerWidth >= 900) {
				selected = null;
			}
		}
	}

	$effect(() => {
		startPolling();
		connectEvents({ onComplete: refreshFacets });
		refreshFacets();
		return () => {
			stopPolling();
			disconnectEvents();
			if (cancelTimer) clearTimeout(cancelTimer);
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="layout">
	<!-- Sidebar -->
	<aside class="sidebar">
		<div class="logo">
			<div class="logo-icon">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="12" cy="12" r="3"/>
					<path d="M2 12s4-8 10-8 10 8 10 8-4 8-10 8-10-8-10-8z"/>
				</svg>
			</div>
			hawk<span class="logo-accent">eye</span>
		</div>

		<div>
			<div class="section-label"><span class="section-label-icon">⚡</span> Inference</div>
			<InferenceBlock />
		</div>

		<div class="sidebar-divider"></div>

		<a href="/graph" class="nav-link">
			<span class="nav-link-icon">🕸️</span> Knowledge Graph
			<span class="nav-link-arrow">→</span>
		</a>

		<div class="sidebar-divider"></div>

		<div>
			<div class="section-label"><span class="section-label-icon">📂</span> Ingest</div>
			<FileBrowser onfacetsrefresh={refreshFacets} />
		</div>

		<div class="sidebar-divider"></div>

		<div>
			<div class="section-label"><span class="section-label-icon">📊</span> Queue</div>
			<QueueStats />
			<button
				class="btn btn--ghost"
				class:confirm={cancelConfirm}
				onclick={handleCancel}
			>
				{cancelConfirm ? 'Confirm cancel?' : 'Cancel queued jobs'}
			</button>
		</div>

		<div class="sidebar-divider"></div>

		<div>
			<div class="section-label"><span class="section-label-icon">🏷</span> Tags</div>
			<FacetCloud
				entries={facets.tags}
				prefix="#"
				emptyText="Ingest documents to see tags"
				variant="tag"
				onclick={addFilter}
			/>
		</div>

		<div>
			<div class="section-label"><span class="section-label-icon">💡</span> Topics</div>
			<FacetCloud
				entries={facets.topics}
				emptyText="—"
				variant="topic"
				onclick={addFilter}
			/>
		</div>

		<div>
			<div class="section-label"><span class="section-label-icon">👤</span> Entities</div>
			<FacetCloud
				entries={facets.entities}
				emptyText="—"
				variant="entity"
				onclick={addFilter}
			/>
		</div>

		{#if activeFilters.length > 0}
			<div>
				<div class="section-label">Active Filters</div>
				<div class="filter-chips">
					{#each activeFilters as tag}
						<button class="chip chip--active" onclick={() => removeFilter(tag)}>
							#{tag} <span class="x">✕</span>
						</button>
					{/each}
				</div>
			</div>
		{/if}
	</aside>

	<!-- Results Column -->
	<main class="results-col">
		<div class="search-bar-wrap">
			<SearchBar bind:value={query} onsubmit={doSearch} bind:this={searchBar} />
		</div>

		<div class="results-area">
			{#if searching}
				<div class="results-list">
					{#each Array(5) as _, i}
						<div class="skeleton sk-card" style:animation-delay="{i * 80}ms"></div>
					{/each}
				</div>
			{:else if results.length > 0}
				<div class="results-count">{results.length} result{results.length !== 1 ? 's' : ''}</div>
				<div class="results-list">
					{#each results as result, i (result.file)}
						<ResultCard
							{result}
							active={selected?.file === result.file}
							featured={i === 0}
							index={i}
							onclick={() => selectResult(result)}
							ontagclick={addFilter}
						/>
					{/each}
				</div>
			{:else if searched}
				<div class="placeholder">
					<div class="placeholder-title">No results for "{query}"</div>
				</div>
			{:else}
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
			{/if}
		</div>
	</main>

	<!-- Detail Panel (desktop) -->
	<DetailPanel
		{selected}
		{results}
		ontagclick={addFilter}
		onselectresult={selectResult}
	/>
</div>

<!-- Summary Drawer (mobile) -->
<SummaryDrawer
	result={drawerResult}
	open={drawerOpen}
	onclose={closeDrawer}
	ontagclick={(tag) => { addFilter(tag); closeDrawer(); }}
/>

<style>
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

	@media (max-width: 900px) { .sidebar { display: none; } }

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
	.logo-accent {
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

	.sidebar-divider { height: 1px; background: var(--border-subtle); margin: 2px 0; }

	.nav-link {
		display: flex; align-items: center; gap: 8px;
		padding: 8px 10px; border-radius: var(--r-sm);
		background: var(--surface-2); border: 1px solid var(--border);
		color: var(--text-2); font-size: 12px; font-weight: 600;
		text-decoration: none; transition: all 0.15s;
		cursor: pointer;
	}
	.nav-link:hover {
		background: var(--accent-dim); border-color: rgba(99, 102, 241, 0.3);
		color: var(--accent-hover); text-decoration: none;
	}
	.nav-link-icon { font-size: 13px; }
	.nav-link-arrow { margin-left: auto; font-size: 11px; opacity: 0.4; transition: opacity 0.15s; }
	.nav-link:hover .nav-link-arrow { opacity: 0.8; }

	/* ── Buttons ── */
	.btn--ghost {
		width: 100%; background: transparent; border: 1px solid var(--border);
		border-radius: var(--r-sm); color: var(--text-2); font-size: 11px;
		padding: 5px 10px; margin-top: 6px; cursor: pointer;
		transition: border-color 0.15s, color 0.15s, background 0.15s;
		font-family: var(--font);
	}
	.btn--ghost:hover {
		border-color: rgba(248, 113, 113, 0.4); color: var(--red);
		background: rgba(248, 113, 113, 0.08);
	}
	.btn--ghost.confirm {
		border-color: rgba(248, 113, 113, 0.5); color: var(--red);
		background: rgba(248, 113, 113, 0.08);
	}

	/* ── Filter Chips ── */
	.filter-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.chip {
		display: inline-flex; align-items: center; gap: 3px;
		border-radius: 4px; font-size: 11px; padding: 2px 8px;
		cursor: pointer; transition: transform 0.1s, background 0.15s;
		font-weight: 500; border: 1px solid; font-family: var(--font);
	}
	.chip:hover { transform: scale(1.03); }
	.chip--active {
		background: var(--accent-dim); border-color: rgba(99, 102, 241, 0.3);
		color: var(--accent-hover);
	}
	.chip--active:hover { background: rgba(99, 102, 241, 0.2); }
	.x { font-size: 9px; margin-left: 2px; opacity: 0.5; }

	/* ── Results Column ── */
	.results-col {
		flex: 1; min-width: 280px; display: flex; flex-direction: column; overflow: hidden;
	}

	.search-bar-wrap {
		padding: 24px 32px 20px; position: sticky; top: 0;
		background: linear-gradient(180deg, var(--bg) 80%, transparent 100%);
		z-index: 10;
	}

	.results-area {
		padding: 4px 32px 32px; flex: 1; overflow-y: auto;
	}
	.results-area::-webkit-scrollbar { width: 4px; }
	.results-area::-webkit-scrollbar-track { background: transparent; }
	.results-area::-webkit-scrollbar-thumb { background: var(--border); border-radius: 4px; }

	.results-count { font-size: 12px; color: var(--text-3); margin-bottom: 16px; font-weight: 500; }
	.results-list { display: flex; flex-direction: column; gap: 6px; }

	/* ── Skeleton ── */
	.skeleton {
		border-radius: var(--r);
		background: linear-gradient(90deg, var(--surface) 25%, var(--surface-2) 50%, var(--surface) 75%);
		background-size: 200% 100%;
		animation: skeleton-sweep 1.6s ease infinite;
	}
	.sk-card { height: 92px; margin-bottom: 6px; }

	/* ── Placeholder ── */
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
	.placeholder-title {
		font-size: 15px; font-weight: 600; color: var(--text-2); letter-spacing: -0.01em;
	}
	.placeholder-hint {
		font-size: 11px; color: var(--text-3); font-family: var(--mono);
		display: flex; gap: 12px; margin-top: 4px;
	}
	.placeholder-hint span { opacity: 0.6; }
</style>
