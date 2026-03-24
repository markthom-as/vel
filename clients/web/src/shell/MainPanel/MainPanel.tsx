import { useCallback, useMemo, useState, type ReactNode } from 'react';
import { chatQueryKeys } from '../../data/chat';
import { appendUniqueMessages, reconcileConfirmedSend } from '../../data/chat-state';
import { contextQueryKeys } from '../../data/context';
import { invalidateQuery, setQueryData } from '../../data/query';
import { MessageComposer, type SubmittedAssistantEntryPayload } from '../../core/MessageComposer';
import type { MainView } from '../../data/operatorSurfaces';
import type { AssistantEntryResponse, MessageData, NowDockedInputIntentData, NowNudgeBarData } from '../../types';
import { submitAssistantEntry } from '../../data/chat';
import { useResolvedThreadConversationId } from '../../views/threads/useResolvedThreadConversationId';
import { NowView } from '../../views/now';
import { AssistantEntryFeedback } from '../../views/now/components/AssistantEntryFeedback';
import type { SystemNavigationTarget } from '../../views/system';
import { SystemView } from '../../views/system';
import { ThreadView } from '../../views/threads';

interface MainPanelProps {
  conversationId: string | null;
  mainView: MainView;
  onNavigate: (view: MainView) => void;
  onOpenThread: (conversationId: string) => void;
  systemTarget: SystemNavigationTarget;
  onOpenSystem: (target?: SystemNavigationTarget) => void;
  onVoiceUnavailable?: () => void;
  onRaiseNudge?: (nudge: NowNudgeBarData) => void;
  shellOwnsNowNudges?: boolean;
}

