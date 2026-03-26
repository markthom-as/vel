import type { ReactNode } from 'react';
import { resolveProjectSemantic } from '../Theme/semanticRegistry';
import { FilterDenseTag } from './FilterDenseTag';

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
  const semantic = resolveProjectSemantic(label);

  return (
    <FilterDenseTag
      casing={casing}
      className={[semantic.tagClassName, className]
        .filter(Boolean)
        .join(' ')}
    >
      {children}
    </FilterDenseTag>
  );
}
