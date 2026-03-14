<script lang="ts">
	import { mlxStatus } from '$lib/stores/status';
	import { healthSummary } from '$lib/stores/health';
	import InferenceBlock from '$lib/features/status/InferenceBlock.svelte';
	import HealthPanel from '$lib/features/status/HealthPanel.svelte';

	let open = $state(false);
</script>

<div class="system-health-card" class:open>
	<button class="health-header" onclick={() => (open = !open)}>
		<div class="header-left">
			<div class="dot" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}></div>
			<span class="header-title">System Status</span>
		</div>
		<div class="header-right">
			{#if $healthSummary.total > 0}
				<span
					class="health-badge"
					class:all-up={$healthSummary.worst === 'up'}
					class:has-degraded={$healthSummary.worst === 'degraded'}
					class:has-down={$healthSummary.worst === 'down'}
				>
					{$healthSummary.up}/{$healthSummary.total}
				</span>
			{/if}
			<span class="toggle-chevron" class:open>
				<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<polyline points="6 9 12 15 18 9" />
				</svg>
			</span>
		</div>
	</button>

	{#if open}
		<div class="health-body">
			<InferenceBlock compact={false} />
			<div class="health-divider"></div>
			<HealthPanel />
		</div>
	{/if}
</div>

<style>
	.system-health-card {
		background: var(--surface);
		border: 1px solid var(--border-subtle);
		border-radius: var(--r);
		overflow: hidden;
		transition: border-color var(--duration-fast), box-shadow var(--duration-fast);
	}
	.system-health-card:hover {
		border-color: var(--border);
	}
	.system-health-card.open {
		border-color: var(--border);
		background: var(--surface-2);
	}

	.health-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 10px 12px;
		background: transparent;
		border: none;
		cursor: pointer;
		font-family: inherit;
		color: var(--text-2);
	}
	.health-header:hover {
		color: var(--text);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.header-title {
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
	}

	.dot {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		flex-shrink: 0;
		background: var(--text-3);
		transition: background 0.3s;
	}
	.dot.online {
		background: var(--green);
		box-shadow: 0 0 8px rgba(52, 211, 153, 0.5);
		animation: pulse 2.5s ease-in-out infinite;
	}
	.dot.warn {
		background: var(--yellow);
		box-shadow: 0 0 8px rgba(251, 191, 36, 0.4);
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.health-badge {
		font-size: 10px;
		font-weight: 700;
		border-radius: var(--r-sm);
		padding: 1px 6px;
		font-family: var(--mono);
		transition: background 0.3s, color 0.3s;
	}
	.health-badge.all-up {
		color: var(--green);
		background: rgba(52, 211, 153, 0.1);
	}
	.health-badge.has-degraded {
		color: var(--yellow);
		background: rgba(251, 191, 36, 0.1);
		animation: pulse 2.5s ease-in-out infinite;
	}
	.health-badge.has-down {
		color: var(--red);
		background: rgba(248, 113, 113, 0.1);
		animation: pulse 2.5s ease-in-out infinite;
	}

	.toggle-chevron {
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-fast);
		opacity: 0.5;
	}
	.toggle-chevron.open {
		transform: rotate(180deg);
	}

	.health-body {
		padding: 0 12px 12px 12px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		animation: fadeInUp 0.2s ease both;
	}

	.health-divider {
		height: 1px;
		background: var(--border-subtle);
		width: 100%;
	}
</style>
