import { useEffect, useMemo } from 'react';
import type { InboxItemData, WsEvent } from '../types';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import { loadInbox, queryKeys } from '../data/resources';
import { subscribeWs } from '../realtime/ws';

export function InboxView() {
  const inboxKey = useMemo(() => queryKeys.inbox(), []);
  const { data: items = [], loading, error } = useQuery<InboxItemData[]>(
    inboxKey,
    async () => {
      const response = await loadInbox();
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWs((event: WsEvent) => {
      if (event.type === 'interventions:new') {
        const payload = event.payload;
        setQueryData<InboxItemData[]>(inboxKey, (prev = []) =>
          upsertInboxItem(prev, payload),
        );
        return;
      }
      if (event.type === 'interventions:updated') {
        invalidateQuery(inboxKey, { refetch: true });
      }
    });
  }, [inboxKey]);

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

function upsertInboxItem(items: InboxItemData[], nextItem: InboxItemData): InboxItemData[] {
  const existingIndex = items.findIndex((item) => item.id === nextItem.id);
  if (existingIndex === -1) {
    return [nextItem, ...items];
  }
  const next = [...items];
  next[existingIndex] = nextItem;
  return next;
}
