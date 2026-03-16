import { useEffect, useMemo } from 'react';
import type { ConversationData, WsEvent } from '../types';
import { invalidateQuery, useQuery } from '../data/query';
import { loadConversationList, queryKeys } from '../data/resources';
import { subscribeWs } from '../realtime/ws';

interface ConversationListProps {
  selectedId: string | null;
  onSelect: (id: string) => void;
}

export function ConversationList({ selectedId, onSelect }: ConversationListProps) {
  const conversationsKey = useMemo(() => queryKeys.conversations(), []);
  const { data: conversations = [], loading, error } = useQuery<ConversationData[]>(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWs((event: WsEvent) => {
      if (event.type === 'messages:new') {
        invalidateQuery(conversationsKey, { refetch: true });
      }
    });
  }, [conversationsKey]);

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
