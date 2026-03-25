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
  compact?: boolean;
  tuiMode?: boolean;
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
  compact = false,
  tuiMode = false,
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

  const messageText =
    textContent?.text
    || (message.kind === 'system_notice' && textContent?.text)
    || null;

  if (tuiMode) {
    const sender = isUser ? '$' : '>';
    const senderClass = isUser
      ? 'text-emerald-300'
      : 'text-[var(--vel-color-accent-strong)]';
    const roleLabel = isUser ? 'YOU' : 'VEL';
  const body = messageText
      ? messageText
      : shouldShowRawFallback
        ? JSON.stringify(message.content, null, 2)
        : '[empty]';
    const whyAction = onShowWhy && message.kind !== 'text' ? (
      <button
        type="button"
        onClick={() => onShowWhy(message.id)}
        className="mt-0.5 text-[9px] uppercase tracking-[0.14em] text-[var(--vel-color-accent-soft)] hover:text-[var(--vel-color-text)]"
      >
        show why
      </button>
    ) : null;

    return (
      <div className={cn('mb-1 flex', isUser ? 'justify-end' : 'justify-start')}>
        <div className={cn('max-w-full', isUser ? 'text-right' : 'text-left')}>
          <p className={cn('inline-flex items-center gap-2 uppercase tracking-[0.08em] text-[var(--vel-color-muted)]', compact ? 'text-[9px]' : 'text-[10px]')}>
            <span className={senderClass}>{sender}</span>
            <span className={cn('font-semibold', compact ? 'text-[10px]' : 'text-[11px]')}>{roleLabel}</span>
            <span>{formatMessageTime(message.created_at).toUpperCase()}</span>
            {message.status === 'sending' ? <span>...</span> : null}
          </p>
          {body ? (
            <pre className={cn('mt-0.5 whitespace-pre-wrap break-words py-px leading-tight text-[var(--vel-color-text)]', compact ? 'text-[9px]' : 'text-[10px]')}>
              {body}
            </pre>
          ) : null}
          {whyAction}
        </div>
      </div>
    );
  }
  const hasInterventionActions =
    interventionId && (onSnooze || onResolve || onDismiss || onShowWhy);

  const metaTimeClass = isUser ? 'text-emerald-600/80' : 'text-[#c9a082]/75';
  const metaTextClass = cn(
    'inline-flex items-center gap-1.5 font-medium uppercase leading-none',
    compact ? 'text-[8px] tracking-[0.12em]' : 'text-[10px] tracking-[0.14em]',
  );
  const cardContentClass = cn('min-w-0', compact ? 'mt-1 space-y-1' : 'mt-2 space-y-2');
  const rowClass = compact ? 'gap-1' : 'gap-3';

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
      className={cn(`flex ${isUser ? 'justify-end' : 'justify-start'}`, compact ? 'mb-1.5' : 'mb-3')}
      data-message-id={message.id}
    >
      <ChatBubbleChrome variant={isUser ? 'user' : 'assistant'}>
        <NowItemRowLayout actions={interventionHeaderActions} actionsLayout="inline">
          <div className={cn('flex items-start justify-between', rowClass)}>
            <div className="min-w-0">
              <p className={cn(metaTextClass, titleClass)}>
                <span>{isUser ? 'YOU' : 'VEL'}</span>
                <span className={metaTimeClass}>|</span>
                <span className={metaTimeClass}>{formatMessageTime(message.created_at).toUpperCase()}</span>
              </p>
            </div>
            <div className={cn('flex min-w-0 flex-wrap justify-end', compact ? 'gap-1' : 'gap-1.5')}>
              <MessageTypeTag variant={isUser ? 'user' : 'assistant'} className="opacity-35">
                {isUser ? 'USER TEXT' : 'ASSISTANT TEXT'}
              </MessageTypeTag>
              {!isUser && textContent ? (
                <button
                  type="button"
                  aria-label="Copy assistant message"
                  onClick={() => void handleCopyMessage()}
                  className={cn(
                    'inline-flex items-center justify-center rounded-full border border-[#ff6b00]/35 bg-[rgba(74,36,18,0.22)] text-[#c9a082] transition hover:text-zinc-100',
                    compact ? 'h-5 w-5' : 'h-6 w-6',
                  )}
                >
                  <CopyIcon size={11} />
                </button>
              ) : null}
              {message.status === 'sending' ? (
                <MessageTypeTag variant={isUser ? 'user' : 'assistant'}>Sending…</MessageTypeTag>
              ) : null}
            </div>
          </div>
          <div className={cardContentClass}>{cardContent}</div>
        </NowItemRowLayout>
      </ChatBubbleChrome>
    </div>
  );
}
