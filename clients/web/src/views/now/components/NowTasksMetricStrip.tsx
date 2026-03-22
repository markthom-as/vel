import { CheckCircleIcon, ClipboardCheckIcon, LayersIcon } from '../../../core/Icons';
import { PanelMetricStrip } from '../../../core/PanelMetricStrip';
import { uiTheme } from '../../../core/Theme';

export function NowTasksMetricStrip({
  completedCount,
  remainingCount,
  backlogCount,
}: {
  completedCount: number;
  remainingCount: number;
  backlogCount: number;
}) {
  const total = Math.max(1, completedCount + remainingCount);
  const pct = Math.round((completedCount / total) * 100);

  return (
    <PanelMetricStrip
      aria-label="Task queue counts"
      items={[
        {
          label: 'Completed',
          value: completedCount,
          displayValue: `${completedCount}/${total}`,
          title: `${pct}% done`,
          icon: (active) => (
            <CheckCircleIcon size={12} className={active ? uiTheme.brandText : 'text-zinc-600'} />
          ),
        },
        {
          label: 'Remaining',
          value: remainingCount,
          icon: (active) => (
            <ClipboardCheckIcon size={12} className={active ? uiTheme.brandSoftText : 'text-zinc-600'} />
          ),
        },
        {
          label: 'Backlog',
          value: backlogCount,
          icon: (active) => (
            <LayersIcon size={12} className={active ? uiTheme.brandSoftText : 'text-zinc-600'} />
          ),
        },
      ]}
    />
  );
}
