<script lang="ts">
	import { mlxStatus } from '$lib/stores/status';

	let { compact = false }: { compact?: boolean } = $props();
</script>

{#if compact}
	<div class="mlx-inline" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}>
		<div class="dot" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}></div>
		<span class="mlx-inline-label">{$mlxStatus.online ? 'Online' : $mlxStatus.message}</span>
	</div>
{:else}
	<div class="mlx-block" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}>
		<div class="mlx-row">
			<div class="dot" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}></div>
			<span class="mlx-label">{$mlxStatus.online ? 'Online' : $mlxStatus.message}</span>
		</div>
		<div class="mlx-model">{$mlxStatus.model ?? '—'}</div>
	</div>
{/if}

<style>
	.mlx-block {
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r);
		padding: 12px 14px;
		transition: border-color 0.2s;
	}
	.mlx-block.online {
		border-color: rgba(52, 211, 153, 0.25);
		background: linear-gradient(135deg, var(--surface-2), rgba(52, 211, 153, 0.04));
	}
	.mlx-block.warn {
		border-color: rgba(251, 191, 36, 0.25);
		background: linear-gradient(135deg, var(--surface-2), rgba(251, 191, 36, 0.04));
	}
	.mlx-row { display: flex; align-items: center; gap: 8px; margin-bottom: 4px; }
	.dot {
		width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0;
		background: var(--text-3); transition: background 0.3s;
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
	.mlx-label { font-size: 12px; font-weight: 600; color: var(--text); }
	.mlx-model {
		font-size: 10px; color: var(--text-3); font-family: var(--mono);
		line-height: 1.5; word-break: break-all;
	}

	/* Compact inline variant */
	.mlx-inline {
		display: flex; align-items: center; gap: 6px;
		padding: 6px 10px; border-radius: var(--r-sm);
		background: var(--surface-2); border: 1px solid var(--border);
		transition: border-color 0.2s;
	}
	.mlx-inline.online { border-color: rgba(52, 211, 153, 0.2); }
	.mlx-inline.warn { border-color: rgba(251, 191, 36, 0.2); }
	.mlx-inline-label { font-size: 11px; font-weight: 600; color: var(--text-2); }
</style>
