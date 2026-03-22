import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { cn } from '../cn';
import { filterPillActionIdle, filterPillFrame } from './filterPillClasses';

/** Same shell as filter toggles / metric strip, for sentence-case actions (e.g. Now nudge row). */
export function FilterPillButton({
  children,
  className,
  ...props
}: {
  children: ReactNode;
  className?: string;
} & Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'className' | 'type'>) {
  return (
    <button type="button" className={cn(filterPillFrame, filterPillActionIdle, 'text-sm font-medium', className)} {...props}>
      {children}
    </button>
  );
}
