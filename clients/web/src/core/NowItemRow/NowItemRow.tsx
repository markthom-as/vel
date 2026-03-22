import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard } from '../itemPill';

export type NowItemRowSurface = Parameters<typeof itemPillCard>[0];
export type ItemPillShellKind = Parameters<typeof itemPillCard>[1];

/**
 * Outer chrome for Now list rows: same `itemPill` shell as {@link CompactTaskLaneRow}
 * (`laneRow` by default) and nudge bars (`compact` when needed).
 */
export function NowItemRowShell({
  surface = 'muted',
  shell = 'laneRow',
  as: Comp = 'div',
  className,
  children,
}: {
  surface?: NowItemRowSurface;
  shell?: ItemPillShellKind;
  as?: 'div' | 'article';
  className?: string;
  children: ReactNode;
}) {
  return <Comp className={cn(itemPillCard(surface, shell), className)}>{children}</Comp>;
}

/**
 * Shared inner grid: optional leading affordance (task checkbox) + main column + right action stack.
 * Matches `CompactTaskLaneRow` / inbox intervention rows.
 */
export function NowItemRowLayout({
  leading,
  children,
  actions,
  /** `stack`: vertical action column (default). `inline`: single row of pills (e.g. dense headers). */
  actionsLayout = 'stack',
}: {
  leading?: ReactNode;
  children: ReactNode;
  actions?: ReactNode;
  actionsLayout?: 'stack' | 'inline';
}) {
  const actionsClassName =
    actionsLayout === 'inline'
      ? 'flex shrink-0 flex-row flex-wrap items-center justify-end gap-1.5 self-center'
      : 'flex shrink-0 flex-col items-end justify-center gap-1.5 self-stretch';

  return (
    <div className="flex items-stretch gap-3">
      {leading ?? null}
      <div className="relative z-10 flex min-w-0 flex-1 flex-row items-stretch gap-2">
        <div className="flex min-w-0 flex-1 flex-col justify-center gap-1">{children}</div>
        {actions ? <div className={actionsClassName}>{actions}</div> : null}
      </div>
    </div>
  );
}
