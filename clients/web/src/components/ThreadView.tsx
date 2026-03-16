import { useEffect, useState, useCallback, useRef } from 'react';
import { apiGet, apiPost } from '../api/client';
import type { ApiResponse, MessageData, InboxItemData, InterventionEventData, WsEnvelope } from '../types';
import { MessageRenderer } from './MessageRenderer';
import { MessageComposer } from './MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';
import { subscribeWs } from '../realtime/ws';

interface ThreadViewProps {
  conversationId: string | null;
}

export function ThreadView({ conversationId }: ThreadViewProps) {
  const [messages, setMessages] = useState<MessageData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [interventionsByMessageId, setInterventionsByMessageId] = useState<Record<string, string>>({});
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement | null>(null);

  const refreshInterventions = useCallback((messageList: MessageData[]) => {
    if (messageList.length === 0) {
      setInterventionsByMessageId({});
      return () => {};
    }

    let cancelled = false;
    const nextMap: Record<string, string> = {};
    apiGet<ApiResponse<InboxItemData[]>>(`/api/conversations/${conversationId}/interventions`)
      .then((res) => {
        if (cancelled || !res.ok || !res.data) {
          return;
        }
        for (const intervention of res.data) {
          if (!(intervention.message_id in nextMap)) {
            nextMap[intervention.message_id] = intervention.id;
          }
        }
        setInterventionsByMessageId(nextMap);
      })
      .catch(() => {
        if (!cancelled) {
          setInterventionsByMessageId({});
        }
      });

    return () => {
      cancelled = true;
    };
  }, [conversationId]);

  useEffect(() => {
    if (!conversationId) {
      setMessages([]);
      setInterventionsByMessageId({});
      return;
    }
    let cancelled = false;
    setLoading(true);
    setError(null);
    apiGet<ApiResponse<MessageData[]>>(`/api/conversations/${conversationId}/messages`)
      .then((res) => {
        if (!cancelled && res.ok && res.data) setMessages(res.data);
      })
      .catch((err) => {
        if (!cancelled) setError(err instanceof Error ? err.message : 'Failed to load thread');
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => { cancelled = true; };
  }, [conversationId]);

  useEffect(() => {
    return refreshInterventions(messages);
  }, [messages, refreshInterventions]);

  useEffect(() => {
    if (!conversationId) {
      return () => {};
    }

    return subscribeWs((event: WsEnvelope) => {
      if (event.type === 'messages:new') {
        const message = event.payload as Partial<MessageData>;
        if (message.conversation_id !== conversationId || typeof message.id !== 'string') {
          return;
        }

        setMessages((prev) => appendUniqueMessages(prev, [message as MessageData]));
        return;
      }
      if (event.type === 'interventions:new' && isInterventionEventData(event.payload)) {
        setInterventionsByMessageId((prev) => {
          if (!messages.some((message) => message.id === event.payload.message_id)) {
            return prev;
          }
          return {
            ...prev,
            [event.payload.message_id]: event.payload.id,
          };
        });
        return;
      }
      if (event.type === 'interventions:updated') {
        void refreshInterventions(messages);
      }
    });
  }, [conversationId, messages, refreshInterventions]);

  // Autoscroll to the bottom when messages change (standard chat behavior).
  useEffect(() => {
    const el = scrollRef.current;
    if (!el) return;
    // Smooth scroll, but fall back gracefully.
    try {
      el.scrollTo({ top: el.scrollHeight, behavior: 'smooth' });
    } catch {
      el.scrollTop = el.scrollHeight;
    }
  }, [messages]);

  const handleSnooze = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/snooze`, { minutes: 15 });
      setInterventionsByMessageId((prev) => {
        const next = { ...prev };
        for (const k of Object.keys(next)) {
          if (next[k] === interventionId) delete next[k];
        }
        return next;
      });
    } catch (_) {}
  }, []);
  const handleResolve = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/resolve`, {});
      setInterventionsByMessageId((prev) => {
        const next = { ...prev };
        for (const k of Object.keys(next)) {
          if (next[k] === interventionId) delete next[k];
        }
        return next;
      });
    } catch (_) {}
  }, []);
  const handleDismiss = useCallback(async (interventionId: string) => {
    try {
      await apiPost(`/api/interventions/${interventionId}/dismiss`, {});
      setInterventionsByMessageId((prev) => {
        const next = { ...prev };
        for (const k of Object.keys(next)) {
          if (next[k] === interventionId) delete next[k];
        }
        return next;
      });
    } catch (_) {}
  }, []);

  if (!conversationId) {
    return (
      <div className="flex-1 flex items-center justify-center text-zinc-500">
        Select a conversation
      </div>
    );
  }
  if (loading) return <div className="flex-1 flex items-center justify-center text-zinc-500">Loading…</div>;
  if (error) return <div className="flex-1 flex items-center justify-center text-red-400">{error}</div>;

  return (
    <>
      <div ref={scrollRef} className="flex-1 overflow-y-auto p-4 relative">
        <div className="max-w-2xl mx-auto">
          {messages.length === 0 && (
            <p className="text-zinc-500 text-sm">No messages yet.</p>
          )}
          {messages.map((m) => (
            <MessageRenderer
              key={m.id}
              message={m}
              interventionId={interventionsByMessageId[m.id]}
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
        onSent={(userMsg, assistantMsg) =>
          setMessages((prev) => appendUniqueMessages(prev, [userMsg, ...(assistantMsg ? [assistantMsg] : [])]))
        }
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

function isInterventionEventData(payload: unknown): payload is InterventionEventData {
  if (!payload || typeof payload !== 'object') {
    return false;
  }
  const candidate = payload as Partial<InterventionEventData>;
  return typeof candidate.id === 'string'
    && typeof candidate.message_id === 'string'
    && typeof candidate.kind === 'string'
    && typeof candidate.state === 'string'
    && typeof candidate.surfaced_at === 'number';
}
