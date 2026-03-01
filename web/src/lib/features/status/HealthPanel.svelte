<script lang="ts">
	import { healthData, healthSummary } from '$lib/stores/health';

	let checkedAgo = $state('');
	let ageInterval: ReturnType<typeof setInterval> | null = null;

	function updateAge() {
		const data = $healthData;
		if (!data) { checkedAgo = ''; return; }
		const secs = Math.round((Date.now() - new Date(data.checked_at).getTime()) / 1000);
		checkedAgo = secs < 3 ? 'just now' : `${secs}s ago`;
	}

	$effect(() => {
		updateAge();
		ageInterval = setInterval(updateAge, 1000);
		return () => { if (ageInterval) clearInterval(ageInterval); };
	});

	const serviceIcons: Record<string, string> = {
		redis: '⚡',
		postgres: '🐘',
		etcd: '🔑',
		minio: '📦',
		milvus: '🧭',
		neo4j: '🕸️',
		qwen3: '🧠'
	};
</script>

<div class="health-panel">
	{#if $healthData}
		<div class="svc-list">
			{#each $healthData.services as svc, i}
				<div class="svc-row" style:animation-delay="{i * 40}ms">
					<div
						class="dot"
						class:online={svc.status === 'up'}
						class:warn={svc.status === 'degraded'}
						class:down={svc.status === 'down'}
					></div>
					<span class="svc-icon">{serviceIcons[svc.name] ?? '●'}</span>
					<span class="svc-name" class:dimmed={svc.status === 'down'}>{svc.name}</span>
					<span
						class="svc-badge"
						class:badge-up={svc.status === 'up'}
						class:badge-degraded={svc.status === 'degraded'}
						class:badge-down={svc.status === 'down'}
					>{svc.status}</span>
					<span class="svc-latency">
						{svc.latency_ms != null ? `${svc.latency_ms}ms` : '—'}
					</span>
				</div>
			{/each}
		</div>

		<div class="health-footer">
			<span class="health-agg">
				{$healthSummary.up}/{$healthSummary.total} up
			</span>
			{#if checkedAgo}
				<span class="health-age">checked {checkedAgo}</span>
			{/if}
		</div>
	{:else}
		<div class="health-loading">
			{#each Array(4) as _, i}
				<div class="svc-skeleton" style:animation-delay="{i * 80}ms"></div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.health-panel {
		display: flex; flex-direction: column; gap: 2px;
	}

	.svc-list {
		display: flex; flex-direction: column; gap: 1px;
	}

	.svc-row {
		display: flex; align-items: center; gap: 6px;
		padding: 5px 8px; border-radius: var(--r-sm);
		transition: background var(--duration-fast);
		animation: fadeInUp 0.25s ease both;
	}
	.svc-row:hover { background: var(--surface-2); }

	.dot {
		width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0;
		background: var(--text-3); transition: background 0.3s, box-shadow 0.3s;
	}
	.dot.online {
		background: var(--green);
		box-shadow: 0 0 6px rgba(52, 211, 153, 0.45);
	}
	.dot.warn {
		background: var(--yellow);
		box-shadow: 0 0 6px rgba(251, 191, 36, 0.4);
	}
	.dot.down {
		background: var(--red);
		box-shadow: 0 0 6px rgba(248, 113, 113, 0.4);
	}

	.svc-icon {
		font-size: 10px; width: 14px; text-align: center; flex-shrink: 0;
		line-height: 1;
	}

	.svc-name {
		font-size: 11px; font-weight: 600; color: var(--text);
		flex: 1; min-width: 0;
	}
	.svc-name.dimmed { color: var(--text-3); }

	.svc-badge {
		font-size: 9px; font-weight: 600; text-transform: uppercase;
		letter-spacing: 0.04em; padding: 1px 5px; border-radius: 3px;
		line-height: 1.4;
	}
	.badge-up {
		background: rgba(52, 211, 153, 0.1); color: var(--green);
	}
	.badge-degraded {
		background: rgba(251, 191, 36, 0.1); color: var(--yellow);
	}
	.badge-down {
		background: rgba(248, 113, 113, 0.1); color: var(--red);
	}

	.svc-latency {
		font-size: 10px; color: var(--text-3); font-family: var(--mono);
		width: 32px; text-align: right; flex-shrink: 0;
	}

	.health-footer {
		display: flex; align-items: center; justify-content: space-between;
		padding: 5px 8px 0; margin-top: 2px;
		border-top: 1px solid var(--border-subtle);
	}
	.health-agg {
		font-size: 10px; font-weight: 600; color: var(--text-2);
	}
	.health-age {
		font-size: 9px; color: var(--text-3); font-style: italic;
	}

	.health-loading {
		display: flex; flex-direction: column; gap: 4px;
	}
	.svc-skeleton {
		height: 24px; border-radius: var(--r-sm);
		background: linear-gradient(90deg, var(--surface) 25%, var(--surface-2) 50%, var(--surface) 75%);
		background-size: 200% 100%;
		animation: skeleton-sweep 1.6s ease infinite;
	}
</style>
