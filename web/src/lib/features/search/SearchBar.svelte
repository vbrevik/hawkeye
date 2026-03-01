<script lang="ts">
	let { value = $bindable(''), onsubmit }: { value: string; onsubmit: () => void } = $props();
	let inputEl: HTMLInputElement;

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (value) {
				value = '';
				onsubmit();
			}
			e.stopPropagation();
		}
	}

	export function focus() {
		inputEl?.focus();
		inputEl?.select();
	}
</script>

<div class="search-bar" class:focused={false}>
	<svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
		<circle cx="11" cy="11" r="8"/>
		<path d="m21 21-4.35-4.35"/>
	</svg>
	<input
		class="search-input"
		bind:this={inputEl}
		bind:value
		placeholder="Search summaries…"
		aria-label="Search summaries"
		autocomplete="off"
		spellcheck="false"
		{onkeydown}
		oninput={() => onsubmit()}
	/>
	<span class="search-kbd">⌘K</span>
</div>

<style>
	.search-bar {
		display: flex; align-items: center; gap: 12px;
		background: var(--surface); border: 1px solid var(--border);
		border-radius: 14px; padding: 12px 18px;
		transition: border-color 0.2s, box-shadow 0.3s, background 0.2s;
	}
	.search-bar:focus-within {
		border-color: var(--accent);
		box-shadow: 0 0 0 3px var(--accent-dim), 0 8px 32px rgba(99, 102, 241, 0.12);
		background: var(--surface-2);
	}
	.search-icon { color: var(--text-3); font-size: 16px; flex-shrink: 0; transition: color 0.2s; }
	.search-bar:focus-within .search-icon { color: var(--accent); }
	.search-input {
		flex: 1; background: none; border: none; color: var(--text);
		font-size: 16px; font-weight: 400; outline: none;
	}
	.search-input::placeholder { color: var(--text-3); font-weight: 400; }
	.search-kbd {
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: 5px; color: var(--text-3); font-size: 10px;
		font-family: var(--mono); padding: 3px 7px; flex-shrink: 0;
		font-weight: 500; letter-spacing: 0.02em;
	}
</style>
