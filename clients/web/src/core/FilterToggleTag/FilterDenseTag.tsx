import type { ReactNode } from 'react';
import { cn } from '../cn';
import { filterDenseTagToneClass, filterPillFrame } from './filterPillClasses';

/** Read-only compact pill: same frame as filter toggles; pass `className` for semantic fills (Now nudge row). */
export function FilterDenseTag({
  children,
  className,
  size = 'dense',
  tone = 'neutral',
  casing = 'upper',
}: {
  children: ReactNode;
  className?: string;
  size?: 'dense' | 'compact';
  tone?: 'neutral' | 'muted' | 'brand' | 'ghost';
  casing?: 'upper' | 'normal';
}) {
  return (
    <span
      className={cn(
        filterPillFrame,
        '!rounded-[0.48rem]',
        size === 'dense'
          ? cn(
              'shrink-0 !min-h-0 !items-center !justify-center !gap-1 !px-1.5 !py-[0.24rem] !text-[9px] !font-medium !leading-none [&_svg]:!h-2.5 [&_svg]:!w-2.5 [&_svg]:!max-h-2.5 [&_svg]:!max-w-2.5 [&_svg]:shrink-0 [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none',
              casing === 'upper' ? '!uppercase !tracking-[0.14em]' : '!normal-case !tracking-normal',
            )
          : cn(
              'shrink-0 !min-h-[1.35rem] !items-center !justify-center !gap-1 !px-2 !py-0.5 !text-[10px] !font-medium !leading-none [&_svg]:!h-3 [&_svg]:!w-3 [&_svg]:shrink-0 [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none',
              casing === 'upper' ? '!uppercase !tracking-[0.13em]' : '!normal-case !tracking-normal',
            ),
        filterDenseTagToneClass(tone),
        className,
      )}
    >
      {children}
    </span>
  );
}
