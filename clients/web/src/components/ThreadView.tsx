import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import type { InboxItemData, MessageData } from '../types';
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
}

export function ThreadView({ conversationId }: ThreadViewProps) {
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [], loading: conversationsLoading, error: conversationsError } = useQuery(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
    { enabled: !conversationId },
  );
  const fallbackConversationId = useMemo(() => {
    if (conversationId || conversations.length === 0) {
      return null;
    }
    return [...conversations]
      .sort((left, right) => right.updated_at - left.updated_at)[0]?.id ?? null;
  }, [conversationId, conversations]);
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
          {messages.length === 0 && (
            <SurfaceState message="No messages yet." />
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
