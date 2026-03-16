import { useCallback, useEffect, useState } from 'react';
import { apiGet } from '../api/client';
import type { ApiResponse, ConversationData, WsEnvelope } from '../types';
import { subscribeWs } from '../realtime/ws';

interface ConversationListProps {
  selectedId: string | null;
  onSelect: (id: string) => void;
}

export function ConversationList({ selectedId, onSelect }: ConversationListProps) {
  const [conversations, setConversations] = useState<ConversationData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadConversations = useCallback((showSpinner: boolean) => {
    let cancelled = false;
    if (showSpinner) {
      setLoading(true);
    }
    setError(null);
    apiGet<ApiResponse<ConversationData[]>>('/api/conversations')
      .then((res) => {
        if (!cancelled && res.ok && res.data) setConversations(res.data);
      })
      .catch((err) => {
        if (!cancelled) setError(err instanceof Error ? err.message : 'Failed to load');
      })
      .finally(() => {
        if (!cancelled && showSpinner) setLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  useEffect(() => loadConversations(true), [loadConversations]);

  useEffect(() => {
    return subscribeWs((event: WsEnvelope) => {
      if (event.type === 'messages:new') {
        loadConversations(false);
      }
    });
  }, [loadConversations]);

  if (loading) return <div className="p-3 text-zinc-500 text-sm">Loading…</div>;
  if (error) return <div className="p-3 text-red-400 text-sm">{error}</div>;

  return (
    <ul className="flex-1 overflow-y-auto">
      {conversations.map((c) => (
        <li key={c.id}>
          <button
            type="button"
            onClick={() => onSelect(c.id)}
            className={`w-full text-left px-3 py-2 border-b border-zinc-800/50 hover:bg-zinc-800/50 flex items-center gap-2 ${
              selectedId === c.id ? 'bg-zinc-800 text-white' : ''
            }`}
          >
            {c.pinned && <span className="text-amber-500" aria-label="Pinned">📌</span>}
            <span className="flex-1 truncate">{c.title || 'Untitled'}</span>
            <span className="text-xs text-zinc-500 shrink-0">
              {formatTs(c.updated_at)}
            </span>
          </button>
        </li>
      ))}
      {conversations.length === 0 && (
        <li className="p-3 text-zinc-500 text-sm">No conversations yet.</li>
      )}
    </ul>
  );
}

function formatTs(ts: number): string {
  const d = new Date(ts * 1000);
  const now = new Date();
  if (d.toDateString() === now.toDateString()) {
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  }
  return d.toLocaleDateString([], { month: 'short', day: 'numeric' });
}
