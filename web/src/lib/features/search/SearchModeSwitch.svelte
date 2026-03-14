<script lang="ts">
	import type { SearchMode } from '$lib/api/types';

	let {
		mode,
		onchange
	}: {
		mode: SearchMode;
		onchange: (mode: SearchMode) => void;
	} = $props();

	const modes: { id: SearchMode; label: string; desc: string }[] = [
		{ id: 'hybrid', label: 'Hybrid', desc: 'keyword + semantic fusion' },
		{ id: 'keyword', label: 'Keyword', desc: 'exact text matching' },
		{ id: 'semantic', label: 'Semantic', desc: 'meaning-based search' },
	];

	const activeIndex = $derived(modes.findIndex((m) => m.id === mode));
	const activeDesc = $derived(modes.find((m) => m.id === mode)?.desc ?? '');
</script>

<div class="mode-wrap">
	<div class="mode-switch" data-mode={mode}>
		<div
			class="mode-indicator"
			style:transform="translateX({activeIndex * 100}%)"
		></div>
		{#each modes as m}
			<button
				class="mode-btn"
				class:active={mode === m.id}
				onclick={() => onchange(m.id)}
				aria-pressed={mode === m.id}
			>
				<span class="mode-dot mode-dot--{m.id}"></span>
				{m.label}
			</button>
		{/each}
	</div>
	<div class="mode-desc" data-mode={mode}>{activeDesc}</div>
</div>

<style>
	.mode-wrap {
		display: flex;
		flex-direction: column;
		gap: 6px;
		animation: fadeInUp 0.3s ease both;
		animation-delay: 50ms;
	}

	.mode-switch {
		position: relative;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 0;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: var(--r);
		padding: 3px;
		overflow: hidden;
	}

	.mode-indicator {
		position: absolute;
		top: 3px;
		left: 3px;
		width: calc(33.333% - 2px);
		height: calc(100% - 6px);
		border-radius: var(--r-sm);
		transition: transform 0.25s var(--ease-out), background 0.2s, box-shadow 0.2s;
		z-index: 0;
		pointer-events: none;
	}

	/* Mode-specific indicator colors */
	.mode-switch[data-mode='hybrid'] .mode-indicator {
		background: var(--mode-hybrid-bg);
		border: 1px solid var(--mode-hybrid-border);
		box-shadow: 0 0 12px rgba(16, 185, 129, 0.10);
	}
	.mode-switch[data-mode='keyword'] .mode-indicator {
		background: var(--mode-keyword-bg);
		border: 1px solid var(--mode-keyword-border);
		box-shadow: 0 0 16px rgba(34, 211, 238, 0.10);
	}
	.mode-switch[data-mode='semantic'] .mode-indicator {
		background: var(--mode-semantic-bg);
		border: 1px solid var(--mode-semantic-border);
		box-shadow: 0 0 16px rgba(232, 121, 249, 0.10);
	}

	.mode-btn {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 8px 12px;
		background: none;
		border: none;
		border-radius: var(--r-sm);
		color: var(--text-3);
		font-size: 11px;
		font-weight: 600;
		cursor: pointer;
		transition: color 0.15s;
		letter-spacing: 0.03em;
		text-transform: uppercase;
		white-space: nowrap;
	}
	.mode-btn:hover { color: var(--text-2); }

	.mode-btn.active { color: var(--text); }
	.mode-switch[data-mode='hybrid'] .mode-btn.active { color: var(--mode-hybrid); }
	.mode-switch[data-mode='keyword'] .mode-btn.active { color: var(--mode-keyword); }
	.mode-switch[data-mode='semantic'] .mode-btn.active { color: var(--mode-semantic); }

	.mode-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: currentColor;
		opacity: 0.4;
		transition: opacity 0.2s;
	}
	.mode-btn.active .mode-dot { opacity: 1; }
	.mode-dot--hybrid { color: var(--mode-hybrid); }
	.mode-dot--keyword { color: var(--mode-keyword); }
	.mode-dot--semantic { color: var(--mode-semantic); }

	.mode-desc {
		font-size: 11px;
		color: var(--text-3);
		text-align: center;
		transition: color 0.3s;
		letter-spacing: 0.02em;
	}
	.mode-desc[data-mode='hybrid'] { color: rgba(16, 185, 129, 0.5); }
	.mode-desc[data-mode='keyword'] { color: rgba(34, 211, 238, 0.45); }
	.mode-desc[data-mode='semantic'] { color: rgba(232, 121, 249, 0.45); }
</style>
