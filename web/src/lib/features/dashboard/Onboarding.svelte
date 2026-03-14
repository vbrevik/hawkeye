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
		<div class="step" style:animation-delay="0.1s">
			<span class="step-num">1</span>
			<div class="step-content">
				<div class="step-title">Browse to a folder</div>
				<div class="step-desc">Navigate to a directory containing <code>.md</code> files</div>
			</div>
		</div>
		<div class="step" style:animation-delay="0.15s">
			<span class="step-num">2</span>
			<div class="step-content">
				<div class="step-title">Ingest markdown files</div>
				<div class="step-desc">AI extracts summaries, tags, topics, and entities</div>
			</div>
		</div>
		<div class="step" style:animation-delay="0.2s">
			<span class="step-num">3</span>
			<div class="step-content">
				<div class="step-title">Search and discover</div>
				<div class="step-desc">Hybrid search, faceted browsing, knowledge graph</div>
			</div>
		</div>
	</div>

	<div class="onboarding-browser">
		<div class="browser-label">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
				stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z"/>
			</svg>
			Select a directory to get started
		</div>
		<FileBrowser {onfacetsrefresh} />
	</div>
</div>

<style>
	.onboarding {
		display: flex; flex-direction: column; align-items: center;
		gap: 36px; padding: 52px 20px; max-width: 520px;
		margin: 0 auto; animation: fadeInUp 0.4s ease both;
	}

	/* ── Hero ── */
	.onboarding-hero {
		display: flex; flex-direction: column; align-items: center;
		gap: 14px; text-align: center;
	}
	.hero-icon {
		width: 48px; height: 48px; border-radius: var(--r);
		background: var(--gradient-accent); display: flex;
		align-items: center; justify-content: center; color: #fff;
	}
	.hero-title {
		font-size: 22px; font-weight: 800; text-transform: uppercase;
		color: var(--text); letter-spacing: 0.04em; line-height: 1.2;
	}
	.hero-subtitle {
		font-size: 14px; color: var(--text-3); line-height: 1.5;
	}

	/* ── Steps ── */
	.steps {
		display: flex; flex-direction: column; gap: 10px; width: 100%;
	}
	.step {
		display: flex; align-items: flex-start; gap: 14px;
		padding: 16px 18px; border-radius: var(--r);
		background: var(--surface); border: 1px solid var(--border-subtle);
		transition: border-color 200ms ease, transform 200ms ease, box-shadow 200ms ease;
		animation: fadeInUp 0.4s ease both;
	}
	.step:hover {
		border-color: var(--border);
		transform: translateY(-1px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}
	.step-num {
		width: 26px; height: 26px; border-radius: var(--r-sm);
		background: var(--accent-dim); border: 1px solid rgba(16, 185, 129, 0.25);
		display: flex; align-items: center; justify-content: center;
		font-size: 12px; font-weight: 700; color: var(--accent);
		flex-shrink: 0; font-family: var(--mono);
	}
	.step-content { display: flex; flex-direction: column; gap: 3px; }
	.step-title { font-size: 14px; font-weight: 600; color: var(--text); }
	.step-desc { font-size: 12px; color: var(--text-3); line-height: 1.5; }
	.step-desc code {
		font-family: var(--mono); font-size: 11px;
		background: var(--surface-2); padding: 1px 5px;
		border-radius: var(--r-sm); color: var(--accent);
	}

	/* ── Browser ── */
	.onboarding-browser {
		width: 100%; background: var(--surface); border: 1px solid var(--border);
		border-radius: var(--r); padding: 18px;
		animation: fadeInUp 0.4s ease 0.25s both;
	}
	.browser-label {
		font-size: 12px; font-weight: 600; color: var(--text-2);
		margin-bottom: 12px; display: flex; align-items: center; gap: 6px;
	}
	.browser-label svg { color: var(--accent); opacity: 0.7; }
</style>
