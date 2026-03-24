import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard } from '../itemPill';

export type ObjectRowTone =
  | 'neutral'
  | 'accent'
  | 'warning'
  | 'emphasis'
  | 'ghost'
  | 'selected'
  | 'activeBrand';

export type ObjectRowDensity =
  | 'compact'
  | 'standard'
  | 'comfortable'
  | 'button'
  | 'sectionHeader';

const rowToneMap = {
  neutral: 'muted',
  accent: 'brand',
  warning: 'warm',
  emphasis: 'emphasis',
  ghost: 'ghost',
  selected: 'muted',
} as const;

const rowDensityMap = {
  compact: 'compact',
  standard: 'laneRow',
  comfortable: 'comfortable',
  button: 'rowButton',
  sectionHeader: 'sectionHeader',
} as const;

export function objectRowFrameClass(tone: ObjectRowTone = 'neutral', density: ObjectRowDensity = 'standard') {
  if (tone === 'selected') {
    return 'relative w-full overflow-hidden rounded-[20px] border border-zinc-100 bg-zinc-100 p-3 text-left text-zinc-950 shadow-none transition';
  }

  if (tone === 'activeBrand') {
    return cn(
      itemPillCard('emphasis', density === 'button' ? 'rowButton' : rowDensityMap[density]),
      'border-[#ff6b00] bg-[color:var(--vel-color-panel-2)]/48 shadow-[0_0_0_1px_rgba(255,107,0,0.74),0_0_36px_rgba(255,107,0,0.24),inset_0_0_0_1px_rgba(255,190,130,0.22)] before:pointer-events-none before:absolute before:inset-0 before:rounded-[20px] before:bg-[linear-gradient(120deg,rgba(255,145,66,0.12),transparent_45%,rgba(255,145,66,0.08))] before:content-[\'\']',
    );
  }

  return itemPillCard(rowToneMap[tone], rowDensityMap[density]);
}

export function ObjectRowFrame({
  tone = 'neutral',
  density = 'standard',
  as: Comp = 'div',
  className,
  children,
}: {
  tone?: ObjectRowTone;
  density?: ObjectRowDensity;
  as?: 'div' | 'article' | 'button' | 'section';
  className?: string;
  children: ReactNode;
}) {
  return <Comp className={cn(objectRowFrameClass(tone, density), className)}>{children}</Comp>;
}

export function ObjectRowLayout({
  leading,
  children,
  actions,
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

export function ObjectRowTitleMetaBand({
  title,
  titleClassName,
  meta,
}: {
  title: ReactNode;
  titleClassName?: string;
  meta: ReactNode;
}) {
  return (
    <div className="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
      <span
        className={cn(
          'max-w-[min(100%,12rem)] shrink-0 truncate text-sm font-medium leading-tight tracking-tight',
          titleClassName,
        )}
      >
        {title}
      </span>
      <div className="flex min-w-0 flex-1 flex-wrap items-center gap-x-1.5 gap-y-1">{meta}</div>
    </div>
  );
}
