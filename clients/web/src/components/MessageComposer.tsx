import { useState, useCallback } from 'react';
import { apiPost } from '../api/client';
import type { ApiResponse, MessageData } from '../types';

interface MessageComposerProps {
  conversationId: string;
  onSent: (message: MessageData) => void;
}

export function MessageComposer({ conversationId, onSent }: MessageComposerProps) {
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const send = useCallback(async () => {
    const trimmed = text.trim();
    if (!trimmed || sending) return;
    setError(null);
    setSending(true);
    try {
      const res = await apiPost<ApiResponse<MessageData>>(
        `/api/conversations/${conversationId}/messages`,
        { role: 'user', kind: 'text', content: { text: trimmed } }
      );
      if (res.ok && res.data) {
        onSent(res.data);
        setText('');
      } else {
        setError(res.error?.message ?? 'Send failed');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Send failed');
    } finally {
      setSending(false);
    }
  }, [text, conversationId, sending, onSent]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  return (
    <div className="shrink-0 border-t border-zinc-800 p-3">
      {error && (
        <p className="max-w-2xl mx-auto mb-2 text-red-400 text-sm" role="alert">
          {error}
        </p>
      )}
      <div className="flex gap-2 max-w-2xl mx-auto">
        <textarea
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Message… (Enter to send, Shift+Enter for newline)"
          rows={2}
          className="flex-1 rounded-lg bg-zinc-800/50 border border-zinc-700 px-3 py-2 text-zinc-100 placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-emerald-500/50 resize-none"
          disabled={sending}
        />
        <button
          type="button"
          onClick={send}
          disabled={sending || !text.trim()}
          className="shrink-0 px-4 py-2 rounded-lg bg-emerald-700 text-white font-medium hover:bg-emerald-600 disabled:opacity-50 disabled:pointer-events-none"
        >
          {sending ? '…' : 'Send'}
        </button>
      </div>
    </div>
  );
}
