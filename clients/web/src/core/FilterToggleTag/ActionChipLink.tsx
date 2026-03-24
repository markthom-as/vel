import type { AnchorHTMLAttributes, ReactNode } from 'react';
import { cn } from '../cn';
import { filterPillActionToneClass, filterPillFrame } from './filterPillClasses';

export function ActionChipLink({
  children,
  className,
  tone = 'muted',
  iconOnly = false,
  variant = 'default',
  ...props
}: {
  children: ReactNode;
  className?: string;
  tone?: 'neutral' | 'muted' | 'brand' | 'success' | 'ghost';
  iconOnly?: boolean;
  variant?: 'default' | 'message';
} & Omit<AnchorHTMLAttributes<HTMLAnchorElement>, 'className'>) {
  return (
    <a
      className={cn(
        filterPillFrame,
        '!rounded-full',
        filterPillActionToneClass(tone),
        iconOnly
          ? '!h-5 !min-h-5 !w-5 !gap-0 !px-0 !py-0 text-[10px] font-medium [&_svg]:!h-3 [&_svg]:!w-3 [&>*]:inline-flex [&>*]:items-center [&>*]:leading-none'
          : variant === 'message'
            ? '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4 [&_svg]:!max-h-4 [&_svg]:!max-w-4'
            : '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4 [&_svg]:!max-h-4 [&_svg]:!max-w-4',
        className,
      )}
      {...props}
    >
      {children}
    </a>
  );
}
