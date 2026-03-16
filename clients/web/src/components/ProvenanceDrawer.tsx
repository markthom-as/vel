import { useMemo } from 'react';
import type { JsonValue, ProvenanceData } from '../types';
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
            <div className="mb-4">
              <p className="text-zinc-500">Message</p>
              <p className="text-zinc-200 font-mono text-xs mt-1 break-all">{data.message_id}</p>
            </div>
            {data.events.length > 0 ? (
              <div className="space-y-3">
                <SectionHeading title="Events" count={data.events.length} />
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
              <StructuredSection title="Signals" items={data.signals} />
            )}
            {data.policy_decisions.length > 0 && (
              <StructuredSection title="Policy decisions" items={data.policy_decisions} />
            )}
            {data.linked_objects.length > 0 && (
              <StructuredSection title="Linked objects" items={data.linked_objects} />
            )}
          </>
        )}
      </div>
    </div>
  );
}

function SectionHeading({ title, count }: { title: string; count: number }) {
  return (
    <div className="flex items-center justify-between gap-3">
      <h4 className="font-medium text-zinc-400">{title}</h4>
      <span className="text-[11px] uppercase tracking-wide text-zinc-500">{count}</span>
    </div>
  );
}

function StructuredSection({ title, items }: { title: string; items: JsonValue[] }) {
  return (
    <div className="mt-4">
      <SectionHeading title={title} count={items.length} />
      <div className="mt-2 space-y-2">
        {items.map((item, index) => (
          <StructuredItem key={`${title}-${index}`} value={item} />
        ))}
      </div>
    </div>
  );
}

function StructuredItem({ value }: { value: JsonValue }) {
  const summary = summarizeValue(value);

  return (
    <div className="rounded border border-zinc-700 bg-zinc-950/60 p-2">
      {summary.length > 0 && (
        <dl className="space-y-1">
          {summary.map(([label, text]) => (
            <div key={`${label}-${text}`} className="grid grid-cols-[96px_1fr] gap-2 text-xs">
              <dt className="text-zinc-500">{label}</dt>
              <dd className="text-zinc-200 break-words">{text}</dd>
            </div>
          ))}
        </dl>
      )}
      <pre className="mt-2 text-[11px] overflow-x-auto whitespace-pre-wrap break-words text-zinc-400">
        {JSON.stringify(value, null, 2)}
      </pre>
    </div>
  );
}

function summarizeValue(value: JsonValue): Array<[string, string]> {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return [['value', String(value)]];
  }

  const record = value as Record<string, JsonValue>;
  const preferredKeys = [
    'signal_type',
    'type',
    'source',
    'decision',
    'action',
    'object_type',
    'object_id',
    'id',
    'message',
    'reason',
    'summary',
    'status',
  ];

  const rows: Array<[string, string]> = [];
  for (const key of preferredKeys) {
    const next = record[key];
    if (next === undefined || next === null) {
      continue;
    }
    rows.push([formatKey(key), formatValue(next)]);
    if (rows.length >= 4) {
      return rows;
    }
  }

  for (const [key, next] of Object.entries(record)) {
    if (preferredKeys.includes(key) || next === undefined || next === null) {
      continue;
    }
    rows.push([formatKey(key), formatValue(next)]);
    if (rows.length >= 4) {
      break;
    }
  }

  return rows;
}

function formatKey(key: string): string {
  return key.replace(/_/g, ' ');
}

function formatValue(value: JsonValue): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean' || value === null) {
    return String(value);
  }
  return JSON.stringify(value);
}
