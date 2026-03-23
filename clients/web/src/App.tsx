import { useState } from 'react';
import { AppShell } from './shell/AppShell';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import type { MainView } from './data/operatorSurfaces';
import type { SystemNavigationTarget } from './views/system';

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [systemTarget, setSystemTarget] = useState<SystemNavigationTarget>({});

  function openSystem(target: SystemNavigationTarget = {}) {
    setSystemTarget(target);
    setMainView('system');
  }

  function openConversationThread(conversationId: string) {
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }

  return (
    <AppShell
      navigation={(
        <Navbar
          activeView={mainView}
          onSelectView={setMainView}
        />
      )}
      main={(
        <MainPanel
          conversationId={selectedConversationId}
          mainView={mainView}
          onNavigate={setMainView}
          onOpenThread={openConversationThread}
          onOpenSystem={openSystem}
          systemTarget={systemTarget}
        />
      )}
    />
  );
}

export default App;
