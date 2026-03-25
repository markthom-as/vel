import { useCallback, useEffect, useMemo, useState, type ReactNode } from 'react';
import { chatQueryKeys } from '../../data/chat';
import { appendUniqueMessages, reconcileConfirmedSend } from '../../data/chat-state';
import { contextQueryKeys } from '../../data/context';
import { buildCoreSetupStatus, loadIntegrations, loadSettings, operatorQueryKeys } from '../../data/operator';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
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
import type { IntegrationsData, SettingsData } from '../../types';
import type { ViewportSurface } from '../../core/hooks/useViewportSurface';

function assistantReplyText(response: AssistantEntryResponse): string | null {
  const content = response.assistant_message?.content;
  if (!content || typeof content !== 'object' || Array.isArray(content)) {
    return null;
  }
  const text = content.text;
  return typeof text === 'string' && text.trim() ? text.trim() : null;
}

const CORE_SETUP_CHECKLIST_ITEMS = [
  { id: 'user_display_name', label: 'Your name' },
  { id: 'node_display_name', label: 'Node name' },
  { id: 'agent_profile', label: 'Agent profile' },
  { id: 'llm_provider', label: 'LLM integration' },
  { id: 'synced_provider', label: 'Synced provider' },
] as const;

function hasMeaningfulText(value: string | null | undefined): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function coreSetupChecklistValue(
  itemId: typeof CORE_SETUP_CHECKLIST_ITEMS[number]['id'],
  settings: SettingsData | null | undefined,
  integrations: IntegrationsData | null | undefined,
): string | null {
  switch (itemId) {
    case 'user_display_name':
      return hasMeaningfulText(settings?.core_settings?.user_display_name)
        ? settings.core_settings.user_display_name.trim()
        : null;
    case 'node_display_name':
      return hasMeaningfulText(settings?.node_display_name)
        ? settings.node_display_name.trim()
        : null;
    case 'agent_profile': {
      const profile = settings?.core_settings?.agent_profile;
      return [
        profile?.role,
        profile?.freeform,
        profile?.preferences,
        profile?.constraints,
      ].find(hasMeaningfulText)?.trim() ?? null;
    }
    case 'llm_provider': {
      const defaultProfileId = settings?.llm?.default_chat_profile_id ?? null;
      const profile = defaultProfileId
        ? settings?.llm?.profiles.find((entry) => entry.enabled && entry.id === defaultProfileId)
        : null;
      if (!profile) {
        return null;
      }
      return [profile.model, profile.id].filter(hasMeaningfulText).join(' · ');
    }
    case 'synced_provider': {
      const providers = [
        integrations?.google_calendar.connected ? 'Google Calendar' : null,
        integrations?.todoist.connected ? 'Todoist' : null,
      ].filter(hasMeaningfulText);
      return providers.length > 0 ? providers.join(', ') : null;
    }
    default:
      return null;
  }
}

function buildCoreSetupNudgeActions(
  status: ReturnType<typeof buildCoreSetupStatus>,
  settings: SettingsData | null | undefined,
  integrations: IntegrationsData | null | undefined,
): NowNudgeBarData['actions'] {
  const missing = new Set(status.missing);
  return CORE_SETUP_CHECKLIST_ITEMS.map((item) => ({
      kind: [
        'open_settings',
        'core_settings',
        item.id,
        missing.has(item.id) ? 'missing' : 'ready',
        coreSetupChecklistValue(item.id, settings, integrations)
          ? encodeURIComponent(coreSetupChecklistValue(item.id, settings, integrations)!)
          : null,
      ].filter(Boolean).join(':'),
      label: item.label,
    }));
}

interface MainPanelProps {
  surface?: ViewportSurface;
  conversationId: string | null;
  mainView: MainView;
  onNavigate: (view: MainView) => void;
  onOpenThread: (conversationId: string) => void;
  miniComposerOpen?: boolean;
  onOpenMiniComposer?: (conversationId: string | null) => void;
  systemTarget: SystemNavigationTarget;
  onOpenSystem: (target?: SystemNavigationTarget) => void;
  onVoiceUnavailable?: () => void;
  onRaiseNudge?: (nudge: NowNudgeBarData) => void;
  onClearNudge?: (nudgeId: string) => void;
  shellOwnsNowNudges?: boolean;
}

