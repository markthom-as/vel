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
import { FilterDenseTag, FilterPillButton } from '../FilterToggleTag';
import { filterPillActionIdle, filterPillFrame } from '../FilterToggleTag/filterPillClasses';
import { ChatBubbleChrome } from '../MessageBubble';
import { ItemRowTitleMetaBand, NowItemRowLayout } from '../NowItemRow';
import { useState } from 'react';

/** Matches nudge / inbox action chip sizing without importing `views/now`. */
const messageActionPillClass =
  '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4';

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

  const tagFrame = isUser
    ? 'border-emerald-800/60 bg-emerald-900/45 text-emerald-200'
    : 'border-[#ff6b00]/40 bg-[#2d1608]/90 text-[#ffd4b8]';
  const metaTimeClass = isUser ? 'text-emerald-600/90' : 'text-[#c9a082]';

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
              <a
                key={key}
                href={action.url}
                target="_blank"
                rel="noreferrer"
                className={cn(filterPillFrame, filterPillActionIdle, 'text-[11px] font-medium', messageActionPillClass)}
              >
                {action.label}
              </a>
            );
          }

          if (action.action_type === 'copy_text' && action.value) {
            const label = copiedAction === action.label ? 'Copied' : action.label;
            return (
              <FilterPillButton
                key={key}
                className={messageActionPillClass}
                onClick={() => void handleCopyAction(action.label, action.value!)}
              >
                {label}
              </FilterPillButton>
            );
          }

          if (action.action_type === 'show_why' && onShowWhy) {
            return (
              <FilterPillButton
                key={key}
                className={messageActionPillClass}
                onClick={() => onShowWhy(message.id)}
              >
                {action.label}
              </FilterPillButton>
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
          <FilterPillButton className={messageActionPillClass} onClick={() => onSnooze(interventionId)}>
            Snooze
          </FilterPillButton>
        )}
        {onResolve && (
          <FilterPillButton
            className={cn(messageActionPillClass, 'border-emerald-800/80 bg-emerald-950/50 text-emerald-200 hover:border-emerald-600')}
            onClick={() => onResolve(interventionId)}
          >
            Resolve
          </FilterPillButton>
        )}
        {onDismiss && (
          <FilterPillButton className={messageActionPillClass} onClick={() => onDismiss(interventionId)}>
            Dismiss
          </FilterPillButton>
        )}
        {onShowWhy && (
          <FilterPillButton className={messageActionPillClass} onClick={() => onShowWhy(message.id)}>
            Show why
          </FilterPillButton>
        )}
      </>
    ) : null;

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
    </>
  );

  const titleClass = isUser ? 'text-emerald-50' : 'text-zinc-100';

  const metaCluster = (
    <>
      <FilterDenseTag
        className={cn('!shrink-0 !normal-case !tracking-normal border-transparent bg-transparent', metaTimeClass)}
      >
        {formatMessageTime(message.created_at)}
      </FilterDenseTag>
      <FilterDenseTag className={cn(tagFrame, '!normal-case !tracking-normal')}>
        {message.role} · {message.kind.replace(/_/g, ' ')}
      </FilterDenseTag>
      {message.status === 'sending' ? (
        <FilterDenseTag className={cn(tagFrame, '!normal-case !tracking-normal')}>Sending…</FilterDenseTag>
      ) : null}
    </>
  );

  return (
    <div
      className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-3`}
      data-message-id={message.id}
    >
      <ChatBubbleChrome variant={isUser ? 'user' : 'assistant'}>
        <NowItemRowLayout actions={interventionHeaderActions} actionsLayout="inline">
          <ItemRowTitleMetaBand
            title={isUser ? 'You' : 'Assistant'}
            titleClassName={titleClass}
            meta={metaCluster}
          />
          <div className="mt-2 min-w-0 space-y-2">{cardContent}</div>
        </NowItemRowLayout>
      </ChatBubbleChrome>
    </div>
  );
}
