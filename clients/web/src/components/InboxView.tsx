import { useEffect, useMemo } from 'react';
import type { InboxItemData } from '../types';
import {
  acknowledgeInboxItem,
  chatQueryKeys,
  dismissInboxItem,
  getInboxThreadPath,
  loadInbox,
  snoozeInboxItem,
} from '../data/chat';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import {
  markPendingInterventionActionConfirmed,
  prunePendingInterventionActions,
  setPendingInterventionAction,
  type PendingInterventionAction,
} from '../data/chat-state';
import { subscribeWsQuerySync } from '../data/ws-sync';
import { SurfaceState } from './SurfaceState';

interface InboxViewProps {
  onOpenThread?: (conversationId: string) => void;
}

export function InboxView({ onOpenThread }: InboxViewProps) {
  const inboxKey = useMemo(() => chatQueryKeys.inbox(), []);
  const pendingInterventionActionsKey = useMemo(
    () => chatQueryKeys.pendingInterventionActions(),
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

  async function runInterventionAction(
    item: InboxItemData,
    nextState: 'acknowledged' | 'dismissed' | 'snoozed',
    action: () => Promise<{ ok: boolean; data?: { state: string }; error?: { message: string } }>,
  ) {
    setQueryData<Record<string, PendingInterventionAction>>(
      pendingInterventionActionsKey,
      (prev = {}) => setPendingInterventionAction(prev, item.id, nextState),
    );

    try {
      const response = await action();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update inbox item');
      }
      setQueryData<Record<string, PendingInterventionAction>>(
        pendingInterventionActionsKey,
        (prev = {}) =>
          markPendingInterventionActionConfirmed(prev, item.id, response.data?.state ?? nextState),
      );
      invalidateQuery(inboxKey, { refetch: true });
    } catch (_) {
      setQueryData<Record<string, PendingInterventionAction>>(
        pendingInterventionActionsKey,
        (prev = {}) => {
          const next = { ...prev };
          delete next[item.id];
          return next;
        },
      );
    }
  }

  if (loading) {
    return <SurfaceState message="Loading inbox…" />;
  }
  if (error) {
    return <SurfaceState message={error} tone="danger" />;
  }

  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-5xl px-6 py-8">
        <header className="mb-6">
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Inbox</p>
          <h1 className="mt-2 text-3xl font-semibold text-zinc-100">Triage newly surfaced work</h1>
          <p className="mt-2 text-sm text-zinc-400">
            Clear items quickly, then open the underlying thread only when you need more evidence.
          </p>
        </header>

        {visibleItems.length === 0 ? (
          <section className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-6">
            <h2 className="text-lg font-medium text-zinc-100">Inbox is clear</h2>
            <p className="mt-2 max-w-2xl text-sm leading-6 text-zinc-400">
              No actions need triage right now. Return to `Now` for the current decision set, or
              open `Threads` to inspect the evidence behind recent state.
            </p>
          </section>
        ) : (
          <div className="space-y-3">
            {visibleItems.map((item) => {
              const threadPath = getInboxThreadPath(item);
              const hasAcknowledge = item.available_actions.includes('acknowledge');
              const hasSnooze = item.available_actions.includes('snooze');
              const hasDismiss = item.available_actions.includes('dismiss');
              const hasOpenThread = Boolean(threadPath && item.conversation_id);

              return (
                <article
                  key={item.id}
                  className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-4"
                >
                  <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
                    <div className="min-w-0 flex-1">
                      <div className="flex flex-wrap items-center gap-2">
                        <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-400">
                          {formatKind(item.kind)}
                        </span>
                        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
                          {item.state}
                        </span>
                        {item.project_label ? (
                          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-emerald-300">
                            {item.project_label}
                          </span>
                        ) : null}
                      </div>
                      <h2 className="mt-3 text-lg font-medium text-zinc-100">{item.title}</h2>
                      <p className="mt-2 text-sm leading-6 text-zinc-300">{item.summary}</p>
                      <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">
                        <span>Surfaced {formatTs(item.surfaced_at)}</span>
                        {item.snoozed_until != null ? (
                          <span>Snoozed until {formatTs(item.snoozed_until)}</span>
                        ) : null}
                        {item.confidence != null ? (
                          <span>Confidence {Math.round(item.confidence * 100)}%</span>
                        ) : null}
                      </div>
                      <div className="mt-3">
                        <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Evidence</p>
                        <div className="mt-2 flex flex-wrap gap-2">
                          {item.evidence.length === 0 ? (
                            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
                              No evidence labels
                            </span>
                          ) : (
                            item.evidence.map((evidence) => (
                              <span
                                key={`${item.id}-${evidence.source_id}-${evidence.label}`}
                                className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-400"
                              >
                                {evidence.label}
                              </span>
                            ))
                          )}
                        </div>
                      </div>
                    </div>

                    <div className="flex w-full flex-col gap-2 lg:w-52">
                      {hasAcknowledge ? (
                        <button
                          type="button"
                          onClick={() => void runInterventionAction(item, 'acknowledged', () => acknowledgeInboxItem(item.id))}
                          className="min-h-[44px] rounded-xl border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 hover:border-zinc-600"
                        >
                          Acknowledge
                        </button>
                      ) : null}
                      {hasSnooze ? (
                        <button
                          type="button"
                          onClick={() => void runInterventionAction(item, 'snoozed', () => snoozeInboxItem(item.id, 10))}
                          className="min-h-[44px] rounded-xl border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 hover:border-zinc-600"
                        >
                          Snooze 10m
                        </button>
                      ) : null}
                      {hasDismiss ? (
                        <button
                          type="button"
                          onClick={() => void runInterventionAction(item, 'dismissed', () => dismissInboxItem(item.id))}
                          className="min-h-[44px] rounded-xl border border-rose-900/70 bg-rose-950/40 px-3 py-2 text-sm text-rose-100 hover:border-rose-800"
                        >
                          Dismiss
                        </button>
                      ) : null}
                      {hasOpenThread ? (
                        <button
                          type="button"
                          onClick={() => onOpenThread?.(item.conversation_id as string)}
                          className="min-h-[44px] rounded-xl bg-emerald-600 px-3 py-2 text-sm font-medium text-zinc-950 hover:bg-emerald-500"
                        >
                          Open thread
                        </button>
                      ) : null}
                    </div>
                  </div>
                </article>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}

function formatTs(ts: number): string {
  return new Date(ts * 1000).toLocaleString();
}

function formatKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}
