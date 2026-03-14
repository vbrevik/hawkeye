<script lang="ts">
	import type { Facets, GraphNode, GraphResponse } from "$lib/api/types";
	import { fetchEntityGraph, fetchDocumentGraph } from "$lib/api/graph";
	import { addToast } from "$lib/stores/toast";
	import GraphCanvas from "./GraphCanvas.svelte";

	let {
		facets,
		onsearchindocuments,
	}: {
		facets: Facets;
		onsearchindocuments: (label: string) => void;
	} = $props();

	const GRAPH_NODE_COLORS: Record<string, string> = {
		entity: "#94a3b8",
		document: "#34d399",
		tag: "#a5b4fc",
		topic: "#f87171",
	};

	let graphQuery = $state("");
	let graphData = $state<GraphResponse>({ nodes: [], edges: [] });
	let graphLoading = $state(false);
	let graphSearched = $state(false);
	let graphHistory = $state<string[]>([]);
	let graphSelectedNode = $state<GraphNode | null>(null);
	let graphBrowseTab = $state<'entities' | 'tags' | 'topics'>('entities');

	const graphActive = $derived(graphData.nodes.length > 0);
	const totalFacets = $derived(
		facets.tags.length + facets.topics.length + facets.entities.length,
	);

	const sortedEntities = $derived(facets.entities.slice().sort((a, b) => b.count - a.count));
	const sortedTags = $derived(facets.tags.slice().sort((a, b) => b.count - a.count));
	const sortedTopics = $derived(facets.topics.slice().sort((a, b) => b.count - a.count));

	const selectedNodeConnections = $derived.by(() => {
		if (!graphSelectedNode) return [];
		const nodeId = graphSelectedNode.id;
		return graphData.edges
			.filter(e => e.source === nodeId || e.target === nodeId)
			.map(edge => {
				const isSource = edge.source === nodeId;
				const otherId = isSource ? edge.target : edge.source;
				const otherNode = graphData.nodes.find(n => n.id === otherId);
				return { edge, direction: isSource ? 'out' as const : 'in' as const, otherNode };
			})
			.filter((c): c is { edge: typeof c.edge; direction: 'out' | 'in'; otherNode: GraphNode } => !!c.otherNode);
	});

	function clearGraph() {
		graphData = { nodes: [], edges: [] };
		graphSearched = false;
		graphSelectedNode = null;
		graphQuery = "";
	}

	function exploreFacetItem(name: string) {
		graphQuery = name;
		searchEntity();
	}

	async function searchEntity() {
		const q = graphQuery.trim();
		if (!q) return;
		graphLoading = true;
		graphSearched = true;
		graphSelectedNode = null;
		try {
			graphData = await fetchEntityGraph(q);
			if (graphData.nodes.length === 0) {
				addToast(`No graph data found for "${q}"`, "info");
			}
			if (!graphHistory.includes(q))
				graphHistory = [...graphHistory.slice(-9), q];
		} catch (e) {
			addToast(
				`Graph query failed: ${e instanceof Error ? e.message : "Unknown error"}`,
				"error",
			);
			graphData = { nodes: [], edges: [] };
		} finally {
			graphLoading = false;
		}
	}

	async function handleGraphNodeClick(node: GraphNode) {
		graphSelectedNode = node;
		if (node.type === "entity") {
			graphQuery = node.label;
			await searchEntity();
		} else if (
			node.type === "document" &&
			node.id.startsWith("document:")
		) {
			const docId = node.id.replace("document:", "");
			graphLoading = true;
			try {
				graphData = await fetchDocumentGraph(docId);
				graphQuery = node.label;
				if (!graphHistory.includes(node.label))
					graphHistory = [...graphHistory.slice(-9), node.label];
			} catch (e) {
				addToast(
					`Failed to load document graph: ${e instanceof Error ? e.message : "Unknown"}`,
					"error",
				);
			} finally {
				graphLoading = false;
			}
		} else if (node.type === "tag" || node.type === "topic") {
			graphQuery = node.label;
			await searchEntity();
		}
	}

	async function loadFromGraphHistory(item: string) {
		graphQuery = item;
		await searchEntity();
	}

	export function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Escape" && graphSelectedNode) {
			graphSelectedNode = null;
		}
		if (e.key === "Enter") {
			const active = document.activeElement;
			if (active && active.classList.contains("graph-input")) {
				searchEntity();
			}
		}
	}
