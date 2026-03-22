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
import { NowTasksSection } from './components/NowTasksSection';
import type { SettingsSectionKey } from '../../views/settings';
import { surfaceShell } from '../../core/Theme';
import { formatNavbarDateTime } from '../../shell/Navbar/formatNavbarDateTime';
import {
  dedupeActionItems,
  dedupeTasks,
  findActiveEvent,
  nowLocationLabel,
  nowNavContextSummary,
  nudgeOpenSettingsTarget,
  scoreNudge,
} from './nowModel';

type SettingsIntegrationTarget =
  | 'google'
  | 'todoist'
  | 'activity'
  | 'git'
  | 'messaging'
  | 'notes'
  | 'transcripts';

interface NowViewProps {
  onOpenInbox?: () => void;
  onOpenThread?: (conversationId: string) => void;
  onOpenSettings?: (target: {
    tab: 'general' | 'integrations' | 'runtime';
    integrationId?: SettingsIntegrationTarget;
    section?: SettingsSectionKey;
  }) => void;
}

export function NowView({ onOpenInbox, onOpenThread, onOpenSettings }: NowViewProps) {
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
    [...(data.action_items ?? [])]
      .filter((item) => item.surface === 'now')
      .sort((left, right) => left.rank - right.rank),
  );
  const header = data.header;
  const meshSummary = data.mesh_summary;
  const nudgeBars = data.nudge_bars ?? [];
  const taskLane = data.task_lane;
  const nowTs = data.computed_at;
  const activeEvent = findActiveEvent(data.schedule.upcoming_events, nowTs);
  const commitmentRows = dedupeTasks([data.tasks.next_commitment, ...(data.tasks.other_open ?? [])]);
  const commitmentIds = new Set(commitmentRows.map((t) => t.id));
  const pullableTasks = dedupeTasks(data.tasks.todoist ?? []);

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
      onOpenSettings?.(nudgeOpenSettingsTarget(bar));
      return;
    }
    if (action.kind === 'open_inbox') {
      onOpenInbox?.();
    }
  };

  const locationCaption = nowLocationLabel(data, activeEvent);
  const titleDateTime = formatNavbarDateTime(nowTs, data.timezone);
  const navContextLine = nowNavContextSummary(data);
  const backupNudge =
    data.trust_readiness.backup.level !== 'ok'
      ? {
          id: 'backup_trust_warning',
          kind: 'trust_warning',
          title: 'No trustworthy backup',
          summary: data.trust_readiness.backup.detail,
          urgent: data.trust_readiness.backup.level === 'fail',
          primary_thread_id: null,
          actions: [{ kind: 'open_settings', label: 'Open backups' }],
        }
      : null;
  const prioritizedNudges = [...(backupNudge ? [backupNudge] : []), ...nudgeBars]
    .sort((left, right) => scoreNudge(right) - scoreNudge(left))
    .slice(0, 4);
  const riskItems = actionItems
    .filter((item) => ['recovery', 'blocked', 'conflict', 'freshness', 'linking'].includes(item.kind))
    .slice(0, 3);
  const nextTasks = pullableTasks
    .filter((task) => task.id !== taskLane?.active?.id && !taskLane?.pending.some((item) => item.id === task.id))
    .slice(0, 3)
    .map((task) => ({
      id: task.id,
      task_kind: 'task' as const,
      text: task.text,
      state: 'pending',
      project: task.project,
      primary_thread_id: null,
    }));
  const groupedTaskCount =
    (taskLane?.active ? 1 : 0) + (taskLane?.pending.length ?? 0) + riskItems.length + nextTasks.length;
  const allTaskMetadata = [...commitmentRows, ...pullableTasks];
  const completedCount = taskLane?.recent_completed.length ?? 0;
  const remainingCount =
    (taskLane?.pending.length ?? 0) + riskItems.length + nextTasks.length + (taskLane?.active ? 1 : 0);
  const backlogCount = Math.max(0, pullableTasks.length - ((taskLane?.pending.length ?? 0) + nextTasks.length));
  const threadAttentionCount =
    actionItems.filter((item) => item.thread_route !== null).length + (data.reflow_status?.thread_id ? 1 : 0);

  return (
    <div className={surfaceShell.mainColumn}>
      <div ref={scrollRef} className={surfaceShell.scrollColumn}>
        <div ref={contentRef} className={surfaceShell.mainContent}>
        <section className={surfaceShell.sectionStack}>
          <PanelSectionHeaderBand mode="section-header">
            <PanelSectionHeaderLead className="space-y-2">
              <h1 className="text-2xl font-semibold tracking-tight text-zinc-100">
                {header?.title ?? 'Now'}
              </h1>
              <p className="text-xs text-zinc-500" title={`${titleDateTime} · ${locationCaption}`}>
                <span className="text-zinc-400">{titleDateTime}</span>
                <span className="text-zinc-600/90" aria-hidden>
                  {' '}
                  |{' '}
                </span>
                <span className="text-zinc-400">{locationCaption}</span>
              </p>
              <p className="flex min-w-0 gap-1.5 text-[11px] leading-snug text-zinc-400 sm:text-xs">
                <span className="shrink-0 font-medium uppercase tracking-[0.14em] text-zinc-500">CONTEXT:</span>
                <span className="min-w-0 truncate">{navContextLine}</span>
              </p>
            </PanelSectionHeaderLead>
            <PanelSectionHeaderTrail>
              <NowMetricStrip
                nudgeCount={prioritizedNudges.length}
                threadAttentionCount={threadAttentionCount}
                queuedWriteCount={meshSummary?.queued_write_count ?? 0}
                meshSummary={meshSummary}
              />
            </PanelSectionHeaderTrail>
          </PanelSectionHeaderBand>

          <NowNudgeStrip
            bars={prioritizedNudges}
            nowTs={nowTs}
            actionItems={actionItems}
            onBarAction={handleNowBarAction}
          />

          <NowTasksSection
            taskLane={taskLane}
            riskItems={riskItems}
            nextTasks={nextTasks}
            allTaskMetadata={allTaskMetadata}
            commitmentIds={commitmentIds}
            completedCount={completedCount}
            remainingCount={remainingCount}
            backlogCount={backlogCount}
            groupedTaskCount={groupedTaskCount}
            pendingCommitments={pendingCommitments}
            commitmentMessages={commitmentMessages}
            onCompleteCommitment={completeCommitment}
            onOpenInbox={onOpenInbox}
            onOpenThread={onOpenThread}
          />
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
