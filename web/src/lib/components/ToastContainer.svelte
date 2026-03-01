<script lang="ts">
	import { toasts, removeToast } from '$lib/stores/toast';

	const icons: Record<string, string> = { success: '✔', error: '✘', info: 'ℹ' };
</script>

<div class="toast-container">
	{#each $toasts as toast (toast.id)}
		<button class="toast toast--{toast.type}" onclick={() => removeToast(toast.id)}>
			<span class="toast-icon">{icons[toast.type]}</span>
			<span>{toast.message}</span>
			<div class="toast-progress"></div>
		</button>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		bottom: 20px;
		right: 20px;
		z-index: 100;
		display: flex;
		flex-direction: column-reverse;
		gap: 8px;
		pointer-events: none;
	}

	.toast {
		pointer-events: auto;
		background: var(--surface-2);
		border: 1px solid var(--border);
		border-radius: var(--r);
		padding: 10px 16px 12px;
		font-size: 13px;
		color: var(--text-2);
		line-height: 1.4;
		display: flex;
		align-items: center;
		gap: 10px;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		animation: toastIn 0.3s ease both;
		max-width: 340px;
		cursor: pointer;
		position: relative;
		overflow: hidden;
		text-align: left;
	}

	.toast-icon { flex-shrink: 0; font-size: 14px; }

	.toast--success { border-color: rgba(52, 211, 153, 0.3); }
	.toast--success .toast-icon { color: var(--green); }
	.toast--error { border-color: rgba(248, 113, 113, 0.3); }
	.toast--error .toast-icon { color: var(--red); }
	.toast--info { border-color: rgba(99, 102, 241, 0.3); }
	.toast--info .toast-icon { color: var(--accent-hover); }

	.toast-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		height: 2px;
		border-radius: 0 0 var(--r) var(--r);
		animation: toastCountdown 4s linear forwards;
	}
	.toast--success .toast-progress { background: var(--green); opacity: 0.5; }
	.toast--error .toast-progress { background: var(--red); opacity: 0.5; }
	.toast--info .toast-progress { background: var(--accent-hover); opacity: 0.5; }

	@keyframes toastIn {
		from { opacity: 0; transform: translateY(12px) scale(0.96); }
		to { opacity: 1; transform: translateY(0) scale(1); }
	}
	@keyframes toastCountdown {
		from { width: 100%; }
		to { width: 0%; }
	}
</style>
