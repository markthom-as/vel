import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard, itemPillMeta } from '../itemPill';

export type PanelItemSurface = Parameters<typeof itemPillCard>[0];

/** Comfortable list / panel card shell (inbox rows, Now action rows, etc.). */
export function PanelItemShell({
  surface,
  as: Comp = 'article',
  className,
  children,
}: {
  surface: PanelItemSurface;
  as?: 'article' | 'div';
  className?: string;
  children: ReactNode;
}) {
  return <Comp className={cn(itemPillCard(surface, 'comfortable'), className)}>{children}</Comp>;
}

export function PanelItemMain({ children }: { children: ReactNode }) {
  return <div className="min-w-0 flex-1">{children}</div>;
}

/** Top row of kind / state / project chips. */
export function PanelItemMetaRow({ children }: { children: ReactNode }) {
  return <div className="flex flex-wrap items-center gap-2">{children}</div>;
}

export function PanelItemTitle({
  children,
  as: Comp = 'h2',
  size = 'lg',
}: {
  children: ReactNode;
  as?: 'h2' | 'h3' | 'p';
  size?: 'lg' | 'sm';
}) {
  const cls = size === 'lg' ? 'text-lg font-medium text-zinc-100' : 'text-sm font-medium text-zinc-100';
  return <Comp className={cls}>{children}</Comp>;
}

export function PanelItemSummary({
  children,
  spacing = 'comfortable',
}: {
  children: ReactNode;
  spacing?: 'comfortable' | 'compact';
}) {
  return (
    <p
      className={cn(
        'text-sm leading-6 text-zinc-300',
        spacing === 'comfortable' ? 'mt-2' : 'mt-1',
      )}
    >
      {children}
    </p>
  );
}

/** Secondary facts (timestamps, confidence, etc.). */
export function PanelItemDetailLine({ children }: { children: ReactNode }) {
  return (
    <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">{children}</div>
  );
}

export function PanelItemSectionLabel({ children }: { children: ReactNode }) {
  return <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">{children}</p>;
}

export function PanelItemChipRow({ children, className }: { children: ReactNode; className?: string }) {
  return <div className={cn('mt-2 flex flex-wrap gap-2', className)}>{children}</div>;
}

export function PanelItemLabeledChips({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="mt-3">
      <PanelItemSectionLabel>{label}</PanelItemSectionLabel>
      <PanelItemChipRow>{children}</PanelItemChipRow>
    </div>
  );
}

/** Now “attention” rows: title + summary beside inline actions. */
export function PanelItemInlineLayout({ children }: { children: ReactNode }) {
  return <div className="flex items-start justify-between gap-3">{children}</div>;
}

/** Inbox-style: main stacks above actions on narrow viewports. */
export function PanelItemStackLayout({ children }: { children: ReactNode }) {
  return (
    <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">{children}</div>
  );
}

export function PanelItemStackedActions({ children }: { children: ReactNode }) {
  return <div className="flex w-full flex-col gap-2 lg:w-56">{children}</div>;
}

export function PanelItemInlineActions({ children }: { children: ReactNode }) {
  return <div className="flex flex-wrap gap-2">{children}</div>;
}

/**
 * Small uppercase / state pills aligned with `itemPillMeta` on dark surfaces.
 * - `kind`: primary classification (intervention kind, etc.)
 * - `state`: workflow state or placeholder text
 */
export function PanelMetaPill({ tone, children }: { tone: 'kind' | 'state'; children: ReactNode }) {
  const base = itemPillMeta.onDark;
  const extra =
    tone === 'kind'
      ? 'bg-zinc-950/90 px-2.5 text-[11px] tracking-[0.18em]'
      : 'border-zinc-800 bg-zinc-950/80 px-2.5 text-xs normal-case tracking-normal text-zinc-500';
  return <span className={cn(base, extra)}>{children}</span>;
}
