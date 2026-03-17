import { useEffect, useMemo } from 'react';
import type { ConversationData } from '../types';
import { chatQueryKeys, loadConversationList } from '../data/chat';
import { useQuery } from '../data/query';
import { subscribeWsQuerySync } from '../data/ws-sync';
import { SurfaceState } from './SurfaceState';

interface ConversationListProps {
  selectedId: string | null;
  onSelect: (id: string) => void;
}

export function ConversationList({ selectedId, onSelect }: ConversationListProps) {
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data: conversations = [], loading, error } = useQuery<ConversationData[]>(
    conversationsKey,
    async () => {
      const response = await loadConversationList();
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  if (loading) return <SurfaceState message="Loading conversations…" />;
  if (error) return <SurfaceState message={error} tone="danger" />;

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
        <li><SurfaceState message="No conversations yet." /></li>
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
