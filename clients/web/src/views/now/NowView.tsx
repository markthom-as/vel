import type { DragEvent, ReactNode } from 'react';
import { useEffect, useMemo, useState } from 'react';
import {
  contextQueryKeys,
  loadNow,
  rescheduleNowTasksToToday,
  updateNowTaskLane,
} from '../../data/context';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import type { NowData, NowNudgeBarData } from '../../types';
import { ActionChipButton, FilterDenseTag, ProjectTag } from '../../core/FilterToggleTag';
import { ArchiveIcon, CalendarIcon, CheckCircleIcon, ChevronRightIcon, ClockIcon, InboxIcon, OpenThreadIcon, SparkIcon } from '../../core/Icons';
import { cn } from '../../core/cn';
import { ObjectRowFrame, ObjectRowLayout, ObjectRowTitleMetaBand } from '../../core/ObjectRow';
import { SurfaceState } from '../../core/SurfaceState';
import { surfaceShell, uiFonts } from '../../core/Theme';
import {
  findActiveEvent,
  formatTime,
} from './nowModel';
import type { SystemNavigationTarget } from '../system';
import { NowNudgeStrip } from './components/NowNudgeStrip';

interface NowViewProps {
  onOpenThread?: (conversationId: string) => void;
  onOpenSystem?: (target?: SystemNavigationTarget) => void;
  onRaiseNudge?: (nudge: NowNudgeBarData) => void;
  onClearNudge?: (nudgeId: string) => void;
  hideNudgeLane?: boolean;
}

interface CommitmentMessage {
  status: 'success' | 'error';
  message: string;
}

type TaskSectionKey = 'active' | 'next' | 'inbox' | 'later' | 'completed';
type TaskDisplay = {
  id: string;
  text: string;
  title: string;
  description: string | null;
  tags: string[];
  project: string | null;
  dueLabel: string | null;
  isOverdue: boolean;
  deadlineLabel: string | null;
  deadlinePassed: boolean;
  threadId: string | null;
};

type NextUpItem =
  | {
      kind: 'event';
      id: string;
      title: string;
      meta: string;
      detail: string;
      dueLabel?: never;
      deadlineLabel?: never;
      deadlinePassed?: never;
      isOverdue?: never;
      threadId?: never;
      project?: never;
      text?: never;
      description?: never;
      tags?: never;
    }
  | ({
      kind: 'task';
    } & TaskDisplay);

function laneForSection(section: TaskSectionKey): 'active' | 'next_up' | 'if_time_allows' | 'completed' {
  switch (section) {
    case 'active':
      return 'active';
    case 'next':
    case 'inbox':
      return 'next_up';
    case 'later':
      return 'if_time_allows';
    case 'completed':
      return 'completed';
  }
}

function formatNowTimestamp(timestamp: number, timezone: string): string {
  return new Intl.DateTimeFormat('en-US', {
    timeZone: timezone,
    weekday: 'long',
    month: 'long',
    day: 'numeric',
    year: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
    timeZoneName: 'short',
  }).format(new Date(timestamp * 1000));
}

function eventWindowLabel(
  event: NonNullable<NowData['schedule']['upcoming_events']>[number],
  timezone: string,
): string {
  const start = formatTime(event.start_ts, timezone);
  const end = event.end_ts ? formatTime(event.end_ts, timezone) : null;
  return end ? `${start}–${end}` : start;
}

function normalizeTaskTags(
  tags: string[] | undefined,
  project: string | null | undefined,
): string[] {
  const projectKey = project?.trim().toLowerCase() ?? null;
  return (tags ?? []).filter((tag, index, all) => {
    const normalized = tag.trim().toLowerCase();
    if (projectKey && normalized === projectKey) {
      return false;
    }
    return all.findIndex((item) => item.trim().toLowerCase() === normalized) === index;
  });
}

function normalizeTaskProject(
  project: string | null | undefined,
): string | null {
  const trimmed = project?.trim() ?? null;
  if (!trimmed) {
    return null;
  }
  return trimmed;
}

