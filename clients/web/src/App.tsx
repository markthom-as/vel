import { useState } from 'react';
import { AppShell } from './shell/AppShell';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import { NudgeZone } from './shell/NudgeZone';
import type { MainView } from './data/operatorSurfaces';
import type { NowNudgeBarData } from './types';
import type { SystemNavigationTarget } from './views/system';

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [systemTarget, setSystemTarget] = useState<SystemNavigationTarget>({});
  const [localNudges, setLocalNudges] = useState<NowNudgeBarData[]>([]);

  function pushLocalNudge(nudge: NowNudgeBarData) {
    setLocalNudges((current) => {
      const withoutExisting = current.filter((item) => item.id !== nudge.id);
      return [nudge, ...withoutExisting];
    });
  }

  function openSystem(target: SystemNavigationTarget = {}) {
    setSystemTarget(target);
    setMainView('system');
  }

  function openConversationThread(conversationId: string) {
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }

  function deepLink(target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) {
    if (target.systemTarget) {
      setSystemTarget(target.systemTarget);
    }
    setMainView(target.view);
    if (!target.anchor) return;
    window.requestAnimationFrame(() => {
      window.requestAnimationFrame(() => {
        const node = document.getElementById(target.anchor!);
        if (typeof node?.scrollIntoView === 'function') {
          node.scrollIntoView({ block: 'start', behavior: 'smooth' });
        }
      });
    });
  }

  function raiseVoiceUnavailableNudge() {
    if (localNudges.some((nudge) => nudge.id === 'browser_voice_unavailable')) {
      return;
    }
    pushLocalNudge({
      id: 'browser_voice_unavailable',
      kind: 'trust_warning',
      title: 'Voice input needs browser support',
      summary: 'Local speech-to-text is unavailable here. Check browser permissions or system configuration.',
      timestamp: Math.floor(Date.now() / 1000),
      urgent: true,
      primary_thread_id: null,
      actions: [{ kind: 'open_settings', label: 'Open system' }],
    });
  }

  return (
    <AppShell
      navigation={(
        <Navbar
          activeView={mainView}
          onSelectView={setMainView}
          onDeepLink={deepLink}
        />
      )}
      nudgeZone={(
        <NudgeZone
          activeView={mainView}
          extraNudges={localNudges}
          onOpenThread={openConversationThread}
          onOpenSystem={openSystem}
        />
      )}
      main={(
        <MainPanel
          conversationId={selectedConversationId}
          mainView={mainView}
          onNavigate={setMainView}
          onOpenThread={openConversationThread}
          onOpenSystem={openSystem}
          onVoiceUnavailable={() => {
            raiseVoiceUnavailableNudge();
            openSystem({ section: 'preferences', subsection: 'accessibility' });
          }}
          onRaiseNudge={pushLocalNudge}
          shellOwnsNowNudges
          systemTarget={systemTarget}
        />
      )}
    />
  );
}

export default App;
