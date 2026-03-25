import type { AssistantEntryResponse, NowDockedInputIntentData } from '../../../types';
import { ActionChipButton, FilterToggleTag } from '../../../core/FilterToggleTag';
import { OpenThreadIcon } from '../../../core/Icons';
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

const intentLabels: Record<NowDockedInputIntentData | 'thread' | 'capture', string> = {
  task: 'Task',
  question: 'Question',
  url: 'URL',
  note: 'Note',
  command: 'Command',
  continuation: 'Continuation',
  reflection: 'Reflection',
  scheduling: 'Scheduling',
  thread: 'Thread',
  capture: 'Capture',
};

export function AssistantEntryFeedback({
  message,
  inlineResponse,
  assistantEntryThreadId,
  canRetry = false,
  onRetry,
  onOpenThread,
  pendingIntentOptions,
  selectedIntent,
  onSelectIntent,
  onDismiss,
}: {
  message: { status: 'success' | 'error'; message: string } | null;
  inlineResponse: AssistantEntryResponse | null;
  assistantEntryThreadId: string | null;
  canRetry?: boolean;
  onRetry?: () => void;
  onOpenThread?: (conversationId: string) => void;
  pendingIntentOptions?: Array<NowDockedInputIntentData | 'thread' | 'capture'>;
  selectedIntent?: NowDockedInputIntentData | 'thread' | 'capture' | null;
  onSelectIntent?: (intent: NowDockedInputIntentData | 'thread' | 'capture') => void;
  onDismiss?: () => void;
}) {
  if (!message && !inlineResponse) {
    return null;
  }

  return (
    <div className="space-y-2">
      {message ? (
        <div
          className={`relative rounded-[1.4rem] px-4 py-3 text-sm shadow-[0_14px_40px_rgba(0,0,0,0.24)] ${
            message.status === 'error'
              ? 'border border-rose-800/60 bg-rose-950/20 text-rose-200'
              : uiTheme.brandAssistantBubble
          }`}
        >
          <div className="flex items-start justify-between gap-3">
            <p className="text-sm leading-6">{message.message}</p>
            <div className="flex items-center gap-2">
              {message.status === 'error' && canRetry ? (
                <ActionChipButton tone="brand" onClick={onRetry}>
                  Retry
                </ActionChipButton>
              ) : null}
              <ActionChipButton tone="ghost" onClick={onDismiss}>
                Dismiss
              </ActionChipButton>
            </div>
          </div>
          {pendingIntentOptions?.length ? (
            <div className="mt-3 sm:mt-0 sm:absolute sm:right-3 sm:top-1/2 sm:-translate-y-1/2">
              <div className="rounded-[1.15rem] border border-[var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel)]/96 p-2 shadow-[0_18px_36px_rgba(0,0,0,0.28)] backdrop-blur">
                <div className="flex flex-wrap items-center justify-end gap-1.5 sm:max-w-[18rem]">
                  {pendingIntentOptions.map((intent, index) => (
                    <FilterToggleTag
                      key={intent}
                      label={intentLabels[intent]}
                      size="dense"
                      selected={selectedIntent === intent || (!selectedIntent && index === 0)}
                      onClick={() => onSelectIntent?.(intent)}
                    />
                  ))}
                </div>
              </div>
            </div>
          ) : null}
          {assistantEntryThreadId ? (
            <div className="mt-3 flex flex-wrap items-center gap-2">
              <ActionChipButton tone="brand" onClick={() => onOpenThread?.(assistantEntryThreadId)}>
                <OpenThreadIcon size={14} />
                Open Thread
              </ActionChipButton>
            </div>
          ) : null}
        </div>
      ) : null}
      {inlineResponse?.assistant_message ? (
        <div className={`rounded-[1.4rem] px-4 py-3 shadow-[0_14px_40px_rgba(0,0,0,0.24)] ${uiTheme.brandAssistantBubble}`}>
          <p className={`text-xs uppercase tracking-[0.16em] ${uiTheme.brandAssistantBubbleMeta}`}>
            Inline
          </p>
          <p className="mt-2 text-sm text-zinc-100">
            {renderInlineAssistantText(inlineResponse.assistant_message.content)}
          </p>
          {assistantEntryThreadId ? (
            <ActionChipButton
              onClick={() => onOpenThread?.(assistantEntryThreadId)}
              tone="brand"
              className="mt-3"
            >
              <OpenThreadIcon size={14} />
              Open Thread
            </ActionChipButton>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}
