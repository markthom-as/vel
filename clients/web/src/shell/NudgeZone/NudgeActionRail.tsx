import type { ReactNode } from 'react';

export function NudgeActionRail({
  children,
  isExpanded,
}: {
  children: ReactNode;
  isExpanded: boolean;
}) {
  return (
    <div
      className={
        isExpanded
          ? '-my-3 flex w-[5.5rem] shrink-0 self-stretch flex-col gap-1 overflow-hidden py-3'
          : '-my-2.5 flex w-[5.5rem] shrink-0 self-stretch flex-col gap-1 overflow-hidden py-2.5'
      }
      onClick={(event) => event.stopPropagation()}
    >
      {children}
    </div>
  );
}
