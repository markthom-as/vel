import type { ReactNode } from 'react';
import { ClockIcon, SparkIcon, ThreadsIcon } from '../../../core/Icons';
import { SurfaceMetricChip } from '../../../core/SurfaceChips';
import { uiTheme } from '../../../core/Theme';

type MetricTone = 'accent' | 'neutral';

function MetricChip({
  tone,
  label,
  value,
  icon,
}: {
  tone: MetricTone;
  label: string;
  value: number;
  icon: (active: boolean) => ReactNode;
}) {
  const active = tone === 'accent';
  return (
    <SurfaceMetricChip tone={tone}>
      {icon(active)}
      <span>{label}</span>
      <span className="text-zinc-200">{value}</span>
    </SurfaceMetricChip>
  );
}

export function NowMetricStrip({
  nudgeCount,
  threadAttentionCount,
  queuedWriteCount,
}: {
  nudgeCount: number;
  threadAttentionCount: number;
  queuedWriteCount: number;
}) {
  return (
    <div className="flex flex-wrap items-center gap-2">
      <MetricChip
        tone={nudgeCount > 0 ? 'accent' : 'neutral'}
        label="Nudges"
        value={nudgeCount}
        icon={(active) => (
          <SparkIcon size={12} className={active ? uiTheme.brandText : 'text-zinc-500'} />
        )}
      />
      <MetricChip
        tone={threadAttentionCount > 0 ? 'accent' : 'neutral'}
        label="Threads"
        value={threadAttentionCount}
        icon={(active) => (
          <ThreadsIcon size={12} className={active ? uiTheme.brandText : 'text-zinc-500'} />
        )}
      />
      <MetricChip
        tone={queuedWriteCount > 0 ? 'accent' : 'neutral'}
        label="Sync"
        value={queuedWriteCount}
        icon={(active) => (
          <ClockIcon size={12} className={active ? uiTheme.brandText : 'text-zinc-500'} />
        )}
      />
    </div>
  );
}
