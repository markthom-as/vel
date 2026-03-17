import type { ReactNode } from 'react';

interface AppShellProps {
  sidebar: ReactNode;
  main: ReactNode;
  contextPanel?: ReactNode;
}

export function AppShell({ sidebar, main, contextPanel }: AppShellProps) {
  return (
    <div className="flex h-screen bg-zinc-950 text-zinc-100">
      <aside className="w-72 shrink-0 border-r border-zinc-800 flex flex-col">
        {sidebar}
      </aside>
      <main className="flex-1 min-w-0 flex flex-col overflow-hidden">
        {main}
      </main>
      {contextPanel ? (
        <aside className="w-80 shrink-0 border-l border-zinc-800 flex flex-col overflow-hidden">
          {contextPanel}
        </aside>
      ) : null}
    </div>
  );
}
