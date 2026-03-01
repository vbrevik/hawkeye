import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const API_TARGET = 'http://localhost:7700';

const proxyPaths = [
	'/ingest', '/cancel', '/shutdown', '/status', '/mlx-status',
	'/search', '/facets', '/summary', '/browse', '/health', '/events',
	'/workspaces', '/api-keys', '/graph'
];

const proxy: Record<string, string> = {};
for (const path of proxyPaths) {
	proxy[path] = API_TARGET;
}

export default defineConfig({
	plugins: [sveltekit()],
	server: { proxy }
});
