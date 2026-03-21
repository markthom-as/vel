import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type { ConversationData, InboxItemData, JsonValue, MessageData } from '../types';
import {
  chatQueryKeys,
  loadConversationList,
  loadConversationInterventions,
  loadConversationMessages,
  mutateIntervention,
} from '../data/chat';
import { getQueryData, invalidateQuery, setQueryData, useQuery } from '../data/query';
import {
  appendUniqueMessages,
  prunePendingInterventionActions,
  reconcileConfirmedSend,
  removeInterventionById,
  setPendingInterventionAction,
  type PendingInterventionAction,
} from '../data/chat-state';
import { subscribeWsQuerySync } from '../data/ws-sync';
import { MessageRenderer } from './MessageRenderer';
import { MessageComposer } from './MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { SurfaceState } from './SurfaceState';

interface ThreadViewProps {
  conversationId: string | null;
  onSelectConversation?: (conversationId: string) => void;
}

function capabilityStateLabel(state: string): string {
  return state.replaceAll('_', ' ');
}

function continuationTokenLabel(value: string): string {
  return value.replaceAll('_', ' ');
}

function formatContextValue(value: JsonValue): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  if (value === null) {
    return 'unknown';
  }
  if (Array.isArray(value)) {
    return value
      .map((item) => (typeof item === 'string' ? item : JSON.stringify(item)))
      .join(' • ');
  }
  return JSON.stringify(value);
}

function continuationContextRows(conversation: ConversationData): Array<{ label: string; value: string }> {
  const context = conversation.continuation?.continuation.continuation_context;
  if (!context || typeof context !== 'object' || Array.isArray(context)) {
    return [];
  }
  return Object.entries(context)
    .filter(([, value]) => value !== null && value !== '')
    .slice(0, 3)
    .map(([label, value]) => ({
      label: label.replaceAll('_', ' '),
      value: formatContextValue(value),
    }));
}

