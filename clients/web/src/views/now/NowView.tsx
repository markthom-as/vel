import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { contextQueryKeys, loadNow, updateCommitment } from '../../data/context';
import { invalidateQuery, useQuery } from '../../data/query';
import type { NowData } from '../../types';
import {
  PanelSectionHeaderBand,
  PanelSectionHeaderLead,
  PanelSectionHeaderTrail,
} from '../../core/PanelChrome';
import { SurfaceState } from '../../core/SurfaceState';
import { NowMetricStrip } from './components/NowMetricStrip';
import { NowNudgeStrip } from './components/NowNudgeStrip';
import { NowScheduleSection } from './components/NowScheduleSection';
import { NowTasksSection } from './components/NowTasksSection';
import { surfaceShell } from '../../core/Theme';
import {
  dedupeActionItems,
  dedupeTasks,
  findActiveEvent,
  nowLocationLabel,
  nudgeOpenSystemTarget,
} from './nowModel';
import type { SystemNavigationTarget } from '../system';

interface NowViewProps {
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
}

export function NowView({ onOpenThread, onOpenSystem }: NowViewProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const commitmentsKey = useMemo(() => contextQueryKeys.commitments(25), []);
  const { data, loading, error, refetch } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );

  const [pendingCommitments, setPendingCommitments] = useState<Record<string, true>>({});
  const [commitmentMessages, setCommitmentMessages] = useState<
    Record<string, { status: 'success' | 'error'; message: string }>
  >({});

  useEffect(() => {
    const handleFocus = () => {
      void refetch();
    };
    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        void refetch();
      }
    };
    const interval = window.setInterval(() => {
      void refetch();
    }, 60_000);

    window.addEventListener('focus', handleFocus);
    document.addEventListener('visibilitychange', handleVisibilityChange);
    return () => {
      window.clearInterval(interval);
      window.removeEventListener('focus', handleFocus);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [refetch]);

  const scrollRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const [bottomFadeVisible, setBottomFadeVisible] = useState(false);

  const updateBottomFade = useCallback(() => {
    const scrollEl = scrollRef.current;
    if (!scrollEl) return;
    const { scrollTop, scrollHeight, clientHeight } = scrollEl;
    const threshold = 12;
    const canScrollMore = scrollHeight > clientHeight + 2;
    const atBottom = scrollTop + clientHeight >= scrollHeight - threshold;
    setBottomFadeVisible(canScrollMore && !atBottom);
  }, []);

  useEffect(() => {
    const scrollEl = scrollRef.current;
    const contentEl = contentRef.current;
    if (!scrollEl) return;

    updateBottomFade();
    scrollEl.addEventListener('scroll', updateBottomFade, { passive: true });
    window.addEventListener('resize', updateBottomFade);

    let ro: ResizeObserver | null = null;
    if (typeof ResizeObserver !== 'undefined') {
      ro = new ResizeObserver(() => updateBottomFade());
      ro.observe(scrollEl);
      if (contentEl) ro.observe(contentEl);
    }

    return () => {
      scrollEl.removeEventListener('scroll', updateBottomFade);
      window.removeEventListener('resize', updateBottomFade);
      ro?.disconnect();
    };
  }, [updateBottomFade, data, loading]);

  if (loading) {
    return <SurfaceState message="Loading your current state…" layout="centered" />;
  }

  if (error) {
    return <SurfaceState message={error} layout="centered" tone="warning" />;
  }

  if (!data) {
    return (
      <SurfaceState
        message="No current context yet. Sync integrations or run an evaluation."
        layout="centered"
      />
    );
  }

  const actionItems = dedupeActionItems(
    [...(data.action_items ?? [])].filter((item) => item.surface === 'now'),
  );
  const meshSummary = data.mesh_summary;
  const nudgeBars = data.nudge_bars ?? [];
  const taskLane = data.task_lane;
  const nowTs = data.computed_at;
  const commitmentRows = dedupeTasks([data.tasks.next_commitment, ...(data.tasks.other_open ?? [])]);
  const commitmentIds = new Set(commitmentRows.map((t) => t.id));

  const completeCommitment = async (commitmentId: string) => {
    setPendingCommitments((current) => ({ ...current, [commitmentId]: true }));
    setCommitmentMessages((current) => {
      const next = { ...current };
      delete next[commitmentId];
      return next;
    });
    try {
      const response = await updateCommitment(commitmentId, { status: 'done' });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to complete commitment');
      }
      invalidateQuery(nowKey, { refetch: true });
      invalidateQuery(commitmentsKey, { refetch: true });
      await refetch();
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: { status: 'success', message: 'Completed.' },
      }));
    } catch (commitmentError) {
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: {
          status: 'error',
          message: commitmentError instanceof Error ? commitmentError.message : String(commitmentError),
        },
      }));
    } finally {
      setPendingCommitments((current) => {
        const next = { ...current };
        delete next[commitmentId];
        return next;
      });
    }
  };

  const handleNowBarAction = (
    bar: NowData['nudge_bars'][number],
    action: NowData['nudge_bars'][number]['actions'][number],
  ) => {
    if (
      (action.kind === 'open_thread' || action.kind === 'expand' || action.kind === 'accept') &&
      bar.primary_thread_id
    ) {
      onOpenThread?.(bar.primary_thread_id);
      return;
    }
    if (action.kind === 'open_settings') {
      onOpenSystem?.(nudgeOpenSystemTarget(bar));
      return;
    }
  };

  const activeEvent = findActiveEvent(data.schedule.upcoming_events, nowTs);
  const locationLabel = nowLocationLabel(data, activeEvent);
  const contextLabel = data.status_row?.context_label ?? data.context_line?.text ?? null;
  const clientName = data.mesh_summary?.authority_label ?? null;

  const dateTimeStr = new Intl.DateTimeFormat('en-US', {
    timeZone: data.timezone,
    weekday: 'short',
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  }).format(new Date(nowTs * 1000));

  const visibleNudges = nudgeBars.slice(0, 4);
  const riskItems = actionItems
    .filter((item) => ['recovery', 'blocked', 'conflict', 'freshness', 'linking'].includes(item.kind))
    .slice(0, 3);
  const groupedTaskCount =
    (taskLane?.active ? 1 : 0) +
    (taskLane?.pending.length ?? 0) +
    riskItems.length +
    (taskLane?.recent_completed.length ?? 0);
  const allTaskMetadata = [...commitmentRows];
  const completedCount = taskLane?.recent_completed.length ?? 0;
  const remainingCount = (taskLane?.pending.length ?? 0) + riskItems.length + (taskLane?.active ? 1 : 0);
  const backlogCount = taskLane?.overflow_count ?? 0;
  const threadAttentionCount =
    actionItems.filter((item) => item.thread_route !== null).length + (data.reflow_status?.thread_id ? 1 : 0);

  return (
    <div className={surfaceShell.mainColumn}>
      <div ref={scrollRef} className={surfaceShell.scrollColumn}>
        <div ref={contentRef} className={surfaceShell.mainContent}>
          <section className={surfaceShell.sectionStack}>
            <PanelSectionHeaderBand mode="section-header">
              <PanelSectionHeaderLead className="space-y-1.5">
                <h1 className="text-2xl font-semibold tracking-tight text-zinc-100">Now</h1>
                <p className="text-xs text-zinc-400">
                  {dateTimeStr}{activeEvent ? ` · ${activeEvent.title}` : ''}
                </p>
                <p className="max-w-2xl text-xs text-zinc-500">
                  {[clientName, locationLabel, contextLabel ? `CONTEXT: ${contextLabel}` : null]
                    .filter(Boolean)
                    .join(' · ')}
                </p>
              </PanelSectionHeaderLead>
              <PanelSectionHeaderTrail>
                <NowMetricStrip
                  nudgeCount={visibleNudges.length}
                  threadAttentionCount={threadAttentionCount}
                  queuedWriteCount={meshSummary?.queued_write_count ?? 0}
                  meshSummary={meshSummary}
                />
              </PanelSectionHeaderTrail>
            </PanelSectionHeaderBand>

            <NowNudgeStrip
              bars={visibleNudges}
              nowTs={nowTs}
              actionItems={actionItems}
              onBarAction={handleNowBarAction}
            />

            <NowTasksSection
              taskLane={taskLane}
              riskItems={riskItems}
              allTaskMetadata={allTaskMetadata}
              commitmentIds={commitmentIds}
              completedCount={completedCount}
              remainingCount={remainingCount}
              backlogCount={backlogCount}
              groupedTaskCount={groupedTaskCount}
              pendingCommitments={pendingCommitments}
              commitmentMessages={commitmentMessages}
              onCompleteCommitment={completeCommitment}
              onOpenThread={onOpenThread}
            />

            <NowScheduleSection schedule={data.schedule} timezone={data.timezone} />
          </section>
        </div>
        <div
          aria-hidden
          className={`pointer-events-none absolute inset-x-0 bottom-0 z-10 h-20 bg-gradient-to-t from-zinc-950 from-10% via-zinc-950/60 to-transparent transition-opacity duration-200 sm:h-28 ${
            bottomFadeVisible ? 'opacity-100' : 'opacity-0'
          }`}
        />
      </div>
    </div>
  );
}
