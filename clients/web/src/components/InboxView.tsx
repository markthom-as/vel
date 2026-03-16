import { useEffect, useState } from 'react';
import { apiGet } from '../api/client';
import type { ApiResponse, InboxItemData } from '../types';

export function InboxView() {
  const [items, setItems] = useState<InboxItemData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    apiGet<ApiResponse<InboxItemData[]>>('/api/inbox')
      .then((res) => {
        if (!cancelled && res.ok && res.data) setItems(res.data);
      })
      .catch((err) => {
        if (!cancelled) setError(err instanceof Error ? err.message : 'Failed to load inbox');
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  if (loading) return <div className="p-4 text-zinc-500 text-sm">Loading…</div>;
  if (error) return <div className="p-4 text-red-400 text-sm">{error}</div>;

  return (
    <div className="flex-1 overflow-y-auto p-4">
      <h2 className="text-lg font-medium text-zinc-200 mb-3">Inbox</h2>
      {items.length === 0 ? (
        <p className="text-zinc-500 text-sm">No active interventions.</p>
      ) : (
        <ul className="space-y-2">
          {items.map((item) => (
            <li
              key={item.id}
              className="rounded-lg border border-zinc-700 bg-zinc-800/50 p-3 text-sm"
            >
              <div className="font-medium text-zinc-200">{item.kind}</div>
              <div className="text-zinc-500 text-xs mt-1">
                {item.state} · {formatTs(item.surfaced_at)}
                {item.snoozed_until != null && ` · snoozed until ${formatTs(item.snoozed_until)}`}
              </div>
              <div className="text-zinc-500 text-xs mt-1">message: {item.message_id}</div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function formatTs(ts: number): string {
  return new Date(ts * 1000).toLocaleString();
}
