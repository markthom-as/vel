import {
  decodeReminderCardContent,
  decodeRiskCardContent,
  decodeSuggestionCardContent,
  decodeSummaryCardContent,
  decodeTextMessageContent,
  type MessageActionContent,
  type MessageData,
} from '../../types';
import {
  ReminderCardView,
  RiskCardView,
  SuggestionCardView,
  SummaryCardView,
} from '../Cards';
import { MarkdownMessage } from '../MarkdownMessage';
import { cn } from '../cn';
import { ActionChipButton, ActionChipLink, MessageTypeTag } from '../FilterToggleTag';
import { ChatBubbleChrome } from '../MessageBubble';
import { CopyIcon } from '../Icons';
import { NowItemRowLayout } from '../NowItemRow';
import { useState } from 'react';

interface MessageRendererProps {
  message: MessageData;
  interventionId?: string | null;
  onSnooze?: (interventionId: string) => void;
  onResolve?: (interventionId: string) => void;
  onDismiss?: (interventionId: string) => void;
  onShowWhy?: (messageId: string) => void;
}

function formatMessageTime(createdAt: number): string {
  try {
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    }).format(new Date(createdAt * 1000));
  } catch {
    return '';
  }
}

function attachmentDescriptor(attachment: { kind: string; label?: string | null; mime_type?: string | null; metadata?: unknown }): string {
  const label = attachment.label?.trim() || attachment.mime_type?.trim() || attachment.kind.replaceAll('_', ' ');
  if (!attachment.mime_type) {
    return label;
  }
  return `${label} · ${attachment.mime_type}`;
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

  const metaTimeClass = isUser ? 'text-emerald-600/80' : 'text-[#c9a082]/75';

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
      <div className="mt-2 flex flex-wrap gap-1.5">
        {actions.map((action, index) => {
          const key = `${action.action_type}-${action.label}-${index}`;
          if (action.action_type === 'open_url' && action.url) {
            return (
              <ActionChipLink key={key} href={action.url} target="_blank" rel="noreferrer" variant="message">
                {action.label}
              </ActionChipLink>
            );
          }

          if (action.action_type === 'copy_text' && action.value) {
            const label = copiedAction === action.label ? 'Copied' : action.label;
            return (
              <ActionChipButton
                key={key}
                variant="message"
                onClick={() => void handleCopyAction(action.label, action.value!)}
              >
                {label}
              </ActionChipButton>
            );
          }

          if (action.action_type === 'show_why' && onShowWhy) {
            return (
              <ActionChipButton
                key={key}
                variant="message"
                onClick={() => onShowWhy(message.id)}
              >
                {action.label}
              </ActionChipButton>
            );
          }

          return null;
        })}
      </div>
    );
  }

  const interventionHeaderActions =
    hasInterventionActions && interventionId ? (
      <>
        {onSnooze && (
          <ActionChipButton variant="message" onClick={() => onSnooze(interventionId)}>
            Snooze
          </ActionChipButton>
        )}
        {onResolve && (
          <ActionChipButton
            variant="message"
            tone="success"
            onClick={() => onResolve(interventionId)}
          >
            Resolve
          </ActionChipButton>
        )}
        {onDismiss && (
          <ActionChipButton variant="message" onClick={() => onDismiss(interventionId)}>
            Dismiss
          </ActionChipButton>
        )}
        {onShowWhy && (
          <ActionChipButton variant="message" onClick={() => onShowWhy(message.id)}>
            Show why
          </ActionChipButton>
        )}
      </>
    ) : null;

  const cardContent = (
    <>
      {message.kind === 'text' && textContent && (
        <>
          <MarkdownMessage text={textContent.text} />
          {textContent.attachments?.length ? (
            <div className="mt-2 flex flex-wrap gap-1.5">
              {textContent.attachments.map((attachment, index) => (
                <MessageTypeTag key={`${attachment.kind}-${attachment.label ?? index}-${index}`} variant={isUser ? 'user' : 'assistant'}>
                  {attachmentDescriptor(attachment)}
                </MessageTypeTag>
              ))}
            </div>
          ) : null}
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
    </>
  );

  const titleClass = isUser ? 'text-emerald-50' : 'text-zinc-100';

  async function handleCopyMessage() {
    const plainText = textContent?.text;
    if (!plainText || typeof navigator === 'undefined' || !navigator.clipboard?.writeText) {
      return;
    }
    await navigator.clipboard.writeText(plainText);
  }

  return (
    <div
      className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-3`}
      data-message-id={message.id}
    >
      <ChatBubbleChrome variant={isUser ? 'user' : 'assistant'}>
        <NowItemRowLayout actions={interventionHeaderActions} actionsLayout="inline">
          <div className="flex items-start justify-between gap-3">
            <div className="min-w-0">
              <p className={cn('inline-flex items-center gap-1.5 text-[10px] font-medium uppercase tracking-[0.14em] leading-none', titleClass)}>
                <span>{isUser ? 'YOU' : 'VEL'}</span>
                <span className={metaTimeClass}>|</span>
                <span className={metaTimeClass}>{formatMessageTime(message.created_at).toUpperCase()}</span>
              </p>
            </div>
            <div className="flex min-w-0 flex-wrap justify-end gap-1.5">
              <MessageTypeTag variant={isUser ? 'user' : 'assistant'} className="opacity-35">
                {isUser ? 'USER TEXT' : 'ASSISTANT TEXT'}
              </MessageTypeTag>
              {!isUser && textContent ? (
                <button
                  type="button"
                  aria-label="Copy assistant message"
                  onClick={() => void handleCopyMessage()}
                  className="inline-flex h-6 w-6 items-center justify-center rounded-full border border-[#ff6b00]/35 bg-[rgba(74,36,18,0.22)] text-[#c9a082] transition hover:text-zinc-100"
                >
                  <CopyIcon size={11} />
                </button>
              ) : null}
              {message.status === 'sending' ? (
                <MessageTypeTag variant={isUser ? 'user' : 'assistant'}>Sending…</MessageTypeTag>
              ) : null}
            </div>
          </div>
          <div className="mt-2 min-w-0 space-y-2">{cardContent}</div>
        </NowItemRowLayout>
      </ChatBubbleChrome>
    </div>
  );
}
