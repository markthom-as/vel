import type { ReactNode } from 'react';
import { PanelMetricChip } from './PanelMetricChip';

export type PanelMetricItem = {
  label: string;
  value: number;
  /** If set, shown instead of stringified `value` (e.g. `3/5`). `value` still drives active styling. */
  displayValue?: string;
  icon: (active: boolean) => ReactNode;
  /** When false, the numeric value is omitted (default true). */
  showValue?: boolean;
  /** Optional tooltip for the chip (e.g. sync posture). */
  title?: string;
};

export function PanelMetricStrip({
  items,
  'aria-label': ariaLabel,
}: {
  items: PanelMetricItem[];
  'aria-label'?: string;
}) {
  return (
    <div
      className="flex flex-wrap items-center gap-1.5 opacity-75"
      {...(ariaLabel ? { role: 'group' as const, 'aria-label': ariaLabel } : {})}
    >
      {items.map((item) => {
        const active = item.value > 0;
        const valueText = item.displayValue ?? String(item.value);
        const showValue = item.showValue !== false;
        return (
          <PanelMetricChip
            key={item.label}
            active={active}
            label={item.label}
            icon={item.icon(active)}
            showValue={showValue}
            valueDisplay={showValue ? valueText : undefined}
            title={item.title}
          />
        );
      })}
    </div>
  );
}
