import { apiFetch } from './client';
import type { CreateWorkspaceResponse, CreateApiKeyResponse, WorkspaceDocsResponse } from './types';

export async function createWorkspace(name: string): Promise<CreateWorkspaceResponse> {
	return apiFetch('/workspaces', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ name })
	});
}

export async function createApiKey(
	workspaceId: string,
	label?: string
): Promise<CreateApiKeyResponse> {
	return apiFetch('/api-keys', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ workspace_id: workspaceId, label: label ?? '' })
	});
}

export async function fetchWorkspaceDocs(
	workspaceId: string
): Promise<WorkspaceDocsResponse> {
	return apiFetch(`/workspaces/${workspaceId}/docs`);
}
