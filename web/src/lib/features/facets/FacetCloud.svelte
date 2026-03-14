<script lang="ts">
	import type { FacetEntry } from '$lib/api/types';

	let {
		entries,
		prefix = '',
		emptyText = '—',
		variant = 'tag',
		onclick
	}: {
		entries: FacetEntry[];
		prefix?: string;
		emptyText?: string;
		variant?: 'tag' | 'topic' | 'entity';
		onclick: (name: string) => void;
	} = $props();
</script>

<div class="filter-chips">
	{#if entries.length === 0}
		<span class="no-filters">{emptyText}</span>
	{:else}
		{#each entries as entry}
			<button class="chip chip--{variant}" onclick={() => onclick(entry.name)}>
				{prefix}{entry.name}&nbsp;<span class="count">{entry.count}</span>
			</button>
		{/each}
	{/if}
</div>

<style>
	.filter-chips { display: flex; flex-wrap: wrap; gap: 4px; }
	.no-filters { font-size: 11px; color: var(--text-3); }

	.chip {
		display: inline-flex; align-items: center; gap: 3px;
		border-radius: var(--r-sm); font-size: 11px; padding: 2px 8px;
		cursor: pointer; transition: background 0.1s;
		font-weight: 500; border: 1px solid; font-family: var(--font);
	}
	.chip:active { transform: scale(0.98); }
	.count { opacity: 0.5; font-size: 9px; font-weight: 400; }

	.chip--tag {
		background: var(--tag-bg); border-color: var(--tag-border);
		color: var(--tag-text);
	}
	.chip--tag:hover { background: rgba(165, 180, 252, 0.15); }

	.chip--topic {
		background: rgba(56, 189, 248, 0.08); border-color: rgba(56, 189, 248, 0.18);
		color: #7dd3fc;
	}
	.chip--topic:hover { background: rgba(56, 189, 248, 0.15); }

	.chip--entity {
		background: rgba(251, 191, 36, 0.08); border-color: rgba(251, 191, 36, 0.18);
		color: #fcd34d;
	}
	.chip--entity:hover { background: rgba(251, 191, 36, 0.15); }
</style>
