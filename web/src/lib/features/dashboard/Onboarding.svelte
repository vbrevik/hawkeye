<script lang="ts">
	import FileBrowser from '$lib/features/browse/FileBrowser.svelte';

	let { onfacetsrefresh }: { onfacetsrefresh?: () => void } = $props();
</script>

<div class="onboarding">
	<div class="onboarding-hero">
		<div class="hero-icon">
			<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor"
				stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
				<circle cx="12" cy="12" r="3"/>
				<path d="M2 12s4-8 10-8 10 8 10 8-4 8-10 8-10-8-10-8z"/>
			</svg>
		</div>
		<h2 class="hero-title">Welcome to Hawkeye</h2>
		<p class="hero-subtitle">Index your markdown files. Discover insights.</p>
	</div>

	<div class="steps">
		<div class="step">
			<span class="step-num">1</span>
			<div class="step-content">
				<div class="step-title">Browse to a folder</div>
				<div class="step-desc">Navigate to a directory containing .md files</div>
			</div>
		</div>
		<div class="step">
			<span class="step-num">2</span>
			<div class="step-content">
				<div class="step-title">Ingest markdown files</div>
				<div class="step-desc">AI extracts summaries, tags, topics, and entities</div>
			</div>
		</div>
		<div class="step">
			<span class="step-num">3</span>
			<div class="step-content">
				<div class="step-title">Search and discover</div>
				<div class="step-desc">Full-text search, faceted browsing, knowledge graph</div>
			</div>
		</div>
	</div>

	<div class="onboarding-browser">
		<div class="browser-label">Select a directory to get started</div>
		<FileBrowser {onfacetsrefresh} />
	</div>
</div>

<style>
	.onboarding {
		display: flex; flex-direction: column; align-items: center;
		gap: 32px; padding: 48px 20px; max-width: 520px;
		margin: 0 auto; animation: fadeInUp 0.4s ease both;
	}

	.onboarding-hero {
		display: flex; flex-direction: column; align-items: center;
		gap: 12px; text-align: center;
	}
	.hero-icon {
		width: 56px; height: 56px; border-radius: 16px;
		background: var(--gradient-accent); display: flex;
		align-items: center; justify-content: center; color: #fff;
	}
	.hero-title {
		font-size: 22px; font-weight: 800; letter-spacing: -0.03em;
		color: var(--text);
	}
	.hero-subtitle {
		font-size: 14px; color: var(--text-3);
	}

	.steps {
		display: flex; flex-direction: column; gap: 16px; width: 100%;
	}
	.step {
		display: flex; align-items: flex-start; gap: 14px;
		padding: 14px 16px; border-radius: var(--r);
		background: var(--surface); border: 1px solid var(--border-subtle);
		transition: border-color var(--duration-fast);
	}
	.step:hover { border-color: var(--border); }
	.step-num {
		width: 26px; height: 26px; border-radius: 8px;
		background: var(--gradient-accent); display: flex;
		align-items: center; justify-content: center;
		font-size: 12px; font-weight: 700; color: #fff; flex-shrink: 0;
	}
	.step-content { display: flex; flex-direction: column; gap: 2px; }
	.step-title { font-size: 14px; font-weight: 600; color: var(--text); }
	.step-desc { font-size: 12px; color: var(--text-3); line-height: 1.5; }

	.onboarding-browser {
		width: 100%; background: var(--surface); border: 1px solid var(--border);
		border-radius: var(--r); padding: 18px;
	}
	.browser-label {
		font-size: 12px; font-weight: 600; color: var(--text-2);
		margin-bottom: 10px;
	}
</style>