export function MainPanel({
  conversationId,
  mainView,
  onNavigate,
  onOpenThread,
  systemTarget,
  onOpenSystem,
  onVoiceUnavailable,
  onRaiseNudge,
  shellOwnsNowNudges = false,
}: MainPanelProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const resolvedThreadId = useResolvedThreadConversationId(conversationId, mainView === 'threads');
  const threadMessagesKey = useMemo(
    () => (resolvedThreadId ? chatQueryKeys.conversationMessages(resolvedThreadId) : null),
    [resolvedThreadId],
  );

  const [assistantEntryMessage, setAssistantEntryMessage] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [assistantInlineResponse, setAssistantInlineResponse] = useState<AssistantEntryResponse | null>(null);
  const [assistantEntryThreadId, setAssistantEntryThreadId] = useState<string | null>(null);
  const [assistantIntentOptions, setAssistantIntentOptions] = useState<Array<NowDockedInputIntentData | 'thread' | 'capture'>>([]);
  const [selectedIntent, setSelectedIntent] = useState<NowDockedInputIntentData | 'thread' | 'capture' | null>(null);
  const [pendingAssistantPayload, setPendingAssistantPayload] = useState<SubmittedAssistantEntryPayload | null>(null);
  const [reclassifyingIntent, setReclassifyingIntent] = useState(false);

  const handleAssistantEntry = useCallback(
    async (response: AssistantEntryResponse, submitted?: SubmittedAssistantEntryPayload | null) => {
      invalidateQuery(conversationsKey, { refetch: true });
      setAssistantEntryMessage(null);
      setAssistantInlineResponse(null);
      setAssistantEntryThreadId(response.conversation.id);
      setAssistantIntentOptions([]);
      setSelectedIntent(response.entry_intent ?? null);
      setPendingAssistantPayload(submitted ?? null);
      setReclassifyingIntent(false);

      if (response.route_target === 'threads') {
        setPendingAssistantPayload(null);
        onOpenThread(response.conversation.id);
        return;
      }
      if (response.route_target === 'inbox') {
        setAssistantEntryMessage({
          status: 'success',
          message: 'Queued in Now for follow-up.',
        });
        setAssistantIntentOptions(['task', 'question', 'thread', 'capture']);
        onNavigate('now');
        return;
      }
      setPendingAssistantPayload(null);
      setAssistantInlineResponse(response);
      if (response.assistant_error) {
        setAssistantEntryMessage({
          status: 'error',
          message: response.assistant_error,
        });
        return;
      }
      setAssistantEntryMessage({
        status: 'success',
        message: 'Handled here in Now.',
      });
    },
    [conversationsKey, onNavigate, onOpenThread],
  );

  const handleAssistantIntentSelection = useCallback(
    async (intent: NowDockedInputIntentData | 'thread' | 'capture') => {
      setSelectedIntent(intent);
      if (intent === 'thread') {
        if (assistantEntryThreadId) {
          setPendingAssistantPayload(null);
          onOpenThread(assistantEntryThreadId);
        }
        return;
      }
      if (!pendingAssistantPayload || reclassifyingIntent) {
        return;
      }
      setReclassifyingIntent(true);
      setAssistantEntryMessage({
        status: 'success',
        message: 'Reclassifying with explicit intent…',
      });
      try {
        const resolvedIntent: NowDockedInputIntentData = intent === 'capture' ? 'note' : intent;
        const response = await submitAssistantEntry(
          pendingAssistantPayload.text,
          pendingAssistantPayload.conversationId,
          pendingAssistantPayload.voice,
          resolvedIntent,
          pendingAssistantPayload.attachments,
        );
        if (!response.ok || !response.data) {
          throw new Error(response.error?.message ?? 'Failed to reclassify assistant entry');
        }
        await handleAssistantEntry(response.data, pendingAssistantPayload);
      } catch (error) {
        setReclassifyingIntent(false);
        setAssistantEntryMessage({
          status: 'error',
          message: error instanceof Error ? error.message : 'Failed to reclassify assistant entry.',
        });
      }
    },
    [assistantEntryThreadId, handleAssistantEntry, onOpenThread, pendingAssistantPayload, reclassifyingIntent],
  );

  let body: ReactNode;
  if (mainView === 'now') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <NowView
          onOpenThread={onOpenThread}
          onOpenSystem={onOpenSystem}
          hideNudgeLane={shellOwnsNowNudges}
        />
      </div>
    );
  } else if (mainView === 'threads') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <ThreadView conversationId={conversationId} onSelectConversation={onOpenThread} />
      </div>
    );
  } else if (mainView === 'system') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <SystemView target={systemTarget} />
      </div>
    );
  } else {
    body = null;
  }

  return (
    <div className="relative flex min-h-0 flex-1 flex-col">
      {body}
      {(assistantEntryMessage || assistantInlineResponse) ? (
        <div className="pointer-events-none fixed inset-x-0 bottom-20 z-[35] flex justify-center px-4 sm:px-6">
          <div className="pointer-events-auto max-h-[min(40vh,14rem)] w-full max-w-5xl overflow-y-auto">
            <AssistantEntryFeedback
              message={assistantEntryMessage}
              inlineResponse={assistantInlineResponse}
              assistantEntryThreadId={assistantEntryThreadId}
              pendingIntentOptions={assistantIntentOptions}
              selectedIntent={selectedIntent}
              onSelectIntent={handleAssistantIntentSelection}
              onDismiss={() => {
                setAssistantEntryMessage(null);
                setAssistantInlineResponse(null);
                setAssistantEntryThreadId(null);
                setAssistantIntentOptions([]);
                setSelectedIntent(null);
                setPendingAssistantPayload(null);
                setReclassifyingIntent(false);
              }}
              onOpenThread={onOpenThread}
            />
          </div>
        </div>
      ) : null}
      <MessageComposer
        compact
        floating
        hideHelperText
        floatingOffsetClassName="bottom-6 sm:bottom-8"
        onVoiceUnavailable={onVoiceUnavailable}
        conversationId={mainView === 'threads' ? resolvedThreadId : undefined}
        onOptimisticSend={
          mainView === 'threads' && resolvedThreadId && threadMessagesKey
            ? (text) => {
                const clientMessageId = `tmp_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
                const optimisticMessage: MessageData = {
                  id: clientMessageId,
                  conversation_id: resolvedThreadId,
                  role: 'user',
                  kind: 'text',
                  content: { text },
                  status: 'sending',
                  importance: null,
                  created_at: Math.floor(Date.now() / 1000),
                  updated_at: null,
                };
                setQueryData<MessageData[]>(threadMessagesKey, (prev = []) =>
                  appendUniqueMessages(prev, [optimisticMessage]),
                );
                return clientMessageId;
              }
            : undefined
        }
        onSent={(clientMessageId, response, submitted) => {
          if (mainView === 'threads' && threadMessagesKey && resolvedThreadId) {
            setQueryData<MessageData[]>(threadMessagesKey, (prev = []) =>
              reconcileConfirmedSend(
                prev,
                clientMessageId,
                response.user_message,
                response.assistant_message ? [response.assistant_message] : [],
              ),
            );
            invalidateQuery(conversationsKey, { refetch: true });
            invalidateQuery(threadMessagesKey, { refetch: true });
          }
          void handleAssistantEntry(response, submitted);
          invalidateQuery(nowKey, { refetch: true });
        }}
        onSendFailed={(clientMessageId) => {
          if (mainView === 'threads' && threadMessagesKey && clientMessageId) {
            setQueryData<MessageData[]>(threadMessagesKey, (prev = []) =>
              prev.filter((message) => message.id !== clientMessageId),
            );
          }
          onRaiseNudge?.({
            id: `assistant_entry_failed_${Date.now()}`,
            kind: 'trust_warning',
            title: 'Assistant entry failed',
            summary: 'Vel could not send this request. Review runtime state or try again.',
            timestamp: Math.floor(Date.now() / 1000),
            urgent: true,
            primary_thread_id: null,
            actions: [{ kind: 'open_settings', label: 'Open system' }],
          });
          setAssistantEntryMessage({
            status: 'error',
            message: 'Failed to send assistant entry.',
          });
        }}
      />
    </div>
  );
}
