<script lang="ts">
	import { mlxStatus } from '$lib/stores/status';
</script>

<div class="mlx-block" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}>
	<div class="mlx-row">
		<div class="dot" class:online={$mlxStatus.online} class:warn={!$mlxStatus.online}></div>
		<span class="mlx-label">{$mlxStatus.online ? 'Online' : $mlxStatus.message}</span>
	</div>
	<div class="mlx-model">{$mlxStatus.model ?? '—'}</div>
</div>

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
</style>
