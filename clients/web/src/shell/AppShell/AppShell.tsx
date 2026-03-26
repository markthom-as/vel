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

const isNudgeZoneVisible = (surface: ViewportSurface) => surface !== 'mobile';

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
  const isDesktopOrTablet = surface !== 'mobile';
  const workspaceClass =
    fullFrameMain
      ? shellChrome.workspaceFullFrame
      : surface === 'mobile'
        ? shellChrome.workspaceMobile
        : surface === 'tablet'
          ? shellChrome.workspaceTablet
          : shellChrome.workspace;
  const workspaceMainClass = fullFrameMain ? shellChrome.workspaceMainFullFrame : shellChrome.workspaceMain;
  const workspaceAsideClass = isDesktopOrTablet ? shellChrome.workspaceAside : shellChrome.workspaceAsideHidden;
  const layoutState = splitModeActive || layoutMode === 'split' ? 'split' : 'single';
  const workspaceTestId = fullFrameMain ? 'app-shell-workspace-full-frame' : `app-shell-workspace-${surface}`;

  return (
    <div className={shellChrome.app} data-layout={layoutState}>
      {navigation}
      <div className={workspaceClass} data-testid={workspaceTestId}>
        <main data-testid="app-shell-main" className={workspaceMainClass}>{main}</main>
        {nudgeZone && isNudgeZoneVisible(surface) ? (
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
