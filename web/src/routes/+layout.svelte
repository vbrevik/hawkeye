<script lang="ts">
	import '../app.css';
	import ToastContainer from '$lib/components/ToastContainer.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>Hawkeye</title>
</svelte:head>

{@render children()}
<ToastContainer />
