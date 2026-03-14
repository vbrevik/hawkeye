<script lang="ts">
	import { queueStatus } from '$lib/stores/status';
</script>

<div class="stats-grid">
	<div class="stat-cell">
		<div class="val">{$queueStatus.total}</div>
		<div class="lbl">Total</div>
	</div>
	<div class="stat-cell">
		<div class="val">{$queueStatus.completed}</div>
		<div class="lbl">Done</div>
	</div>
	<div class="stat-cell prog" class:active={$queueStatus.in_progress > 0}>
		<div class="val">{$queueStatus.in_progress}</div>
		<div class="lbl">Active</div>
	</div>
	<div class="stat-cell" class:fail={$queueStatus.failed > 0}>
		<div class="val">{$queueStatus.failed}</div>
		<div class="lbl">Failed</div>
	</div>
</div>

{#if $queueStatus.total > 0}
	{@const pct = Math.round(($queueStatus.completed + $queueStatus.failed) / $queueStatus.total * 100)}
	<div class="progress-bar">
		<div class="progress-fill" style:width="{pct}%"></div>
	</div>
	<div class="progress-info">{$queueStatus.completed + $queueStatus.failed} / {$queueStatus.total} ({pct}%)</div>
{/if}

<style>
	.stats-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 6px; }
	.stat-cell {
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); padding: 10px 8px; text-align: center;
		transition: border-color 0.2s, background 0.2s;
	}
	.stat-cell:hover { border-color: var(--border); background: rgba(16, 185, 129, 0.03); }
	.val {
		font-family: var(--mono); font-size: 20px; font-weight: 700; color: var(--text);
		line-height: 1; margin-bottom: 2px;
	}
	.lbl {
		font-size: 9px; color: var(--text-3); text-transform: uppercase;
		letter-spacing: 0.08em; font-weight: 500;
	}
	.prog .val { color: var(--accent); }
	.prog.active {
		border-color: rgba(16, 185, 129, 0.25);
	}
	.fail .val { color: var(--red); }
	.fail { border-color: rgba(248, 113, 113, 0.15); }

	.progress-bar {
		background: var(--surface-2); border-radius: 2px; height: 3px;
		overflow: hidden; margin-top: 8px;
	}
	.progress-fill {
		height: 100%; border-radius: 2px;
		background: linear-gradient(90deg, var(--accent), var(--accent-hover), var(--accent));
		background-size: 200% 100%;
		animation: progress-shimmer 2s ease infinite;
		transition: width 0.4s ease;
	}
	@keyframes progress-shimmer {
		0% { background-position: -200% 0; }
		100% { background-position: 200% 0; }
	}
	.progress-info {
		font-size: 11px; color: var(--text-2); margin-top: 6px; line-height: 1.4;
	}
</style>
