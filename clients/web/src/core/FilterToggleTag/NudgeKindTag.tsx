import type { ReactNode } from 'react';
import { FilterDenseTag } from './FilterDenseTag';

export function NudgeKindTag({
  urgent,
  children,
}: {
  urgent: boolean;
  children: ReactNode;
}) {
  return (
    <FilterDenseTag
      className={
        urgent
          ? 'border-amber-600/45 bg-amber-950/70 text-amber-100'
          : 'border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)]/80 text-[var(--vel-color-accent-soft)]'
      }
    >
      {children}
    </FilterDenseTag>
  );
}
