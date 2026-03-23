import type { ReactNode } from 'react';

interface AppShellProps {
  navigation: ReactNode;
  main: ReactNode;
}

export function AppShell({ navigation, main }: AppShellProps) {
  return (
    <div className="flex h-screen flex-col bg-zinc-950 text-zinc-100">
      {navigation}
      <main className="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
        {main}
      </main>
    </div>
  );
}
