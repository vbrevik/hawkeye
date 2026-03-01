import type { ShutdownResponse } from './types';

export async function shutdown(docker: boolean): Promise<ShutdownResponse> {
	const url = docker ? '/shutdown?docker=true' : '/shutdown';
	const res = await fetch(url, { method: 'POST' });
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}
