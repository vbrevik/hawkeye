<script lang="ts">
	import type {
		DisplayResult,
		SearchMode,
		Facets,
	} from "$lib/api/types";
	import {
		search,
		hybridSearch,
		semanticSearch,
		normalizeKeyword,
		normalizeHybrid,
		normalizeSemantic,
		fetchFacets,
	} from "$lib/api/search";
	import {
		connectEvents,
		disconnectEvents,
	} from "$lib/features/events/useEvents";
	import { addToast } from "$lib/stores/toast";

	import SearchBar from "$lib/features/search/SearchBar.svelte";
	import SearchModeSwitch from "$lib/features/search/SearchModeSwitch.svelte";
	import ResultCard from "$lib/features/search/ResultCard.svelte";
	import DetailPanel from "$lib/features/summary/DetailPanel.svelte";
	import SummaryDrawer from "$lib/features/summary/SummaryDrawer.svelte";
	import Dashboard from "$lib/features/dashboard/Dashboard.svelte";
	import Onboarding from "$lib/features/dashboard/Onboarding.svelte";
	import FacetExplorer from "$lib/features/facets/FacetExplorer.svelte";
	import GraphView from "$lib/features/graph/GraphView.svelte";
	import Sidebar from "$lib/features/sidebar/Sidebar.svelte";

	/* ── Page mode: search vs graph ── */
	type PageMode = "search" | "graph";
	let pageMode = $state<PageMode>("search");

	let searchBar = $state<SearchBar>();
	let graphView = $state<GraphView>();
	let query = $state("");
	let searchMode = $state<SearchMode>("hybrid");
	let results = $state<DisplayResult[]>([]);
	let selected = $state<DisplayResult | null>(null);
	let searching = $state(false);
	let searched = $state(false);

	let activeFilters = $state<string[]>([]);
	let facets = $state<Facets>({ document_count: 0, tags: [], topics: [], entities: [] });

	let drawerOpen = $state(false);
	let drawerResult = $state<DisplayResult | null>(null);

	type ViewMode = "onboarding" | "dashboard" | "search";
	const totalFacets = $derived(
		facets.tags.length + facets.topics.length + facets.entities.length,
	);
	const viewMode: ViewMode = $derived(
		searched || query.trim() || activeFilters.length > 0
			? "search"
			: totalFacets > 0
				? "dashboard"
				: "onboarding",
	);

	function handleModeChange(mode: SearchMode) {
		searchMode = mode;
		if (searched) doSearch();
	}

	async function doSearch() {
		const rawQuery = query.trim();
		const filterQ = activeFilters.join(" ");
		// Semantic search uses embeddings — appending filter keywords degrades quality
		const q = searchMode === "semantic"
			? rawQuery
			: [rawQuery, filterQ].filter(Boolean).join(" ");

		if (!q) {
			results = [];
			selected = null;
			searched = false;
			return;
		}

		searching = true;
		searched = true;
		try {
			switch (searchMode) {
				case "hybrid": {
					const hits = await hybridSearch(q, 25);
					results = normalizeHybrid(hits);
					break;
				}
				case "keyword": {
					const hits = await search(q, 25);
					results = normalizeKeyword(hits);
					break;
				}
				case "semantic": {
					const hits = await semanticSearch(q, 25);
					results = normalizeSemantic(hits);
					break;
				}
			}
			if (results.length > 0 && window.innerWidth >= 900) {
				selected = results[0];
			}
		} catch (e) {
			results = [];
			addToast(
				`Search failed: ${e instanceof Error ? e.message : "Unknown error"}`,
				"error",
			);
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

	function selectResult(r: DisplayResult) {
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
		} catch {
			/* ignore */
		}
	}

	function viewInGraph(entityQuery: string) {
		pageMode = "graph";
	}

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === "k") {
			e.preventDefault();
			if (pageMode === "graph") pageMode = "search";
			searchBar?.focus();
		}

		if (pageMode === "graph") {
			graphView?.handleKeydown(e);
			return;
		}

		if (e.key === "Escape") {
			if (drawerOpen) {
				closeDrawer();
			} else if (selected && window.innerWidth >= 900) {
				selected = null;
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
	<Sidebar
		{pageMode}
		onnavsearch={() => {
			pageMode = "search";
		}}
		onnavgraph={() => {
			pageMode = "graph";
		}}
		onrefreshfacets={refreshFacets}
	/>

	{#if pageMode === "search"}
		<!-- Results Column -->
		<main class="results-col">
			<div class="search-bar-wrap">
				<SearchBar
					bind:value={query}
					onsubmit={doSearch}
					bind:this={searchBar}
				/>
				<div class="mode-switch-row">
					<SearchModeSwitch mode={searchMode} onchange={handleModeChange} />
				</div>
			</div>

			{#if viewMode === "search"}
				<FacetExplorer
					{facets}
					{activeFilters}
					{searchMode}
					onaddfilter={addFilter}
					onremovefilter={removeFilter}
				/>
			{/if}

			<div class="results-area">
				{#if viewMode === "search"}
					{#if searching}
						<div class="results-list">
							{#each Array(5) as _, i}
								<div
									class="skeleton sk-card"
									style:animation-delay="{i * 80}ms"
								></div>
							{/each}
						</div>
					{:else if results.length > 0}
						<div class="results-count">
							<span>{results.length} result{results.length !== 1 ? "s" : ""}</span>
							<span class="results-mode" data-mode={searchMode}>{searchMode}</span>
						</div>
						<div class="results-list">
							{#each results as result, i (result.file)}
								<ResultCard
									{result}
									active={selected?.file === result.file}
									featured={i === 0}
									index={i}
									{searchMode}
									onclick={() => selectResult(result)}
									ontagclick={addFilter}
								/>
							{/each}
						</div>
					{:else if searched}
						<div class="placeholder">
							<div class="placeholder-title">
								No results for "{query}"
							</div>
						</div>
					{/if}
				{:else if viewMode === "dashboard"}
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
		<GraphView
			bind:this={graphView}
			{facets}
			onsearchindocuments={(label) => {
				pageMode = "search";
				query = label;
				searched = true;
				doSearch();
			}}
		/>
	{/if}
</div>

<!-- Summary Drawer (mobile, search mode only) -->
{#if pageMode === "search"}
	<SummaryDrawer
		result={drawerResult}
		open={drawerOpen}
		onclose={closeDrawer}
		ontagclick={(tag) => {
			addFilter(tag);
			closeDrawer();
		}}
	/>
{/if}

<style>
	.layout {
		display: flex;
		height: 100vh;
		overflow: hidden;
		position: relative;
	}

	/* ── Results Column ── */
	.results-col {
		flex: 1;
		min-width: 280px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.search-bar-wrap {
		padding: 24px 32px 12px;
		position: sticky;
		top: 0;
		background: linear-gradient(180deg, var(--bg) 85%, transparent 100%);
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}
	.mode-switch-row {
		max-width: 380px;
	}

	.results-area {
		padding: 4px 32px 32px;
		flex: 1;
		overflow-y: auto;
	}
	.results-area::-webkit-scrollbar {
		width: 4px;
	}
	.results-area::-webkit-scrollbar-track {
		background: transparent;
	}
	.results-area::-webkit-scrollbar-thumb {
		background: var(--border);
		border-radius: 4px;
	}

	.results-count {
		font-size: 12px;
		color: var(--text-3);
		margin-bottom: 16px;
		font-weight: 500;
		display: flex;
		align-items: center;
		gap: 8px;
	}
	.results-mode {
		font-size: 9px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		padding: 2px 7px;
		border-radius: var(--r-sm);
		border: 1px solid;
	}
	.results-mode[data-mode='hybrid'] {
		color: var(--mode-hybrid); background: var(--mode-hybrid-bg); border-color: var(--mode-hybrid-border);
	}
	.results-mode[data-mode='keyword'] {
		color: var(--mode-keyword); background: var(--mode-keyword-bg); border-color: var(--mode-keyword-border);
	}
	.results-mode[data-mode='semantic'] {
		color: var(--mode-semantic); background: var(--mode-semantic-bg); border-color: var(--mode-semantic-border);
	}
	.results-list {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	/* ── Skeleton ── */
	.skeleton {
		border-radius: var(--r);
		background: linear-gradient(
			90deg,
			var(--surface) 25%,
			var(--surface-2) 50%,
			var(--surface) 75%
		);
		background-size: 200% 100%;
		animation: skeleton-sweep 1.6s ease infinite;
	}
	.sk-card {
		height: 92px;
		margin-bottom: 6px;
	}

	/* ── Placeholder ── */
	.placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 100px 0;
		gap: 16px;
		color: var(--text-3);
	}
	.placeholder-title {
		font-size: 15px;
		font-weight: 600;
		color: var(--text-2);
		letter-spacing: -0.01em;
	}
</style>
