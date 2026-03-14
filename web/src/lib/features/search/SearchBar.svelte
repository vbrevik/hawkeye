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

	function clearInput() {
		value = '';
		onsubmit();
		inputEl?.focus();
	}

	export function focus() {
		inputEl?.focus();
		inputEl?.select();
	}
</script>

<div class="search-bar">
	<svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
		<circle cx="11" cy="11" r="8"/>
		<path d="m21 21-4.35-4.35"/>
	</svg>
	<input
		class="search-input"
		bind:this={inputEl}
		bind:value
		placeholder="Search your knowledge…"
		aria-label="Search your knowledge"
		autocomplete="off"
		spellcheck="false"
		{onkeydown}
		oninput={() => onsubmit()}
	/>
	{#if value}
		<button class="search-clear" onclick={clearInput} aria-label="Clear search">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
				stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
			</svg>
		</button>
	{/if}
	<span class="search-kbd">⌘K</span>
</div>

<style>
	.search-bar {
		display: flex; align-items: center; gap: 12px;
		background: var(--surface); border: 1px solid var(--border);
		border-radius: var(--r); padding: 12px 16px;
		transition: border-color 0.25s, box-shadow 0.4s, background 0.25s;
	}
	.search-bar:focus-within {
		border-color: var(--accent);
		box-shadow: 0 0 0 2px var(--accent-dim);
		background: var(--surface-2);
	}
	.search-icon { color: var(--text-3); flex-shrink: 0; transition: color 0.25s; }
	.search-bar:focus-within .search-icon { color: var(--accent); }
	.search-input {
		flex: 1; background: none; border: none; color: var(--text);
		font-size: 16px; font-weight: 400; outline: none;
		letter-spacing: -0.01em;
	}
	.search-input::placeholder { color: var(--text-3); font-weight: 400; }
	.search-clear {
		background: none; border: none; color: var(--text-3); cursor: pointer;
		display: flex; align-items: center; justify-content: center;
		padding: 4px;		border-radius: var(--r-sm); flex-shrink: 0;
		transition: color var(--duration-fast), background var(--duration-fast);
	}
	.search-clear:hover { color: var(--text); background: var(--surface-2); }
	.search-kbd {
		background: var(--surface-2); border: 1px solid var(--border);
		border-radius: var(--r-sm); color: var(--text-3); font-size: 10px;
		font-family: var(--mono); padding: 3px 7px; flex-shrink: 0;
		font-weight: 500; letter-spacing: 0.02em;
	}
</style>
