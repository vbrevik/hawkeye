export class ApiError extends Error {
	constructor(
		public status: number,
		message: string
	) {
		super(message);
		this.name = 'ApiError';
	}
}

function getAuthToken(): string | null {
	if (typeof window === 'undefined') return null;
	return localStorage.getItem('hawkeye_api_key');
}

export function setAuthToken(key: string | null): void {
	if (typeof window === 'undefined') return;
	if (key) {
		localStorage.setItem('hawkeye_api_key', key);
	} else {
		localStorage.removeItem('hawkeye_api_key');
	}
}

export async function apiFetch<T>(url: string, init?: RequestInit): Promise<T> {
	const headers: Record<string, string> = {
		...(init?.headers as Record<string, string>)
	};

	const token = getAuthToken();
	if (token) headers['Authorization'] = `Bearer ${token}`;

	const res = await fetch(url, { ...init, headers });

	if (!res.ok) {
		const text = await res.text();
		throw new ApiError(res.status, text || res.statusText);
	}

	return res.json();
}
