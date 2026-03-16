import { useMemo } from 'react';
import type { ProvenanceData } from '../types';
import { useQuery } from '../data/query';
import { loadProvenance, queryKeys } from '../data/resources';

interface ProvenanceDrawerProps {
  messageId: string | null;
  onClose: () => void;
}

export function ProvenanceDrawer({ messageId, onClose }: ProvenanceDrawerProps) {
  const provenanceKey = useMemo(() => queryKeys.provenance(messageId), [messageId]);
  const { data, loading, error } = useQuery<ProvenanceData>(
    provenanceKey,
    async () => {
      if (!messageId) {
        throw new Error('No message selected');
      }
      const response = await loadProvenance(messageId);
      if (!response.ok || !response.data) {
        throw new Error('Failed to load');
      }
      return response.data;
    },
    { enabled: Boolean(messageId) },
  );

  if (!messageId) return null;

  return (
    <div className="absolute inset-y-0 right-0 w-96 bg-zinc-900 border-l border-zinc-700 shadow-xl flex flex-col z-10">
      <div className="shrink-0 flex items-center justify-between px-4 py-3 border-b border-zinc-700">
        <h3 className="font-medium text-zinc-200">Provenance</h3>
        <button
          type="button"
          onClick={onClose}
          className="text-zinc-500 hover:text-zinc-300"
          aria-label="Close"
        >
          ✕
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-4 text-sm">
        {loading && <p className="text-zinc-500">Loading…</p>}
        {error && <p className="text-red-400">{error}</p>}
        {data && (
          <>
            <p className="text-zinc-500 mb-2">Message: {data.message_id}</p>
            {data.events.length > 0 ? (
              <div className="space-y-3">
                <h4 className="font-medium text-zinc-400">Events</h4>
                {data.events.map((ev) => (
                  <div key={ev.id} className="rounded border border-zinc-700 p-2">
                    <div className="text-xs text-zinc-500">{ev.event_name} · {new Date(ev.created_at * 1000).toISOString()}</div>
                    <pre className="mt-1 text-xs overflow-x-auto whitespace-pre-wrap break-words text-zinc-300">
                      {JSON.stringify(ev.payload, null, 2)}
                    </pre>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-zinc-500">No events recorded for this message.</p>
            )}
            {data.signals.length > 0 && (
              <div className="mt-4">
                <h4 className="font-medium text-zinc-400">Signals</h4>
                <pre className="text-xs text-zinc-400">{JSON.stringify(data.signals, null, 2)}</pre>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
