import type { ReactNode } from 'react';
import { brandTagPalette } from '../Theme';
import { FilterDenseTag } from './FilterDenseTag';

function projectTagHash(label: string): number {
  const normalized = label.trim().toLowerCase();
  if (normalized.length === 0) {
    return 0;
  }
  return Array.from(normalized).reduce((sum, char) => sum + char.charCodeAt(0), 0);
}

export function ProjectTag({
  label,
  children,
  className,
  casing = 'upper',
}: {
  label: string;
  children: ReactNode;
  className?: string;
  casing?: 'upper' | 'normal';
}) {
  return (
    <FilterDenseTag
      casing={casing}
      className={[brandTagPalette[projectTagHash(label) % brandTagPalette.length], className]
        .filter(Boolean)
        .join(' ')}
    >
      {children}
    </FilterDenseTag>
  );
}
