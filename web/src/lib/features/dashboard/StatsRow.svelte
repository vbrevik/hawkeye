<script lang="ts">
	import type { Facets } from '$lib/api/types';

	let { facets }: { facets: Facets } = $props();

	const stats = $derived([
		{ label: 'Tags', value: facets.tags.length, color: 'var(--tag-text)' },
		{ label: 'Topics', value: facets.topics.length, color: 'var(--topic-text)' },
		{ label: 'Entities', value: facets.entities.length, color: 'var(--entity-text)' },
	]);
</script>

<div class="stats-row">
	{#each stats as stat, i}
		<div class="stat-card" style:animation-delay="{i * 60}ms">
			<div class="stat-value" style:color={stat.color}>{stat.value}</div>
			<div class="stat-label">{stat.label}</div>
		</div>
	{/each}
</div>

<style>
	.stats-row { display: flex; gap: 8px; }
	.stat-card {
		flex: 1; background: var(--surface); border: 1px solid var(--border);
		border-radius: var(--r); padding: 16px 12px; text-align: center;
		animation: scaleIn 0.3s ease both;
		transition: border-color var(--duration-fast), transform var(--duration-fast);
	}
	.stat-card:hover {
		border-color: var(--border); transform: translateY(-1px);
	}
	.stat-value {
		font-size: 28px; font-weight: 800; font-family: var(--mono);
		line-height: 1; margin-bottom: 4px; letter-spacing: -0.03em;
	}
	.stat-label {
		font-size: 10px; text-transform: uppercase; color: var(--text-3);
		letter-spacing: 0.08em; font-weight: 600;
	}
</style>
