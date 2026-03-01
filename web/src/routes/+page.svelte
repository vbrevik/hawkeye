<script lang="ts">
	import type { SearchResult, Facets, GraphNode, GraphResponse } from '$lib/api/types';
	import { search, fetchFacets } from '$lib/api/search';
	import { fetchEntityGraph, fetchDocumentGraph } from '$lib/api/graph';
	import { connectEvents, disconnectEvents } from '$lib/features/events/useEvents';
	import { addToast } from '$lib/stores/toast';

	import SearchBar from '$lib/features/search/SearchBar.svelte';
	import ResultCard from '$lib/features/search/ResultCard.svelte';
	import DetailPanel from '$lib/features/summary/DetailPanel.svelte';
	import SummaryDrawer from '$lib/features/summary/SummaryDrawer.svelte';
	import Dashboard from '$lib/features/dashboard/Dashboard.svelte';
	import Onboarding from '$lib/features/dashboard/Onboarding.svelte';
	import FacetExplorer from '$lib/features/facets/FacetExplorer.svelte';
	import GraphCanvas from '$lib/features/graph/GraphCanvas.svelte';
	import Sidebar from '$lib/features/sidebar/Sidebar.svelte';

	/* ── Page mode: search vs graph ── */
	type PageMode = 'search' | 'graph';
	let pageMode = $state<PageMode>('search');

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

	/* ── Graph state ── */
	const GRAPH_NODE_COLORS: Record<string, string> = {
		entity: '#6366f1',
		document: '#34d399',
		tag: '#fbbf24',
		topic: '#f87171',
	};

	let graphQuery = $state('');
	let graphData = $state<GraphResponse>({ nodes: [], edges: [] });
	let graphLoading = $state(false);
	let graphSearched = $state(false);
	let graphHistory = $state<string[]>([]);
	let graphSelectedNode = $state<GraphNode | null>(null);

	async function searchEntity() {
		const q = graphQuery.trim();
		if (!q) return;
		graphLoading = true;
		graphSearched = true;
		graphSelectedNode = null;
		try {
			graphData = await fetchEntityGraph(q);
			if (graphData.nodes.length === 0) {
				addToast(`No graph data found for "${q}"`, 'info');
			}
			if (!graphHistory.includes(q)) graphHistory = [...graphHistory.slice(-9), q];
		} catch (e) {
			addToast(`Graph query failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
			graphData = { nodes: [], edges: [] };
		} finally {
			graphLoading = false;
		}
	}

	async function handleGraphNodeClick(node: GraphNode) {
		graphSelectedNode = node;
		if (node.type === 'entity') {
			graphQuery = node.label;
			await searchEntity();
		} else if (node.type === 'document' && node.id.startsWith('document:')) {
			const docId = node.id.replace('document:', '');
			graphLoading = true;
			try {
				graphData = await fetchDocumentGraph(docId);
				graphQuery = node.label;
				if (!graphHistory.includes(node.label)) graphHistory = [...graphHistory.slice(-9), node.label];
			} catch (e) {
				addToast(`Failed to load document graph: ${e instanceof Error ? e.message : 'Unknown'}`, 'error');
			} finally {
				graphLoading = false;
			}
		} else if (node.type === 'tag' || node.type === 'topic') {
			graphQuery = node.label;
			await searchEntity();
		}
	}

	function graphSearchInDocuments(label: string) {
		pageMode = 'search';
		query = label;
		searched = true;
		doSearch();
	}

	function viewInGraph(entityQuery: string) {
		pageMode = 'graph';
		graphQuery = entityQuery;
		searchEntity();
	}

	async function loadFromGraphHistory(item: string) {
		graphQuery = item;
		await searchEntity();
	}

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

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			if (pageMode === 'graph') pageMode = 'search';
			searchBar?.focus();
		}
		if (e.key === 'Escape') {
			if (pageMode === 'graph' && graphSelectedNode) {
				graphSelectedNode = null;
			} else if (drawerOpen) {
				closeDrawer();
			} else if (selected && window.innerWidth >= 900) {
				selected = null;
			}
		}
		if (pageMode === 'graph' && e.key === 'Enter') {
			const active = document.activeElement;
			if (active && active.classList.contains('graph-input')) {
				searchEntity();
			}
		}
	}

	$effect(() => {
		connectEvents({ onComplete: refreshFacets });
		refreshFacets();
		return () => {
			disconnectEvents();
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="layout">
	<Sidebar bind:pageMode onrefreshfacets={refreshFacets} />

	{#if pageMode === 'search'}
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
			ongraphclick={viewInGraph}
		/>
	{:else}
		<!-- Graph View -->
		<div class="graph-area">
			<header class="graph-header">
				<div class="graph-title-row">
					<h2 class="graph-title">Knowledge Graph</h2>
					{#if graphData.nodes.length > 0}
						<span class="graph-stats">{graphData.nodes.length} nodes / {graphData.edges.length} edges</span>
					{/if}
				</div>
				<div class="graph-search-row">
					<input
						type="text"
						bind:value={graphQuery}
						placeholder="Search entity, tag, or topic…"
						aria-label="Search knowledge graph"
						class="graph-input"
					/>
					<button onclick={searchEntity} disabled={graphLoading || !graphQuery.trim()} class="graph-btn">
						{graphLoading ? 'Loading…' : 'Explore'}
					</button>
				</div>
				{#if graphHistory.length > 0}
					<div class="graph-history">
						{#each graphHistory as item}
							<button class="history-chip" onclick={() => loadFromGraphHistory(item)}>{item}</button>
						{/each}
					</div>
				{/if}
			</header>

			<div class="graph-content">
				<div class="graph-body">
					{#if graphData.nodes.length > 0}
						<div class="canvas-wrap">
							<GraphCanvas nodes={graphData.nodes} edges={graphData.edges} onNodeClick={handleGraphNodeClick} />
						</div>
					{:else if graphSearched && !graphLoading}
						<div class="graph-empty">
							<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor"
								stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.3">
								<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/>
								<circle cx="18" cy="19" r="3"/><path d="M8.59 13.51 15.42 17.49"/>
								<path d="M15.41 6.51 8.59 10.49"/>
							</svg>
							<p>No graph data found. Try searching for an entity that appears in your ingested documents.</p>
						</div>
					{:else if !graphSearched}
						<div class="graph-empty">
							<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor"
								stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.3">
								<circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
							</svg>
							<p>Enter an entity name to explore its knowledge graph connections.</p>
							<p class="graph-empty-hint">Entities, tags, and topics are extracted during document ingestion.</p>
						</div>
					{/if}
				</div>

				{#if graphSelectedNode}
					<aside class="graph-detail">
						<div class="node-detail-header">
							<span class="node-type-badge" style:background={GRAPH_NODE_COLORS[graphSelectedNode.type] ?? '#6366f1'}>
								{graphSelectedNode.type}
							</span>
							<button class="node-close" onclick={() => graphSelectedNode = null} aria-label="Close node detail">
								<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
									stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
									<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
								</svg>
							</button>
						</div>
						<div class="node-detail-title">{graphSelectedNode.label}</div>
						{#if graphSelectedNode.source_path}
							<div class="node-detail-path">{graphSelectedNode.source_path}</div>
						{/if}
						<div class="node-actions">
							{#if graphSelectedNode.type === 'entity'}
								<button class="action-btn" onclick={() => { graphQuery = graphSelectedNode!.label; searchEntity(); }}>
									Explore connections
								</button>
							{/if}
							<button class="action-btn" onclick={() => graphSearchInDocuments(graphSelectedNode!.label)}>
								Search in documents
							</button>
						</div>
					</aside>
				{/if}
			</div>

			<div class="graph-footer">
				<div class="legend">
					<span class="legend-item"><span class="legend-dot" style="background: #6366f1;"></span> Entity</span>
					<span class="legend-item"><span class="legend-dot" style="background: #34d399;"></span> Document</span>
					<span class="legend-item"><span class="legend-dot" style="background: #fbbf24;"></span> Tag</span>
					<span class="legend-item"><span class="legend-dot" style="background: #f87171;"></span> Topic</span>
				</div>
				<span class="legend-hint">Click nodes to explore · Scroll to zoom · Drag to pan</span>
			</div>
		</div>
	{/if}
</div>

<!-- Summary Drawer (mobile, search mode only) -->
{#if pageMode === 'search'}
	<SummaryDrawer
		result={drawerResult}
		open={drawerOpen}
		onclose={closeDrawer}
		ontagclick={(tag) => { addFilter(tag); closeDrawer(); }}
	/>
{/if}

<style>
	.layout { display: flex; height: 100vh; overflow: hidden; }

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

	/* ── Graph View ── */
	.graph-area {
		flex: 1; display: flex; flex-direction: column; overflow: hidden;
		min-width: 0;
	}

	.graph-header {
		padding: 16px 24px 12px;
		border-bottom: 1px solid var(--border-subtle);
		background: var(--bg);
		flex-shrink: 0;
	}
	.graph-title-row {
		display: flex; align-items: baseline; gap: 12px; margin-bottom: 10px;
	}
	.graph-title {
		font-size: 16px; font-weight: 700;
		background: var(--gradient-accent);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent;
		background-clip: text;
	}
	.graph-stats {
		font-size: 11px; color: var(--text-3); font-family: var(--mono);
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); padding: 2px 8px;
	}

	.graph-search-row { display: flex; gap: 8px; }
	.graph-input {
		flex: 1; padding: 9px 14px; background: var(--surface);
		border: 1px solid var(--border); border-radius: var(--r-sm);
		color: var(--text); font-size: 14px; outline: none;
		transition: border-color 0.15s; font-family: var(--font);
	}
	.graph-input:focus { border-color: var(--accent); }
	.graph-input::placeholder { color: var(--text-3); }
	.graph-btn {
		padding: 9px 18px; background: var(--gradient-accent);
		color: #fff; border: none; border-radius: var(--r-sm);
		font-weight: 600; font-size: 13px; cursor: pointer;
		transition: opacity 0.15s; font-family: var(--font);
	}
	.graph-btn:hover { opacity: 0.9; }
	.graph-btn:disabled { opacity: 0.5; cursor: default; }

	.graph-history { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }
	.history-chip {
		padding: 3px 10px; background: var(--accent-dim);
		border: 1px solid var(--border); border-radius: 99px;
		color: var(--text-2); font-size: 11px; cursor: pointer;
		transition: all 0.15s; font-family: var(--font);
	}
	.history-chip:hover {
		background: var(--accent-glow); color: var(--accent-hover);
		border-color: var(--accent);
	}

	.graph-content { flex: 1; display: flex; overflow: hidden; }
	.graph-body { flex: 1; position: relative; overflow: hidden; }
	.canvas-wrap { width: 100%; height: 100%; }

	.graph-empty {
		display: flex; flex-direction: column; align-items: center;
		justify-content: center; height: 100%; color: var(--text-3);
		text-align: center; gap: 12px;
	}
	.graph-empty p { font-size: 14px; max-width: 400px; }
	.graph-empty-hint { font-size: 12px; opacity: 0.7; }

	.graph-detail {
		width: 260px; flex-shrink: 0;
		background: var(--surface); border-left: 1px solid var(--border);
		padding: 18px; overflow-y: auto;
		animation: fadeInUp 0.2s ease both;
		display: flex; flex-direction: column; gap: 14px;
	}
	.node-detail-header {
		display: flex; align-items: center; justify-content: space-between;
	}
	.node-type-badge {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; padding: 3px 10px; border-radius: 4px; color: #fff;
	}
	.node-close {
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); width: 26px; height: 26px;
		display: flex; align-items: center; justify-content: center;
		cursor: pointer; color: var(--text-3);
		transition: background var(--duration-fast), color var(--duration-fast);
	}
	.node-close:hover { background: var(--border); color: var(--text); }
	.node-detail-title {
		font-size: 18px; font-weight: 700; color: var(--text); line-height: 1.3;
	}
	.node-detail-path {
		font-size: 11px; font-family: var(--mono); color: var(--text-3); word-break: break-all;
	}
	.node-actions { display: flex; flex-direction: column; gap: 6px; margin-top: 4px; }
	.action-btn {
		display: block; width: 100%; padding: 8px 12px;
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); color: var(--text-2);
		font-size: 12px; font-weight: 600; text-align: center;
		cursor: pointer; text-decoration: none; font-family: var(--font);
		transition: all var(--duration-fast);
	}
	.action-btn:hover {
		background: var(--accent-dim); border-color: rgba(99, 102, 241, 0.3);
		color: var(--accent-hover);
	}

	.graph-footer {
		display: flex; align-items: center; gap: 16px;
		padding: 10px 24px; border-top: 1px solid var(--border-subtle);
		background: var(--surface); flex-shrink: 0;
	}
	.legend { display: flex; align-items: center; gap: 14px; font-size: 12px; color: var(--text-2); }
	.legend-item { display: flex; align-items: center; gap: 5px; }
	.legend-dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }
	.legend-hint {
		margin-left: auto; font-size: 11px; color: var(--text-3); font-family: var(--mono);
	}
</style>
