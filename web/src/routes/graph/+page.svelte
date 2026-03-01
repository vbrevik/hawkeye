<script lang="ts">
	import type { GraphNode, GraphResponse } from '$lib/api/types';
	import { fetchEntityGraph, fetchDocumentGraph } from '$lib/api/graph';
	import { addToast } from '$lib/stores/toast';
	import GraphCanvas from '$lib/features/graph/GraphCanvas.svelte';

	const NODE_COLORS: Record<string, string> = {
		entity: '#6366f1',
		document: '#34d399',
		tag: '#fbbf24',
		topic: '#f87171',
	};

	let query = $state('');
	let graphData = $state<GraphResponse>({ nodes: [], edges: [] });
	let loading = $state(false);
	let searched = $state(false);
	let history = $state<string[]>([]);
	let selectedNode = $state<GraphNode | null>(null);

	async function searchEntity() {
		const q = query.trim();
		if (!q) return;
		loading = true;
		searched = true;
		selectedNode = null;
		try {
			graphData = await fetchEntityGraph(q);
			if (graphData.nodes.length === 0) {
				addToast(`No graph data found for "${q}"`, 'info');
			}
			if (!history.includes(q)) history = [...history.slice(-9), q];
		} catch (e) {
			addToast(`Graph query failed: ${e instanceof Error ? e.message : 'Unknown error'}`, 'error');
			graphData = { nodes: [], edges: [] };
		} finally {
			loading = false;
		}
	}

	async function handleNodeClick(node: GraphNode) {
		selectedNode = node;
		if (node.type === 'entity') {
			query = node.label;
			await searchEntity();
		} else if (node.type === 'document' && node.id.startsWith('document:')) {
			const docId = node.id.replace('document:', '');
			loading = true;
			try {
				graphData = await fetchDocumentGraph(docId);
				query = node.label;
				if (!history.includes(node.label)) history = [...history.slice(-9), node.label];
			} catch (e) {
				addToast(`Failed to load document graph: ${e instanceof Error ? e.message : 'Unknown'}`, 'error');
			} finally {
				loading = false;
			}
		} else if (node.type === 'tag' || node.type === 'topic') {
			query = node.label;
			await searchEntity();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') searchEntity();
		if (e.key === 'Escape' && selectedNode) {
			selectedNode = null;
		}
	}

	async function loadFromHistory(item: string) {
		query = item;
		await searchEntity();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="graph-page">
	<header class="graph-header">
		<a href="/" class="back-link">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
				stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<polyline points="15 18 9 12 15 6"/>
			</svg>
			Back to Search
		</a>
		<h1>Knowledge Graph</h1>
		<div class="search-row">
			<input
				type="text"
				bind:value={query}
				placeholder="Search entity, tag, or topic..."
				aria-label="Search knowledge graph"
				class="graph-input"
			/>
			<button onclick={searchEntity} disabled={loading || !query.trim()} class="graph-btn">
				{loading ? 'Loading...' : 'Explore'}
			</button>
		</div>
		{#if history.length > 0}
			<div class="history">
				{#each history as item}
					<button class="history-chip" onclick={() => loadFromHistory(item)}>{item}</button>
				{/each}
			</div>
		{/if}
	</header>

	<div class="graph-main">
		<div class="graph-body">
			{#if graphData.nodes.length > 0}
				<div class="canvas-wrap">
					<GraphCanvas nodes={graphData.nodes} edges={graphData.edges} onNodeClick={handleNodeClick} />
				</div>
			{:else if searched && !loading}
				<div class="empty">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor"
						stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.3">
						<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/>
						<circle cx="18" cy="19" r="3"/><path d="M8.59 13.51 15.42 17.49"/>
						<path d="M15.41 6.51 8.59 10.49"/>
					</svg>
					<p>No graph data found. Try searching for an entity that appears in your ingested documents.</p>
				</div>
			{:else if !searched}
				<div class="empty">
					<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor"
						stroke-width="1" stroke-linecap="round" stroke-linejoin="round" opacity="0.3">
						<circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/>
					</svg>
					<p>Enter an entity name to explore its knowledge graph connections.</p>
					<p class="empty-hint">Entities, tags, and topics are extracted during document ingestion.</p>
				</div>
			{/if}
		</div>

		{#if selectedNode}
			<aside class="graph-sidebar">
				<div class="node-detail-header">
					<span class="node-type-badge" style:background={NODE_COLORS[selectedNode.type] ?? '#6366f1'}>
						{selectedNode.type}
					</span>
					<button class="node-close" onclick={() => selectedNode = null} aria-label="Close node detail">
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
							stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
							<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
						</svg>
					</button>
				</div>
				<div class="node-detail-title">{selectedNode.label}</div>
				{#if selectedNode.source_path}
					<div class="node-detail-path">{selectedNode.source_path}</div>
				{/if}
				<div class="node-actions">
					{#if selectedNode.type === 'entity'}
						<button class="action-btn" onclick={() => { query = selectedNode!.label; searchEntity(); }}>
							Explore connections
						</button>
					{/if}
					<a href="/?q={encodeURIComponent(selectedNode.label)}" class="action-btn">
						Search in documents
					</a>
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
		{#if graphData.nodes.length > 0}
			<div class="graph-stats-inline">
				{graphData.nodes.length} nodes / {graphData.edges.length} edges
			</div>
		{/if}
		<span class="legend-hint">Click nodes to explore · Scroll to zoom · Drag to pan</span>
	</div>
</div>

<style>
	.graph-page {
		display: flex; flex-direction: column;
		height: 100vh; background: var(--bg); overflow: hidden;
	}

	.graph-header {
		padding: 14px 24px 12px;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
	}

	.back-link {
		font-size: 12px; color: var(--text-3); text-decoration: none;
		margin-bottom: 4px; display: inline-flex; align-items: center; gap: 4px;
		transition: color var(--duration-fast);
	}
	.back-link:hover { color: var(--accent-hover); text-decoration: none; }

	h1 {
		font-size: 18px; font-weight: 700; margin: 4px 0 10px;
		background: var(--gradient-accent);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.search-row { display: flex; gap: 8px; }

	.graph-input {
		flex: 1; padding: 9px 14px; background: var(--bg);
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

	.history { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }
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

	/* Main content area: graph + sidebar */
	.graph-main { flex: 1; display: flex; overflow: hidden; }

	.graph-body { flex: 1; position: relative; overflow: hidden; }
	.canvas-wrap { width: 100%; height: 100%; }

	.empty {
		display: flex; flex-direction: column; align-items: center;
		justify-content: center; height: 100%; color: var(--text-3);
		text-align: center; gap: 12px;
	}
	.empty p { font-size: 14px; max-width: 400px; }
	.empty-hint { font-size: 12px; color: var(--text-3); opacity: 0.7; }

	/* Node detail sidebar */
	.graph-sidebar {
		width: 280px; flex-shrink: 0;
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
		color: var(--accent-hover); text-decoration: none;
	}

	/* Footer */
	.graph-footer {
		display: flex; align-items: center; gap: 16px;
		padding: 10px 24px; border-top: 1px solid var(--border);
		background: var(--surface);
	}
	.legend { display: flex; align-items: center; gap: 14px; font-size: 12px; color: var(--text-2); }
	.legend-item { display: flex; align-items: center; gap: 5px; }
	.legend-dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }
	.graph-stats-inline {
		font-size: 11px; color: var(--text-3); font-family: var(--mono);
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); padding: 2px 8px;
	}
	.legend-hint {
		margin-left: auto; font-size: 11px; color: var(--text-3); font-family: var(--mono);
	}
</style>
