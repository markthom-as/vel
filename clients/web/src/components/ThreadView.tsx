import { useEffect, useState, useCallback } from 'react';
import { apiGet, apiPost } from '../api/client';
import type { ApiResponse, MessageData, InboxItemData } from '../types';
import { MessageRenderer } from './MessageRenderer';
import { MessageComposer } from './MessageComposer';
import { ProvenanceDrawer } from './ProvenanceDrawer';

interface ThreadViewProps {
  conversationId: string | null;
}

export function ThreadView({ conversationId }: ThreadViewProps) {
  const [messages, setMessages] = useState<MessageData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [interventionsByMessageId, setInterventionsByMessageId] = useState<Record<string, string>>({});
  const [provenanceMessageId, setProvenanceMessageId] = useState<string | null>(null);

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

  // Fetch interventions for assistant messages (for inline actions)
  useEffect(() => {
    if (messages.length === 0) return;
    const assistantMessages = messages.filter((m) => m.role !== 'user');
    let cancelled = false;
    const map: Record<string, string> = {};
    Promise.all(
      assistantMessages.map((m) =>
        apiGet<ApiResponse<InboxItemData[]>>(`/api/messages/${m.id}/interventions`).then((res) => {
          if (!cancelled && res.ok && res.data && res.data.length > 0) {
            map[m.id] = res.data[0].id;
          }
        }).catch(() => {})
      )
    ).then(() => {
      if (!cancelled) setInterventionsByMessageId((prev) => ({ ...prev, ...map }));
    });
    return () => { cancelled = true; };
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
      <div className="flex-1 overflow-y-auto p-4 relative">
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
      <MessageComposer
        conversationId={conversationId}
        onSent={(msg) => setMessages((prev) => [...prev, msg])}
      />
    </>
  );
}