export function MainPanel({
  conversationId,
  mainView,
  onNavigate,
  onOpenThread,
  miniComposerOpen = false,
  onOpenMiniComposer,
  surface = 'desktop',
  systemTarget,
  onOpenSystem,
  onVoiceUnavailable,
  onRaiseNudge,
  onClearNudge,
  shellOwnsNowNudges = false,
}: MainPanelProps) {
  void onOpenSystem;
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const settingsKey = useMemo(() => operatorQueryKeys.settings(), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const resolvedThreadId = useResolvedThreadConversationId(conversationId, mainView === 'threads');
  const threadMessagesKey = useMemo(
    () => (resolvedThreadId ? chatQueryKeys.conversationMessages(resolvedThreadId) : null),
    [resolvedThreadId],
  );
  const { data: settings } = useQuery(
    settingsKey,
    async () => {
      const response = await loadSettings();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to load settings');
      }
      return response.data ?? null;
    },
  );
  const { data: integrations } = useQuery(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to load integrations');
      }
      return response.data ?? null;
    },
  );
  const coreSetupStatus = useMemo(
    () => buildCoreSetupStatus(settings, integrations),
    [settings, integrations],
  );
  const coreSetupNudgeSummary = coreSetupStatus.ready
    ? null
    : 'Finish the checklist below to enable Vel.';
  const coreSetupNudgeActions = useMemo(
    () => buildCoreSetupNudgeActions(coreSetupStatus, settings, integrations),
    [coreSetupStatus, integrations, settings],
  );
  const composerDisabledReason = coreSetupStatus.ready
    ? null
    : 'Core setup is incomplete. Use the nudge to open Core settings and finish setup.';

  const [assistantEntryMessage, setAssistantEntryMessage] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [assistantInlineResponse, setAssistantInlineResponse] = useState<AssistantEntryResponse | null>(null);
  const [assistantEntryThreadId, setAssistantEntryThreadId] = useState<string | null>(null);
  const [assistantIntentOptions, setAssistantIntentOptions] = useState<Array<NowDockedInputIntentData | 'thread' | 'capture'>>([]);
  const [selectedIntent, setSelectedIntent] = useState<NowDockedInputIntentData | 'thread' | 'capture' | null>(null);
  const [pendingAssistantPayload, setPendingAssistantPayload] = useState<SubmittedAssistantEntryPayload | null>(null);
  const [assistantErrorRetryable, setAssistantErrorRetryable] = useState(false);
  const [reclassifyingIntent, setReclassifyingIntent] = useState(false);

  const speakAssistantReply = useCallback((response: AssistantEntryResponse) => {
    if (!response.conversation.call_mode_active) {
      return;
    }
    if (typeof window === 'undefined' || !('speechSynthesis' in window) || typeof SpeechSynthesisUtterance === 'undefined') {
      return;
    }
    const text = assistantReplyText(response);
    if (!text) {
      return;
    }
    window.speechSynthesis.cancel();
    window.speechSynthesis.speak(new SpeechSynthesisUtterance(text));
  }, []);

  useEffect(() => {
    if (coreSetupStatus.ready) {
      onClearNudge?.('core_setup_required');
      return;
    }
    onRaiseNudge?.({
      id: 'core_setup_required',
      kind: 'needs_input',
      title: coreSetupStatus.title,
      summary: coreSetupNudgeSummary ?? coreSetupStatus.summary,
      timestamp: Math.floor(Date.now() / 1000),
      urgent: true,
      primary_thread_id: null,
      actions: coreSetupNudgeActions,
    });
  }, [coreSetupNudgeActions, coreSetupStatus, onClearNudge, onRaiseNudge]);

  const handleAssistantEntry = useCallback(
    async (response: AssistantEntryResponse, submitted?: SubmittedAssistantEntryPayload | null) => {
      invalidateQuery(conversationsKey, { refetch: true });
      setAssistantEntryMessage(null);
      setAssistantInlineResponse(null);
      setAssistantEntryThreadId(response.conversation.id);
      setAssistantIntentOptions([]);
      setSelectedIntent(response.entry_intent ?? null);
      setPendingAssistantPayload(submitted ?? null);
      setAssistantErrorRetryable(Boolean(response.assistant_error_retryable));
      setReclassifyingIntent(false);
      speakAssistantReply(response);

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
        if (response.assistant_error_retryable) {
          onRaiseNudge?.({
            id: `assistant_entry_retry_${response.user_message.id}`,
            kind: 'trust_warning',
            title: 'Assistant reply failed',
            summary: 'Vel hit a retryable provider/runtime error. Retry from the inline card or inspect System if it persists.',
            timestamp: Math.floor(Date.now() / 1000),
            urgent: true,
            primary_thread_id: response.conversation.id,
            actions: [{ kind: 'expand', label: 'Open thread' }],
          });
        }
        return;
      }
      setAssistantErrorRetryable(false);
      setAssistantEntryMessage({
        status: 'success',
        message: 'Handled here in Now.',
      });
    },
    [conversationsKey, onNavigate, onOpenThread, speakAssistantReply],
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

  const handleAssistantRetry = useCallback(async () => {
    if (!pendingAssistantPayload || reclassifyingIntent) {
      return;
    }
    setReclassifyingIntent(true);
    setAssistantEntryMessage({
      status: 'success',
      message: 'Retrying assistant reply…',
    });
    try {
      const response = await submitAssistantEntry(
        pendingAssistantPayload.text,
        pendingAssistantPayload.conversationId,
        pendingAssistantPayload.voice,
        pendingAssistantPayload.intent,
        pendingAssistantPayload.attachments,
      );
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Retry failed');
      }
      await handleAssistantEntry(response.data, pendingAssistantPayload);
    } catch (error) {
      setAssistantEntryMessage({
        status: 'error',
        message: error instanceof Error ? error.message : 'Retry failed',
      });
    } finally {
      setReclassifyingIntent(false);
    }
  }, [handleAssistantEntry, pendingAssistantPayload, reclassifyingIntent]);

  let body: ReactNode;
  if (mainView === 'now') {
    body = (
      <div className="relative flex min-h-0 flex-1 flex-col bg-transparent">
        <NowView
          onOpenThread={onOpenThread}
          hideNudgeLane={shellOwnsNowNudges}
        />
      </div>
    );
  } else if (mainView === 'threads') {
    body = (
      <div className="relative flex min-h-0 flex-1 flex-col bg-transparent">
        <ThreadView conversationId={conversationId} onSelectConversation={onOpenThread} surface={surface} />
      </div>
    );
  } else if (mainView === 'system') {
    body = (
      <div className="relative flex min-h-0 flex-1 flex-col bg-transparent">
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
              canRetry={assistantErrorRetryable && Boolean(pendingAssistantPayload)}
              onRetry={() => {
                void handleAssistantRetry();
              }}
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
                setAssistantErrorRetryable(false);
                setReclassifyingIntent(false);
              }}
              onOpenThread={onOpenThread}
            />
          </div>
        </div>
      ) : null}
      {miniComposerOpen ? null : (
        <MessageComposer
          compact
          floating
          hideHelperText
          onOpenMiniMode={onOpenMiniComposer}
          floatingOffsetClassName={surface === 'mobile' ? 'bottom-[calc(0.9rem+env(safe-area-inset-bottom))]' : 'bottom-6 sm:bottom-8'}
          disabled={!coreSetupStatus.ready}
          disabledReason={composerDisabledReason}
          onDisabledInteract={() => {
            if (!coreSetupStatus.ready) {
              onRaiseNudge?.({
                id: 'core_setup_required',
                kind: 'needs_input',
                title: coreSetupStatus.title,
                summary: coreSetupNudgeSummary ?? coreSetupStatus.summary,
                timestamp: Math.floor(Date.now() / 1000),
                urgent: true,
                primary_thread_id: null,
                actions: coreSetupNudgeActions,
              });
            }
          }}
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
      )}
    </div>
  );
}
