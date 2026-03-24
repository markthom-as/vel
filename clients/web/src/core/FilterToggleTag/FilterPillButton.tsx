import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { cn } from '../cn';
import { filterPillActionToneClass, filterPillFrame } from './filterPillClasses';

/** Same shell as filter toggles / metric strip, for sentence-case actions (e.g. Now nudge row). */
export function FilterPillButton({
  children,
  className,
  size = 'default',
  tone = 'neutral',
  ...props
}: {
  children: ReactNode;
  className?: string;
  size?: 'default' | 'dense' | 'icon';
  tone?: 'neutral' | 'muted' | 'brand' | 'success' | 'ghost';
} & Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'className' | 'type'>) {
  return (
    <button
      type="button"
      className={cn(
        filterPillFrame,
        '!rounded-full',
        filterPillActionToneClass(tone),
        size === 'icon'
          ? '!h-5 !min-h-5 !w-5 !gap-0 !px-0 !py-0 text-[10px] font-medium [&_svg]:!h-3 [&_svg]:!w-3 [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none'
          : size === 'dense'
            ? '!min-h-5 !gap-1 !px-1.5 !py-0.5 text-[10px] font-medium [&_svg]:!h-3 [&_svg]:!w-3 [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none'
            : 'text-sm font-medium [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none',
        className,
      )}
      {...props}
    >
      {children}
    </button>
  );
}
