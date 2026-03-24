import { useCallback, useState } from 'react';
import { AppShell } from './shell/AppShell';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import { NudgeZone } from './shell/NudgeZone';
import type { MainView } from './data/operatorSurfaces';
import type { NowNudgeBarData } from './types';
import type { SystemNavigationTarget } from './views/system';

function focusDeepLinkedNode(anchor: string) {
  const node = document.getElementById(anchor);
  if (!(node instanceof HTMLElement)) {
    return;
  }
  node.scrollIntoView({ block: 'start', behavior: 'smooth' });
  const focusTarget =
    node instanceof HTMLInputElement
    || node instanceof HTMLTextAreaElement
    || node instanceof HTMLButtonElement
      ? node
      : node.querySelector<HTMLElement>('input, textarea, button, [tabindex]:not([tabindex="-1"])');
  focusTarget?.focus({ preventScroll: true });
}

function App() {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [mainView, setMainView] = useState<MainView>('now');
  const [systemTarget, setSystemTarget] = useState<SystemNavigationTarget>({});
  const [localNudges, setLocalNudges] = useState<NowNudgeBarData[]>([]);
  const [highlightedNudge, setHighlightedNudge] = useState<{ id: string; nonce: number } | null>(null);

  const pushLocalNudge = useCallback((nudge: NowNudgeBarData) => {
    setLocalNudges((current) => {
      const withoutExisting = current.filter((item) => item.id !== nudge.id);
      return [nudge, ...withoutExisting];
    });
    setHighlightedNudge({ id: nudge.id, nonce: Date.now() });
  }, []);

  const clearLocalNudge = useCallback((nudgeId: string) => {
    setLocalNudges((current) => current.filter((item) => item.id !== nudgeId));
    setHighlightedNudge((current) => (current?.id === nudgeId ? null : current));
  }, []);

  const openSystem = useCallback((target: SystemNavigationTarget = {}) => {
    setSystemTarget(target);
    setMainView('system');
    if (!target.anchor) return;
    window.requestAnimationFrame(() => {
      window.requestAnimationFrame(() => {
        focusDeepLinkedNode(target.anchor!);
      });
    });
  }, []);

  const openConversationThread = useCallback((conversationId: string) => {
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }, []);

  const deepLink = useCallback((target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) => {
    if (target.systemTarget) {
      setSystemTarget(target.systemTarget);
    }
    setMainView(target.view);
    if (!target.anchor) return;
    window.requestAnimationFrame(() => {
      window.requestAnimationFrame(() => {
        focusDeepLinkedNode(target.anchor!);
      });
    });
  }, []);

  const raiseVoiceUnavailableNudge = useCallback(() => {
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
  }, [localNudges, pushLocalNudge]);

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
          highlightedNudgeId={highlightedNudge?.id ?? null}
          highlightedNudgeNonce={highlightedNudge?.nonce ?? null}
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
            openSystem({
              section: 'core',
              subsection: 'core_settings',
              anchor: 'core-settings-required-setup',
            });
          }}
          onRaiseNudge={pushLocalNudge}
          onClearNudge={clearLocalNudge}
          shellOwnsNowNudges
          systemTarget={systemTarget}
        />
      )}
    />
  );
}

export default App;
