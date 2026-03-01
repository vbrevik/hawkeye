<script lang="ts">
	import type { SearchResult } from '$lib/api/types';

	let {
		result,
		active = false,
		featured = false,
		index = 0,
		onclick,
		ontagclick
	}: {
		result: SearchResult;
		active?: boolean;
		featured?: boolean;
		index?: number;
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
</script>

<button
	class="card"
	class:active
	class:featured
	style:animation-delay="{index * 40}ms"
	{onclick}
>
	{#if featured}
		<div class="featured-badge">Top Result</div>
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
		<div class="card-score">{result.score.toFixed(2)}</div>
	</div>
	<div class="card-file">{result.file}</div>
	<div class="card-tldr">{result.tldr}</div>

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
</button>

<style>
	.card {
		background: var(--surface); border: 1px solid var(--border-subtle);
		border-radius: var(--r); padding: 14px 16px; cursor: pointer;
		transition: transform 0.2s, border-color 0.2s, box-shadow 0.2s, background 0.2s;
		position: relative; animation: fadeInUp 0.35s ease both;
		text-align: left; width: 100%; display: block;
		color: var(--text); font-family: var(--font);
	}
	.card:hover {
		transform: translateY(-1px); border-color: rgba(99, 102, 241, 0.3);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25); background: var(--surface-2);
	}
	.card.active {
		border-color: var(--accent); background: var(--accent-dim);
		box-shadow: inset 3px 0 0 var(--accent), 0 0 16px var(--accent-glow);
	}
	.card.featured {
		padding: 20px 22px; border-left: 3px solid var(--accent);
		background: linear-gradient(135deg, var(--surface) 0%, rgba(99, 102, 241, 0.05) 100%);
		border-color: rgba(99, 102, 241, 0.25);
	}
	.card.featured .card-title { font-size: 17px; }
	.card.featured .card-tldr { -webkit-line-clamp: 4; font-size: 14px; }

	.featured-badge {
		font-size: 9px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.1em; color: var(--accent-hover); margin-bottom: 6px;
	}

	.card-header {
		display: flex; align-items: flex-start; justify-content: space-between;
		gap: 10px; margin-bottom: 4px;
	}
	.card-title-wrap {
		display: flex; align-items: flex-start; gap: 8px; min-width: 0;
	}
	.card-doc-icon { color: var(--text-3); flex-shrink: 0; margin-top: 2px; }
	.card-title {
		font-size: 14px; font-weight: 700; color: var(--text);
		line-height: 1.3; letter-spacing: -0.01em;
	}
	.card-score {
		font-size: 10px; font-weight: 700; color: var(--accent-hover);
		background: var(--accent-dim); border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 5px; padding: 2px 7px; flex-shrink: 0;
		font-family: var(--mono); letter-spacing: -0.02em;
	}
	.card-file {
		font-size: 11px; font-family: var(--mono); color: var(--text-3);
		margin-bottom: 6px; letter-spacing: -0.01em;
		padding-left: 22px;
	}
	.card-tldr {
		font-size: 13px; color: var(--text-2); line-height: 1.6; margin-bottom: 8px;
		display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;
	}

	.card-facets { display: flex; flex-wrap: wrap; gap: 4px; }
	.chip {
		border-radius: 4px; font-size: 10px; padding: 2px 7px;
		cursor: pointer; font-weight: 500; border: 1px solid;
		transition: background var(--duration-fast), transform 0.1s;
	}
	.chip:hover { transform: scale(1.04); }

	.tag-variant {
		background: var(--tag-bg); border-color: var(--tag-border); color: var(--tag-text);
	}
	.tag-variant:hover { background: rgba(99, 102, 241, 0.18); }

	.entity-variant {
		background: var(--entity-bg); border-color: var(--entity-border); color: var(--entity-text);
	}
	.entity-variant:hover { background: rgba(251, 191, 36, 0.15); }

	.topic-variant {
		background: var(--topic-bg); border-color: var(--topic-border); color: var(--topic-text);
		cursor: default;
	}
</style>
