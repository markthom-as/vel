import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { apiPost } from '../api/client';
import type { InboxItemData, MessageData, WsEvent } from '../types';
import { getQueryData, invalidateQuery, setQueryData, useQuery } from '../data/query';
import { loadConversationInterventions, loadConversationMessages, queryKeys } from '../data/resources';
import { MessageRenderer } from './MessageRenderer';
import { MessageComposer } from './MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { subscribeWs } from '../realtime/ws';

interface ThreadViewProps {
  conversationId: string | null;
}

interface PendingInterventionAction {
  state: string;
  confirmed: boolean;
}

export function ThreadView({ conversationId }: ThreadViewProps) {
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const messagesKey = useMemo(() => queryKeys.conversationMessages(conversationId), [conversationId]);
  const interventionsKey = useMemo(
    () => queryKeys.conversationInterventions(conversationId),
    [conversationId],
  );
  const conversationsKey = useMemo(() => queryKeys.conversations(), []);
  const inboxKey = useMemo(() => queryKeys.inbox(), []);
  const pendingInterventionActionsKey = useMemo(
    () => queryKeys.pendingInterventionActions(),
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
      if (!conversationId) {
        return [];
      }
      const response = await loadConversationMessages(conversationId);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: Boolean(conversationId) },
  );
  const {
    data: interventions = [],
    error: interventionsError,
  } = useQuery<InboxItemData[]>(
    interventionsKey,
    async () => {
      if (!conversationId) {
        return [];
      }
      const response = await loadConversationInterventions(conversationId);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: Boolean(conversationId) },
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
    if (!conversationId) {
      return () => {};
    }

    return subscribeWs((event: WsEvent) => {
      if (event.type === 'messages:new') {
        const message = event.payload;
        if (message.conversation_id !== conversationId) {
          return;
        }
        setQueryData<MessageData[]>(messagesKey, (prev = []) =>
          reconcileIncomingMessage(prev, message),
        );
        invalidateQuery(conversationsKey, { refetch: true });
        return;
      }
      if (event.type === 'interventions:new') {
        const payload = event.payload;
        if (!messages.some((message) => message.id === payload.message_id)) {
          return;
        }
        setQueryData<InboxItemData[]>(
          interventionsKey,
          (prev = []) => upsertInboxItem(prev, payload, pendingInterventionActions),
        );
        setQueryData<InboxItemData[]>(inboxKey, (prev = []) =>
          upsertInboxItem(prev, payload, pendingInterventionActions),
        );
        return;
      }
      if (event.type === 'interventions:updated') {
        setQueryData<Record<string, PendingInterventionAction>>(pendingInterventionActionsKey, (prev = {}) => {
          const pendingAction = prev[event.payload.id];
          if (!pendingAction || pendingAction.state !== event.payload.state) {
            return prev;
          }

          return {
            ...prev,
            [event.payload.id]: {
              ...pendingAction,
              confirmed: true,
            },
          };
        });
        invalidateQuery(interventionsKey, { refetch: true });
        invalidateQuery(inboxKey, { refetch: true });
      }
    });
  }, [
    conversationId,
    conversationsKey,
    inboxKey,
    interventionsKey,
    messages,
    messagesKey,
    pendingInterventionActions,
    pendingInterventionActionsKey,
  ]);

  useEffect(() => {
    setQueryData<Record<string, PendingInterventionAction>>(pendingInterventionActionsKey, (prev = {}) => {
      let changed = false;
      const next: Record<string, PendingInterventionAction> = {};

      for (const [interventionId, pendingAction] of Object.entries(prev)) {
        if (!pendingAction.confirmed || interventions.some((intervention) => intervention.id === interventionId)) {
          next[interventionId] = pendingAction;
        } else {
          changed = true;
        }
      }

      return changed ? next : prev;
    });
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
      (prev = []) => prev.filter((item) => item.id !== interventionId),
    );
    setQueryData<InboxItemData[]>(
      inboxKey,
      (prev = []) => prev.filter((item) => item.id !== interventionId),
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
    setQueryData<Record<string, PendingInterventionAction>>(pendingInterventionActionsKey, (prev = {}) => ({
      ...prev,
      [interventionId]: {
        state,
        confirmed: false,
      },
    }));
  }, [pendingInterventionActionsKey]);

  const handleSnooze = useCallback(async (interventionId: string) => {
    const previousConversationInterventions = getQueryData<InboxItemData[]>(interventionsKey);
    const previousInboxItems = getQueryData<InboxItemData[]>(inboxKey);
    startInterventionAction(interventionId, 'snoozed');
    removeIntervention(interventionId);
    try {
      await apiPost(`/api/interventions/${interventionId}/snooze`, { minutes: 15 });
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
      await apiPost(`/api/interventions/${interventionId}/resolve`, {});
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
      await apiPost(`/api/interventions/${interventionId}/dismiss`, {});
      invalidateQuery(interventionsKey, { refetch: true });
      invalidateQuery(inboxKey, { refetch: true });
    } catch (_) {
      restoreInterventions(interventionId, previousConversationInterventions, previousInboxItems);
    }
  }, [inboxKey, interventionsKey, removeIntervention, restoreInterventions, startInterventionAction]);

  if (!conversationId) {
    return (
      <div className="flex-1 flex items-center justify-center text-zinc-500">
        Select a conversation
      </div>
    );
  }

  const error = messagesError ?? interventionsError;
  if (messagesLoading) {
    return <div className="flex-1 flex items-center justify-center text-zinc-500">Loading…</div>;
  }
  if (error) {
    return <div className="flex-1 flex items-center justify-center text-red-400">{error}</div>;
  }

  return (
    <>
      <div ref={scrollRef} className="flex-1 overflow-y-auto p-4 relative">
        <div className="max-w-2xl mx-auto">
          {messages.length === 0 && (
            <p className="text-zinc-500 text-sm">No messages yet.</p>
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
        To get assistant replies, configure a chat model in configs/models/routing.toml and run the model backend.
      </p>
      <MessageComposer
        conversationId={conversationId}
        onOptimisticSend={(text) => {
          const clientMessageId = `tmp_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
          const optimisticMessage: MessageData = {
            id: clientMessageId,
            conversation_id: conversationId,
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
        onSent={(clientMessageId, userMessage, assistantMessage) => {
          setQueryData<MessageData[]>(messagesKey, (prev = []) =>
            reconcileConfirmedSend(
              prev,
              clientMessageId,
              userMessage,
              assistantMessage ? [assistantMessage] : [],
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

function appendUniqueMessages(existing: MessageData[], nextMessages: MessageData[]): MessageData[] {
  if (nextMessages.length === 0) {
    return existing;
  }

  const seen = new Set(existing.map((message) => message.id));
  const additions = nextMessages.filter((message) => {
    if (seen.has(message.id)) {
      return false;
    }
    seen.add(message.id);
    return true;
  });

  return additions.length > 0 ? [...existing, ...additions] : existing;
}

function reconcileIncomingMessage(existing: MessageData[], incoming: MessageData): MessageData[] {
  const existingIndex = existing.findIndex((message) => message.id === incoming.id);
  if (existingIndex !== -1) {
    const next = [...existing];
    next[existingIndex] = incoming;
    return next;
  }

  const pendingIndex = existing.findIndex(
    (message) =>
      message.status === 'sending'
      && message.conversation_id === incoming.conversation_id
      && message.role === incoming.role
      && message.kind === incoming.kind
      && JSON.stringify(message.content) === JSON.stringify(incoming.content),
  );
  if (pendingIndex !== -1) {
    const next = [...existing];
    next[pendingIndex] = incoming;
    return next;
  }

  return [...existing, incoming];
}

function reconcileConfirmedSend(
  existing: MessageData[],
  clientMessageId: string | undefined,
  userMessage: MessageData,
  assistantMessages: MessageData[],
): MessageData[] {
  let next = clientMessageId
    ? existing.filter((message) => message.id !== clientMessageId)
    : [...existing];
  next = reconcileIncomingMessage(next, userMessage);
  for (const assistantMessage of assistantMessages) {
    next = reconcileIncomingMessage(next, assistantMessage);
  }
  return next;
}

function upsertInboxItem(
  items: InboxItemData[],
  nextItem: InboxItemData,
  pendingInterventionActions: Record<string, PendingInterventionAction>,
): InboxItemData[] {
  if (pendingInterventionActions[nextItem.id]) {
    return items;
  }
  const existingIndex = items.findIndex((item) => item.id === nextItem.id);
  if (existingIndex === -1) {
    return [nextItem, ...items];
  }
  const next = [...items];
  next[existingIndex] = nextItem;
  return next;
}
