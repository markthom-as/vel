import type { AssistantEntryResponse } from '../../../types';

function renderInlineAssistantText(content: unknown): string {
  if (typeof content === 'string') {
    return content;
  }
  if (content && typeof content === 'object' && 'text' in (content as Record<string, unknown>)) {
    const text = (content as Record<string, unknown>).text;
    return typeof text === 'string' ? text : 'Handled here in Now.';
  }
  return 'Handled here in Now.';
}

export function AssistantEntryFeedback({
  message,
  inlineResponse,
  assistantEntryThreadId,
  onOpenThread,
}: {
  message: { status: 'success' | 'error'; message: string } | null;
  inlineResponse: AssistantEntryResponse | null;
  assistantEntryThreadId: string | null;
  onOpenThread?: (conversationId: string) => void;
}) {
  if (!message && !inlineResponse) {
    return null;
  }

  return (
    <div className="space-y-2">
      {message ? (
        <div
          className={`rounded-2xl border px-4 py-3 text-sm ${
            message.status === 'error'
              ? 'border-rose-800/60 bg-rose-950/20 text-rose-200'
              : 'border-emerald-800/60 bg-emerald-950/20 text-emerald-200'
          }`}
        >
          {message.message}
        </div>
      ) : null}
      {inlineResponse?.assistant_message ? (
        <div className="rounded-2xl border border-zinc-800 bg-zinc-900/50 px-4 py-3">
          <p className="text-[10px] uppercase tracking-[0.2em] text-zinc-500">Inline</p>
          <p className="mt-2 text-sm text-zinc-200">
            {renderInlineAssistantText(inlineResponse.assistant_message.content)}
          </p>
          {assistantEntryThreadId ? (
            <button
              type="button"
              onClick={() => onOpenThread?.(assistantEntryThreadId)}
              className="mt-3 rounded-full border border-zinc-700 bg-zinc-950/80 px-3 py-2 text-[10px] uppercase tracking-[0.18em] text-zinc-300 transition hover:border-zinc-500 hover:text-zinc-100"
            >
              Open Thread
            </button>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}
