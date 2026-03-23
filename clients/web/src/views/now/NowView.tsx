import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { contextQueryKeys, loadNow, updateCommitment } from '../../data/context';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import type { ActionItemData, NowData, NowTaskData } from '../../types';
import {
  PanelEmptyRow,
  PanelEyebrow,
  PanelInsetCard,
  PanelPageSection,
  PanelSectionHeader,
  PanelSectionHeaderBand,
  PanelSectionHeaderLead,
  PanelSectionHeaderTrail,
} from '../../core/PanelChrome';
import { FilterDenseTag, FilterPillButton } from '../../core/FilterToggleTag';
import { OpenThreadIcon } from '../../core/Icons';
import { SurfaceState } from '../../core/SurfaceState';
import { NowMetricStrip } from './components/NowMetricStrip';
import { NowNudgeStrip } from './components/NowNudgeStrip';
import { NowScheduleSection } from './components/NowScheduleSection';
import { CompactTaskLaneRow } from './components/CompactTaskLaneRow';
import { ActionRow } from './components/ActionRow';
import { surfaceShell } from '../../core/Theme';
import {
  buildCurrentStatus,
  dedupeActionItems,
  dedupeTasks,
  findActiveEvent,
  findActiveRoutineBlock,
  findNextEvent,
  formatTaskDate,
  nowLocationLabel,
  nudgeOpenSystemTarget,
  projectTagClasses,
} from './nowModel';
import type { SystemNavigationTarget } from '../system';

interface NowViewProps {
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
}

interface CommitmentMessage {
  status: 'success' | 'error';
  message: string;
}

interface CommitmentBuckets {
  overdue: NowTaskData[];
  dueSoon: NowTaskData[];
  queue: NowTaskData[];
}

function reconcileCompletedNowData(current: NowData, commitmentId: string): NowData {
  const active = current.task_lane?.active;
  const pending = current.task_lane?.pending ?? [];
  const recentCompleted = current.task_lane?.recent_completed ?? [];
  const completedEntry =
    (active && active.id === commitmentId
      ? {
          ...active,
          state: 'done',
        }
      : pending.find((item) => item.id === commitmentId)
        ? {
            ...pending.find((item) => item.id === commitmentId)!,
            state: 'done',
          }
        : null);

  return {
    ...current,
    task_lane: {
      active: active?.id === commitmentId ? null : (active ?? null),
      pending: pending.filter((item) => item.id !== commitmentId),
      recent_completed: completedEntry
        ? [
            completedEntry,
            ...recentCompleted.filter((item) => item.id !== commitmentId),
          ]
        : recentCompleted,
      overflow_count: current.task_lane?.overflow_count ?? 0,
    },
    tasks: {
      ...current.tasks,
      next_commitment: current.tasks.next_commitment?.id === commitmentId ? null : current.tasks.next_commitment,
      other_open: (current.tasks.other_open ?? []).filter((task) => task.id !== commitmentId),
    },
  };
}

function bucketCommitments(
  tasks: NowTaskData[],
  nowTs: number,
): CommitmentBuckets {
  const now = new Date(nowTs * 1000).getTime();
  const soonCutoff = now + 24 * 60 * 60 * 1000;
  const overdue: NowTaskData[] = [];
  const dueSoon: NowTaskData[] = [];
  const queue: NowTaskData[] = [];

  for (const task of tasks) {
    if (!task.due_at) {
      queue.push(task);
      continue;
    }

    const dueTime = Date.parse(task.due_at);
    if (Number.isNaN(dueTime)) {
      queue.push(task);
      continue;
    }

    if (dueTime < now) {
      overdue.push(task);
      continue;
    }

    if (dueTime <= soonCutoff) {
      dueSoon.push(task);
      continue;
    }

    queue.push(task);
  }

  return { overdue, dueSoon, queue };
}

function dueLabel(task: NowTaskData): string {
  return task.due_at ? `Due ${formatTaskDate(task.due_at)}` : 'No due date';
}

function triageThreadCount(items: ActionItemData[], reflowThreadId: string | null | undefined): number {
  return items.filter((item) => item.thread_route !== null).length + (reflowThreadId ? 1 : 0);
}

