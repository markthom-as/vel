import type { ReactNode } from 'react';
import { cn } from '../cn';
import { filterPillFrame } from './filterPillClasses';

/** Read-only compact pill: same frame as filter toggles; pass `className` for semantic fills (Now nudge row). */
export function FilterDenseTag({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <span
      className={cn(
        filterPillFrame,
        'shrink-0 !min-h-0 !gap-1 !px-1.5 !py-0.5 !text-[9px] !font-medium !uppercase !leading-none !tracking-[0.14em] [&_svg]:!h-2.5 [&_svg]:!w-2.5 [&_svg]:!max-h-2.5 [&_svg]:!max-w-2.5 [&_svg]:shrink-0',
        className,
      )}
    >
      {children}
    </span>
  );
}
