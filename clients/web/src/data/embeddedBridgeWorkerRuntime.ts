import { bootstrapEmbeddedBridgePacketRuntime } from './embeddedBridgeWasmRuntime';
import { runEmbeddedBridgeWorkerTask } from './embeddedBridgeWorker';

type EmbeddedBridgeWorkerRequest = {
  id: number;
  task: Parameters<typeof runEmbeddedBridgeWorkerTask>[0];
};

type EmbeddedBridgeWorkerResponse =
  | {
      id: number;
      ok: true;
      result: Awaited<ReturnType<typeof runEmbeddedBridgeWorkerTask>>;
    }
  | {
      id: number;
      ok: false;
      error: string;
    };

let runtimeReady: Promise<void> | null = null;

async function ensureRuntimeReady(): Promise<void> {
  if (runtimeReady == null) {
    runtimeReady = bootstrapEmbeddedBridgePacketRuntime().then((runtime) => {
      if (runtime == null) {
        throw new Error('Embedded bridge WASM runtime is unavailable inside worker.');
      }
    });
  }
  await runtimeReady;
}

self.addEventListener('message', async (event: MessageEvent<EmbeddedBridgeWorkerRequest>) => {
  const { id, task } = event.data;
  try {
    await ensureRuntimeReady();
    const result = await runEmbeddedBridgeWorkerTask(task);
    const response: EmbeddedBridgeWorkerResponse = { id, ok: true, result };
    self.postMessage(response);
  } catch (error) {
    const response: EmbeddedBridgeWorkerResponse = {
      id,
      ok: false,
      error: error instanceof Error ? error.message : 'Embedded bridge worker task failed.',
    };
    self.postMessage(response);
  }
});