function taskMetadataFromLaneItem(
  laneItem: NonNullable<NowData['task_lane']>['active'] | null | undefined,
  fallback: NowTaskData | null,
): NowTaskData | null {
  if (fallback) {
    return fallback;
  }
  if (!laneItem) {
    return null;
  }
  return {
    id: laneItem.id,
    text: laneItem.text,
    source_type: laneItem.task_kind === 'commitment' ? 'local' : laneItem.task_kind,
    due_at: null,
    project: laneItem.project ?? null,
    commitment_kind: laneItem.task_kind === 'commitment' ? 'commitment' : null,
  };
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
    Record<string, CommitmentMessage>
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
      setQueryData(nowKey, (current: NowData | null | undefined) =>
        current ? reconcileCompletedNowData(current, commitmentId) : current,
      );
      invalidateQuery(nowKey, { refetch: true });
      invalidateQuery(commitmentsKey, { refetch: true });
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
  const nextEvent = findNextEvent(data.schedule.upcoming_events, nowTs);
  const activeRoutineBlock = findActiveRoutineBlock(data.day_plan, nowTs);
  const locationLabel = nowLocationLabel(data, activeEvent);
  const contextLabel = data.status_row?.context_label ?? null;
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
  const completedCount = taskLane?.recent_completed.length ?? 0;
  const backlogCount = taskLane?.overflow_count ?? 0;
  const threadAttentionCount = triageThreadCount(actionItems, data.reflow_status?.thread_id);
  const focusMetadata = taskMetadataFromLaneItem(
    taskLane?.active ?? null,
    (taskLane?.active ? commitmentRows.find((task) => task.id === taskLane.active?.id) : null) ?? null,
  );
  const focusStatus = buildCurrentStatus(data, activeEvent, activeRoutineBlock, focusMetadata, nextEvent);
  const nextUpItem =
    (taskLane?.active && focusStatus.kind !== 'Commitment' ? taskLane.active : null)
    ?? taskLane?.pending[0]
    ?? null;
  const nonFocusCommitments = commitmentRows.filter((task) => task.id !== focusMetadata?.id);
  const commitmentBuckets = bucketCommitments(nonFocusCommitments, nowTs);
  const triageCount = visibleNudges.length + riskItems.length;

  return (
    <div className={surfaceShell.mainColumn}>
      <div ref={scrollRef} className={surfaceShell.scrollColumn}>
        <div ref={contentRef} className={surfaceShell.mainContent}>
          <section className="space-y-6">
            <PanelSectionHeaderBand mode="section-header">
              <PanelSectionHeaderLead className="space-y-2">
                <PanelEyebrow tracking="wide">Now</PanelEyebrow>
                <h1 className="text-3xl font-semibold tracking-tight text-zinc-100">Now</h1>
                <p className="text-xs text-zinc-400">
                  {dateTimeStr}{activeEvent ? ` · ${activeEvent.title}` : ''}
                </p>
                <p className="max-w-2xl text-sm leading-6 text-zinc-400">
                  {[clientName, locationLabel, contextLabel].filter(Boolean).join(' · ')}
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

            <PanelPageSection className="border-[#ff6b00]/35 bg-[linear-gradient(135deg,rgba(255,107,0,0.10),rgba(24,24,27,0.98)_38%,rgba(24,24,27,0.98))] shadow-[0_24px_70px_rgba(255,107,0,0.10)]">
              <div className="space-y-5">
                <div className="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
                  <div className="space-y-2">
                    <PanelEyebrow tracking="wide" className="text-[#ffb27a]">Focus</PanelEyebrow>
                    <div className="flex flex-wrap items-center gap-2">
                      <h2 className="text-[28px] font-semibold leading-none tracking-tight text-zinc-50">
                        {focusStatus.title}
                      </h2>
                      <FilterDenseTag className="border-[#ff6b00]/35 bg-[#2d1608]/90 text-[#ffd4b8]">
                        {focusStatus.kind}
                      </FilterDenseTag>
                      {focusStatus.detail ? (
                        <FilterDenseTag className="border-zinc-700 bg-zinc-950/70 text-zinc-300">
                          {focusStatus.detail}
                        </FilterDenseTag>
                      ) : null}
                    </div>
                    <p className="max-w-3xl text-sm leading-6 text-zinc-300">{focusStatus.subtitle}</p>
                  </div>
                  <div className="flex flex-wrap items-center gap-2">
                    {taskLane?.active?.primary_thread_id ? (
                      <FilterPillButton onClick={() => onOpenThread?.(taskLane.active!.primary_thread_id!)}>
                        <OpenThreadIcon size={16} className="shrink-0" aria-hidden />
                        <span>Open thread</span>
                      </FilterPillButton>
                    ) : null}
                    {taskLane?.active && commitmentIds.has(taskLane.active.id) ? (
                      <FilterPillButton
                        onClick={() => void completeCommitment(taskLane.active!.id)}
                        disabled={Boolean(pendingCommitments[taskLane.active.id])}
                        aria-label="Complete commitment"
                      >
                        <span>{pendingCommitments[taskLane.active.id] ? 'Completing…' : 'Complete commitment'}</span>
                      </FilterPillButton>
                    ) : null}
                  </div>
                </div>

                <PanelInsetCard className="space-y-3 border-[#ff6b00]/20 bg-[#120d0a]/90">
                  <div className="flex flex-wrap items-center gap-2 text-xs uppercase tracking-[0.18em] text-zinc-500">
                    <span>Execution locus</span>
                    {focusMetadata?.project ? (
                      <FilterDenseTag className={projectTagClasses(focusMetadata.project)}>
                        {focusMetadata.project}
                      </FilterDenseTag>
                    ) : null}
                    {focusMetadata?.due_at ? (
                      <FilterDenseTag className="border-zinc-700 bg-zinc-900/90 text-zinc-400">
                        {dueLabel(focusMetadata)}
                      </FilterDenseTag>
                    ) : null}
                  </div>
                  <p className="text-base leading-7 text-zinc-200">{focusStatus.summary}</p>
                  {nextUpItem ? (
                    <div className="rounded-2xl border border-zinc-800/90 bg-zinc-950/70 px-4 py-3">
                      <p className="text-[10px] uppercase tracking-[0.24em] text-zinc-500">Next up</p>
                      <div className="mt-2 flex items-start justify-between gap-3">
                        <div className="min-w-0">
                          <p className="truncate text-sm font-medium text-zinc-200">{nextUpItem.text}</p>
                          <p className="mt-1 text-xs text-zinc-500">
                            {nextUpItem.project ?? 'No project'}
                            {commitmentRows.find((task) => task.id === nextUpItem.id)?.due_at
                              ? ` · ${dueLabel(commitmentRows.find((task) => task.id === nextUpItem.id) as NowTaskData)}`
                              : ''}
                          </p>
                        </div>
                        {nextUpItem.primary_thread_id ? (
                          <FilterPillButton onClick={() => onOpenThread?.(nextUpItem.primary_thread_id!)}>
                            <OpenThreadIcon size={16} className="shrink-0" aria-hidden />
                            <span>Open thread</span>
                          </FilterPillButton>
                        ) : null}
                      </div>
                    </div>
                  ) : null}
                </PanelInsetCard>
              </div>
            </PanelPageSection>

            <div className="grid gap-6 xl:grid-cols-[minmax(0,1.35fr)_minmax(18rem,0.9fr)]">
              <PanelPageSection className="space-y-5">
                <PanelSectionHeader
                  title="Commitments"
                  description="Canonical work items competing for the remainder of today. The client groups by explicit due data only and never invents priority."
                />

                {commitmentBuckets.overdue.length > 0 ? (
                  <section className="space-y-2">
                    <p className="text-xs uppercase tracking-[0.18em] text-rose-300">Overdue</p>
                    <div className="space-y-2">
                      {commitmentBuckets.overdue.map((task) => (
                        (() => {
                          const laneItem = taskLane?.pending.find((item) => item.id === task.id);
                          if (!laneItem) return null;
                          return (
                            <CompactTaskLaneRow
                              key={task.id}
                              item={laneItem}
                              flat
                              metadata={task}
                              pending={Boolean(pendingCommitments[task.id])}
                              feedback={commitmentMessages[task.id]}
                              onOpenThread={laneItem.primary_thread_id ? () => onOpenThread?.(laneItem.primary_thread_id!) : undefined}
                              onComplete={() => void completeCommitment(task.id)}
                            />
                          );
                        })()
                      ))}
                    </div>
                  </section>
                ) : null}

                {commitmentBuckets.dueSoon.length > 0 ? (
                  <section className="space-y-2">
                    <p className="text-xs uppercase tracking-[0.18em] text-amber-200">Due soon</p>
                    <div className="space-y-2">
                      {commitmentBuckets.dueSoon.map((task) => {
                        const laneItem = taskLane?.pending.find((item) => item.id === task.id);
                        if (!laneItem) return null;
                        return (
                          <CompactTaskLaneRow
                            key={task.id}
                            item={laneItem}
                            flat
                            metadata={task}
                            pending={Boolean(pendingCommitments[task.id])}
                            feedback={commitmentMessages[task.id]}
                            onOpenThread={laneItem.primary_thread_id ? () => onOpenThread?.(laneItem.primary_thread_id!) : undefined}
                            onComplete={() => void completeCommitment(task.id)}
                          />
                        );
                      })}
                    </div>
                  </section>
                ) : null}

                {commitmentBuckets.queue.length > 0 ? (
                  <section className="space-y-2">
                    <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                      {commitmentBuckets.overdue.length > 0 || commitmentBuckets.dueSoon.length > 0 ? 'Queue' : 'Commitments'}
                    </p>
                    <div className="space-y-2">
                      {commitmentBuckets.queue.map((task) => {
                        const laneItem = taskLane?.pending.find((item) => item.id === task.id);
                        if (!laneItem) return null;
                        return (
                          <CompactTaskLaneRow
                            key={task.id}
                            item={laneItem}
                            flat
                            metadata={task}
                            pending={Boolean(pendingCommitments[task.id])}
                            feedback={commitmentMessages[task.id]}
                            onOpenThread={laneItem.primary_thread_id ? () => onOpenThread?.(laneItem.primary_thread_id!) : undefined}
                            onComplete={() => void completeCommitment(task.id)}
                          />
                        );
                      })}
                    </div>
                  </section>
                ) : null}

                {completedCount > 0 ? (
                  <section className="space-y-2">
                    <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Recently completed</p>
                    <div className="space-y-2">
                      {taskLane?.recent_completed.map((item) => (
                        <CompactTaskLaneRow
                          key={item.id}
                          item={item}
                          flat
                          emphasis="completed"
                          metadata={null}
                          onOpenThread={item.primary_thread_id ? () => onOpenThread?.(item.primary_thread_id!) : undefined}
                        />
                      ))}
                    </div>
                  </section>
                ) : null}

                {commitmentBuckets.overdue.length === 0
                && commitmentBuckets.dueSoon.length === 0
                && commitmentBuckets.queue.length === 0
                && completedCount === 0 ? (
                  <PanelEmptyRow>No commitments are surfaced for the remainder of today.</PanelEmptyRow>
                ) : null}
              </PanelPageSection>

              <div className="space-y-6">
                <PanelPageSection>
                  <NowScheduleSection schedule={data.schedule} timezone={data.timezone} />
                </PanelPageSection>

                <PanelPageSection className="space-y-5">
                  <PanelSectionHeader
                    title="Triage"
                    description="Everything else competing for operator attention. Keep it visible, compact, and subordinate to Focus."
                  />

                  {visibleNudges.length > 0 ? (
                    <NowNudgeStrip
                      bars={visibleNudges}
                      nowTs={nowTs}
                      actionItems={actionItems}
                      onBarAction={handleNowBarAction}
                    />
                  ) : null}

                  {riskItems.length > 0 ? (
                    <div className="space-y-2">
                      {riskItems.map((item) => (
                        <ActionRow key={item.id} item={item} onOpenThread={onOpenThread} />
                      ))}
                    </div>
                  ) : null}

                  {triageCount === 0 ? (
                    <PanelEmptyRow>No triage pressure is active right now.</PanelEmptyRow>
                  ) : null}

                  <div className="flex flex-wrap items-center gap-2 text-xs text-zinc-500">
                    <span>{backlogCount} backlog</span>
                    <span>·</span>
                    <span>{threadAttentionCount} thread edges</span>
                  </div>
                </PanelPageSection>
              </div>
            </div>
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
