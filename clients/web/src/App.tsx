import { useCallback, useState } from 'react';
import { AppShell } from './shell/AppShell';
import { MainPanel } from './shell/MainPanel';
import { Navbar } from './shell/Navbar';
import { NudgeZone } from './shell/NudgeZone';
import { useShellBootstrap } from './shell/useShellBootstrap';
import { useViewportSurface } from './core/hooks/useViewportSurface';
import type { MainView } from './data/operatorSurfaces';
import type { NowNudgeBarData } from './types';
import { systemTargetForCoreSetting, type SystemNavigationTarget } from './views/system';

type TabletLayoutMode = 'auto' | 'single' | 'split';

const TABLET_LAYOUT_KEY = 'vel-webui-tablet-layout';

function readTabletLayoutMode(): TabletLayoutMode {
  if (typeof window === 'undefined') {
    return 'auto';
  }
  const stored = window.localStorage.getItem(TABLET_LAYOUT_KEY);
  if (stored === 'single' || stored === 'split' || stored === 'auto') {
    return stored;
  }
  return 'auto';
}

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
  const [activeNowAnchor, setActiveNowAnchor] = useState<string | null>(null);
  const [systemTarget, setSystemTarget] = useState<SystemNavigationTarget>({});
  const [localNudges, setLocalNudges] = useState<NowNudgeBarData[]>([]);
  const [highlightedNudge, setHighlightedNudge] = useState<{ id: string; nonce: number } | null>(null);
  const [miniChatOpen, setMiniChatOpen] = useState(false);
  const [miniChatThreadId, setMiniChatThreadId] = useState<string | null>(null);
  const { surface: viewportSurface, isLandscape } = useViewportSurface();
  const { shellBootLoading } = useShellBootstrap();
  const [tabletLayoutMode, setTabletLayoutMode] = useState<TabletLayoutMode>(() => readTabletLayoutMode());
  const tabletSplitMode =
    viewportSurface === 'tablet' && (tabletLayoutMode === 'split' || (tabletLayoutMode === 'auto' && isLandscape));
  const shellNudgeRailVisible = viewportSurface === 'desktop' || tabletSplitMode;
  const mobileNudgesActive = !shellNudgeRailVisible && mainView === 'now' && activeNowAnchor === 'nudges-section';

  const setLayoutMode = useCallback((nextMode: TabletLayoutMode) => {
    setTabletLayoutMode(nextMode);
    if (typeof window === 'undefined') {
      return;
    }
    window.localStorage.setItem(TABLET_LAYOUT_KEY, nextMode);
  }, []);

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

  const selectMainView = useCallback((view: MainView) => {
    setActiveNowAnchor(null);
    setMainView(view);
  }, []);

  const openSystem = useCallback((target: SystemNavigationTarget = {}) => {
    setActiveNowAnchor(null);
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
    setActiveNowAnchor(null);
    setSelectedConversationId(conversationId);
    setMainView('threads');
  }, []);

  const deepLink = useCallback((target: { view: MainView; anchor?: string; systemTarget?: SystemNavigationTarget }) => {
    setActiveNowAnchor(target.view === 'now' ? target.anchor ?? null : null);
    if (target.systemTarget) {
      setSystemTarget(target.systemTarget);
    }
    setMainView(target.view);
    const focusAnchor = target.anchor ?? target.systemTarget?.anchor;
    if (!focusAnchor) return;
    window.requestAnimationFrame(() => {
      window.requestAnimationFrame(() => {
        focusDeepLinkedNode(focusAnchor);
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

  const openMiniComposer = useCallback((conversationId: string | null) => {
    setMiniChatOpen(true);
    setMiniChatThreadId((current) => conversationId ?? current);
  }, []);

  const closeMiniComposer = useCallback(() => {
    setMiniChatOpen(false);
  }, []);

  const setMiniComposerThread = useCallback((conversationId: string) => {
    setMiniChatThreadId(conversationId);
    setMiniChatOpen(true);
  }, []);

  return (
    <AppShell
      navigation={(
        <Navbar
          activeView={mainView}
          activeAnchor={activeNowAnchor}
          surface={viewportSurface}
          onSelectView={selectMainView}
          onDeepLink={deepLink}
          layoutMode={tabletLayoutMode}
          onLayoutMode={(nextMode) => setLayoutMode(nextMode)}
          layoutSurfaceSupportsToggle={viewportSurface === 'tablet'}
          splitModeActive={tabletSplitMode}
        />
      )}
      nudgeZone={shellBootLoading || !shellNudgeRailVisible ? undefined : (
        <NudgeZone
          activeView={mainView}
          railCollapsible={viewportSurface === 'tablet' && tabletSplitMode}
          extraNudges={localNudges}
          highlightedNudgeId={highlightedNudge?.id ?? null}
          highlightedNudgeNonce={highlightedNudge?.nonce ?? null}
          onOpenThread={openConversationThread}
          miniChatOpen={miniChatOpen}
          miniChatThreadId={miniChatThreadId}
          onMiniChatThreadSelect={setMiniComposerThread}
          onMiniChatClose={closeMiniComposer}
          onOpenSystem={openSystem}
        />
      )}
      main={(
        <MainPanel
          surface={viewportSurface}
          conversationId={selectedConversationId}
          mainView={mainView}
          onNavigate={selectMainView}
          onOpenThread={openConversationThread}
          onOpenSystem={openSystem}
          threadLayoutSplit={tabletSplitMode}
          miniComposerOpen={miniChatOpen}
          onOpenMiniComposer={openMiniComposer}
          onVoiceUnavailable={() => {
            raiseVoiceUnavailableNudge();
            openSystem(systemTargetForCoreSetting('required_setup'));
          }}
          onRaiseNudge={pushLocalNudge}
          onClearNudge={clearLocalNudge}
          shellOwnsNowNudges={shellNudgeRailVisible}
          mobileNudgeZone={shellBootLoading || !mobileNudgesActive ? undefined : (
            <NudgeZone
              activeView={mainView}
              variant="compact"
              compactInitiallyOpen
              extraNudges={localNudges}
              highlightedNudgeId={highlightedNudge?.id ?? null}
              highlightedNudgeNonce={highlightedNudge?.nonce ?? null}
              onOpenThread={openConversationThread}
              miniChatOpen={miniChatOpen}
              miniChatThreadId={miniChatThreadId}
              onMiniChatThreadSelect={setMiniComposerThread}
              onMiniChatClose={closeMiniComposer}
              onOpenSystem={openSystem}
            />
          )}
          systemTarget={systemTarget}
          shellBootLoading={shellBootLoading}
        />
      )}
      surface={viewportSurface}
      layoutMode={tabletLayoutMode}
      splitModeActive={tabletSplitMode}
      fullFrameMain={shellBootLoading}
    />
  );
}

export default App;
