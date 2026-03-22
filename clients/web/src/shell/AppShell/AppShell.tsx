import type { ReactNode } from 'react';
import { APP_SHELL_BELOW_NAVBAR_TOP_CLASS } from '../Navbar/navbarChrome';

interface AppShellProps {
  navigation: ReactNode;
  main: ReactNode;
  infoPanel?: ReactNode;
  infoPanelOpen?: boolean;
}

export function AppShell({
  navigation,
  main,
  infoPanel,
  infoPanelOpen = false,
}: AppShellProps) {
  return (
    <div className="flex h-screen flex-col bg-zinc-950 text-zinc-100">
      {navigation}
      <div className="flex min-h-0 flex-1">
        <main className="flex min-w-0 flex-1 flex-col overflow-hidden">
          {main}
        </main>
        {infoPanel ? (
          <>
            {infoPanelOpen ? (
              <aside className="hidden h-full w-[22rem] shrink-0 overflow-hidden border-l border-zinc-800/90 bg-zinc-950/95 transition-[width] duration-200 md:flex md:flex-col">
                {infoPanel}
              </aside>
            ) : null}
            {infoPanelOpen ? (
              <aside
                className={`fixed bottom-0 right-0 z-40 flex w-[min(88vw,22rem)] flex-col overflow-hidden border-l border-zinc-800/90 bg-zinc-950 md:hidden ${APP_SHELL_BELOW_NAVBAR_TOP_CLASS}`}
              >
                {infoPanel}
              </aside>
            ) : null}
          </>
        ) : null}
      </div>
    </div>
  );
}
