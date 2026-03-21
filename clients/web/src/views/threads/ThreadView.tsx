import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type { ConversationData, InboxItemData, JsonValue, MessageData } from '../../types';
import {
  chatQueryKeys,
  loadConversationList,
  loadConversationInterventions,
  loadConversationMessages,
  mutateIntervention,
} from '../../data/chat';
import { getQueryData, invalidateQuery, setQueryData, useQuery } from '../../data/query';
import {
  appendUniqueMessages,
  prunePendingInterventionActions,
  reconcileConfirmedSend,
  removeInterventionById,
  setPendingInterventionAction,
  type PendingInterventionAction,
} from '../../data/chat-state';
import { subscribeWsQuerySync } from '../../data/ws-sync';
import { MessageRenderer } from '../../core/MessageRenderer';
import { MessageComposer } from '../../core/MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { SurfaceState } from '../../core/SurfaceState';

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
  const [filterMode, setFilterMode] = useState<'all' | 'unread' | 'active'>('all');
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
    return sorted
      .filter((conversation) => {
        if (filterMode === 'unread') {
          return Boolean(conversation.continuation);
        }
        if (filterMode === 'active') {
          return conversation.id === (conversationId ?? fallbackConversationId);
        }
        return true;
      })
      .filter((conversation) => {
        const haystacks = [
          conversation.title ?? '',
          conversation.kind,
          conversation.id,
        ];
        return haystacks.some((value) => value.toLowerCase().includes(query));
      })
      .slice(0, 8);
  }, [conversationId, conversations, fallbackConversationId, filterMode, threadFilter]);
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
      <div className="flex min-h-0 flex-1">
        <aside className="flex w-full max-w-sm shrink-0 flex-col border-r border-zinc-900 bg-zinc-950/60">
          <div className="border-b border-zinc-900 px-4 py-4">
            <p className="text-[11px] uppercase tracking-[0.22em] text-zinc-500">Threads</p>
            <div className="mt-3 flex flex-wrap gap-2">
              {(['all', 'unread', 'active'] as const).map((mode) => (
                <button
                  key={mode}
                  type="button"
                  onClick={() => setFilterMode(mode)}
                  className={`rounded-full border px-2.5 py-1 text-[10px] uppercase tracking-[0.16em] ${
                    filterMode === mode
                      ? 'border-amber-300/60 text-amber-200'
                      : 'border-zinc-800 text-zinc-500'
                  }`}
                >
                  {mode}
                </button>
              ))}
            </div>
            <label className="mt-3 block relative">
              <span className="sr-only">Filter threads</span>
              <input
                type="text"
                value={threadFilter}
                onChange={(event) => setThreadFilter(event.target.value)}
                placeholder="Find thread"
                className="w-full rounded-xl border border-zinc-800 bg-zinc-950 px-3 py-2 pr-10 text-sm text-zinc-100 placeholder:text-zinc-500"
              />
              <span className="pointer-events-none absolute inset-y-0 right-3 flex items-center text-zinc-500">⌕</span>
            </label>
          </div>
          <div className="min-h-0 flex-1 overflow-y-auto p-2">
            {filteredConversations.length === 0 ? (
              <p className="px-2 py-3 text-sm text-zinc-500">No threads match that filter.</p>
            ) : (
              <div className="space-y-2">
                {filteredConversations.map((conversation) => (
                  <ThreadListRow
                    key={conversation.id}
                    conversation={conversation}
                    active={conversation.id === resolvedConversationId}
                    disabled={!onSelectConversation || conversation.id === resolvedConversationId}
                    onSelect={onSelectConversation}
                  />
                ))}
              </div>
            )}
          </div>
          <div className="border-t border-zinc-900 px-4 py-3 text-center text-xs text-zinc-500">
            {filteredConversations.length} thread{filteredConversations.length === 1 ? '' : 's'} in view
          </div>
        </aside>

        <section className="relative flex min-w-0 flex-1 flex-col">
          <div ref={scrollRef} className="flex-1 overflow-y-auto p-4">
            <div className="mx-auto max-w-3xl">
              <header className="mb-5 border-b border-zinc-900 pb-4">
                <div className="flex flex-wrap items-start justify-between gap-3">
                  <div className="min-w-0">
                    <p className="text-[11px] uppercase tracking-[0.22em] text-zinc-500">Thread</p>
                    <h1 className="mt-2 truncate text-2xl font-semibold tracking-tight text-zinc-100">
                      {threadTitle(selectedConversation)}
                    </h1>
                  </div>
                  {selectedConversation?.continuation ? (
                    <div className="flex flex-wrap gap-2">
                      <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.16em] text-zinc-300">
                        {continuationTokenLabel(selectedConversation.continuation.continuation.continuation_category)}
                      </span>
                      {selectedConversation.continuation.lifecycle_stage ? (
                        <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.16em] text-zinc-300">
                          {selectedConversation.continuation.lifecycle_stage}
                        </span>
                      ) : null}
                    </div>
                  ) : null}
                </div>
                {selectedConversation?.continuation ? (
                  <div className="mt-3 rounded-2xl border border-zinc-800 bg-zinc-900/60 p-3">
                    <p className="text-sm leading-6 text-zinc-300">
                      {selectedConversation.continuation.continuation.escalation_reason}
                    </p>
                    <div className="mt-3 flex flex-wrap gap-2">
                      <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-zinc-300">
                        {capabilityStateLabel(selectedConversation.continuation.continuation.bounded_capability_state)}
                      </span>
                      <span className="rounded-full border border-zinc-800 px-2 py-1 text-[11px] uppercase tracking-[0.14em] text-zinc-300">
                        {continuationTokenLabel(selectedConversation.continuation.continuation.open_target)}
                      </span>
                    </div>
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
                  </div>
                ) : null}
              </header>
              {messages.length === 0 && (
                <SurfaceState message="No messages in this thread yet." />
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
          </div>
          {provenanceMessageId && (
            <ProvenanceDrawer
              messageId={provenanceMessageId}
              onClose={() => setProvenanceMessageId(null)}
            />
          )}
          <div className="border-t border-zinc-900 px-4 py-3">
            <div className="mx-auto max-w-3xl">
              <MessageComposer
                conversationId={resolvedConversationId}
                compact
                hideHelperText
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
            </div>
          </div>
        </section>
      </div>
    </>
  );
}