export function ThreadView({ conversationId, onSelectConversation }: ThreadViewProps) {
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);
  const [threadFilter, setThreadFilter] = useState('');
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [], loading: conversationsLoading, error: conversationsError } = useQuery(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
  );
  const fallbackConversationId = useMemo(() => {
    if (conversationId || conversations.length === 0) {
      return null;
    }
    return [...conversations]
      .sort((left, right) => right.updated_at - left.updated_at)[0]?.id ?? null;
  }, [conversationId, conversations]);
  const selectedConversation = useMemo(
    () => conversations.find((conversation) => conversation.id === (conversationId ?? fallbackConversationId)) ?? null,
    [conversationId, conversations, fallbackConversationId],
  );
  const filteredConversations = useMemo(() => {
    const query = threadFilter.trim().toLowerCase();
    const sorted = [...conversations].sort((left, right) => right.updated_at - left.updated_at);
    if (!query) {
      return sorted.slice(0, 6);
    }
    return sorted
      .filter((conversation) => {
        const haystacks = [
          conversation.title ?? '',
          conversation.kind,
          conversation.id,
        ];
        return haystacks.some((value) => value.toLowerCase().includes(query));
      })
      .slice(0, 6);
  }, [conversations, threadFilter]);
  const resolvedConversationId = conversationId ?? fallbackConversationId;
  const messagesKey = useMemo(
    () => chatQueryKeys.conversationMessages(resolvedConversationId),
    [resolvedConversationId],
  );
  const interventionsKey = useMemo(
    () => chatQueryKeys.conversationInterventions(resolvedConversationId),
    [resolvedConversationId],
  );
  const inboxKey = useMemo(() => chatQueryKeys.inbox(), []);
  const pendingInterventionActionsKey = useMemo(
    () => chatQueryKeys.pendingInterventionActions(),
    [],
  );

  const {
    data: messages = [],
    loading: messagesLoading,
    error: messagesError,
    refetch: refetchMessages,
  } = useQuery<MessageData[]>(
    messagesKey,
    async () => {
      if (!resolvedConversationId) {
        return [];
      }
      const response = await loadConversationMessages(resolvedConversationId);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: Boolean(resolvedConversationId) },
  );
  const {
    data: interventions = [],
    error: interventionsError,
  } = useQuery<InboxItemData[]>(
    interventionsKey,
    async () => {
      if (!resolvedConversationId) {
        return [];
      }
      const response = await loadConversationInterventions(resolvedConversationId);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: Boolean(resolvedConversationId) },
  );
  const { data: pendingInterventionActions = {} } = useQuery<Record<string, PendingInterventionAction>>(
    pendingInterventionActionsKey,
    async () => ({}),
    { enabled: false },
  );

  const visibleInterventions = interventions.filter(
    (intervention) => pendingInterventionActions[intervention.id] === undefined,
  );
  const interventionsByMessageId = visibleInterventions.reduce<Record<string, string>>((next, intervention) => {
    if (!(intervention.message_id in next)) {
      next[intervention.message_id] = intervention.id;
    }
    return next;
  }, {});

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  useEffect(() => {
    setQueryData<Record<string, PendingInterventionAction>>(
      pendingInterventionActionsKey,
      (prev = {}) => prunePendingInterventionActions(prev, interventions),
    );
  }, [interventions, pendingInterventionActionsKey]);

  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;
    try {
      el.scrollTo({ top: el.scrollHeight, behavior: 'smooth' });
    } catch {
      el.scrollTop = el.scrollHeight;
    }
  }, [messages]);

  const removeIntervention = useCallback((interventionId: string) => {
    setQueryData<InboxItemData[]>(
      interventionsKey,
      (prev = []) => removeInterventionById(prev, interventionId),
    );
    setQueryData<InboxItemData[]>(
      inboxKey,
      (prev = []) => removeInterventionById(prev, interventionId),
    );
  }, [inboxKey, interventionsKey]);

  const restoreInterventions = useCallback(
    (
      interventionId: string,
      previousConversationInterventions: InboxItemData[] | undefined,
      previousInboxItems: InboxItemData[] | undefined,
    ) => {
      setQueryData<InboxItemData[]>(interventionsKey, previousConversationInterventions ?? []);
      setQueryData<InboxItemData[]>(inboxKey, previousInboxItems ?? []);
      setQueryData<Record<string, PendingInterventionAction>>(pendingInterventionActionsKey, (prev = {}) => {
        if (!(interventionId in prev)) {
          return prev;
        }
        const next = { ...prev };
        delete next[interventionId];
        return next;
      });
    },
    [inboxKey, interventionsKey, pendingInterventionActionsKey],
  );

  const startInterventionAction = useCallback((interventionId: string, state: string) => {
    setQueryData<Record<string, PendingInterventionAction>>(pendingInterventionActionsKey, (prev = {}) =>
      setPendingInterventionAction(prev, interventionId, state),
    );
  }, [pendingInterventionActionsKey]);

  const handleSnooze = useCallback(async (interventionId: string) => {
    const previousConversationInterventions = getQueryData<InboxItemData[]>(interventionsKey);
    const previousInboxItems = getQueryData<InboxItemData[]>(inboxKey);
    startInterventionAction(interventionId, 'snoozed');
    removeIntervention(interventionId);
    try {
      await mutateIntervention(interventionId, 'snooze', { minutes: 15 });
      invalidateQuery(interventionsKey, { refetch: true });
      invalidateQuery(inboxKey, { refetch: true });
    } catch (_) {
      restoreInterventions(interventionId, previousConversationInterventions, previousInboxItems);
    }
  }, [inboxKey, interventionsKey, removeIntervention, restoreInterventions, startInterventionAction]);

  const handleResolve = useCallback(async (interventionId: string) => {
    const previousConversationInterventions = getQueryData<InboxItemData[]>(interventionsKey);
    const previousInboxItems = getQueryData<InboxItemData[]>(inboxKey);
    startInterventionAction(interventionId, 'resolved');
    removeIntervention(interventionId);
    try {
      await mutateIntervention(interventionId, 'resolve', {});
      invalidateQuery(interventionsKey, { refetch: true });
      invalidateQuery(inboxKey, { refetch: true });
    } catch (_) {
      restoreInterventions(interventionId, previousConversationInterventions, previousInboxItems);
    }
  }, [inboxKey, interventionsKey, removeIntervention, restoreInterventions, startInterventionAction]);

  const handleDismiss = useCallback(async (interventionId: string) => {
    const previousConversationInterventions = getQueryData<InboxItemData[]>(interventionsKey);
    const previousInboxItems = getQueryData<InboxItemData[]>(inboxKey);
    startInterventionAction(interventionId, 'dismissed');
    removeIntervention(interventionId);
    try {
      await mutateIntervention(interventionId, 'dismiss', {});
      invalidateQuery(interventionsKey, { refetch: true });
      invalidateQuery(inboxKey, { refetch: true });
    } catch (_) {
      restoreInterventions(interventionId, previousConversationInterventions, previousInboxItems);
    }
  }, [inboxKey, interventionsKey, removeIntervention, restoreInterventions, startInterventionAction]);

  if (!resolvedConversationId) {
    if (conversationsLoading) {
      return <SurfaceState message="Loading latest conversation…" layout="centered" />;
    }
    if (conversationsError) {
      return <SurfaceState message={conversationsError} layout="centered" tone="danger" />;
    }
    return <SurfaceState message="No conversations yet." layout="centered" />;
  }

  const error = conversationsError ?? messagesError ?? interventionsError;
  if (messagesLoading) {
    return <SurfaceState message="Loading conversation…" layout="centered" />;
  }
  if (error) {
    return <SurfaceState message={error} layout="centered" tone="danger" />;
  }

  return (
    <>
      <div ref={scrollRef} className="flex-1 overflow-y-auto p-4 relative">
        <div className="max-w-2xl mx-auto">
          <header className="mb-5 rounded-2xl border border-zinc-800 bg-zinc-900/60 p-4">
            <p className="text-xs uppercase tracking-[0.22em] text-zinc-500">Continuity</p>
            <h1 className="mt-2 text-2xl font-semibold text-zinc-100">Resume longer follow-through</h1>
            <p className="mt-2 text-sm leading-6 text-zinc-400">
              Use Threads only when `Now` or `Inbox` needs longer follow-through, deeper context, or searchable history.
            </p>
            <p className="mt-2 text-xs leading-5 text-zinc-500">
              Reflow edits, planning disagreements, and schedule shaping belong here after `Now` has already shown the compact bounded summary.
            </p>
            <div className="mt-4 rounded-xl border border-zinc-800 bg-zinc-950/60 p-3">
              <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
                <label className="flex-1">
                  <span className="sr-only">Filter threads</span>
                  <input
                    type="text"
                    value={threadFilter}
                    onChange={(event) => setThreadFilter(event.target.value)}
                    placeholder="Find recent follow-up"
                    className="w-full rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500"
                  />
                </label>
                <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                  Longer follow-through only. Triage stays in Inbox.
                </p>
              </div>
              {filteredConversations.length > 0 ? (
                <div className="mt-3 flex flex-wrap gap-2">
                  {filteredConversations.map((conversation) => {
                    const active = conversation.id === resolvedConversationId;
                    return (
                      <button
                        key={conversation.id}
                        type="button"
                        onClick={() => onSelectConversation?.(conversation.id)}
                        disabled={!onSelectConversation || active}
                        className={`rounded-full border px-3 py-1.5 text-xs ${
                          active
                            ? 'border-emerald-700 bg-emerald-950/40 text-emerald-200'
                            : 'border-zinc-700 bg-zinc-950 text-zinc-300 hover:border-zinc-600 hover:text-zinc-100'
                        } disabled:cursor-default`}
                      >
                        {conversation.title ?? 'Untitled thread'}
                      </button>
                    );
                  })}
                </div>
              ) : (
                <p className="mt-3 text-sm text-zinc-500">No recent threads match that filter.</p>
              )}
            </div>
            {selectedConversation?.continuation ? (
              <div className="mt-4 rounded-xl border border-emerald-900/70 bg-emerald-950/20 p-3">
                <div className="flex flex-wrap items-center gap-2">
                  <p className="text-xs uppercase tracking-[0.18em] text-emerald-300">Continuation</p>
                  <span className="rounded-full border border-emerald-900/80 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-emerald-200">
                    {capabilityStateLabel(selectedConversation.continuation.continuation.bounded_capability_state)}
                  </span>
                  <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-zinc-300">
                    {continuationTokenLabel(selectedConversation.continuation.continuation.continuation_category)}
                  </span>
                  {selectedConversation.continuation.lifecycle_stage ? (
                    <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-zinc-300">
                      {selectedConversation.continuation.lifecycle_stage}
                    </span>
                  ) : null}
                  <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-zinc-300">
                    {continuationTokenLabel(selectedConversation.continuation.continuation.open_target)}
                  </span>
                </div>
                <p className="mt-3 text-sm leading-6 text-zinc-300">
                  {selectedConversation.continuation.continuation.escalation_reason}
                </p>
                {continuationContextRows(selectedConversation).length > 0 ? (
                  <dl className="mt-3 grid gap-2 text-xs text-zinc-400">
                    {continuationContextRows(selectedConversation).map((entry) => (
                      <div key={entry.label} className="grid gap-1 rounded-lg border border-zinc-800/80 bg-zinc-950/60 px-3 py-2">
                        <dt className="uppercase tracking-[0.16em] text-zinc-500">{entry.label}</dt>
                        <dd className="text-sm leading-5 text-zinc-300">{entry.value}</dd>
                      </div>
                    ))}
                  </dl>
                ) : null}
                {selectedConversation.continuation.continuation.review_requirements.length > 0 ? (
                  <div className="mt-3 rounded-lg border border-zinc-800/80 bg-zinc-950/60 px-3 py-2">
                    <p className="text-[11px] uppercase tracking-[0.16em] text-zinc-500">Review gate</p>
                    <ul className="mt-2 space-y-2 text-sm leading-5 text-zinc-300">
                      {selectedConversation.continuation.continuation.review_requirements.map((requirement) => (
                        <li key={requirement}>{requirement}</li>
                      ))}
                    </ul>
                  </div>
                ) : null}
              </div>
            ) : null}
          </header>
          {messages.length === 0 && (
            <SurfaceState message="Nothing needs longer follow-up yet. Start here when `Now` or `Inbox` needs continuity." />
          )}
          {messages.map((message) => (
            <MessageRenderer
              key={message.id}
              message={message}
              interventionId={interventionsByMessageId[message.id]}
              onSnooze={handleSnooze}
              onResolve={handleResolve}
              onDismiss={handleDismiss}
              onShowWhy={setProvenanceMessageId}
            />
          ))}
        </div>
        {provenanceMessageId && (
          <ProvenanceDrawer
            messageId={provenanceMessageId}
            onClose={() => setProvenanceMessageId(null)}
          />
        )}
      </div>
      <p className="shrink-0 px-4 py-1 text-zinc-600 text-xs max-w-2xl mx-auto">
        Assistant replies require a configured chat model. Capture and bounded recall still work without one.
      </p>
      <MessageComposer
        conversationId={resolvedConversationId}
        onOptimisticSend={(text) => {
          const clientMessageId = `tmp_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
          const optimisticMessage: MessageData = {
            id: clientMessageId,
            conversation_id: resolvedConversationId,
            role: 'user',
            kind: 'text',
            content: { text },
            status: 'sending',
            importance: null,
            created_at: Math.floor(Date.now() / 1000),
            updated_at: null,
          };
          setQueryData<MessageData[]>(messagesKey, (prev = []) =>
            appendUniqueMessages(prev, [optimisticMessage]),
          );
          return clientMessageId;
        }}
        onSent={(clientMessageId, response) => {
          setQueryData<MessageData[]>(messagesKey, (prev = []) =>
            reconcileConfirmedSend(
              prev,
              clientMessageId,
              response.user_message,
              response.assistant_message ? [response.assistant_message] : [],
            ),
          );
          invalidateQuery(conversationsKey, { refetch: true });
          void refetchMessages();
        }}
        onSendFailed={(clientMessageId) => {
          if (!clientMessageId) {
            return;
          }
          setQueryData<MessageData[]>(messagesKey, (prev = []) =>
            prev.filter((message) => message.id !== clientMessageId),
          );
        }}
      />
    </>
  );
}
