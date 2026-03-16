import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { apiPost } from '../api/client';
import type { InboxItemData, MessageData, WsEvent } from '../types';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import { loadConversationInterventions, loadConversationMessages, queryKeys } from '../data/resources';
import { MessageRenderer } from './MessageRenderer';
import { MessageComposer } from './MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { subscribeWs } from '../realtime/ws';

interface ThreadViewProps {
  conversationId: string | null;
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

  const interventionsByMessageId = interventions.reduce<Record<string, string>>((next, intervention) => {
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
          appendUniqueMessages(prev, [message]),
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
          (prev = []) => upsertInboxItem(prev, payload),
        );
        setQueryData<InboxItemData[]>(inboxKey, (prev = []) =>
          upsertInboxItem(prev, payload),
        );
        return;
      }
      if (event.type === 'interventions:updated') {
        invalidateQuery(interventionsKey, { refetch: true });
        invalidateQuery(inboxKey, { refetch: true });
      }
    });
  }, [conversationId, conversationsKey, inboxKey, interventionsKey, messages, messagesKey]);

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

  const handleSnooze = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/snooze`, { minutes: 15 });
      removeIntervention(interventionId);
    } catch (_) {}
  }, [removeIntervention]);

  const handleResolve = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/resolve`, {});
      removeIntervention(interventionId);
    } catch (_) {}
  }, [removeIntervention]);

  const handleDismiss = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/dismiss`, {});
      removeIntervention(interventionId);
    } catch (_) {}
  }, [removeIntervention]);

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
        onSent={(userMessage, assistantMessage) => {
          setQueryData<MessageData[]>(messagesKey, (prev = []) =>
            appendUniqueMessages(prev, [
              userMessage,
              ...(assistantMessage ? [assistantMessage] : []),
            ]),
          );
          invalidateQuery(conversationsKey, { refetch: true });
          void refetchMessages();
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

function upsertInboxItem(items: InboxItemData[], nextItem: InboxItemData): InboxItemData[] {
  const existingIndex = items.findIndex((item) => item.id === nextItem.id);
  if (existingIndex === -1) {
    return [nextItem, ...items];
  }
  const next = [...items];
  next[existingIndex] = nextItem;
  return next;
}
