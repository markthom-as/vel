import { CheckCircleIcon, SparkIcon, SyncIcon, ThreadsIcon, WarningIcon } from '../../../core/Icons';
import { PanelMetricStrip } from '../../../core/PanelMetricStrip';
import { uiTheme } from '../../../core/Theme';
import type { NowMeshSummaryData, NowMeshSyncStateData } from '../../../types';

function syncMeshMetric(
  mesh: NowMeshSummaryData | null | undefined,
  queued: number,
): {
  kind: 'idle' | 'active' | 'error';
  showValue: boolean;
  title: string;
} {
  const syncState = mesh?.sync_state as NowMeshSyncStateData | undefined;
  const error = syncState === 'offline' || syncState === 'stale';

  if (error) {
    return {
      kind: 'error',
      showValue: queued > 0,
      title:
        queued > 0
          ? `Mesh sync needs attention · ${queued} queued write${queued === 1 ? '' : 's'}`
          : 'Mesh sync needs attention',
    };
  }
  if (queued > 0) {
    return {
      kind: 'active',
      showValue: true,
      title: `${queued} queued write${queued === 1 ? '' : 's'} pending`,
    };
  }
  return {
    kind: 'idle',
    showValue: false,
    title: 'Sync queue clear',
  };
}

export function NowMetricStrip({
  nudgeCount,
  threadAttentionCount,
  queuedWriteCount,
  meshSummary,
}: {
  nudgeCount: number;
  threadAttentionCount: number;
  queuedWriteCount: number;
  meshSummary?: NowMeshSummaryData | null;
}) {
  const sync = syncMeshMetric(meshSummary, queuedWriteCount);

  return (
    <PanelMetricStrip
      items={[
        {
          label: 'Nudges',
          value: nudgeCount,
          icon: (active) => (
            <SparkIcon size={12} className={active ? uiTheme.brandText : 'text-zinc-600'} />
          ),
        },
        {
          label: 'Threads',
          value: threadAttentionCount,
          icon: (active) => (
            <ThreadsIcon size={12} className={active ? uiTheme.brandSoftText : 'text-zinc-600'} />
          ),
        },
        {
          label: 'Sync',
          value: queuedWriteCount,
          showValue: sync.showValue,
          title: sync.title,
          icon: () =>
            sync.kind === 'idle' ? (
              <CheckCircleIcon size={12} className="text-emerald-400" aria-hidden />
            ) : sync.kind === 'active' ? (
              <SyncIcon size={12} className="animate-spin text-amber-400/90" aria-hidden />
            ) : (
              <WarningIcon size={12} className="text-rose-400" aria-hidden />
            ),
        },
      ]}
    />
  );
}
