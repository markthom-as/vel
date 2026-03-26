import type { ReactNode } from 'react';
import { resolveNudgeKindSemantic } from '../Theme/semanticRegistry';
import { FilterDenseTag } from './FilterDenseTag';

export function NudgeKindTag({
  urgent,
  children,
}: {
  urgent: boolean;
  children: ReactNode;
}) {
  const semantic = resolveNudgeKindSemantic(urgent);

  return (
    <FilterDenseTag
      className={semantic.tagClassName}
    >
      {children}
    </FilterDenseTag>
  );
}
