import {
  actionItemDedupeKeysValue,
  normalizeTaskDisplayBatchValue,
} from './embeddedBridgeAdapter';

export type EmbeddedBridgeWorkerTask =
  | {
      kind: 'normalizeTaskDisplayBatch';
      entries: Array<{ tags?: string[] | null; project?: string | null }>;
    }
  | {
      kind: 'actionItemDedupeKeys';
      entries: Array<{
        kind: string;
        title: string;
        summary: string;
        projectLabel?: string | null;
        threadId?: string | null;
        threadLabel?: string | null;
      }>;
    };

export type EmbeddedBridgeWorkerResult =
  | {
      kind: 'normalizeTaskDisplayBatch';
      items: Array<{ tags: string[]; project: string | null }>;
    }
  | {
      kind: 'actionItemDedupeKeys';
      keys: string[];
    };

export async function runEmbeddedBridgeWorkerTask(
  task: EmbeddedBridgeWorkerTask,
): Promise<EmbeddedBridgeWorkerResult> {
  if (task.kind === 'normalizeTaskDisplayBatch') {
    return {
      kind: 'normalizeTaskDisplayBatch',
      items: normalizeTaskDisplayBatchValue(task.entries),
    };
  }

  return {
    kind: 'actionItemDedupeKeys',
    keys: actionItemDedupeKeysValue(task.entries),
  };
}

type EmbeddedBridgeWorkerRequest = {
  id: number;
  task: EmbeddedBridgeWorkerTask;
};

type EmbeddedBridgeWorkerResponse =
  | {
      id: number;
      ok: true;
      result: EmbeddedBridgeWorkerResult;
    }
  | {
      id: number;
      ok: false;
      error: string;
    };

let embeddedBridgeWorker: Worker | null = null;
let nextRequestId = 1;
const pendingRequests = new Map<
  number,
  {
    resolve: (value: EmbeddedBridgeWorkerResult) => void;
    reject: (error: Error) => void;
  }
>();

function ensureEmbeddedBridgeWorker(): Worker {
  if (embeddedBridgeWorker != null) {
    return embeddedBridgeWorker;
  }

  const worker = new Worker(
    new URL('./embeddedBridgeWorkerRuntime.ts', import.meta.url),
    { type: 'module' },
  );

  worker.addEventListener('message', (event: MessageEvent<EmbeddedBridgeWorkerResponse>) => {
    const payload = event.data;
    const pending = pendingRequests.get(payload.id);
    if (!pending) {
      return;
    }
    pendingRequests.delete(payload.id);
    if (payload.ok) {
      pending.resolve(payload.result);
      return;
    }
    pending.reject(new Error(payload.error));
  });

  worker.addEventListener('error', (event) => {
    const error = event.error instanceof Error
      ? event.error
      : new Error(event.message || 'Embedded bridge worker failure.');
    for (const pending of pendingRequests.values()) {
      pending.reject(error);
    }
    pendingRequests.clear();
    embeddedBridgeWorker = null;
  });

  embeddedBridgeWorker = worker;
  return worker;
}

export async function runEmbeddedBridgeTaskInWorker(
  task: EmbeddedBridgeWorkerTask,
): Promise<EmbeddedBridgeWorkerResult> {
  if (typeof Worker === 'undefined') {
    return runEmbeddedBridgeWorkerTask(task);
  }

  const worker = ensureEmbeddedBridgeWorker();
  const id = nextRequestId++;
  const request: EmbeddedBridgeWorkerRequest = { id, task };

  return await new Promise<EmbeddedBridgeWorkerResult>((resolve, reject) => {
    pendingRequests.set(id, { resolve, reject });
    worker.postMessage(request);
  });
}

export async function normalizeTaskDisplayBatchInWorker(
  entries: Array<{ tags?: string[] | null; project?: string | null }>,
): Promise<Array<{ tags: string[]; project: string | null }>> {
  const result = await runEmbeddedBridgeTaskInWorker({
    kind: 'normalizeTaskDisplayBatch',
    entries,
  });
  return result.kind === 'normalizeTaskDisplayBatch'
    ? result.items
    : normalizeTaskDisplayBatchValue(entries);
}
