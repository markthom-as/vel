import type { ReactNode } from 'react';

export function TaskGroup({
  title,
  visible,
  children,
}: {
  title: string;
  visible: boolean;
  children: ReactNode;
}) {
  if (!visible) {
    return null;
  }

  return (
    <section className="space-y-2">
      <p className="text-[10px] uppercase tracking-[0.2em] text-zinc-500">{title}</p>
      <div className="space-y-2">{children}</div>
    </section>
  );
}
