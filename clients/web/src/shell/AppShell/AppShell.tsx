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
        <main className={shellChrome.workspaceMain}>{main}</main>
        {nudgeZone ? (
          <div className={shellChrome.workspaceAside}>
            <div className={shellChrome.workspaceAsideInner}>{nudgeZone}</div>
          </div>
        ) : null}
      </div>
      {actionBar}
    </div>
  );
}
