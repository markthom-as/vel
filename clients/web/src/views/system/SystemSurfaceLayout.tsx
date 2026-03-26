import type { ReactNode } from 'react';

export function SystemSurfaceLayout({
  header,
  stats,
  sidebar,
  content,
}: {
  header: ReactNode;
  stats: ReactNode;
  sidebar: ReactNode;
  content: ReactNode;
}) {
  return (
    <div className="flex-1 bg-transparent">
      <div className="mx-auto max-w-7xl px-4 py-4 pb-32 sm:px-6">
        <div className="space-y-4">
          <div className="space-y-2">
            {header}
            {stats}
          </div>

          <div className="grid gap-4 xl:grid-cols-[14rem_minmax(0,1fr)]">
            <aside className="self-start xl:sticky xl:top-[5.25rem] xl:overflow-visible">
              {sidebar}
            </aside>

            <section className="min-w-0 space-y-4">
              {content}
            </section>
          </div>
        </div>
      </div>
    </div>
  );
}
