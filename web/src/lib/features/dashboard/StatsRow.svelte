<script lang="ts">
	import type { Facets } from '$lib/api/types';

	let { facets }: { facets: Facets } = $props();

	const stats = $derived([
		{ label: 'Docs', value: facets.document_count, icon: '▤', color: 'var(--green)', glow: 'rgba(52, 211, 153, 0.12)' },
		{ label: 'Tags', value: facets.tags.length, icon: '#', color: 'var(--tag-text)', glow: 'rgba(165, 180, 252, 0.12)' },
		{ label: 'Topics', value: facets.topics.length, icon: '◆', color: 'var(--topic-text)', glow: 'rgba(56, 189, 248, 0.12)' },
		{ label: 'Entities', value: facets.entities.length, icon: '●', color: 'var(--entity-text)', glow: 'rgba(251, 191, 36, 0.12)' },
	]);

	const total = $derived(facets.tags.length + facets.topics.length + facets.entities.length);
</script>

<div class="stats-row">
	{#each stats as stat, i}
		<div class="stat-card" style:animation-delay="{i * 60}ms" style:--stat-glow={stat.glow}>
			<div class="stat-value" style:color={stat.color}>{stat.value}</div>
			<div class="stat-meta">
				<span class="stat-icon" style:color={stat.color}>{stat.icon}</span>
				<span class="stat-label">{stat.label}</span>
			</div>
		</div>
	{/each}
	<div class="stat-card stat-total" style:animation-delay="180ms">
		<div class="stat-value" style:color="var(--accent)">{total}</div>
		<div class="stat-meta">
			<span class="stat-icon" style:color="var(--accent)">Σ</span>
			<span class="stat-label">Total</span>
		</div>
	</div>
</div>

<style>
	.stats-row { display: flex; gap: 8px; }
	.stat-card {
		flex: 1;
		background: var(--surface);
		border: 1px solid var(--border-subtle);
		border-radius: var(--r); padding: 14px 12px;
		text-align: center;
		animation: scaleIn 0.25s ease both;
		transition: border-color 100ms ease, box-shadow 100ms ease;
		display: flex; flex-direction: column; gap: 4px;
		position: relative; overflow: hidden;
	}
	.stat-card::before {
		content: ''; position: absolute; inset: 0;
		background: radial-gradient(circle at 50% 0%, var(--stat-glow, transparent), transparent 70%);
		opacity: 0; transition: opacity 300ms ease;
		pointer-events: none;
	}
	.stat-card:hover::before { opacity: 1; }
	.stat-card:hover {
		border-color: var(--border);
	}
	.stat-total {
		--stat-glow: rgba(16, 185, 129, 0.12);
		border-style: dashed;
	}
	.stat-value {
		font-family: var(--mono);
		font-size: 26px; font-weight: 700;
		line-height: 1; letter-spacing: -0.02em;
	}
	.stat-meta {
		display: flex; align-items: center; justify-content: center; gap: 4px;
	}
	.stat-icon {
		font-size: 9px; opacity: 0.7;
	}
	.stat-label {
		font-size: 10px; text-transform: uppercase; color: var(--text-3);
		letter-spacing: 0.08em; font-weight: 600;
	}
</style>
