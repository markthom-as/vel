import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { cn } from '../cn';
import { FilterPillButton } from './FilterPillButton';

export function ActionChipButton({
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
} & Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'className' | 'type'>) {
  return (
    <FilterPillButton
      size={iconOnly ? 'icon' : 'dense'}
      tone={tone}
      className={cn(
        iconOnly
          ? '!gap-0 opacity-85'
          : variant === 'message'
            ? '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4 [&_svg]:!max-h-4 [&_svg]:!max-w-4'
            : '!gap-2 !px-3 !py-1.5 !text-[11px] !font-medium !normal-case leading-tight tracking-normal [&_svg]:!h-4 [&_svg]:!w-4 [&_svg]:!max-h-4 [&_svg]:!max-w-4',
        className,
      )}
      {...props}
    >
      {children}
    </FilterPillButton>
  );
}
