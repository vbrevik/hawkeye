import type { Summary } from './types';

export async function fetchSummary(file: string): Promise<Summary> {
	const res = await fetch(`/summary/${encodeURIComponent(file)}`);
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}
