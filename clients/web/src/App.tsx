import { useState } from 'react';
import { apiPost } from './api/client';
import { decodeApiResponse, decodeConversationData, type ApiResponse, type ConversationData } from './types';
import { AppShell } from './components/AppShell';
import { ContextPanel } from './components/ContextPanel';
import { MainPanel } from './components/MainPanel';
import { Sidebar } from './components/Sidebar';
import { chatQueryKeys } from './data/chat';
import type { MainView } from './data/operatorSurfaces';
import { invalidateQuery } from './data/query';

export type SettingsNavigationTarget = {
  tab: 'general' | 'integrations' | 'runtime';
  integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
};

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [settingsTarget, setSettingsTarget] = useState<SettingsNavigationTarget>({ tab: 'general' });

  function openSettings(target: SettingsNavigationTarget = { tab: 'general' }) {
    setSettingsTarget(target);
    setMainView('settings');
  }

  async function startNewConversation() {
    const res = await apiPost<ApiResponse<ConversationData>>('/api/conversations', {
      title: 'New conversation',
      kind: 'general',
    }, (value) => decodeApiResponse(value, decodeConversationData));
    if (res.ok && res.data) {
      setSelectedConversationId(res.data.id);
      setMainView('threads');
      invalidateQuery(chatQueryKeys.conversations(), { refetch: true });
    }
  }

  function openConversationThread(conversationId: string) {
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }

  function openInbox() {
    setMainView('inbox');
  }

  return (
    <AppShell
      sidebar={(
        <Sidebar
          activeView={mainView}
          onSelectView={setMainView}
          selectedConversationId={selectedConversationId}
          onSelectConversation={(id) => {
            setSelectedConversationId(id);
            setMainView('threads');
          }}
          onNewConversation={startNewConversation}
        />
      )}
      main={(
        <MainPanel
          conversationId={selectedConversationId}
          mainView={mainView}
          onNavigate={setMainView}
          onOpenInbox={openInbox}
          onOpenThread={openConversationThread}
          onOpenSettings={openSettings}
          settingsTarget={settingsTarget}
        />
      )}
      contextPanel={mainView === 'settings' ? null : <ContextPanel />}
    />
  );
}

export default App;
