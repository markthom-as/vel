import type { ReactNode } from 'react';
import { cn } from '../cn';
import {
  filterPillCountMuted,
  filterPillCountMutedStrong,
  filterPillMetricClassNames,
} from '../FilterToggleTag/filterPillClasses';

type PanelMetricChipBase = {
  /** When true, uses metric “active” border/text (same semantics as count &gt; 0 on Now). */
  active: boolean;
  label: string;
  icon: ReactNode;
  showValue?: boolean;
  /** Omitted when `showValue` is false. */
  valueDisplay?: string;
  /** Optional tooltip on the outer wrapper (read-only strip). */
  title?: string;
  as?: 'span' | 'button';
  className?: string;
  children?: ReactNode;
  onClick?: () => void;
  'aria-label'?: string;
  'aria-pressed'?: boolean | 'mixed';
  disabled?: boolean;
};

/**
 * Single visual primitive for Now header metrics and Inbox metric-style toggles.
 * Matches {@link PanelMetricStrip} read-only chips; use `as="button"` for filters.
 */
export function PanelMetricChip({
  active,
  label,
  icon,
  showValue = true,
  valueDisplay,
  title,
  as = 'span',
  className,
  children,
  onClick,
  'aria-label': ariaLabel,
  'aria-pressed': ariaPressed,
  disabled,
}: PanelMetricChipBase) {
  const inner = (
    <>
      <span className="shrink-0 [&_svg]:block">{icon}</span>
      <span className={cn('min-w-0 truncate', active ? 'text-zinc-200' : 'text-zinc-500')}>{label}</span>
      {showValue && valueDisplay !== undefined ? (
        <span className={cn(active ? filterPillCountMutedStrong : filterPillCountMuted)}>{valueDisplay}</span>
      ) : null}
      {children}
    </>
  );

  const chipClassName = cn(
    filterPillMetricClassNames(active),
    /**
     * Keep hover/focus subtle so interactive chips match read-only {@link PanelMetricStrip} spans
     * (those have no hover — only border/bg from `filterPillMetricClassNames`).
     */
    as === 'button' &&
      'cursor-pointer transition-[background-color] duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-500/40 focus-visible:ring-offset-2 focus-visible:ring-offset-zinc-950',
    as === 'button' && 'hover:bg-zinc-900/25 active:bg-zinc-900/40',
    className,
  );

  if (as === 'button') {
    return (
      <button
        type="button"
        className={chipClassName}
        onClick={onClick}
        aria-label={ariaLabel}
        aria-pressed={ariaPressed}
        disabled={disabled}
      >
        {inner}
      </button>
    );
  }

  const chip = <span className={chipClassName}>{inner}</span>;
  return title ? (
    <span title={title} className="inline-flex">
      {chip}
    </span>
  ) : (
    chip
  );
}
