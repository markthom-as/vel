import type { MessageData } from '../types';
import {
  ReminderCardView,
  RiskCardView,
  SuggestionCardView,
  SummaryCardView,
} from './cards';

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
  const content = message.content as Record<string, unknown> | null;
  const hasActions =
    interventionId && (onSnooze || onResolve || onDismiss || onShowWhy);

  const cardContent = (
    <>
      {message.kind === 'text' && content && 'text' in content && (
        <p className="whitespace-pre-wrap text-zinc-200">{(content as { text: string }).text}</p>
      )}
      {message.kind === 'reminder_card' && content && (
        <ReminderCardView content={content as unknown as { title: string; due_time?: number; reason?: string; confidence?: number }} />
      )}
      {message.kind === 'risk_card' && content && (
        <RiskCardView content={content as unknown as { commitment_title: string; risk_level: string; top_drivers?: string[]; proposed_next_step?: string }} />
      )}
      {message.kind === 'suggestion_card' && content && (
        <SuggestionCardView content={content as unknown as { suggestion_text: string; linked_goal?: string; expected_benefit?: string }} />
      )}
      {message.kind === 'summary_card' && content && (
        <SummaryCardView content={content as unknown as { title: string; timeframe?: string; top_items?: string[]; recommended_actions?: string[] }} />
      )}
      {message.kind === 'system_notice' && content && 'text' in content && (
        <p className="text-zinc-400 italic">{(content as { text: string }).text}</p>
      )}
      {!['text', 'reminder_card', 'risk_card', 'suggestion_card', 'summary_card', 'system_notice'].includes(message.kind) && (
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
