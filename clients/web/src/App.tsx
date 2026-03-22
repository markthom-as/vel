import { useState } from 'react';
import { AppShell } from './shell/AppShell';
import { ContextPanel } from './views/context';
import { DocumentationPanel } from './views/context';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import { IconButton } from './core/Button';
import { PanelEyebrow } from './core/PanelChrome';
import { ChevronRightIcon } from './core/Icons';
import { getSurfaceDefinition, type MainView } from './data/operatorSurfaces';
import type { SettingsSectionKey } from './views/settings';

export type SettingsNavigationTarget = {
  tab: 'general' | 'integrations' | 'runtime';
  integrationId?: 'google' | 'todoist' | 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts';
  section?: SettingsSectionKey;
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

  function openConversationThread(conversationId: string) {
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }

  function openInbox() {
    setMainView('inbox');
  }

  function toggleDocumentationPanel() {
    setInfoPanelOpen((open) => !open);
  }

  return (
    <AppShell
      navigation={(
        <Navbar
          activeView={mainView}
          onSelectView={setMainView}
          onOpenDocumentation={toggleDocumentationPanel}
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
            <PanelEyebrow tracking="wide">Info</PanelEyebrow>
            <IconButton
              variant="ghost"
              onClick={() => setInfoPanelOpen(false)}
              aria-label="Collapse info panel"
              className="rounded-full text-zinc-400 hover:bg-zinc-900 hover:text-zinc-100"
            >
              <ChevronRightIcon size={18} />
            </IconButton>
          </div>
          <div className="min-h-0 flex-1 overflow-y-auto">
            <div className="border-b border-zinc-900">
              <ContextPanel />
            </div>
            <DocumentationPanel currentView={getSurfaceDefinition(mainView).label} />
          </div>
        </div>
      )}
      infoPanelOpen={infoPanelOpen}
    />
  );
}

export default App;