</script>

<div class="graph-area">
	<header class="graph-header">
		<div class="graph-title-row">
			{#if graphActive}
				<button class="graph-back-btn" onclick={clearGraph} aria-label="Back to index">
					<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M19 12H5" /><path d="m12 19-7-7 7-7" />
					</svg>
					INDEX
				</button>
				<span class="graph-breadcrumb-sep">/</span>
				{#each graphHistory as item, i}
					{#if i > 0}<span class="graph-breadcrumb-sep">/</span>{/if}
					<button
						class="graph-breadcrumb"
						class:active={item === graphQuery}
						onclick={() => loadFromGraphHistory(item)}
					>{item}</button>
				{/each}
				<span class="graph-node-count">{graphData.nodes.length}N / {graphData.edges.length}E</span>
			{:else}
				<h2 class="graph-title">KNOWLEDGE GRAPH</h2>
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
			<button
				onclick={searchEntity}
				disabled={graphLoading || !graphQuery.trim()}
				class="graph-btn"
			>
				{graphLoading ? "LOADING…" : "EXPLORE"}
			</button>
		</div>
	</header>

	{#if graphActive}
		<!-- MAP MODE -->
		<div class="graph-content">
			<div class="graph-body">
				<div class="canvas-wrap">
					<GraphCanvas
						nodes={graphData.nodes}
						edges={graphData.edges}
						onNodeClick={handleGraphNodeClick}
					/>
				</div>
			</div>

			{#if graphSelectedNode}
				<aside class="graph-detail">
					<div class="node-detail-header">
						<span
							class="node-type-badge"
							style:background={GRAPH_NODE_COLORS[graphSelectedNode.type] ?? "#94a3b8"}
						>
							{graphSelectedNode.type}
						</span>
						<button
							class="node-close"
							onclick={() => (graphSelectedNode = null)}
							aria-label="Close node detail"
						>
							<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
								<line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" />
							</svg>
						</button>
					</div>
					<div class="node-detail-title">
						{graphSelectedNode.label}
					</div>
					{#if graphSelectedNode.source_path}
						<div class="node-detail-path">
							{graphSelectedNode.source_path}
						</div>
					{/if}

					{#if selectedNodeConnections.length > 0}
						<div class="conn-section">
							<div class="conn-label">CONNECTIONS</div>
							<div class="conn-list">
								{#each selectedNodeConnections as conn}
									<button
										class="conn-item"
										onclick={() => handleGraphNodeClick(conn.otherNode)}
									>
										<span class="conn-dir">{conn.direction === 'out' ? '→' : '←'}</span>
										<span class="conn-rel">{conn.edge.label}</span>
										<span class="conn-name">{conn.otherNode.label}</span>
										<span
											class="conn-type-dot"
											style:background={GRAPH_NODE_COLORS[conn.otherNode.type] ?? '#94a3b8'}
										></span>
									</button>
								{/each}
							</div>
						</div>
					{/if}

					<div class="node-actions">
						{#if graphSelectedNode.type === "entity"}
							<button
								class="action-btn"
								onclick={() => {
									graphQuery = graphSelectedNode!.label;
									searchEntity();
								}}
							>
								Explore connections
							</button>
						{/if}
						<button
							class="action-btn"
							onclick={() => onsearchindocuments(graphSelectedNode!.label)}
						>
							Search in documents
						</button>
					</div>
				</aside>
			{/if}
		</div>

		<div class="graph-footer">
			<div class="legend">
				<span class="legend-item"><span class="legend-dot" style="background: #94a3b8;"></span> Entity</span>
				<span class="legend-item"><span class="legend-dot" style="background: #34d399;"></span> Document</span>
				<span class="legend-item"><span class="legend-dot" style="background: #a5b4fc;"></span> Tag</span>
				<span class="legend-item"><span class="legend-dot" style="background: #f87171;"></span> Topic</span>
			</div>
			<span class="legend-hint">Click nodes to traverse · Scroll to zoom · Drag to pan</span>
		</div>

	{:else}
		<!-- BROWSE MODE -->
		<div class="graph-browse">
			{#if graphLoading}
				<div class="graph-loading">
					<div class="loading-pulse"></div>
					<span>Loading graph data…</span>
				</div>
			{:else if totalFacets === 0}
				<div class="graph-empty">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.3">
						<circle cx="18" cy="5" r="3" /><circle cx="6" cy="12" r="3" />
						<circle cx="18" cy="19" r="3" /><path d="M8.59 13.51 15.42 17.49" />
						<path d="M15.41 6.51 8.59 10.49" />
					</svg>
					<p>No indexed data yet. Ingest documents to populate the knowledge graph.</p>
				</div>
			{:else}
				{#if graphSearched && !graphLoading}
					<div class="graph-no-results">
						<span class="no-results-icon">⊘</span>
						No graph data for "{graphQuery}" — select an item below
					</div>
				{/if}
				<nav class="browse-tabs">
					<button
						class="browse-tab"
						class:active={graphBrowseTab === 'entities'}
						onclick={() => graphBrowseTab = 'entities'}
					>
						<span class="tab-icon">●</span>
						ENTITIES
						<span class="tab-count">{facets.entities.length}</span>
					</button>
					<button
						class="browse-tab"
						class:active={graphBrowseTab === 'tags'}
						onclick={() => graphBrowseTab = 'tags'}
					>
						<span class="tab-icon">#</span>
						TAGS
						<span class="tab-count">{facets.tags.length}</span>
					</button>
					<button
						class="browse-tab"
						class:active={graphBrowseTab === 'topics'}
						onclick={() => graphBrowseTab = 'topics'}
					>
						<span class="tab-icon">◆</span>
						TOPICS
						<span class="tab-count">{facets.topics.length}</span>
					</button>
				</nav>

				<div class="browse-list">
					{#if graphBrowseTab === 'entities'}
						{#each sortedEntities as entry, i}
							<button
								class="browse-item entity-item"
								style:animation-delay="{Math.min(i * 15, 300)}ms"
								onclick={() => exploreFacetItem(entry.name)}
							>
								<span class="browse-item-icon" style="color: #94a3b8">●</span>
								<span class="browse-item-name">{entry.name}</span>
								<span class="browse-item-count">{entry.count}</span>
								<span class="browse-item-arrow">→</span>
							</button>
						{/each}
					{:else if graphBrowseTab === 'tags'}
						{#each sortedTags as entry, i}
							<button
								class="browse-item tag-item"
								style:animation-delay="{Math.min(i * 15, 300)}ms"
								onclick={() => exploreFacetItem(entry.name)}
							>
								<span class="browse-item-icon" style="color: #a5b4fc">#</span>
								<span class="browse-item-name">{entry.name}</span>
								<span class="browse-item-count">{entry.count}</span>
								<span class="browse-item-arrow">→</span>
							</button>
						{/each}
					{:else}
						{#each sortedTopics as entry, i}
							<button
								class="browse-item topic-item"
								style:animation-delay="{Math.min(i * 15, 300)}ms"
								onclick={() => exploreFacetItem(entry.name)}
							>
								<span class="browse-item-icon" style="color: #f87171">◆</span>
								<span class="browse-item-name">{entry.name}</span>
								<span class="browse-item-count">{entry.count}</span>
								<span class="browse-item-arrow">→</span>
							</button>
						{/each}
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.graph-area {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		min-width: 0;
	}

	.graph-header {
		padding: 14px 24px 12px;
		border-bottom: 1px solid var(--border-subtle);
		background: var(--bg);
		flex-shrink: 0;
	}
	.graph-title-row {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 10px;
		flex-wrap: wrap;
		min-height: 24px;
	}
	.graph-title {
		font-size: 14px;
		font-weight: 800;
		color: var(--text);
		letter-spacing: 0.08em;
	}
	.graph-back-btn {
		display: flex;
		align-items: center;
		gap: 5px;
		background: none;
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		padding: 3px 10px;
		color: var(--text-2);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.08em;
		cursor: pointer;
		transition: all var(--duration-fast);
		font-family: var(--mono);
	}
	.graph-back-btn:hover {
		background: var(--accent-dim);
		border-color: var(--accent);
		color: var(--accent-hover);
	}
	.graph-breadcrumb-sep {
		color: var(--text-3);
		font-size: 12px;
		opacity: 0.5;
	}
	.graph-breadcrumb {
		background: none;
		border: none;
		padding: 2px 6px;
		color: var(--text-3);
		font-size: 11px;
		font-family: var(--mono);
		cursor: pointer;
		border-radius: var(--r-sm);
		transition: all var(--duration-fast);
	}
	.graph-breadcrumb:hover {
		color: var(--text);
		background: var(--surface-2);
	}
	.graph-breadcrumb.active {
		color: var(--accent);
		font-weight: 600;
	}
	.graph-node-count {
		margin-left: auto;
		font-size: 10px;
		color: var(--text-3);
		font-family: var(--mono);
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		padding: 2px 8px;
		letter-spacing: 0.04em;
	}

	.graph-search-row {
		display: flex;
		gap: 8px;
	}
	.graph-input {
		flex: 1;
		padding: 8px 12px;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text);
		font-size: 13px;
		outline: none;
		transition: border-color 0.15s, box-shadow 0.15s;
		font-family: var(--font);
	}
	.graph-input:focus {
		border-color: var(--accent);
		box-shadow: 0 0 0 2px var(--accent-dim);
	}
	.graph-input::placeholder {
		color: var(--text-3);
	}
	.graph-btn {
		padding: 8px 16px;
		background: var(--gradient-accent);
		color: #fff;
		border: none;
		border-radius: var(--r-sm);
		font-weight: 700;
		font-size: 11px;
		letter-spacing: 0.06em;
		cursor: pointer;
		transition: opacity 0.15s;
		font-family: var(--mono);
	}
	.graph-btn:hover {
		opacity: 0.9;
	}
	.graph-btn:disabled {
		opacity: 0.5;
		cursor: default;
	}

	/* Graph content (map mode) */
	.graph-content {
		flex: 1;
		display: flex;
		overflow: hidden;
	}
	.graph-body {
		flex: 1;
		position: relative;
		overflow: hidden;
	}
	.canvas-wrap {
		width: 100%;
		height: 100%;
	}

	.graph-detail {
		width: 280px;
		flex-shrink: 0;
		background: var(--surface);
		border-left: 1px solid var(--border);
		padding: 16px;
		overflow-y: auto;
		animation: fadeInUp 0.2s ease both;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}
	.node-detail-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.node-type-badge {
		font-size: 9px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		padding: 2px 8px;
		border-radius: var(--r-sm);
		color: #fff;
		font-family: var(--mono);
	}
	.node-close {
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		color: var(--text-3);
		transition: background var(--duration-fast), color var(--duration-fast);
	}
	.node-close:hover {
		background: var(--border);
		color: var(--text);
	}
	.node-detail-title {
		font-size: 15px;
		font-weight: 700;
		color: var(--text);
		line-height: 1.3;
	}
	.node-detail-path {
		font-size: 10px;
		font-family: var(--mono);
		color: var(--text-3);
		word-break: break-all;
	}

	/* connections in detail panel */
	.conn-section {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.conn-label {
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.1em;
		color: var(--text-3);
		font-family: var(--mono);
		padding-bottom: 4px;
		border-bottom: 1px solid var(--border-subtle);
	}
	.conn-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
		max-height: 260px;
		overflow-y: auto;
	}
	.conn-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 5px 8px;
		background: transparent;
		border: 1px solid transparent;
		border-radius: var(--r-sm);
		cursor: pointer;
		transition: all var(--duration-fast);
		text-align: left;
		font-family: var(--font);
	}
	.conn-item:hover {
		background: var(--surface-2);
		border-color: var(--border);
	}
	.conn-dir {
		font-size: 11px;
		color: var(--text-3);
		flex-shrink: 0;
		width: 14px;
		text-align: center;
	}
	.conn-rel {
		font-size: 9px;
		font-family: var(--mono);
		color: var(--text-3);
		flex-shrink: 0;
		text-transform: lowercase;
		min-width: 50px;
	}
	.conn-name {
		font-size: 11px;
		font-weight: 600;
		color: var(--text-2);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		flex: 1;
	}
	.conn-item:hover .conn-name {
		color: var(--text);
	}
	.conn-type-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.node-actions {
		display: flex;
		flex-direction: column;
		gap: 4px;
		margin-top: 4px;
	}
	.action-btn {
		display: block;
		width: 100%;
		padding: 7px 10px;
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text-2);
		font-size: 11px;
		font-weight: 600;
		text-align: center;
		cursor: pointer;
		font-family: var(--font);
		transition: all var(--duration-fast);
	}
	.action-btn:hover {
		background: var(--accent-dim);
		border-color: rgba(16, 185, 129, 0.3);
		color: var(--accent-hover);
	}

	.graph-footer {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 8px 24px;
		border-top: 1px solid var(--border-subtle);
		background: var(--surface);
		flex-shrink: 0;
	}
	.graph-footer .legend-dot {
		box-shadow: none;
	}
	.legend {
		display: flex;
		align-items: center;
		gap: 14px;
		font-size: 11px;
		color: var(--text-2);
	}
	.legend-item {
		display: flex;
		align-items: center;
		gap: 4px;
	}
	.legend-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		display: inline-block;
	}
	.legend-hint {
		margin-left: auto;
		font-size: 10px;
		color: var(--text-3);
		font-family: var(--mono);
	}

	/* Browse Mode */
	.graph-browse {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}
	.graph-loading {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 16px;
		color: var(--text-3);
		font-size: 13px;
	}
	.loading-pulse {
		width: 32px;
		height: 32px;
		border: 2px solid var(--accent);
		border-radius: 50%;
		animation: pulse 1.2s ease infinite;
	}
	.graph-empty {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: var(--text-3);
		text-align: center;
		gap: 14px;
		animation: fadeIn 0.3s ease both;
	}
	.graph-empty p {
		font-size: 13px;
		max-width: 380px;
		line-height: 1.6;
	}

	/* Browse Tabs */
	.browse-tabs {
		display: flex;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
		flex-shrink: 0;
	}
	.browse-tab {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 10px 12px;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--text-3);
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.1em;
		cursor: pointer;
		transition: all var(--duration-fast);
		font-family: var(--mono);
	}
	.browse-tab:hover {
		color: var(--text-2);
		background: var(--surface-2);
	}
	.browse-tab.active {
		color: var(--accent);
		border-bottom-color: var(--accent);
	}
	.tab-icon {
		font-size: 12px;
	}
	.tab-count {
		font-size: 9px;
		color: var(--text-3);
		background: var(--surface-2);
		border-radius: var(--r-sm);
		padding: 1px 5px;
	}
	.browse-tab.active .tab-count {
		background: var(--accent-dim);
		color: var(--accent);
	}

	/* Browse List */
	.browse-list {
		flex: 1;
		overflow-y: auto;
		padding: 4px 0;
	}
	.browse-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 9px 24px;
		background: transparent;
		border: none;
		border-bottom: 1px solid var(--border-subtle);
		color: var(--text-2);
		font-size: 13px;
		cursor: pointer;
		transition: all var(--duration-fast);
		text-align: left;
		font-family: var(--font);
		animation: fadeInUp 0.2s var(--ease-out) both;
	}
	.browse-item:hover {
		background: var(--surface-2);
		color: var(--text);
	}
	.browse-item:last-child {
		border-bottom: none;
	}
	.browse-item-icon {
		font-size: 10px;
		flex-shrink: 0;
		width: 16px;
		text-align: center;
	}
	.browse-item-name {
		flex: 1;
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	.browse-item-count {
		font-size: 10px;
		font-family: var(--mono);
		color: var(--text-3);
		min-width: 24px;
		text-align: right;
	}
	.browse-item-arrow {
		font-size: 12px;
		color: var(--text-3);
		opacity: 0;
		transform: translateX(-4px);
		transition: all var(--duration-fast);
	}
	.browse-item:hover .browse-item-arrow {
		opacity: 1;
		transform: translateX(0);
	}

	/* No results banner */
	.graph-no-results {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 24px;
		background: rgba(248, 113, 113, 0.06);
		border-bottom: 1px solid rgba(248, 113, 113, 0.12);
		color: var(--text-3);
		font-size: 11px;
		flex-shrink: 0;
	}
	.no-results-icon {
		color: var(--red);
		font-size: 13px;
	}
</style>
