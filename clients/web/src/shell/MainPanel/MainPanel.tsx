import { useCallback, useMemo, useState, type ReactNode } from 'react';
import { chatQueryKeys, invalidateInboxQueries } from '../../data/chat';
import { appendUniqueMessages, reconcileConfirmedSend } from '../../data/chat-state';
import { contextQueryKeys } from '../../data/context';
import { invalidateQuery, setQueryData } from '../../data/query';
import { MessageComposer } from '../../core/MessageComposer';
import type { MainView } from '../../data/operatorSurfaces';
import type { AssistantEntryResponse, MessageData } from '../../types';
import { useResolvedThreadConversationId } from '../../views/threads/useResolvedThreadConversationId';
import { InboxView } from '../../views/inbox';
import { NowView } from '../../views/now';
import { AssistantEntryFeedback } from '../../views/now/components/AssistantEntryFeedback';
import { ProjectsView } from '../../views/projects';
import type { SettingsSectionKey, SettingsTab } from '../../views/settings';
import { SettingsPage } from '../../views/settings';
import { ThreadView } from '../../views/threads';

type SettingsNavigationTarget = {
  tab: SettingsTab;
  integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
  section?: SettingsSectionKey;
};

interface MainPanelProps {
  conversationId: string | null;
  mainView: MainView;
  onNavigate: (view: MainView) => void;
  onOpenInbox: () => void;
  onOpenThread: (conversationId: string) => void;
  settingsTarget: SettingsNavigationTarget;
  onOpenSettings: (target?: {
    tab: SettingsTab;
    integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
    section?: SettingsSectionKey;
  }) => void;
}

export function MainPanel({
  conversationId,
  mainView,
  onNavigate,
  onOpenInbox,
  onOpenThread,
  settingsTarget,
  onOpenSettings,
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

  const handleAssistantEntry = useCallback(
    async (response: AssistantEntryResponse) => {
      invalidateQuery(conversationsKey, { refetch: true });
      invalidateInboxQueries();
      setAssistantEntryMessage(null);
      setAssistantInlineResponse(null);
      setAssistantEntryThreadId(response.conversation.id);

      if (response.route_target === 'threads') {
        onOpenThread(response.conversation.id);
        return;
      }
      if (response.route_target === 'inbox') {
        setAssistantEntryMessage({
          status: 'success',
          message: 'Saved to Inbox for follow-up.',
        });
        onOpenInbox();
        return;
      }
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
    [conversationsKey, onOpenInbox, onOpenThread],
  );

  let body: ReactNode;
  if (mainView === 'now') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <NowView
          onOpenInbox={onOpenInbox}
          onOpenThread={onOpenThread}
          onOpenSettings={onOpenSettings}
        />
      </div>
    );
  } else if (mainView === 'inbox') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <InboxView onOpenThread={onOpenThread} />
      </div>
    );
  } else if (mainView === 'threads') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <ThreadView conversationId={conversationId} onSelectConversation={onOpenThread} />
      </div>
    );
  } else if (mainView === 'settings') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden bg-zinc-950 text-zinc-100">
        <SettingsPage
          onBack={() => onNavigate('now')}
          initialTab={settingsTarget.tab}
          initialIntegrationId={settingsTarget.integrationId}
          initialSection={settingsTarget.section}
        />
      </div>
    );
  } else if (mainView === 'projects') {
    body = (
      <div className="flex min-h-0 flex-1 flex-col overflow-hidden">
        <ProjectsView />
      </div>
    );
  } else {
    body = null;
  }

  return (
    <div className="relative flex min-h-0 flex-1 flex-col">
      {body}
      {(assistantEntryMessage || assistantInlineResponse) ? (
        <div className="pointer-events-none fixed inset-x-0 bottom-32 z-[35] flex justify-center px-4 sm:px-6">
          <div className="pointer-events-auto max-h-[min(40vh,14rem)] w-full max-w-5xl overflow-y-auto">
            <AssistantEntryFeedback
              message={assistantEntryMessage}
              inlineResponse={assistantInlineResponse}
              assistantEntryThreadId={assistantEntryThreadId}
              onOpenThread={onOpenThread}
            />
          </div>
        </div>
      ) : null}
      <MessageComposer
        compact
        floating
        hideHelperText
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
        onSent={(clientMessageId, response) => {
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
          void handleAssistantEntry(response);
          invalidateQuery(nowKey, { refetch: true });
        }}
        onSendFailed={(clientMessageId) => {
          if (mainView === 'threads' && threadMessagesKey && clientMessageId) {
            setQueryData<MessageData[]>(threadMessagesKey, (prev = []) =>
              prev.filter((message) => message.id !== clientMessageId),
            );
          }
          setAssistantEntryMessage({
            status: 'error',
            message: 'Failed to send assistant entry.',
          });
        }}
      />
    </div>
  );
}
