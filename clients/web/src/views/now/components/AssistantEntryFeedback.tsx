import type { AssistantEntryResponse } from '../../../types';
import { uiTheme } from '../../../core/Theme';

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
          className={`rounded-2xl px-4 py-3 text-sm ${
            message.status === 'error'
              ? 'border border-rose-800/60 bg-rose-950/20 text-rose-200'
              : uiTheme.brandAssistantBubble
          }`}
        >
          {message.message}
        </div>
      ) : null}
      {inlineResponse?.assistant_message ? (
        <div className={`rounded-2xl px-4 py-3 ${uiTheme.brandAssistantBubble}`}>
          <p className={`text-xs uppercase tracking-[0.16em] ${uiTheme.brandAssistantBubbleMeta}`}>
            Inline
          </p>
          <p className="mt-2 text-sm text-zinc-100">
            {renderInlineAssistantText(inlineResponse.assistant_message.content)}
          </p>
          {assistantEntryThreadId ? (
            <button
              type="button"
              onClick={() => onOpenThread?.(assistantEntryThreadId)}
              className="mt-3 rounded-full border border-[#ff6b00]/45 bg-zinc-950/60 px-3 py-2 text-xs uppercase tracking-[0.14em] text-[#ffb27a] transition hover:border-[#ff8f40]/65 hover:text-zinc-100"
            >
              Open Thread
            </button>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}
