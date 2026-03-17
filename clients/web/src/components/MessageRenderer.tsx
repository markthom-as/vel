import {
  decodeReminderCardContent,
  decodeRiskCardContent,
  decodeSuggestionCardContent,
  decodeSummaryCardContent,
  decodeTextMessageContent,
  type MessageActionContent,
  type MessageData,
} from '../types';
import {
  ReminderCardView,
  RiskCardView,
  SuggestionCardView,
  SummaryCardView,
} from './cards';
import { MarkdownMessage } from './MarkdownMessage';
import { useState } from 'react';

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
  const [copiedAction, setCopiedAction] = useState<string | null>(null);
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
  const hasInterventionActions =
    interventionId && (onSnooze || onResolve || onDismiss || onShowWhy);

  async function handleCopyAction(label: string, value: string) {
    if (typeof navigator === 'undefined' || !navigator.clipboard?.writeText) {
      return;
    }
    await navigator.clipboard.writeText(value);
    setCopiedAction(label);
    window.setTimeout(() => setCopiedAction((current) => (current === label ? null : current)), 1200);
  }

  function renderMessageActions(actions: MessageActionContent[] | undefined) {
    if (!actions || actions.length === 0) {
      return null;
    }

    return (
      <div className="flex flex-wrap gap-2 mt-3">
        {actions.map((action, index) => {
          const key = `${action.action_type}-${action.label}-${index}`;
          if (action.action_type === 'open_url' && action.url) {
            return (
              <a
                key={key}
                href={action.url}
                target="_blank"
                rel="noreferrer"
                className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              >
                {action.label}
              </a>
            );
          }

          if (action.action_type === 'copy_text' && action.value) {
            const label = copiedAction === action.label ? 'Copied' : action.label;
            return (
              <button
                key={key}
                type="button"
                onClick={() => void handleCopyAction(action.label, action.value!)}
                className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              >
                {label}
              </button>
            );
          }

          if (action.action_type === 'show_why' && onShowWhy) {
            return (
              <button
                key={key}
                type="button"
                onClick={() => onShowWhy(message.id)}
                className="text-xs px-2 py-1 rounded bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              >
                {action.label}
              </button>
            );
          }

          return null;
        })}
      </div>
    );
  }

  const cardContent = (
    <>
      {message.kind === 'text' && textContent && (
        <>
          <MarkdownMessage text={textContent.text} />
          {renderMessageActions(textContent.actions)}
        </>
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
        <>
          <MarkdownMessage text={textContent.text} muted />
          {renderMessageActions(textContent.actions)}
        </>
      )}
      {(shouldShowRawFallback
        || !['text', 'reminder_card', 'risk_card', 'suggestion_card', 'summary_card', 'system_notice'].includes(message.kind)) && (
        <pre className="text-sm overflow-x-auto whitespace-pre-wrap break-words text-zinc-400">
          {JSON.stringify(message.content, null, 2)}
        </pre>
      )}
      {hasInterventionActions && interventionId && (
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
