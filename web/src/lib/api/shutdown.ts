import { apiFetch } from './client';
import type { ShutdownResponse } from './types';

export async function shutdown(docker: boolean): Promise<ShutdownResponse> {
	const url = docker ? '/shutdown?docker=true' : '/shutdown';
	return apiFetch<ShutdownResponse>(url, { method: 'POST' });
}
