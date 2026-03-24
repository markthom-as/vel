import type { ReactNode } from 'react';
import { shellChrome } from '../../core/Theme';

interface AppShellProps {
  navigation: ReactNode;
  main: ReactNode;
  nudgeZone?: ReactNode;
  actionBar?: ReactNode;
}

export function AppShell({ navigation, main, nudgeZone, actionBar }: AppShellProps) {
  return (
    <div className={shellChrome.app}>
      {navigation}
      <div className={shellChrome.workspace}>
        <main data-testid="app-shell-main" className={shellChrome.workspaceMain}>{main}</main>
        {nudgeZone ? (
          <div className={shellChrome.workspaceAside}>
            <div data-testid="app-shell-nudges" className={shellChrome.workspaceAsideInner}>
              <div data-testid="app-shell-nudges-scroll" className={shellChrome.workspaceAsideScroll}>{nudgeZone}</div>
            </div>
          </div>
        ) : null}
      </div>
      {actionBar}
    </div>
  );
}