export function NowView({ onOpenThread, hideNudgeLane = false }: NowViewProps) {
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
  const [commitmentMessages, setCommitmentMessages] = useState<Record<string, CommitmentMessage>>({});
  const [sectionTasks, setSectionTasks] = useState<Record<TaskSectionKey, TaskDisplay[]>>({
    active: [],
    next: [],
    inbox: [],
    later: [],
    completed: [],
  });
  const [draggedTaskId, setDraggedTaskId] = useState<string | null>(null);
  const [laneEdited, setLaneEdited] = useState(false);
  const [reschedulingOverdue, setReschedulingOverdue] = useState(false);
  const [backlogOpen, setBacklogOpen] = useState(false);
  const [expandedSections, setExpandedSections] = useState<Record<TaskSectionKey, boolean>>({
    active: true,
    next: true,
    inbox: false,
    later: false,
    completed: false,
  });
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

  useEffect(() => {
    if (!data) {
      return;
    }
    const activeItems = data.task_lane?.active_items ?? (data.task_lane?.active ? [data.task_lane.active] : []);
    const nextItems = data.task_lane?.next_up ?? data.task_lane?.pending ?? [];
    const inboxItems = data.task_lane?.inbox ?? [];
    const laterItems = data.task_lane?.if_time_allows ?? [];
    const completedItems = data.task_lane?.completed ?? data.task_lane?.recent_completed ?? [];
    if (laneEdited) {
      return;
    }
    setSectionTasks({
      active: activeItems.map((task) => {
        return {
        id: task.id,
        text: task.text,
        title: task.title ?? task.text,
        description: task.description ?? null,
        tags: normalizeTaskTags(task.tags, task.project ?? null),
        project: normalizeTaskProject(task.project ?? null),
        dueLabel: task.due_label ?? null,
        isOverdue: task.is_overdue,
        deadlineLabel: task.deadline_label ?? null,
        deadlinePassed: task.deadline_passed,
        threadId: task.primary_thread_id ?? null,
      };
      }),
      next: nextItems.map((task) => {
        return {
        id: task.id,
        text: task.text,
        title: task.title ?? task.text,
        description: task.description ?? null,
        tags: normalizeTaskTags(task.tags, task.project ?? null),
        dueLabel: task.due_label ?? null,
        isOverdue: task.is_overdue,
        deadlineLabel: task.deadline_label ?? null,
        deadlinePassed: task.deadline_passed,
        threadId: task.primary_thread_id ?? null,
        project: normalizeTaskProject(task.project ?? null),
      };
      }),
      later: laterItems.map((task) => {
        return {
        id: task.id,
        text: task.text,
        title: task.title ?? task.text,
        description: task.description ?? null,
        tags: normalizeTaskTags(task.tags, task.project ?? null),
        project: normalizeTaskProject(task.project ?? null),
        dueLabel: task.due_label ?? null,
        isOverdue: task.is_overdue,
        deadlineLabel: task.deadline_label ?? null,
        deadlinePassed: task.deadline_passed,
        threadId: task.primary_thread_id ?? null,
      };
      }),
      completed: completedItems.map((item) => {
        return {
          id: item.id,
          text: item.text,
          title: item.title ?? item.text,
          description: item.description ?? null,
          tags: normalizeTaskTags(item.tags, item.project ?? null),
          project: normalizeTaskProject(item.project ?? null),
          dueLabel: item.due_label ?? null,
          isOverdue: item.is_overdue,
          deadlineLabel: item.deadline_label ?? null,
          deadlinePassed: item.deadline_passed,
          threadId: item.primary_thread_id ?? null,
        };
      }),
      inbox: inboxItems.map((task) => {
        return {
          id: task.id,
          text: task.text,
          title: task.title ?? task.text,
          description: task.description ?? null,
          tags: normalizeTaskTags(task.tags, task.project ?? null),
          project: normalizeTaskProject(task.project ?? null),
          dueLabel: task.due_label ?? null,
          isOverdue: task.is_overdue,
          deadlineLabel: task.deadline_label ?? null,
          deadlinePassed: task.deadline_passed,
          threadId: task.primary_thread_id ?? null,
        };
      }),
    });
  }, [data, laneEdited]);

  const nowTs = data?.computed_at ?? 0;
  const overdueCount =
    sectionTasks.active.filter((task) => task.isOverdue).length
    + sectionTasks.next.filter((task) => task.isOverdue).length
    + sectionTasks.inbox.filter((task) => task.isOverdue).length
    + sectionTasks.later.filter((task) => task.isOverdue).length;
  const overdueSidebarNudge =
    data?.nudge_bars.find((bar) => bar.id === 'todoist_overdue_backlog') ?? null;

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

  const upcomingEvents = data.schedule?.upcoming_events ?? [];
  const activeEvent = findActiveEvent(upcomingEvents, nowTs);
  const currentEventLabel = activeEvent?.title ?? 'No current event';
  const contextLabel = data.status_row?.context_label ?? data.context_line?.text ?? 'No context';
  const progressBaseCount = Math.max(1, sectionTasks.active.length + sectionTasks.next.length + sectionTasks.completed.length);
  const completedRatio = Math.min(1, sectionTasks.completed.length / progressBaseCount);
  const stretchGoals = sectionTasks.later;
  const overflowRatio = stretchGoals.length > 0 ? Math.min(1, stretchGoals.length / progressBaseCount) : 0;
  const inboxCount = sectionTasks.inbox.length;
  const nextUpEvents = upcomingEvents.filter((event) => {
    if (activeEvent) {
      return event !== activeEvent && event.start_ts >= activeEvent.start_ts;
    }
    return true;
  });
  const nextUpItems: NextUpItem[] = [
    ...nextUpEvents.map((event) => ({
      kind: 'event' as const,
      id: `${event.title}-${event.start_ts}`,
      title: event.title,
      meta: eventWindowLabel(event, data.timezone),
      detail: event.location ?? 'Calendar event',
    })),
    ...sectionTasks.next.map((task) => ({
      kind: 'task' as const,
      id: task.id,
      title: task.title,
      dueLabel: task.dueLabel,
      deadlineLabel: task.deadlineLabel,
      deadlinePassed: task.deadlinePassed,
      isOverdue: task.isOverdue,
      threadId: task.threadId,
      project: task.project,
      text: task.text,
      description: task.description,
      tags: task.tags,
    })),
  ];
  const overdueNudgeBars: NowData['nudge_bars'] = overdueSidebarNudge ? [overdueSidebarNudge] : [];
  const showInlineOverdueNudge = !hideNudgeLane;

  const updateCommitmentStatus = async (commitmentId: string, status: 'done' | 'active') => {
    setPendingCommitments((current) => ({ ...current, [commitmentId]: true }));
    setCommitmentMessages((current) => {
      const next = { ...current };
      delete next[commitmentId];
      return next;
    });
    try {
      const response = await updateNowTaskLane(
        commitmentId,
        status === 'done' ? 'completed' : 'active',
      );
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update commitment');
      }
      setQueryData(nowKey, () => response.data ?? null);
      invalidateQuery(nowKey, { refetch: true });
      invalidateQuery(commitmentsKey, { refetch: true });
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: { status: 'success', message: status === 'done' ? 'Completed.' : 'Reopened.' },
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

  const moveTask = (taskId: string, target: TaskSectionKey) => {
    let moved: TaskDisplay | null = null;
    setLaneEdited(true);
    setSectionTasks((current) => {
      const next = {
        active: current.active.filter((task) => {
          if (task.id === taskId) moved = task;
          return task.id !== taskId;
        }),
        next: current.next.filter((task) => {
          if (task.id === taskId) moved = task;
          return task.id !== taskId;
        }),
        inbox: current.inbox.filter((task) => {
          if (task.id === taskId) moved = task;
          return task.id !== taskId;
        }),
        later: current.later.filter((task) => {
          if (task.id === taskId) moved = task;
          return task.id !== taskId;
        }),
        completed: current.completed.filter((task) => {
          if (task.id === taskId) moved = task;
          return task.id !== taskId;
        }),
      };
      if (moved) next[target] = [...next[target], moved];
      return next;
    });
    void (async () => {
      const response = await updateNowTaskLane(taskId, laneForSection(target));
      if (response.ok) {
        setQueryData(nowKey, () => response.data ?? null);
        invalidateQuery(nowKey, { refetch: true });
      }
    })();
  };

  const renderTaskRow = (task: TaskDisplay, section: TaskSectionKey) => {
    const isActive = section === 'active';
    const isLater = section === 'later';
    const isCompleted = section === 'completed';
    const isInbox = section === 'inbox';
    const metaTags = (
      <>
        {task.dueLabel ? (
          <FilterDenseTag
            tone="muted"
            className={task.isOverdue ? 'border-red-700/35 bg-red-950/18 text-red-200' : undefined}
          >
            {task.dueLabel}
          </FilterDenseTag>
        ) : null}
        {task.deadlineLabel ? (
          <FilterDenseTag
            tone="muted"
            className={task.deadlinePassed ? 'border-red-700/35 bg-red-950/18 text-red-200' : undefined}
          >
            {task.deadlineLabel}
          </FilterDenseTag>
        ) : null}
        {task.project ? (
          <ProjectTag label={task.project}>{task.project}</ProjectTag>
        ) : null}
        {task.tags.map((tag) => (
          <FilterDenseTag key={`${task.id}-${tag}`} tone="muted">
            {tag}
          </FilterDenseTag>
        ))}
      </>
    );

    return (
      <div
        key={task.id}
        draggable={!isCompleted && !isInbox}
        onDragStart={() => setDraggedTaskId(task.id)}
        onDragEnd={() => setDraggedTaskId(null)}
      >
        <ObjectRowFrame
          tone={isActive ? 'activeBrand' : isLater || isCompleted ? 'ghost' : 'neutral'}
          density="standard"
          className={cn(
            'px-4 py-3 transition',
            isActive
              ? 'scale-[1.045] bg-[color:var(--vel-color-panel-2)]/72'
              : '',
            isLater ? 'opacity-75' : '',
            isInbox ? 'border-[color:var(--vel-color-border)]/80 bg-[color:var(--vel-color-panel)]/55' : '',
          )}
        >
          <ObjectRowLayout
            leading={(
              <button
                type="button"
                onClick={() => void updateCommitmentStatus(task.id, isCompleted ? 'active' : 'done')}
                disabled={Boolean(pendingCommitments[task.id])}
                aria-label={`${isCompleted ? 'Reopen' : 'Complete'} ${task.text}`}
                className={cn(
                  'inline-flex h-10 w-10 items-center justify-center rounded-[0.7rem] border transition disabled:opacity-50',
                  isCompleted
                    ? 'border-[var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)] text-[var(--vel-color-accent-soft)]'
                    : 'border-[var(--vel-color-border)] bg-transparent text-transparent hover:border-[var(--vel-color-accent-border)]',
                )}
              >
                {isCompleted ? <CheckCircleIcon size={18} /> : <span className="h-4 w-4 rounded-[0.2rem] border border-current" />}
              </button>
            )}
            actionsLayout="inline"
            actions={(
              <div className="flex flex-wrap items-center justify-end gap-1.5">
                {task.threadId ? (
                  <ActionChipButton onClick={() => onOpenThread?.(task.threadId!)}>
                    <OpenThreadIcon size={15} className="shrink-0" aria-hidden />
                    <span>Open</span>
                  </ActionChipButton>
                ) : null}
              </div>
            )}
          >
            <div className="flex min-w-0 flex-col gap-1">
              <ObjectRowTitleMetaBand
                title={
                  <span className={cn('inline-flex items-center gap-2 text-[15px] font-medium', isCompleted ? 'text-[var(--vel-color-muted)] line-through' : 'text-[var(--vel-color-text)]')}>
                    <span className="text-[14px]">{task.title}</span>
                  </span>
                }
                meta={
                  <div className="flex min-w-0 flex-1 flex-wrap items-center justify-end gap-1.5">
                    {metaTags}
                  </div>
                }
              />
              {task.description ? (
                <p className="text-sm leading-5 text-[var(--vel-color-muted)]">{task.description}</p>
              ) : null}
              {commitmentMessages[task.id] ? (
                <p className={cn('text-xs', commitmentMessages[task.id].status === 'error' ? 'text-[var(--vel-color-error)]' : 'text-[var(--vel-color-done)]')}>
                  {commitmentMessages[task.id].message}
                </p>
              ) : null}
            </div>
          </ObjectRowLayout>
        </ObjectRowFrame>
      </div>
    );
  };

  const dropZoneProps = (section: TaskSectionKey) => ({
    onDragOver: (event: DragEvent) => event.preventDefault(),
    onDrop: (event: DragEvent) => {
      event.preventDefault();
      if (draggedTaskId) moveTask(draggedTaskId, section);
    },
  });

  const toggleSection = (section: TaskSectionKey) => {
    if (section === 'later') {
      setBacklogOpen((current) => {
        const next = !current;
        setExpandedSections((state) => ({ ...state, later: next }));
        return next;
      });
      return;
    }
    setExpandedSections((current) => ({ ...current, [section]: !current[section] }));
  };

  const renderSectionTrigger = (
    section: TaskSectionKey,
    options: {
      icon: ReactNode;
      label: string;
      toneClass?: string;
      controlsId: string;
    },
  ) => {
    const expanded = section === 'later' ? backlogOpen : expandedSections[section];
    return (
      <button
        type="button"
        onClick={() => toggleSection(section)}
        className={cn(
          uiFonts.display,
          'flex w-full items-center gap-1 text-[5px] uppercase leading-none tracking-[0.05em] transition hover:text-[var(--vel-color-text)]',
          options.toneClass ?? 'text-[var(--vel-color-muted)]',
        )}
        aria-expanded={expanded}
        aria-controls={options.controlsId}
      >
        <span className="inline-flex min-w-0 items-center gap-1">
          {options.icon}
          <span>{options.label}</span>
        </span>
        <span className="ml-auto inline-flex min-w-0 flex-1 items-center justify-end gap-1.5">
          <span className="h-px min-w-4 flex-1 bg-[var(--vel-color-border)]/80" aria-hidden />
          <ChevronRightIcon
            size={9}
            className={cn('shrink-0 transition-transform', expanded ? 'rotate-90' : '')}
          />
        </span>
      </button>
    );
  };

  const handleOverdueNudgeAction = async (
    bar: typeof overdueNudgeBars[number],
    action: typeof overdueNudgeBars[number]['actions'][number],
  ) => {
    if (action.kind.startsWith('jump_backlog')) {
      setBacklogOpen(true);
      window.setTimeout(() => {
        document.getElementById('now-backlog')?.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }, 0);
      return;
    }
    if (action.kind === 'open_thread' && bar.primary_thread_id) {
      onOpenThread?.(bar.primary_thread_id);
      return;
    }
    if (!action.kind.startsWith('reschedule_today') || reschedulingOverdue) {
      return;
    }
    const [, encodedIds = ''] = action.kind.split(':', 2);
    const commitmentIds = encodedIds
      .split(',')
      .map((id) => id.trim())
      .filter(Boolean);
    if (commitmentIds.length === 0) {
      return;
    }

    setReschedulingOverdue(true);
    try {
      const response = await rescheduleNowTasksToToday(commitmentIds);
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to reschedule overdue items');
      }
      setQueryData(nowKey, () => response.data ?? null);
      invalidateQuery(nowKey, { refetch: true });
      invalidateQuery(commitmentsKey, { refetch: true });
    } finally {
      setReschedulingOverdue(false);
    }
  };

  return (
    <div className={surfaceShell.mainColumn}>
      <div className={surfaceShell.flowColumn}>
        <div className={surfaceShell.mainContent}>
          <section className="flex flex-col gap-5">
            <div className="space-y-3">
              <div className="flex flex-wrap items-start justify-between gap-3">
                <div className="space-y-2">
                  <h1 className="flex items-center gap-2 text-3xl font-semibold tracking-tight text-[var(--vel-color-text)]">
                    <SparkIcon size={20} className="text-[var(--vel-color-accent-soft)]" />
                    <span>Now</span>
                  </h1>
                  <p className={`text-sm text-[var(--vel-color-muted)] ${uiFonts.mono}`}>{formatNowTimestamp(nowTs, data.timezone)}</p>
                  <p className="flex max-w-3xl items-center gap-2 text-sm leading-6 text-[var(--vel-color-muted)]">
                    <CalendarIcon size={14} className="shrink-0 text-[var(--vel-color-dim)]" />
                    <span className="truncate">{currentEventLabel} | {contextLabel}</span>
                  </p>
                  <div className="max-w-3xl space-y-1 pt-1">
                    <div className="flex items-center justify-between gap-2 text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
                      <span>Task completion</span>
                      <span className={uiFonts.mono}>
                        {sectionTasks.completed.length}/{progressBaseCount}
                        {sectionTasks.later.length ? ` +${sectionTasks.later.length}` : ''}
                      </span>
                    </div>
                    <div className="relative h-2 overflow-hidden rounded-full bg-white/6">
                      <div
                        className="absolute inset-y-0 left-0 rounded-full bg-[var(--vel-color-accent)]"
                        style={{ width: `${completedRatio * 100}%` }}
                      />
                      {overflowRatio > 0 ? (
                        <div
                          className="absolute inset-y-0 rounded-full bg-[var(--vel-color-offline)]/65"
                          style={{ left: `${completedRatio * 100}%`, width: `${Math.max(3, overflowRatio * 100)}%` }}
                        />
                      ) : null}
                    </div>
                  </div>
                </div>
                <div className="flex flex-wrap items-center justify-end gap-2">
                  {sectionTasks.completed.length ? (
                    <FilterDenseTag tone="muted" className="border-emerald-700/35 bg-emerald-950/18 text-emerald-200">
                      <CheckCircleIcon size={11} />
                      <span>{sectionTasks.completed.length} COMPLETED</span>
                    </FilterDenseTag>
                  ) : null}
                  <FilterDenseTag tone="brand" className="border-[color:var(--vel-color-accent-border)] bg-[color:var(--vel-color-panel-2)]/78 text-[var(--vel-color-accent-soft)]">
                    <SparkIcon size={11} />
                    <span>{sectionTasks.active.length} ACTIVE</span>
                  </FilterDenseTag>
                  <FilterDenseTag tone="muted" className="border-sky-700/35 bg-sky-950/20 text-sky-200">
                    <CalendarIcon size={11} />
                    <span>{nextUpItems.length} NEXT</span>
                  </FilterDenseTag>
                  <FilterDenseTag tone="muted" className="border-amber-700/35 bg-amber-950/18 text-amber-200">
                    <InboxIcon size={11} />
                    <span>{inboxCount} INBOX</span>
                  </FilterDenseTag>
                  {stretchGoals.length ? (
                    <FilterDenseTag tone="muted" className="border-stone-700/35 bg-stone-950/18 text-stone-200">
                      <ArchiveIcon size={11} />
                      <span>{stretchGoals.length} BACKLOG</span>
                    </FilterDenseTag>
                  ) : null}
                  {overdueCount ? (
                    <FilterDenseTag tone="muted" className="border-red-700/35 bg-red-950/18 text-red-200">
                      <ClockIcon size={11} />
                      <span>{overdueCount} OVERDUE</span>
                    </FilterDenseTag>
                  ) : null}
                </div>
              </div>
            </div>

            {showInlineOverdueNudge ? (
              <NowNudgeStrip
                bars={overdueNudgeBars}
                nowTs={nowTs}
                actionItems={data.action_items ?? []}
                onBarAction={(bar, action) => {
                  void handleOverdueNudgeAction(bar, action);
                }}
              />
            ) : null}

            {sectionTasks.completed.length ? (
              <section id="now-completed" className="space-y-2 pt-4" {...dropZoneProps('completed')}>
                {renderSectionTrigger('completed', {
                  icon: <CheckCircleIcon size={11} />,
                  label: `COMPLETED (${sectionTasks.completed.length})`,
                  controlsId: 'now-completed-drawer',
                })}
                {expandedSections.completed ? (
                  <div id="now-completed-drawer" className="space-y-2 pt-2">
                    {sectionTasks.completed.map((task) => renderTaskRow(task, 'completed'))}
                  </div>
                ) : null}
              </section>
            ) : null}

            <section id="now-active" className="space-y-2 pt-4" {...dropZoneProps('active')}>
              {renderSectionTrigger('active', {
                icon: <SparkIcon size={11} />,
                label: sectionTasks.active.length > 1 ? `ACTIVE TASKS (${sectionTasks.active.length})` : `ACTIVE TASK (${sectionTasks.active.length})`,
                toneClass: 'text-[var(--vel-color-accent-soft)]',
                controlsId: 'now-active-drawer',
              })}
              {expandedSections.active ? (
                sectionTasks.active.length > 0 ? (
                  <div id="now-active-drawer" className="space-y-2">
                    {sectionTasks.active.map((task) => renderTaskRow(task, 'active'))}
                  </div>
                ) : (
                  <p id="now-active-drawer" className={`text-sm uppercase tracking-[0.14em] text-[var(--vel-color-muted)] ${uiFonts.display}`}>NONE</p>
                )
              ) : null}
            </section>

            <section id="now-next-up" className="space-y-3 pt-3" {...dropZoneProps('next')}>
              {renderSectionTrigger('next', {
                icon: <CalendarIcon size={11} />,
                label: `NEXT UP (${nextUpItems.length})`,
                controlsId: 'now-next-up-drawer',
              })}
              {expandedSections.next ? (
                nextUpItems.length > 0 ? (
                  <div id="now-next-up-drawer" className="space-y-2">
                    {nextUpItems.map((item) => (
                      item.kind === 'event' ? (
                        <ObjectRowFrame key={item.id} tone="neutral" density="standard" className="px-4 py-3">
                          <ObjectRowLayout
                            leading={(
                              <span className="inline-flex h-10 w-10 items-center justify-center rounded-full border border-[var(--vel-color-border)] bg-[color:var(--vel-color-panel)] text-[var(--vel-color-offline)]">
                                <CalendarIcon size={16} />
                              </span>
                            )}
                          >
                            <ObjectRowTitleMetaBand
                              title={<h3 className="text-base font-medium text-[var(--vel-color-text)]">{item.title}</h3>}
                              meta={<FilterDenseTag tone="muted">{item.meta}</FilterDenseTag>}
                            />
                            <p className="text-sm leading-6 text-[var(--vel-color-muted)]">{item.detail}</p>
                          </ObjectRowLayout>
                        </ObjectRowFrame>
                      ) : (
                        renderTaskRow(item, 'next')
                      )
                    ))}
                  </div>
                ) : (
                  <p id="now-next-up-drawer" className="text-sm text-[var(--vel-color-muted)]">No next item is staged right now.</p>
                )
              ) : null}
            </section>

            <section id="now-inbox" className="space-y-2 pt-3">
              {renderSectionTrigger('inbox', {
                icon: <InboxIcon size={11} />,
                label: `INBOX (${sectionTasks.inbox.length})`,
                controlsId: 'now-inbox-drawer',
              })}
              {expandedSections.inbox ? (
                sectionTasks.inbox.length > 0 ? (
                  <div id="now-inbox-drawer" className="space-y-2">
                    {sectionTasks.inbox.map((task) => renderTaskRow(task, 'inbox'))}
                  </div>
                ) : (
                  <p id="now-inbox-drawer" className="text-sm text-[var(--vel-color-muted)]">No Inbox tasks are waiting right now.</p>
                )
              ) : null}
            </section>

            {stretchGoals.length > 0 ? (
              <section id="now-backlog" className="space-y-2" {...dropZoneProps('later')}>
                {renderSectionTrigger('later', {
                  icon: <ClockIcon size={11} />,
                  label: `BACKLOG (${stretchGoals.length})`,
                  controlsId: 'now-backlog-drawer',
                })}
                {backlogOpen ? (
                  <div id="now-backlog-drawer" className="space-y-2">
                    {stretchGoals.length > 0 ? (
                      <p className="text-sm leading-6 text-[var(--vel-color-muted)]">
                        Tasks outside today&apos;s due context stay here until you pull them into the day.
                      </p>
                    ) : null}
                    <div className="space-y-2">
                      {stretchGoals.map((task) => renderTaskRow(task, 'later'))}
                    </div>
                  </div>
                ) : null}
              </section>
            ) : null}

          </section>
        </div>
      </div>
    </div>
  );
}
