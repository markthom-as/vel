import type { ReactNode } from 'react';
import { cn } from '../cn';
import { objectRowFrameClass } from './objectRowFrameClass';

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
  titleTruncate = false,
}: {
  title: ReactNode;
  titleClassName?: string;
  meta: ReactNode;
  titleTruncate?: boolean;
}) {
  return (
    <div className="flex min-w-0 flex-col gap-y-1.5 sm:flex-row sm:flex-wrap sm:items-start sm:gap-x-2 sm:gap-y-1">
      <span
        className={cn(
          titleTruncate
            ? 'max-w-[min(100%,12rem)] shrink-0 truncate text-sm font-medium leading-tight tracking-tight'
            : 'min-w-0 sm:flex-1 whitespace-normal break-words text-sm font-medium leading-tight tracking-tight',
          titleClassName,
        )}
      >
        {title}
      </span>
      <div className="flex min-w-0 flex-wrap items-center gap-x-1.5 gap-y-1 sm:flex-1 sm:justify-end">{meta}</div>
    </div>
  );
}
