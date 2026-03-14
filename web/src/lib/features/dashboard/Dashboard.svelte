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

	function chipWeight(count: number, max: number): number {
		if (max <= 1) return 0;
		return (count - 1) / (max - 1);
	}
</script>

<div class="dashboard">
	<header class="dashboard-header">
		<div class="header-accent"></div>
		<h2 class="dashboard-title">Discover</h2>
		<p class="dashboard-subtitle">Browse your indexed knowledge by facet</p>
	</header>

	<StatsRow {facets} />

	{#if topTags.length > 0}
		<section class="facet-section" style:animation-delay="0.1s">
			<div class="facet-section-header">
				<span class="facet-icon tag-icon">#</span>
				<span class="facet-label">Tags</span>
				<span class="facet-rule"></span>
				<span class="facet-count">{facets.tags.length}</span>
			</div>
			<div class="facet-cloud">
				{#each topTags as entry, i}
					{@const maxCount = topTags[0]?.count ?? 1}
					{@const weight = chipWeight(entry.count, maxCount)}
					<button
						class="cloud-chip tag-chip"
						class:prominent={weight > 0.6}
						style:animation-delay="{i * 25}ms"
						style:--chip-weight="{weight}"
						onclick={() => ontagclick(entry.name)}
					>
						<span class="chip-name">{entry.name}</span>
						<span class="chip-count">{entry.count}</span>
					</button>
				{/each}
			</div>
		</section>
	{/if}

	<div class="facet-grid">
		{#if topTopics.length > 0}
			<section class="facet-section" style:animation-delay="0.2s">
				<div class="facet-section-header">
					<span class="facet-icon topic-icon">◆</span>
					<span class="facet-label">Topics</span>
					<span class="facet-rule"></span>
					<span class="facet-count">{facets.topics.length}</span>
				</div>
				<div class="facet-cloud">
					{#each topTopics as entry, i}
						{@const maxCount = topTopics[0]?.count ?? 1}
						{@const weight = chipWeight(entry.count, maxCount)}
						<button
							class="cloud-chip topic-chip"
							class:prominent={weight > 0.6}
							style:animation-delay="{i * 25}ms"
							style:--chip-weight="{weight}"
							onclick={() => ontagclick(entry.name)}
						>
							<span class="chip-name">{entry.name}</span>
							<span class="chip-count">{entry.count}</span>
						</button>
					{/each}
				</div>
			</section>
		{/if}

		{#if topEntities.length > 0}
			<section class="facet-section" style:animation-delay="0.3s">
				<div class="facet-section-header">
					<span class="facet-icon entity-icon">●</span>
					<span class="facet-label">Entities</span>
					<span class="facet-rule"></span>
					<span class="facet-count">{facets.entities.length}</span>
				</div>
				<div class="facet-cloud">
					{#each topEntities as entry, i}
						{@const maxCount = topEntities[0]?.count ?? 1}
						{@const weight = chipWeight(entry.count, maxCount)}
						<button
							class="cloud-chip entity-chip"
							class:prominent={weight > 0.6}
							style:animation-delay="{i * 25}ms"
							style:--chip-weight="{weight}"
							onclick={() => ontagclick(entry.name)}
						>
							<span class="chip-name">{entry.name}</span>
							<span class="chip-count">{entry.count}</span>
						</button>
					{/each}
				</div>
			</section>
		{/if}
	</div>
</div>

<style>
	.dashboard {
		padding: 24px 0 40px; max-width: 860px; margin: 0 auto;
		display: flex; flex-direction: column; gap: 32px;
	}

	/* ── Header ── */
	.dashboard-header {
		display: flex; flex-direction: column; gap: 6px;
		position: relative; padding-left: 18px;
	}
	.header-accent {
		position: absolute; left: 0; top: 4px; bottom: 4px; width: 3px;
		background: var(--accent); border-radius: 0;
	}
	.dashboard-title {
		font-size: 20px; font-weight: 800; text-transform: uppercase;
		color: var(--text); letter-spacing: 0.06em; line-height: 1.1;
		animation: fadeInUp 0.3s ease both;
	}
	.dashboard-subtitle {
		font-size: 13px; color: var(--text-3); font-weight: 400;
		letter-spacing: 0.01em;
		animation: fadeInUp 0.3s ease 0.05s both;
	}

	/* ── Facet Sections ── */
	.facet-section {
		display: flex; flex-direction: column; gap: 14px;
		animation: fadeInUp 0.3s ease both;
	}
	.facet-grid {
		display: grid; grid-template-columns: 1fr 1fr; gap: 24px;
	}
	@media (max-width: 720px) {
		.facet-grid { grid-template-columns: 1fr; }
	}

	/* ── Section Header ── */
	.facet-section-header {
		display: flex; align-items: center; gap: 8px;
	}
	.facet-icon {
		font-size: 10px; width: 20px; height: 20px;
		display: flex; align-items: center; justify-content: center;
		border-radius: var(--r-sm); font-weight: 700;
	}
	.tag-icon { background: var(--tag-bg); color: var(--tag-text); }
	.topic-icon { background: var(--topic-bg); color: var(--topic-text); font-size: 7px; }
	.entity-icon { background: var(--entity-bg); color: var(--entity-text); font-size: 7px; }
	.facet-label {
		font-size: 11px; font-weight: 600; text-transform: uppercase;
		letter-spacing: 0.08em; color: var(--text-2);
	}
	.facet-rule {
		flex: 1; height: 1px; background: var(--border-subtle);
	}
	.facet-count {
		font-size: 11px; color: var(--text-3); font-family: var(--mono);
		font-weight: 400;
	}

	/* ── Chip Cloud ── */
	.facet-cloud {
		display: flex; flex-wrap: wrap; gap: 6px; align-items: center;
	}

	.cloud-chip {
		display: inline-flex; align-items: center; gap: 5px;
		border-radius: var(--r);
		padding: 4px 10px;
		cursor: pointer; border: 1px solid;
		font-family: var(--font);
		font-size: calc(11.5px + var(--chip-weight, 0) * 3px);
		font-weight: calc(450 + var(--chip-weight, 0) * 200);
		line-height: 1.4;
		transition: transform 150ms ease, background 150ms ease, box-shadow 150ms ease, border-color 150ms ease;
		animation: chipReveal 0.3s ease both;
		position: relative;
	}
	.cloud-chip:hover {
		transform: translateY(-1px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25);
	}
	.cloud-chip:active { transform: translateY(0) scale(0.97); }

	.cloud-chip.prominent {
		padding: 6px 14px;
	}

	.chip-name { position: relative; z-index: 1; }
	.chip-count {
		opacity: 0.4; font-size: 0.75em; font-weight: 400;
		font-family: var(--mono); position: relative; z-index: 1;
	}

	/* Tag chips */
	.tag-chip {
		background: var(--tag-bg); border-color: var(--tag-border); color: var(--tag-text);
	}
	.tag-chip:hover {
		background: rgba(165, 180, 252, 0.16); border-color: rgba(165, 180, 252, 0.35);
	}
	.tag-chip.prominent { border-color: rgba(165, 180, 252, 0.30); }

	/* Topic chips */
	.topic-chip {
		background: var(--topic-bg); border-color: var(--topic-border); color: var(--topic-text);
	}
	.topic-chip:hover {
		background: rgba(56, 189, 248, 0.16); border-color: rgba(56, 189, 248, 0.35);
	}
	.topic-chip.prominent { border-color: rgba(56, 189, 248, 0.30); }

	/* Entity chips */
	.entity-chip {
		background: var(--entity-bg); border-color: var(--entity-border); color: var(--entity-text);
	}
	.entity-chip:hover {
		background: rgba(251, 191, 36, 0.16); border-color: rgba(251, 191, 36, 0.35);
	}
	.entity-chip.prominent { border-color: rgba(251, 191, 36, 0.30); }

	@keyframes chipReveal {
		from { opacity: 0; transform: scale(0.92) translateY(4px); }
		to { opacity: 1; transform: scale(1) translateY(0); }
	}
</style>
