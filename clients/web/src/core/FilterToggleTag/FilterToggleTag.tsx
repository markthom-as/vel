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
  'aria-label': ariaLabelProp,
}: {
  label: string;
  count?: number;
  icon?: ReactNode;
  selected: boolean;
  onClick: () => void;
  className?: string;
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
      className={filterPillToggleClassNames(selected, className)}
    >
      {icon ? <span className="shrink-0 [&_svg]:block">{icon}</span> : null}
      <span className="min-w-0 truncate">{label}</span>
      {typeof count === 'number' ? (
        <span className={cn(selected ? filterPillCountSelected : filterPillCountMuted)}>{count}</span>
      ) : null}
    </button>
  );
}
