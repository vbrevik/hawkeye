<script lang="ts">
	import type { SearchResult, Facets } from '$lib/api/types';
	import { search, fetchFacets, reindex } from '$lib/api/search';
	import { cancelJobs } from '$lib/api/ingest';
	import { startPolling, stopPolling, queueStatus } from '$lib/stores/status';
	import { startHealthPolling, stopHealthPolling, healthSummary } from '$lib/stores/health';
	import { connectEvents, disconnectEvents } from '$lib/features/events/useEvents';
	import { addToast } from '$lib/stores/toast';

	import SearchBar from '$lib/features/search/SearchBar.svelte';
	import ResultCard from '$lib/features/search/ResultCard.svelte';
	import FileBrowser from '$lib/features/browse/FileBrowser.svelte';
	import InferenceBlock from '$lib/features/status/InferenceBlock.svelte';
	import QueueStats from '$lib/features/status/QueueStats.svelte';
	import HealthPanel from '$lib/features/status/HealthPanel.svelte';
	import DetailPanel from '$lib/features/summary/DetailPanel.svelte';
	import SummaryDrawer from '$lib/features/summary/SummaryDrawer.svelte';
	import Dashboard from '$lib/features/dashboard/Dashboard.svelte';
	import Onboarding from '$lib/features/dashboard/Onboarding.svelte';
	import FacetExplorer from '$lib/features/facets/FacetExplorer.svelte';

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

	type ViewMode = 'onboarding' | 'dashboard' | 'search';
	const totalFacets = $derived(facets.tags.length + facets.topics.length + facets.entities.length);
	const viewMode: ViewMode = $derived(
		(searched || query.trim() || activeFilters.length > 0) ? 'search'
			: totalFacets > 0 ? 'dashboard'
			: 'onboarding'
	);

	let cancelConfirm = $state(false);
	let cancelTimer: ReturnType<typeof setTimeout> | null = null;

	let reindexing = $state(false);
	let reindexConfirm = $state(false);
	let reindexTimer: ReturnType<typeof setTimeout> | null = null;

	/* Sidebar collapsible state */
	let showHealth = $state(false);
	let showIngest = $state(false);
	let showQueue = $state(false);

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
		searched = true;
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

	async function handleReindex() {
		if (!reindexConfirm) {
			reindexConfirm = true;
			reindexTimer = setTimeout(() => { reindexConfirm = false; }, 3000);
			return;
		}
		if (reindexTimer) clearTimeout(reindexTimer);
		reindexConfirm = false;
		reindexing = true;
		try {
			const result = await reindex();
			addToast(`Rebuilt search index — ${result.indexed} document${result.indexed !== 1 ? 's' : ''} indexed`, 'success');
			await refreshFacets();
		} catch (e) {
			addToast(`Reindex failed: ${e instanceof Error ? e.message : 'Unknown'}`, 'error');
		} finally {
			reindexing = false;
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

	/* Auto-expand queue when jobs are active */
	$effect(() => {
		if ($queueStatus.in_progress > 0) showQueue = true;
	});

	$effect(() => {
		startPolling();
		startHealthPolling();
		connectEvents({ onComplete: refreshFacets });
		refreshFacets();
		return () => {
			stopPolling();
			stopHealthPolling();
			disconnectEvents();
			if (cancelTimer) clearTimeout(cancelTimer);
			if (reindexTimer) clearTimeout(reindexTimer);
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="layout">
	<!-- Sidebar: Slim Command Panel -->
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

		<InferenceBlock compact />

		<nav class="sidebar-nav">
			<a href="/" class="nav-tab active" aria-current="page">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
				</svg>
				Search
			</a>
			<a href="/graph" class="nav-tab">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/>
					<path d="M8.59 13.51 15.42 17.49"/><path d="M15.41 6.51 8.59 10.49"/>
				</svg>
				Graph
			</a>
		</nav>

		<div class="sidebar-divider"></div>

		<!-- Infrastructure: collapsible -->
		<button class="collapse-toggle" onclick={() => showHealth = !showHealth}>
			<span class="collapse-label">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
				</svg>
				Health
			</span>
			<span class="collapse-right">
				{#if $healthSummary.total > 0}
					<span
						class="health-badge"
						class:all-up={$healthSummary.worst === 'up'}
						class:has-degraded={$healthSummary.worst === 'degraded'}
						class:has-down={$healthSummary.worst === 'down'}
					>{$healthSummary.up}/{$healthSummary.total}</span>
				{/if}
				<span class="toggle-chevron" class:open={showHealth}>
					<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<polyline points="6 9 12 15 18 9"/>
					</svg>
				</span>
			</span>
		</button>
		{#if showHealth}
			<div class="collapsible-body">
				<HealthPanel />
			</div>
		{/if}

		<div class="sidebar-divider"></div>

		<!-- Ingest: collapsible -->
		<button class="collapse-toggle" onclick={() => showIngest = !showIngest}>
			<span class="collapse-label">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
				</svg>
				Ingest
			</span>
			<span class="toggle-chevron" class:open={showIngest}>
				<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<polyline points="6 9 12 15 18 9"/>
				</svg>
			</span>
		</button>
		{#if showIngest}
			<div class="collapsible-body">
				<FileBrowser onfacetsrefresh={refreshFacets} />
			</div>
		{/if}

		<div class="sidebar-divider"></div>

		<!-- Queue: collapsible, auto-opens when active -->
		<button class="collapse-toggle" onclick={() => showQueue = !showQueue}>
			<span class="collapse-label">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
				</svg>
				Queue
			</span>
			<span class="collapse-right">
				{#if $queueStatus.in_progress > 0}
					<span class="queue-badge">{$queueStatus.in_progress}</span>
				{/if}
				<span class="toggle-chevron" class:open={showQueue}>
					<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<polyline points="6 9 12 15 18 9"/>
					</svg>
				</span>
			</span>
		</button>
		{#if showQueue}
			<div class="collapsible-body">
				<QueueStats />
				<button
					class="btn btn--ghost"
					class:confirm={cancelConfirm}
					onclick={handleCancel}
				>
					{cancelConfirm ? 'Confirm cancel?' : 'Cancel queued jobs'}
				</button>
			</div>
		{/if}

		<div class="sidebar-divider"></div>

		<button
			class="btn--tool"
			class:confirm={reindexConfirm}
			class:running={reindexing}
			onclick={handleReindex}
			disabled={reindexing}
		>
			{#if reindexing}
				<svg class="spin" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<path d="M21 12a9 9 0 1 1-6.219-8.56"/>
				</svg>
				Rebuilding…
			{:else if reindexConfirm}
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
					<path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
					<path d="M16 16h5v5"/>
				</svg>
				Confirm rebuild?
			{:else}
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
					<path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/>
					<path d="M16 16h5v5"/>
				</svg>
				Rebuild search index
			{/if}
		</button>
	</aside>

	<!-- Results Column -->
	<main class="results-col">
		<div class="search-bar-wrap">
			<SearchBar bind:value={query} onsubmit={doSearch} bind:this={searchBar} />
		</div>

		{#if viewMode === 'search'}
			<FacetExplorer
				{facets}
				{activeFilters}
				onaddfilter={addFilter}
				onremovefilter={removeFilter}
			/>
		{/if}

		<div class="results-area">
			{#if viewMode === 'search'}
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
				{/if}
			{:else if viewMode === 'dashboard'}
				<Dashboard {facets} ontagclick={addFilter} />
			{:else}
				<Onboarding onfacetsrefresh={refreshFacets} />
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
		padding: 16px 12px; display: flex; flex-direction: column; gap: 14px;
		height: 100vh; overflow-y: auto; overflow-x: hidden;
	}
	.sidebar::-webkit-scrollbar { width: 3px; }
	.sidebar::-webkit-scrollbar-track { background: transparent; }
	.sidebar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }

	@media (max-width: 900px) { .sidebar { display: none; } }

	.logo {
		font-size: 16px; font-weight: 800; letter-spacing: -0.04em;
		color: var(--text); display: flex; align-items: center; gap: 7px;
		padding-bottom: 2px;
	}
	.logo-icon {
		width: 24px; height: 24px; border-radius: 7px;
		background: var(--gradient-accent);
		display: flex; align-items: center; justify-content: center;
		font-size: 13px; color: #fff; flex-shrink: 0;
	}
	.logo-accent {
		background: var(--gradient-accent);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent;
		background-clip: text;
	}

	/* ── Nav Tabs ── */
	.sidebar-nav {
		display: flex; gap: 4px;
	}
	.nav-tab {
		flex: 1; display: flex; align-items: center; justify-content: center; gap: 5px;
		padding: 7px 6px; border-radius: var(--r-sm);
		background: transparent; border: 1px solid var(--border-subtle);
		color: var(--text-3); font-size: 11px; font-weight: 600;
		text-decoration: none; transition: all var(--duration-fast);
		cursor: pointer;
	}
	.nav-tab:hover {
		background: var(--surface-2); color: var(--text-2);
		border-color: var(--border); text-decoration: none;
	}
	.nav-tab.active {
		background: var(--accent-dim); border-color: rgba(99, 102, 241, 0.25);
		color: var(--accent-hover);
	}

	/* ── Collapsible sections ── */
	.collapse-toggle {
		display: flex; align-items: center; justify-content: space-between;
		width: 100%; padding: 0; background: none; border: none;
		cursor: pointer; color: var(--text-3);
	}
	.collapse-toggle:hover { color: var(--text-2); }
	.collapse-label {
		font-size: 10px; font-weight: 600; text-transform: uppercase;
		letter-spacing: 0.1em; display: flex; align-items: center; gap: 6px;
	}
	.collapse-right {
		display: flex; align-items: center; gap: 6px;
	}
	.toggle-chevron {
		display: flex; align-items: center; justify-content: center;
		transition: transform var(--duration-fast);
		opacity: 0.5;
	}
	.toggle-chevron.open { transform: rotate(180deg); }
	.queue-badge {
		font-size: 10px; font-weight: 700; color: var(--accent-hover);
		background: var(--accent-dim); border-radius: 99px;
		padding: 1px 6px; font-family: var(--mono);
		animation: pulse 2.5s ease-in-out infinite;
	}
	.health-badge {
		font-size: 10px; font-weight: 700; border-radius: 99px;
		padding: 1px 6px; font-family: var(--mono);
		transition: background 0.3s, color 0.3s;
	}
	.health-badge.all-up {
		color: var(--green); background: rgba(52, 211, 153, 0.1);
	}
	.health-badge.has-degraded {
		color: var(--yellow); background: rgba(251, 191, 36, 0.1);
		animation: pulse 2.5s ease-in-out infinite;
	}
	.health-badge.has-down {
		color: var(--red); background: rgba(248, 113, 113, 0.1);
		animation: pulse 2.5s ease-in-out infinite;
	}

	.collapsible-body {
		animation: fadeInUp 0.2s ease both;
	}

	.sidebar-divider { height: 1px; background: var(--border-subtle); margin: 0; }

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

	/* ── Tool button ── */
	.btn--tool {
		width: 100%; display: flex; align-items: center; gap: 6px;
		background: transparent; border: 1px solid var(--border-subtle);
		border-radius: var(--r-sm); color: var(--text-3); font-size: 11px;
		padding: 7px 10px; cursor: pointer; font-weight: 500;
		transition: all var(--duration-fast); font-family: var(--font);
	}
	.btn--tool:hover {
		border-color: rgba(99, 102, 241, 0.3); color: var(--accent-hover);
		background: var(--accent-dim);
	}
	.btn--tool.confirm {
		border-color: rgba(99, 102, 241, 0.4); color: var(--accent-hover);
		background: var(--accent-dim);
	}
	.btn--tool.running {
		border-color: rgba(99, 102, 241, 0.25); color: var(--accent);
		background: var(--accent-dim); cursor: default;
	}
	.btn--tool:disabled { opacity: 0.7; }
	.btn--tool .spin {
		animation: spin 1s linear infinite;
	}
	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

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
	.placeholder-title {
		font-size: 15px; font-weight: 600; color: var(--text-2); letter-spacing: -0.01em;
	}
</style>
