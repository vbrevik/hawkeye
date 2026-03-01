<script lang="ts">
	import type { GraphNode, GraphResponse } from '$lib/api/types';
	import { fetchEntityGraph, fetchDocumentGraph } from '$lib/api/graph';
	import { addToast } from '$lib/stores/toast';
	import GraphCanvas from '$lib/features/graph/GraphCanvas.svelte';

	let query = $state('');
	let graphData = $state<GraphResponse>({ nodes: [], edges: [] });
	let loading = $state(false);
	let searched = $state(false);
	let history = $state<string[]>([]);

	async function searchEntity() {
		const q = query.trim();
		if (!q) return;
		loading = true;
		searched = true;
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
	}

	async function loadFromHistory(item: string) {
		query = item;
		await searchEntity();
	}
</script>

<div class="graph-page">
	<header class="graph-header">
		<a href="/" class="back-link">← Search</a>
		<h1>Knowledge Graph</h1>
		<div class="search-row">
			<input
				type="text"
				bind:value={query}
				onkeydown={handleKeydown}
				placeholder="Search entity, tag, or topic…"
				class="graph-input"
			/>
			<button onclick={searchEntity} disabled={loading || !query.trim()} class="graph-btn">
				{loading ? 'Loading…' : 'Explore'}
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

	<div class="graph-body">
		{#if graphData.nodes.length > 0}
			<div class="canvas-wrap">
				<GraphCanvas nodes={graphData.nodes} edges={graphData.edges} onNodeClick={handleNodeClick} />
			</div>
			<div class="graph-stats">
				{graphData.nodes.length} nodes · {graphData.edges.length} edges
			</div>
		{:else if searched && !loading}
			<div class="empty">
				<span class="empty-icon">🕸️</span>
				<p>No graph data found. Try searching for an entity that appears in your ingested documents.</p>
			</div>
		{:else if !searched}
			<div class="empty">
				<span class="empty-icon">🔍</span>
				<p>Enter an entity name to explore its knowledge graph connections.</p>
				<p class="empty-hint">Entities, tags, and topics are extracted during document ingestion.</p>
			</div>
		{/if}
	</div>

	<div class="legend">
		<span class="legend-item"><span class="legend-dot" style="background: #6366f1;"></span> Entity</span>
		<span class="legend-item"><span class="legend-dot" style="background: #34d399;"></span> Document</span>
		<span class="legend-item"><span class="legend-dot" style="background: #fbbf24;"></span> Tag</span>
		<span class="legend-item"><span class="legend-dot" style="background: #f87171;"></span> Topic</span>
		<span class="legend-hint">Click nodes to explore · Scroll to zoom · Drag to pan</span>
	</div>
</div>

<style>
	.graph-page {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: var(--bg);
		overflow: hidden;
	}

	.graph-header {
		padding: 16px 24px 12px;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
	}

	.back-link {
		font-size: 13px;
		color: var(--text-2);
		text-decoration: none;
		margin-bottom: 4px;
		display: inline-block;
	}
	.back-link:hover { color: var(--accent-hover); text-decoration: none; }

	h1 {
		font-size: 20px;
		font-weight: 700;
		margin: 4px 0 12px;
		background: var(--gradient-accent);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.search-row {
		display: flex;
		gap: 8px;
	}

	.graph-input {
		flex: 1;
		padding: 10px 14px;
		background: var(--bg);
		border: 1px solid var(--border);
		border-radius: var(--r-sm);
		color: var(--text);
		font-size: 14px;
		outline: none;
		transition: border-color 0.15s;
	}
	.graph-input:focus { border-color: var(--accent); }
	.graph-input::placeholder { color: var(--text-3); }

	.graph-btn {
		padding: 10px 20px;
		background: var(--gradient-accent);
		color: #fff;
		border: none;
		border-radius: var(--r-sm);
		font-weight: 600;
		font-size: 13px;
		cursor: pointer;
		transition: opacity 0.15s;
	}
	.graph-btn:hover { opacity: 0.9; }
	.graph-btn:disabled { opacity: 0.5; cursor: default; }

	.history {
		display: flex;
		gap: 6px;
		flex-wrap: wrap;
		margin-top: 8px;
	}

	.history-chip {
		padding: 3px 10px;
		background: var(--accent-dim);
		border: 1px solid var(--border);
		border-radius: 99px;
		color: var(--text-2);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}
	.history-chip:hover {
		background: var(--accent-glow);
		color: var(--accent-hover);
		border-color: var(--accent);
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

	.graph-stats {
		position: absolute;
		top: 12px;
		right: 16px;
		font-size: 11px;
		color: var(--text-3);
		font-family: var(--mono);
		background: var(--surface);
		padding: 4px 10px;
		border-radius: var(--r-sm);
		border: 1px solid var(--border);
	}

	.empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-3);
		text-align: center;
		gap: 8px;
	}
	.empty-icon { font-size: 48px; }
	.empty p { font-size: 14px; max-width: 400px; }
	.empty-hint { font-size: 12px; color: var(--text-3); opacity: 0.7; }

	.legend {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 10px 24px;
		border-top: 1px solid var(--border);
		background: var(--surface);
		font-size: 12px;
		color: var(--text-2);
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 5px;
	}

	.legend-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		display: inline-block;
	}

	.legend-hint {
		margin-left: auto;
		font-size: 11px;
		color: var(--text-3);
		font-family: var(--mono);
	}
</style>
