import { get } from 'svelte/store';
import { queueStatus } from '$lib/stores/status';
import { addToast } from '$lib/stores/toast';
import type { DocumentDoneEvent, DocumentFailedEvent } from '$lib/api/types';

let eventSource: EventSource | null = null;
let onCompleteCallback: (() => void) | null = null;

export function connectEvents(options: { onComplete?: () => void } = {}) {
	if (eventSource) return;

	onCompleteCallback = options.onComplete ?? null;
	eventSource = new EventSource('/events');

	eventSource.addEventListener('document_done', (e: MessageEvent) => {
		const data: DocumentDoneEvent = JSON.parse(e.data);

		queueStatus.update((s) => ({
			...s,
			completed: s.completed + 1,
			in_progress: Math.max(0, s.in_progress - 1)
		}));

		const status = get(queueStatus);
		if (status.in_progress === 0 && status.total > 0) {
			addToast(`Ingestion complete: ${status.completed} processed`, 'success');
			onCompleteCallback?.();
		}
	});

	eventSource.addEventListener('document_failed', (e: MessageEvent) => {
		const data: DocumentFailedEvent = JSON.parse(e.data);

		queueStatus.update((s) => ({
			...s,
			failed: s.failed + 1,
			in_progress: Math.max(0, s.in_progress - 1)
		}));

		const filename = data.file.split('/').pop() ?? data.file;
		addToast(`Failed: ${filename}`, 'error');

		const status = get(queueStatus);
		if (status.in_progress === 0 && status.total > 0) {
			onCompleteCallback?.();
		}
	});
}

export function disconnectEvents() {
	if (eventSource) {
		eventSource.close();
		eventSource = null;
		onCompleteCallback = null;
	}
}
