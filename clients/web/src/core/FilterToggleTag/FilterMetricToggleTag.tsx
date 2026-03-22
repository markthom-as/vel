import type { ReactNode } from 'react';
import { PanelMetricChip } from '../PanelMetricStrip/PanelMetricChip';

/**
 * Single-select filter control — same {@link PanelMetricChip} as Now’s {@link PanelMetricStrip}.
 */
export function FilterMetricToggleTag({
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
  icon: ReactNode;
  selected: boolean;
  onClick: () => void;
  className?: string;
  'aria-label'?: string;
}) {
  const ariaLabel =
    ariaLabelProp ??
    (typeof count === 'number' ? `${label}, ${count} ${count === 1 ? 'item' : 'items'}` : label);

  return (
    <PanelMetricChip
      as="button"
      active={selected}
      label={label}
      icon={icon}
      showValue={typeof count === 'number'}
      valueDisplay={typeof count === 'number' ? String(count) : undefined}
      onClick={onClick}
      aria-label={ariaLabel}
      aria-pressed={selected}
      className={className}
    />
  );
}
