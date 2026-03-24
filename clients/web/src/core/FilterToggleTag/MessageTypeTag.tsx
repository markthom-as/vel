import type { ReactNode } from 'react';
import { cn } from '../cn';
import { FilterDenseTag } from './FilterDenseTag';

export function MessageTypeTag({
  variant,
  children,
  className,
}: {
  variant: 'user' | 'assistant';
  children: ReactNode;
  className?: string;
}) {
  const frame =
    variant === 'user'
      ? 'border-emerald-800/60 bg-emerald-900/45 text-emerald-200'
      : 'border-[#ff6b00]/40 bg-[#2d1608]/90 text-[#ffd4b8]';

  return (
    <FilterDenseTag
      size="compact"
      casing="normal"
      className={cn(frame, className)}
    >
      {children}
    </FilterDenseTag>
  );
}
