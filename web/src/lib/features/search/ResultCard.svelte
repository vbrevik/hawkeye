<script lang="ts">
	import type { DisplayResult, SearchMode } from '$lib/api/types';

	let {
		result,
		active = false,
		featured = false,
		index = 0,
		searchMode = 'hybrid' as SearchMode,
		onclick,
		ontagclick
	}: {
		result: DisplayResult;
		active?: boolean;
		featured?: boolean;
		index?: number;
		searchMode?: SearchMode;
		onclick: () => void;
		ontagclick: (tag: string) => void;
	} = $props();

	const tags = $derived(
		(result.tags || '').split(' ').filter(Boolean)
	);
	const entities = $derived(
		(result.entities || '').split(' ').filter(Boolean)
	);
	const topics = $derived(
		(result.topics || '').split(' ').filter(Boolean)
	);

	const isDualMatch = $derived(
		result.keyword_rank != null && result.semantic_rank != null
	);
	const hasProvenance = $derived(
		searchMode === 'hybrid' && (result.keyword_rank != null || result.semantic_rank != null)
	);

	function formatScore(score: number): string {
		if (score < 0.01) return score.toExponential(1);
		if (score < 1) return score.toFixed(3);
		return score.toFixed(1);
	}
</script>

<button
	class="card"
	class:active
	class:featured
	data-mode={searchMode}
	style:animation-delay="{index * 45}ms"
	{onclick}
