<script lang="ts">
	import type { Facets } from '$lib/api/types';
	import StatsRow from './StatsRow.svelte';

	let { facets, ontagclick }: {
		facets: Facets;
		ontagclick: (tag: string) => void;
	} = $props();

	const topTags = $derived(facets.tags.slice().sort((a, b) => b.count - a.count).slice(0, 20));
	const topTopics = $derived(facets.topics.slice().sort((a, b) => b.count - a.count).slice(0, 15));
	const topEntities = $derived(facets.entities.slice().sort((a, b) => b.count - a.count).slice(0, 15));

	function chipSize(count: number, max: number): number {
		const min = 12;
		const maxS = 19;
		if (max <= 1) return min;
		return min + ((count - 1) / (max - 1)) * (maxS - min);
	}
</script>

<div class="dashboard">
	<div class="dashboard-header">
		<h2 class="dashboard-title">Discover</h2>
		<p class="dashboard-subtitle">Browse your indexed knowledge by facet</p>
	</div>

	<StatsRow {facets} />

	{#if topTags.length > 0}
		<div class="facet-section">
			<div class="facet-section-header">
				<span class="facet-label tag-label">Tags</span>
				<span class="facet-count">{facets.tags.length} total</span>
			</div>
			<div class="facet-cloud">
				{#each topTags as entry}
					{@const maxCount = topTags[0]?.count ?? 1}
					<button
						class="cloud-chip tag-chip"
						style:font-size="{chipSize(entry.count, maxCount)}px"
						onclick={() => ontagclick(entry.name)}
					>
						#{entry.name}
						<span class="chip-count">{entry.count}</span>
					</button>
				{/each}
			</div>
		</div>
	{/if}

	{#if topTopics.length > 0}
		<div class="facet-section">
			<div class="facet-section-header">
				<span class="facet-label topic-label">Topics</span>
				<span class="facet-count">{facets.topics.length} total</span>
			</div>
			<div class="facet-cloud">
				{#each topTopics as entry}
					{@const maxCount = topTopics[0]?.count ?? 1}
					<button
						class="cloud-chip topic-chip"
						style:font-size="{chipSize(entry.count, maxCount)}px"
						onclick={() => ontagclick(entry.name)}
					>
						{entry.name}
						<span class="chip-count">{entry.count}</span>
					</button>
				{/each}
			</div>
		</div>
	{/if}

	{#if topEntities.length > 0}
		<div class="facet-section">
			<div class="facet-section-header">
				<span class="facet-label entity-label">Entities</span>
				<span class="facet-count">{facets.entities.length} total</span>
			</div>
			<div class="facet-cloud">
				{#each topEntities as entry}
					{@const maxCount = topEntities[0]?.count ?? 1}
					<button
						class="cloud-chip entity-chip"
						style:font-size="{chipSize(entry.count, maxCount)}px"
						onclick={() => ontagclick(entry.name)}
					>
						{entry.name}
						<span class="chip-count">{entry.count}</span>
					</button>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.dashboard {
		padding: 20px 0; max-width: 800px; margin: 0 auto;
		display: flex; flex-direction: column; gap: 28px;
		animation: fadeInUp 0.4s ease both;
	}

	.dashboard-header { display: flex; flex-direction: column; gap: 4px; }
	.dashboard-title {
		font-size: 22px; font-weight: 800; letter-spacing: -0.03em;
		background: var(--gradient-accent);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent;
		background-clip: text;
	}
	.dashboard-subtitle {
		font-size: 13px; color: var(--text-3); font-weight: 400;
	}

	.facet-section {
		display: flex; flex-direction: column; gap: 10px;
	}
	.facet-section-header {
		display: flex; align-items: center; gap: 10px;
	}
	.facet-label {
		font-size: 10px; font-weight: 700; text-transform: uppercase;
		letter-spacing: 0.08em; padding: 2px 8px; border-radius: 4px;
	}
	.tag-label { background: var(--tag-bg); color: var(--tag-text); }
	.topic-label { background: var(--topic-bg); color: var(--topic-text); }
	.entity-label { background: var(--entity-bg); color: var(--entity-text); }
	.facet-count { font-size: 11px; color: var(--text-3); }

	.facet-cloud {
		display: flex; flex-wrap: wrap; gap: 6px; align-items: baseline;
	}
	.cloud-chip {
		display: inline-flex; align-items: center; gap: 4px;
		border-radius: 6px; padding: 4px 10px;
		cursor: pointer; font-weight: 500; border: 1px solid;
		font-family: var(--font);
		transition: transform var(--duration-fast), background var(--duration-fast), box-shadow var(--duration-fast);
	}
	.cloud-chip:hover {
		transform: scale(1.04);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}
	.cloud-chip:active { transform: scale(0.97); }
	.chip-count { opacity: 0.45; font-size: 0.75em; font-weight: 400; }

	.tag-chip {
		background: var(--tag-bg); border-color: var(--tag-border); color: var(--tag-text);
	}
	.tag-chip:hover { background: rgba(99, 102, 241, 0.18); }
	.topic-chip {
		background: var(--topic-bg); border-color: var(--topic-border); color: var(--topic-text);
	}
	.topic-chip:hover { background: rgba(56, 189, 248, 0.15); }
	.entity-chip {
		background: var(--entity-bg); border-color: var(--entity-border); color: var(--entity-text);
	}
	.entity-chip:hover { background: rgba(251, 191, 36, 0.15); }
</style>
