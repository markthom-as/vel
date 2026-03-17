import { useState } from 'react';
import { apiPost } from './api/client';
import { decodeApiResponse, decodeConversationData, type ApiResponse, type ConversationData } from './types';
import { AppShell } from './components/AppShell';
import { ContextPanel } from './components/ContextPanel';
import { MainPanel } from './components/MainPanel';
import { Sidebar, type MainView } from './components/Sidebar';
import { invalidateQuery } from './data/query';
import { queryKeys } from './data/resources';

export type SettingsNavigationTarget = {
  tab: 'general' | 'integrations' | 'components' | 'runs' | 'loops';
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
      invalidateQuery(queryKeys.conversations(), { refetch: true });
    }
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
          onOpenSettings={openSettings}
          settingsTarget={settingsTarget}
        />
      )}
      contextPanel={mainView === 'settings' ? null : <ContextPanel />}
    />
  );
}

export default App;
