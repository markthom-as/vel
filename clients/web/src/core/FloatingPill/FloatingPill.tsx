import type { ReactNode } from 'react';
import { cn } from '../cn';
import { itemPillCard } from '../itemPill';

export interface FloatingPillProps {
  children: ReactNode;
  decoration?: ReactNode;
  decorationClassName?: string;
  className?: string;
  contentClassName?: string;
  decorationOffsetClassName?: string;
  onPress?: () => void;
}

/**
 * Shared floating pill shell with an outboard left decoration that should not
 * affect the body layout. Used for nudges and similar interruption-style pills.
 */
export function FloatingPill({
  children,
  decoration,
  decorationClassName,
  className,
  contentClassName,
  decorationOffsetClassName,
  onPress,
}: FloatingPillProps) {
  return (
    <article
      className={cn('relative overflow-visible', onPress ? 'cursor-pointer' : null, className)}
      onClick={(event) => {
        if (!onPress) return;
        const target = event.target as HTMLElement | null;
        if (target?.closest('button, a, input, textarea, select, [role=\"button\"]')) {
          return;
        }
        onPress();
      }}
    >
      {decoration ? (
        <span
          aria-hidden
          className={cn(
            'pointer-events-none absolute left-0 top-1/2 z-[2] inline-flex h-7 w-7 -translate-y-1/2 items-center justify-center rounded-full border leading-none',
            decorationOffsetClassName ?? '-translate-x-[62%]',
            decorationClassName,
          )}
        >
          {decoration}
        </span>
      ) : null}
      <div
        className={cn(
          itemPillCard('queue', 'laneRow'),
          'flex min-w-0 items-center justify-between gap-2 overflow-visible border px-4 py-2.5',
          contentClassName,
        )}
      >
        {children}
      </div>
    </article>
  );
}
