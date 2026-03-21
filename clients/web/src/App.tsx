import { useState } from 'react';
import { apiPost } from './api/client';
import { decodeApiResponse, decodeConversationData, type ApiResponse, type ConversationData } from './types';
import { AppShell } from './shell/AppShell';
import { ContextPanel } from './views/context';
import { DocumentationPanel } from './views/context';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import { ChevronLeftIcon, ChevronRightIcon } from './core/Icons';
import { chatQueryKeys } from './data/chat';
import { getSurfaceDefinition, type MainView } from './data/operatorSurfaces';
import { invalidateQuery } from './data/query';

export type SettingsNavigationTarget = {
  tab: 'general' | 'integrations' | 'runtime';
  integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
};

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [settingsTarget, setSettingsTarget] = useState<SettingsNavigationTarget>({ tab: 'general' });
  const [infoPanelOpen, setInfoPanelOpen] = useState(false);

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

  function openDocumentationPanel() {
    setInfoPanelOpen(true);
  }

  return (
    <AppShell
      navigation={(
        <Navbar
          activeView={mainView}
          onSelectView={setMainView}
          onOpenDocumentation={openDocumentationPanel}
          infoPanelOpen={infoPanelOpen}
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
      infoPanel={(
        <div className="flex h-full min-h-0 flex-col overflow-hidden">
          <div className="flex items-center justify-between border-b border-zinc-800 px-3 py-3">
            <p className="text-[10px] uppercase tracking-[0.24em] text-zinc-500">Info</p>
            <button
              type="button"
              onClick={() => setInfoPanelOpen((current) => !current)}
              aria-label={infoPanelOpen ? 'Collapse info panel' : 'Open info panel'}
              className="rounded-full px-2 py-1 text-lg leading-none text-zinc-400 transition hover:bg-zinc-900 hover:text-zinc-100"
            >
              {infoPanelOpen ? <ChevronRightIcon size={18} /> : <ChevronLeftIcon size={18} />}
            </button>
          </div>
          {infoPanelOpen ? (
            <div className="min-h-0 flex-1 overflow-y-auto">
              <div className="border-b border-zinc-900">
                <ContextPanel />
              </div>
              <DocumentationPanel currentView={getSurfaceDefinition(mainView).label} />
            </div>
          ) : (
            <div className="flex h-full items-start justify-center px-2 py-3">
              <button
                type="button"
                onClick={() => setInfoPanelOpen(true)}
                aria-label="Open info panel"
                className="rounded-full px-1 py-2 text-lg leading-none text-zinc-400 transition hover:text-zinc-100"
              >
                <ChevronLeftIcon size={18} />
              </button>
            </div>
          )}
        </div>
      )}
      infoPanelOpen={infoPanelOpen}
    />
  );
}

export default App;
