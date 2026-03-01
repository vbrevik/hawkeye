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
</script>

<button
	class="card"
	class:active
	class:featured
	style:animation-delay="{index * 40}ms"
	{onclick}
>
	<div class="card-header">
		<div class="card-title">{result.title || result.file}</div>
		<div class="card-score">{result.score.toFixed(2)}</div>
	</div>
	<div class="card-file">{result.file}</div>
	<div class="card-tldr">{result.tldr}</div>
	{#if tags.length > 0}
		<div class="card-tags">
			{#each tags as tag}
				<span
					class="tag"
					role="button"
					tabindex="0"
					onclick={(e) => { e.stopPropagation(); ontagclick(tag); }}
					onkeydown={(e) => { if (e.key === 'Enter') { e.stopPropagation(); ontagclick(tag); } }}
				>#{tag}</span>
			{/each}
		</div>
	{/if}
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
		padding: 18px 20px; border-left: 3px solid var(--accent);
		background: linear-gradient(135deg, var(--surface) 0%, rgba(99, 102, 241, 0.03) 100%);
	}
	.card.featured .card-title { font-size: 16px; }
	.card.featured .card-tldr { -webkit-line-clamp: 3; }

	.card-header {
		display: flex; align-items: flex-start; justify-content: space-between;
		gap: 10px; margin-bottom: 4px;
	}
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
	}
	.card-tldr {
		font-size: 13px; color: var(--text-2); line-height: 1.6; margin-bottom: 8px;
		display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;
	}
	.card-tags { display: flex; flex-wrap: wrap; gap: 4px; }
	.tag {
		background: var(--surface-2); border-radius: 4px; color: var(--text-3);
		font-size: 10px; padding: 2px 7px; cursor: pointer;
		transition: color 0.15s, background 0.15s, transform 0.1s; font-weight: 500;
		border: none;
	}
	.tag:hover { background: var(--accent-dim); color: var(--accent-hover); transform: scale(1.04); }
</style>
