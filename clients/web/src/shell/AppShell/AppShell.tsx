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
}: AppShellProps) {
  const isDesktopOrTablet = surface !== 'mobile';
  const workspaceClass =
    surface === 'mobile'
      ? shellChrome.workspaceMobile
      : surface === 'tablet'
        ? shellChrome.workspaceTablet
        : shellChrome.workspace;
  const workspaceAsideClass = isDesktopOrTablet ? shellChrome.workspaceAside : shellChrome.workspaceAsideHidden;
  const layoutState = splitModeActive || layoutMode === 'split' ? 'split' : 'single';

  return (
    <div className={shellChrome.app} data-layout={layoutState}>
      {navigation}
      <div className={workspaceClass} data-testid={`app-shell-workspace-${surface}`}>
        <main data-testid="app-shell-main" className={shellChrome.workspaceMain}>{main}</main>
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
