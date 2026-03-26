import type { ReactNode } from 'react';

export function SystemSurfaceLayout({
  header,
  groupCards,
  sidebar,
  content,
}: {
  header: ReactNode;
  groupCards: ReactNode;
  sidebar: ReactNode;
  content: ReactNode;
}) {
  return (
    <div className="flex-1 bg-transparent">
      <div className="mx-auto max-w-7xl px-4 py-4 pb-32 sm:px-6">
        <div className="space-y-5">
          <div className="space-y-3">
            {header}
            {groupCards}
          </div>

          <div className="grid gap-5 xl:grid-cols-[18rem_minmax(0,1fr)]">
            <aside className="self-start xl:sticky xl:top-[5.25rem] xl:overflow-visible">
              {sidebar}
            </aside>

            <section className="min-w-0 space-y-5">
              {content}
            </section>
          </div>
        </div>
      </div>
    </div>
  );
}
