import type { ReactNode } from 'react';
import { cn } from '../cn';
import {
  filterPillCountMuted,
  filterPillCountSelected,
  filterPillToggleClassNames,
} from './filterPillClasses';

export function FilterToggleTag({
  label,
  count,
  icon,
  selected,
  onClick,
  className,
  size = 'default',
  'aria-label': ariaLabelProp,
}: {
  label: string;
  count?: number;
  icon?: ReactNode;
  selected: boolean;
  onClick: () => void;
  className?: string;
  size?: 'default' | 'dense';
  'aria-label'?: string;
}) {
  const ariaLabel =
    ariaLabelProp ??
    (typeof count === 'number' ? `${label}, ${count} ${count === 1 ? 'item' : 'items'}` : label);

  return (
    <button
      type="button"
      onClick={onClick}
      aria-pressed={selected}
      aria-label={ariaLabel}
      className={filterPillToggleClassNames(
        selected,
        cn(size === 'dense' ? '!min-h-[1.3rem] !gap-1 !px-2 !py-[0.24rem] !text-[10px] !tracking-[0.13em]' : '', className),
      )}
    >
      {icon ? <span className="inline-flex h-full shrink-0 items-center justify-center self-stretch leading-none [&_svg]:block [&_svg]:align-middle">{icon}</span> : null}
      <span className="min-w-0 truncate leading-none">{label}</span>
      {typeof count === 'number' ? (
        <span className={cn(selected ? filterPillCountSelected : filterPillCountMuted, 'inline-flex h-full items-center self-stretch justify-center leading-none')}>{count}</span>
      ) : null}
    </button>
  );
}
