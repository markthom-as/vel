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
