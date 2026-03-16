import { useMemo } from 'react';
import type { CurrentContextData, JsonObject } from '../types';
import { useQuery } from '../data/query';
import { loadCurrentContext, queryKeys } from '../data/resources';

export function ContextPanel() {
  const contextKey = useMemo(() => queryKeys.currentContext(), []);
  const { data: context, loading, error } = useQuery<CurrentContextData | null>(
    contextKey,
    async () => {
      const response = await loadCurrentContext();
      return response.ok ? response.data ?? null : null;
    },
  );
  const entries = context ? asContextEntries(context.context) : [];

  if (loading) return <div className="p-4 text-zinc-500 text-sm">Loading context…</div>;
  if (error) return <div className="p-4 text-amber-500 text-sm">{error}</div>;
  if (!context || entries.length === 0) {
    return (
      <div className="p-4 text-zinc-500 text-sm">
        <h3 className="font-medium text-zinc-400 mb-2">Context</h3>
        <p>No context data. Run evaluate or start the engine.</p>
      </div>
    );
  }

  return (
    <div className="p-4 text-sm overflow-y-auto">
      <h3 className="font-medium text-zinc-400 mb-2">Context</h3>
      <p className="mb-3 text-xs text-zinc-500">
        computed at {new Date(context.computed_at * 1000).toLocaleString()}
      </p>
      <dl className="space-y-2">
        {entries.map(([k, v]) => (
          <div key={k}>
            <dt className="text-zinc-500 text-xs">{k}</dt>
            <dd className="text-zinc-200 break-words">
              {typeof v === 'object' && v !== null ? JSON.stringify(v) : String(v)}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  );
}

function asContextEntries(value: CurrentContextData['context']): [string, CurrentContextData['context']][] {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return [['value', value]];
  }
  return Object.entries(value as JsonObject);
}