>
	{#if featured}
		<div class="featured-badge" data-mode={searchMode}>
			{#if isDualMatch}★ Best Match{:else}Top Result{/if}
		</div>
	{/if}

	<div class="card-header">
		<div class="card-title-wrap">
			<svg class="card-doc-icon" width="14" height="14" viewBox="0 0 24 24" fill="none"
				stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
				<polyline points="14 2 14 8 20 8"/>
			</svg>
			<div class="card-title">{result.title || result.file}</div>
		</div>
		<div class="card-score" data-mode={searchMode}>{formatScore(result.score)}</div>
	</div>
	<div class="card-file">{result.file}</div>
	<div class="card-tldr">{result.tldr}</div>

	{#if tags.length > 0 || entities.length > 0}
		<div class="card-facets">
			{#each tags.slice(0, featured ? 5 : 3) as tag}
				<span
					class="chip tag-variant"
					role="button"
					tabindex="0"
					onclick={(e) => { e.stopPropagation(); ontagclick(tag); }}
					onkeydown={(e) => { if (e.key === 'Enter') { e.stopPropagation(); ontagclick(tag); } }}
				>#{tag}</span>
			{/each}
			{#each entities.slice(0, featured ? 4 : 2) as entity}
				<span
					class="chip entity-variant"
					role="button"
					tabindex="0"
					onclick={(e) => { e.stopPropagation(); ontagclick(entity); }}
					onkeydown={(e) => { if (e.key === 'Enter') { e.stopPropagation(); ontagclick(entity); } }}
				>{entity}</span>
			{/each}
			{#if featured}
				{#each topics.slice(0, 3) as topic}
					<span class="chip topic-variant">{topic}</span>
				{/each}
			{/if}
		</div>
	{/if}

	{#if hasProvenance}
		<div class="provenance">
			{#if isDualMatch}
				<span class="prov-badge prov-dual">★ dual</span>
			{/if}
			{#if result.keyword_rank != null}
				<span class="prov-badge prov-kw">KW #{result.keyword_rank}</span>
			{/if}
			{#if result.semantic_rank != null}
				<span class="prov-badge prov-sem">SEM #{result.semantic_rank}</span>
			{/if}
		</div>
	{/if}
</button>

<style>
	.card {
		background: var(--surface); border: 1px solid var(--border-subtle);
		border-radius: var(--r); padding: 16px 18px; cursor: pointer;
		transition: transform 0.2s var(--ease-out), border-color 0.25s, box-shadow 0.3s, background 0.2s;
		position: relative; animation: fadeInUp 0.4s ease both;
		text-align: left; width: 100%; display: block;
		color: var(--text); font-family: var(--font);
	}
	.card:hover {
		transform: translateY(-1px); border-color: rgba(16, 185, 129, 0.25);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3); background: var(--surface-2);
	}
	.card.active {
		border-color: var(--accent); background: var(--accent-dim);
		box-shadow: inset 3px 0 0 var(--accent), 0 0 20px var(--accent-glow);
	}
	.card.featured {
		padding: 22px 24px;
		background: linear-gradient(135deg, var(--surface) 0%, rgba(16, 185, 129, 0.03) 100%);
		border-color: rgba(16, 185, 129, 0.2);
	}
	.card.featured .card-title { font-size: 18px; }
	.card.featured .card-tldr { -webkit-line-clamp: 4; line-clamp: 4; font-size: 14px; }

	.featured-badge {
		font-size: 9px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.12em; margin-bottom: 8px;
	}
	.featured-badge[data-mode='hybrid'] { color: var(--mode-hybrid); }
	.featured-badge[data-mode='keyword'] { color: var(--mode-keyword); }
	.featured-badge[data-mode='semantic'] { color: var(--mode-semantic); }

	.card-header {
		display: flex; align-items: flex-start; justify-content: space-between;
		gap: 12px; margin-bottom: 4px;
	}
	.card-title-wrap {
		display: flex; align-items: flex-start; gap: 8px; min-width: 0;
	}
	.card-doc-icon { color: var(--text-3); flex-shrink: 0; margin-top: 4px; }
	.card-title {
		font-size: 14px; font-weight: 700; color: var(--text);
		line-height: 1.35; letter-spacing: -0.01em;
	}

	.card-score {
		font-size: 10px; font-weight: 700;
		border-radius: var(--r-sm); padding: 2px 8px; flex-shrink: 0;
		font-family: var(--mono); letter-spacing: -0.02em;
		border: 1px solid;
	}
	.card-score[data-mode='hybrid'] {
		color: var(--mode-hybrid); background: var(--mode-hybrid-bg); border-color: var(--mode-hybrid-border);
	}
	.card-score[data-mode='keyword'] {
		color: var(--mode-keyword); background: var(--mode-keyword-bg); border-color: var(--mode-keyword-border);
	}
	.card-score[data-mode='semantic'] {
		color: var(--mode-semantic); background: var(--mode-semantic-bg); border-color: var(--mode-semantic-border);
	}

	.card-file {
		font-size: 11px; font-family: var(--mono); color: var(--text-3);
		margin-bottom: 8px; letter-spacing: -0.01em;
		padding-left: 22px;
	}
	.card-tldr {
		font-size: 13px; color: var(--text-2); line-height: 1.65; margin-bottom: 10px;
		display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;
	}

	.card-facets { display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 2px; }
	.chip {
		border-radius: var(--r-sm); font-size: 10px; padding: 2px 7px;
		cursor: pointer; font-weight: 500; border: 1px solid;
		transition: background var(--duration-fast), transform 0.1s;
	}
	.chip:hover { transform: scale(1.04); }

	.tag-variant {
		background: var(--tag-bg); border-color: var(--tag-border); color: var(--tag-text);
	}
	.tag-variant:hover { background: rgba(165, 180, 252, 0.15); }

	.entity-variant {
		background: var(--entity-bg); border-color: var(--entity-border); color: var(--entity-text);
	}
	.entity-variant:hover { background: rgba(251, 191, 36, 0.15); }

	.topic-variant {
		background: var(--topic-bg); border-color: var(--topic-border); color: var(--topic-text);
		cursor: default;
	}

	/* Provenance badges */
	.provenance {
		display: flex; gap: 5px; margin-top: 8px;
		padding-top: 8px; border-top: 1px solid var(--border-subtle);
	}
	.prov-badge {
		font-size: 9px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.06em; padding: 2px 7px; border-radius: var(--r-sm);
		border: 1px solid;
	}
	.prov-dual {
		color: var(--mode-hybrid); background: var(--mode-hybrid-bg);
		border-color: var(--mode-hybrid-border);
	}
	.prov-kw {
		color: var(--mode-keyword); background: var(--mode-keyword-bg);
		border-color: var(--mode-keyword-border);
	}
	.prov-sem {
		color: var(--mode-semantic); background: var(--mode-semantic-bg);
		border-color: var(--mode-semantic-border);
	}
</style>