function threadTitle(conversation: ConversationData | null): string {
  if (conversation?.title?.trim()) {
    return conversation.title.trim();
  }
  if (conversation?.continuation?.continuation.escalation_reason?.trim()) {
    return conversation.continuation.continuation.escalation_reason.trim();
  }
  if (conversation?.continuation?.continuation.open_target?.trim()) {
    return continuationTokenLabel(conversation.continuation.continuation.open_target);
  }
  return 'Untitled thread';
}

function previewText(conversation: ConversationData, messages: MessageData[]): string {
  const latestMessage = [...messages].sort((left, right) => right.created_at - left.created_at)[0];
  const messageText =
    typeof latestMessage?.content === 'object' && latestMessage?.content !== null && !Array.isArray(latestMessage.content)
      ? latestMessage.content.text
      : null;
  if (typeof messageText === 'string' && messageText.trim().length > 0) {
    return messageText.trim();
  }
  if (conversation.continuation?.continuation.escalation_reason?.trim()) {
    return conversation.continuation.continuation.escalation_reason.trim();
  }
  return conversation.kind.replaceAll('_', ' ');
}

function ThreadListRow({
  conversation,
  active,
  disabled,
  onSelect,
}: {
  conversation: ConversationData;
  active: boolean;
  disabled: boolean;
  onSelect?: (conversationId: string) => void;
}) {
  const messagesKey = useMemo(
    () => chatQueryKeys.conversationMessages(conversation.id),
    [conversation.id],
  );
  const { data: messages = [] } = useQuery<MessageData[]>(
    messagesKey,
    async () => {
      try {
        const response = await loadConversationMessages(conversation.id);
        return response.ok && response.data ? response.data : [];
      } catch {
        return [];
      }
    },
    { enabled: true },
  );
  const unreadCount = conversation.continuation ? 1 : 0;

  return (
    <button
      type="button"
      onClick={() => onSelect?.(conversation.id)}
      disabled={disabled}
      className={`w-full rounded-2xl border p-3 text-left transition ${
        active
          ? 'border-zinc-100 bg-zinc-100 text-zinc-950'
          : 'border-zinc-800 bg-zinc-900/70 text-zinc-100 hover:border-zinc-700'
      } disabled:cursor-default`}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <p className="truncate text-sm font-medium">{threadTitle(conversation)}</p>
          <p className={`mt-1 truncate text-xs ${active ? 'text-zinc-700' : 'text-zinc-400'}`}>
            {previewText(conversation, messages)}
          </p>
          <p className={`mt-2 text-[10px] uppercase tracking-[0.16em] ${active ? 'text-zinc-600' : 'text-zinc-500'}`}>
            {formatThreadDate(conversation.created_at)} · last {formatThreadDate(latestThreadTimestamp(conversation, messages))}
          </p>
        </div>
        <div className="flex shrink-0 items-center gap-2">
          {unreadCount > 0 ? (
            <>
              <span className={`h-2.5 w-2.5 rounded-full ${active ? 'bg-zinc-950' : 'bg-emerald-400'}`} />
              <span className={`text-[11px] font-medium ${active ? 'text-zinc-800' : 'text-zinc-300'}`}>
                {unreadCount}
              </span>
            </>
          ) : null}
        </div>
      </div>
      <div className="mt-3 flex flex-wrap gap-2">
        <span className={`rounded-full border px-2 py-1 text-[10px] uppercase tracking-[0.16em] ${
          active ? 'border-zinc-300 text-zinc-800' : 'border-zinc-700 text-zinc-400'
        }`}>
          {conversation.kind.replaceAll('_', ' ')}
        </span>
        {conversation.continuation ? (
          <span className={`rounded-full border px-2 py-1 text-[10px] uppercase tracking-[0.16em] ${
            active ? 'border-zinc-300 text-zinc-800' : 'border-zinc-700 text-zinc-400'
          }`}>
            {continuationTokenLabel(conversation.continuation.continuation.continuation_category)}
          </span>
        ) : null}
      </div>
    </button>
  );
}

function latestThreadTimestamp(conversation: ConversationData, messages: MessageData[]): number {
  const latestMessage = [...messages].sort((left, right) => right.created_at - left.created_at)[0];
  return latestMessage?.created_at ?? conversation.updated_at;
}

function formatThreadDate(timestamp: number): string {
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
    }).format(new Date(timestamp * 1000));
  } catch {
    return String(timestamp);
  }
}
