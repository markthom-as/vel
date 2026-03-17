import { useEffect, useMemo } from 'react';
import type { InboxItemData } from '../types';
import { setQueryData, useQuery } from '../data/query';
import {
  prunePendingInterventionActions,
  type PendingInterventionAction,
} from '../data/chat-state';
import { loadInbox, queryKeys } from '../data/resources';
import { subscribeWsQuerySync } from '../data/ws-sync';
import { SurfaceState } from './SurfaceState';

export function InboxView() {
  const inboxKey = useMemo(() => queryKeys.inbox(), []);
  const pendingInterventionActionsKey = useMemo(
    () => queryKeys.pendingInterventionActions(),
    [],
  );
  const { data: items = [], loading, error } = useQuery<InboxItemData[]>(
    inboxKey,
    async () => {
      const response = await loadInbox();
      return response.ok && response.data ? response.data : [];
    },
  );
  const { data: pendingInterventionActions = {} } = useQuery<Record<string, PendingInterventionAction>>(
    pendingInterventionActionsKey,
    async () => ({}),
    { enabled: false },
  );
  const visibleItems = items.filter((item) => pendingInterventionActions[item.id] === undefined);

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  useEffect(() => {
    setQueryData<Record<string, PendingInterventionAction>>(
      pendingInterventionActionsKey,
      (prev = {}) => prunePendingInterventionActions(prev, items),
    );
  }, [items, pendingInterventionActionsKey]);

  if (loading) return <SurfaceState message="Loading inbox…" />;
  if (error) return <SurfaceState message={error} tone="danger" />;

  return (
    <div className="flex-1 overflow-y-auto p-4">
      <h2 className="text-lg font-medium text-zinc-200 mb-3">Inbox</h2>
      {visibleItems.length === 0 ? (
        <SurfaceState message="No active interventions." />
      ) : (
        <ul className="space-y-2">
          {visibleItems.map((item) => (
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
