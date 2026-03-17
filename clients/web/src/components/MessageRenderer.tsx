import {
  decodeReminderCardContent,
  decodeRiskCardContent,
  decodeSuggestionCardContent,
  decodeSummaryCardContent,
  decodeTextMessageContent,
  type MessageData,
} from '../types';
import {
  ReminderCardView,
  RiskCardView,
  SuggestionCardView,
  SummaryCardView,
} from './cards';
import { MarkdownMessage } from './MarkdownMessage';

interface MessageRendererProps {
  message: MessageData;
  interventionId?: string | null;
  onSnooze?: (interventionId: string) => void;
  onResolve?: (interventionId: string) => void;
  onDismiss?: (interventionId: string) => void;
  onShowWhy?: (messageId: string) => void;
}

export function MessageRenderer({
  message,
  interventionId,
  onSnooze,
  onResolve,
  onDismiss,
  onShowWhy,
}: MessageRendererProps) {
  const isUser = message.role === 'user';
  const textContent = decodeTextMessageContent(message.content);
  const reminderCardContent = decodeReminderCardContent(message.content);
  const riskCardContent = decodeRiskCardContent(message.content);
  const suggestionCardContent = decodeSuggestionCardContent(message.content);
  const summaryCardContent = decodeSummaryCardContent(message.content);
  const shouldShowRawFallback =
    (message.kind === 'text' || message.kind === 'system_notice') && !textContent
    || (message.kind === 'reminder_card' && !reminderCardContent)
    || (message.kind === 'risk_card' && !riskCardContent)
    || (message.kind === 'suggestion_card' && !suggestionCardContent)
    || (message.kind === 'summary_card' && !summaryCardContent);
  const hasActions =
    interventionId && (onSnooze || onResolve || onDismiss || onShowWhy);

  const cardContent = (
    <>
      {message.kind === 'text' && textContent && (
        <MarkdownMessage text={textContent.text} />
      )}
      {message.kind === 'reminder_card' && reminderCardContent && (
        <ReminderCardView content={reminderCardContent} />
      )}
      {message.kind === 'risk_card' && riskCardContent && (
        <RiskCardView content={riskCardContent} />
      )}
      {message.kind === 'suggestion_card' && suggestionCardContent && (
        <SuggestionCardView content={suggestionCardContent} />
      )}
      {message.kind === 'summary_card' && summaryCardContent && (
        <SummaryCardView content={summaryCardContent} />
      )}
      {message.kind === 'system_notice' && textContent && (
        <MarkdownMessage text={textContent.text} muted />
      )}
      {(shouldShowRawFallback
        || !['text', 'reminder_card', 'risk_card', 'suggestion_card', 'summary_card', 'system_notice'].includes(message.kind)) && (
        <pre className="text-sm overflow-x-auto whitespace-pre-wrap break-words text-zinc-400">
          {JSON.stringify(message.content, null, 2)}
        </pre>
      )}
      {hasActions && interventionId && (
        <div className="flex flex-wrap gap-2 mt-2 pt-2 border-t border-zinc-700">
          {onSnooze && (
            <button
              type="button"
              onClick={() => onSnooze(interventionId)}
              className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
            >
              Snooze
            </button>
          )}
          {onResolve && (
            <button
              type="button"
              onClick={() => onResolve(interventionId)}
              className="text-xs px-2 py-1 rounded bg-emerald-800 hover:bg-emerald-700 text-emerald-100"
            >
              Resolve
            </button>
          )}
          {onDismiss && (
            <button
              type="button"
              onClick={() => onDismiss(interventionId)}
              className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
            >
              Dismiss
            </button>
          )}
          {onShowWhy && (
            <button
              type="button"
              onClick={() => onShowWhy(message.id)}
              className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
            >
              Show why
            </button>
          )}
        </div>
      )}
    </>
  );

  return (
    <div
      className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-3`}
      data-message-id={message.id}
    >
      <div
        className={`max-w-[85%] rounded-lg px-3 py-2 ${
          isUser ? 'bg-emerald-900/50 text-emerald-100' : 'bg-zinc-800 text-zinc-200'
        }`}
      >
        <div className="text-xs text-zinc-500 mb-1">
          {message.role} · {message.kind.replace(/_/g, ' ')}
        </div>
        {cardContent}
      </div>
    </div>
  );
}
