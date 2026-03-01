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
	.no-filters { font-size: 11px; color: var(--text-3); font-style: italic; }

	.chip {
		display: inline-flex; align-items: center; gap: 3px;
		border-radius: 4px; font-size: 11px; padding: 2px 8px;
		cursor: pointer; transition: transform 0.1s, background 0.15s;
		font-weight: 500; border: 1px solid; font-family: var(--font);
	}
	.chip:hover { transform: scale(1.03); }
	.chip:active { transform: scale(0.97); }
	.count { opacity: 0.5; font-size: 9px; font-weight: 400; }

	.chip--tag {
		background: var(--accent-dim); border-color: rgba(99, 102, 241, 0.2);
		color: var(--accent-hover);
	}
	.chip--tag:hover { background: rgba(99, 102, 241, 0.2); }

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
