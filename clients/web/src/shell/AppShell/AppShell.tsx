import type { ReactNode } from 'react';
import type { ViewportSurface } from '../../core/hooks/useViewportSurface';
import { shellChrome } from '../../core/Theme';

type TabletLayoutMode = 'auto' | 'single' | 'split';

interface AppShellProps {
  navigation: ReactNode;
  main: ReactNode;
  nudgeZone?: ReactNode;
  actionBar?: ReactNode;
  surface?: ViewportSurface;
  layoutMode?: TabletLayoutMode;
  splitModeActive?: boolean;
  fullFrameMain?: boolean;
}

type AppShellLayoutState = 'single' | 'split';

function resolveLayoutState(
  surface: ViewportSurface,
  layoutMode: TabletLayoutMode,
  splitModeActive: boolean,
): AppShellLayoutState {
  if (surface !== 'tablet') {
    return 'single';
  }
  return splitModeActive || layoutMode === 'split' ? 'split' : 'single';
}

function workspaceClassForSurface(
  surface: ViewportSurface,
  layoutState: AppShellLayoutState,
  fullFrameMain: boolean,
): string {
  if (fullFrameMain) {
    return shellChrome.workspaceFullFrame;
  }
  if (surface === 'mobile') {
    return shellChrome.workspaceMobile;
  }
  if (surface === 'tablet') {
    return layoutState === 'split' ? shellChrome.workspaceTabletSplit : shellChrome.workspaceTablet;
  }
  return shellChrome.workspace;
}

function isNudgeZoneVisibleForLayout(
  surface: ViewportSurface,
  layoutState: AppShellLayoutState,
): boolean {
  if (surface === 'mobile') {
    return false;
  }
  if (surface === 'tablet') {
    return layoutState === 'split';
  }
  return true;
}

export function AppShell({
  navigation,
  main,
  nudgeZone,
  actionBar,
  surface = 'desktop',
  layoutMode = 'auto',
  splitModeActive = false,
  fullFrameMain = false,
}: AppShellProps) {
  const layoutState = resolveLayoutState(surface, layoutMode, splitModeActive);
  const workspaceClass = workspaceClassForSurface(surface, layoutState, fullFrameMain);
  const workspaceMainClass = fullFrameMain ? shellChrome.workspaceMainFullFrame : shellChrome.workspaceMain;
  const showNudgeZone = nudgeZone && isNudgeZoneVisibleForLayout(surface, layoutState);
  const workspaceAsideClass = showNudgeZone ? shellChrome.workspaceAside : shellChrome.workspaceAsideHidden;
  const workspaceTestId = fullFrameMain ? 'app-shell-workspace-full-frame' : `app-shell-workspace-${surface}`;

  return (
    <div className={shellChrome.app} data-layout={layoutState} data-surface={surface}>
      {navigation}
      <div className={workspaceClass} data-testid={workspaceTestId}>
        <main data-testid="app-shell-main" className={workspaceMainClass}>{main}</main>
        {showNudgeZone ? (
          <div data-testid="app-shell-nudges" className={workspaceAsideClass}>
            <div className={shellChrome.workspaceAsideInner}>
              <div data-testid="app-shell-nudges-scroll" className={shellChrome.workspaceAsideScroll}>{nudgeZone}</div>
            </div>
          </div>
        ) : null}
      </div>
      {actionBar}
    </div>
  );
}
