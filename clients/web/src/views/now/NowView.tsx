import type { DragEvent, ReactNode } from 'react';
import { useEffect, useMemo, useState } from 'react';
import {
  contextQueryKeys,
  loadNow,
  rescheduleNowTasksToToday,
  updateNowTaskLane,
} from '../../data/context';
import { invalidateQuery, setQueryData, useQuery } from '../../data/query';
import type { NowData } from '../../types';
import { ActionChipButton, FilterDenseTag, ProjectTag } from '../../core/FilterToggleTag';
import { ArchiveIcon, CalendarIcon, CheckCircleIcon, ChevronRightIcon, ClockIcon, InboxIcon, OpenThreadIcon, SparkIcon } from '../../core/Icons';
import { cn } from '../../core/cn';
import { ObjectRowFrame, ObjectRowLayout, ObjectRowTitleMetaBand } from '../../core/ObjectRow';
import { SurfaceState } from '../../core/SurfaceState';
import { surfaceShell, uiFonts } from '../../core/Theme';
import {
  findActiveEvent,
  findNextEvent,
  formatTime,
  formatTimeUntil,
} from './nowModel';
import { NowNudgeStrip } from './components/NowNudgeStrip';

interface NowViewProps {
  onOpenThread?: (conversationId: string) => void;
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

type SectionTasks = Record<TaskSectionKey, TaskDisplay[]>;

type NextUpItem = NowData['next_up_items'][number];

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
  if (event.all_day) {
    return 'All day';
  }
  const start = formatTime(event.start_ts, timezone);
  const end = event.end_ts ? formatTime(event.end_ts, timezone) : null;
  return end ? `${start}–${end}` : start;
}

function moveTaskBetweenSections(
  current: SectionTasks,
  taskId: string,
  target: TaskSectionKey,
): SectionTasks {
  let moved: TaskDisplay | null = null;
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

  if (!moved) {
    return current;
  }

  const insertAtFront = target === 'active';
  next[target] = insertAtFront ? [moved, ...next[target]] : [...next[target], moved];
  return next;
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
  const [sectionTasks, setSectionTasks] = useState<SectionTasks>({
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
        tags: task.tags ?? [],
        project: task.project ?? null,
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
        tags: task.tags ?? [],
        dueLabel: task.due_label ?? null,
        isOverdue: task.is_overdue,
        deadlineLabel: task.deadline_label ?? null,
        deadlinePassed: task.deadline_passed,
        threadId: task.primary_thread_id ?? null,
        project: task.project ?? null,
      };
      }),
      later: laterItems.map((task) => {
        return {
        id: task.id,
        text: task.text,
        title: task.title ?? task.text,
        description: task.description ?? null,
        tags: task.tags ?? [],
        project: task.project ?? null,
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
          tags: item.tags ?? [],
          project: item.project ?? null,
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
          tags: task.tags ?? [],
          project: task.project ?? null,
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
  const nextEvent = data.schedule?.next_event ?? findNextEvent(upcomingEvents, nowTs);
  const stretchGoals = sectionTasks.later;
  const fallbackProgressBaseCount = Math.max(1, sectionTasks.active.length + sectionTasks.next.length + sectionTasks.completed.length);
  const progress = data.progress ?? {
    base_count: fallbackProgressBaseCount,
    completed_count: sectionTasks.completed.length,
    backlog_count: stretchGoals.length,
    completed_ratio: Math.min(1, sectionTasks.completed.length / fallbackProgressBaseCount),
    backlog_ratio: stretchGoals.length > 0 ? Math.min(1, stretchGoals.length / fallbackProgressBaseCount) : 0,
  };
  const inboxCount = sectionTasks.inbox.length;
  const nextUpEvents = upcomingEvents.filter((event) => {
    if (activeEvent) {
      return event !== activeEvent && event.start_ts >= activeEvent.start_ts;
    }
    return true;
  });
  const nextUpItems: NextUpItem[] = [
    ...((data.next_up_items ?? []).length > 0
      ? (data.next_up_items ?? [])
      : [
          ...nextUpEvents.map((event) => ({
            kind: 'event' as const,
            id: `${event.title}-${event.start_ts}`,
            title: event.title,
            meta: eventWindowLabel(event, data.timezone),
            detail: event.location ?? 'Calendar event',
            task: null,
          })),
          ...sectionTasks.next.map((task) => ({
            kind: 'task' as const,
            id: task.id,
            title: task.title,
            meta: null,
            detail: null,
            task: {
              id: task.id,
              task_kind: 'commitment' as const,
              text: task.text,
              title: task.title,
              description: task.description,
              tags: task.tags,
              state: 'next_up',
              lane: 'next_up',
              project: task.project,
              primary_thread_id: task.threadId,
              due_at: null,
              deadline: null,
              due_label: task.dueLabel,
              is_overdue: task.isOverdue,
              deadline_label: task.deadlineLabel,
              deadline_passed: task.deadlinePassed,
            },
          })),
        ]),
  ];
  const overdueNudgeBars: NowData['nudge_bars'] = overdueSidebarNudge ? [overdueSidebarNudge] : [];
  const showInlineOverdueNudge = !hideNudgeLane;

  const updateCommitmentStatus = async (commitmentId: string, status: 'done' | 'active') => {
    const previousSectionTasks = sectionTasks;
    const nextSection = status === 'done' ? 'completed' : 'active';
    setPendingCommitments((current) => ({ ...current, [commitmentId]: true }));
    setCommitmentMessages((current) => {
      const next = { ...current };
      delete next[commitmentId];
      return next;
    });
    setSectionTasks((current) => moveTaskBetweenSections(current, commitmentId, nextSection));
    if (status === 'done') {
      setExpandedSections((current) => ({ ...current, completed: true }));
    }
    try {
      const response = await updateNowTaskLane(
        commitmentId,
        status === 'done' ? 'completed' : 'active',
      );
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to update commitment');
      }
      setQueryData(nowKey, () => response.data ?? null);
      invalidateQuery(commitmentsKey, { refetch: true });
      setCommitmentMessages((current) => ({
        ...current,
        [commitmentId]: { status: 'success', message: status === 'done' ? 'Completed.' : 'Reopened.' },
      }));
    } catch (commitmentError) {
      setSectionTasks(previousSectionTasks);
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
    const previousSectionTasks = sectionTasks;
    setLaneEdited(true);
    setSectionTasks((current) => moveTaskBetweenSections(current, taskId, target));
    void (async () => {
      try {
        const response = await updateNowTaskLane(taskId, laneForSection(target));
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to update task lane');
        }
        setQueryData(nowKey, () => response.data ?? null);
      } catch {
        setSectionTasks(previousSectionTasks);
      } finally {
        setLaneEdited(false);
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
          'flex w-full items-center gap-1 text-[10px] uppercase leading-none tracking-[0.05em] transition hover:text-[var(--vel-color-text)]',
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
            size={11}
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
      invalidateQuery(commitmentsKey, { refetch: true });
    } finally {
      setReschedulingOverdue(false);
    }
  };

  const scrollToSidebarEvent = (eventTitle: string) => {
    const calendar = document.getElementById('sidebar-calendar');
    if (!calendar) return;
    calendar.scrollIntoView({ behavior: 'smooth', block: 'start' });
    const row = calendar.querySelector<HTMLElement>(`tr[data-event-title="${CSS.escape(eventTitle)}"]`);
    if (row) {
      row.style.outline = '1px solid var(--vel-color-accent-border)';
      row.style.outlineOffset = '-1px';
      row.style.borderRadius = '4px';
      window.setTimeout(() => {
        row.style.outline = '';
        row.style.outlineOffset = '';
        row.style.borderRadius = '';
      }, 2200);
    }
  };

  return (
    <div className={surfaceShell.mainColumn}>
      <div className={surfaceShell.flowColumn}>
        <div className={surfaceShell.mainContent}>
          <section className="flex flex-col gap-5">
            <div className="space-y-3">
              <div className="space-y-2">
                <div className="flex flex-col gap-2 sm:flex-row sm:flex-wrap sm:items-start sm:justify-between sm:gap-3">
                  <h1 className="flex items-center gap-2 text-3xl font-semibold tracking-tight text-[var(--vel-color-text)]">
                    <SparkIcon size={20} className="text-[var(--vel-color-accent-soft)]" />
                    <span>Now</span>
                  </h1>
                  <div className="flex flex-wrap items-center gap-2 sm:justify-end">
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
                  <p className={`text-sm text-[var(--vel-color-muted)] ${uiFonts.mono}`}>{formatNowTimestamp(nowTs, data.timezone)}</p>
                  <p className="flex max-w-3xl items-center gap-2 text-sm leading-6 text-[var(--vel-color-muted)]">
                    <CalendarIcon size={14} className="shrink-0 text-[var(--vel-color-dim)]" />
                    {activeEvent ? (
                      <button
                        type="button"
                        onClick={() => scrollToSidebarEvent(activeEvent.title)}
                        className="min-w-0 truncate transition hover:text-[var(--vel-color-text)]"
                      >
                        {activeEvent.title}
                      </button>
                    ) : (
                      <span className="opacity-35">None</span>
                    )}
                    {nextEvent ? (
                      <>
                        <span className="shrink-0 text-[var(--vel-color-dim)]">→</span>
                        <button
                          type="button"
                          onClick={() => scrollToSidebarEvent(nextEvent.title)}
                          className="inline-flex min-w-0 items-center gap-1.5 truncate transition hover:text-[var(--vel-color-text)]"
                        >
                          <CalendarIcon size={12} className="shrink-0" />
                          <span className="truncate">{nextEvent.title}</span>
                          <span className={`shrink-0 text-xs opacity-60 ${uiFonts.mono}`}>
                            {formatTimeUntil(nextEvent.start_ts, nowTs)}
                          </span>
                        </button>
                      </>
                    ) : (
                      null
                    )}
                  </p>
                  <div className="max-w-3xl space-y-1 pt-1">
                    <div className="flex items-center justify-between gap-2 text-[10px] uppercase tracking-[0.14em] text-[var(--vel-color-muted)]">
                      <span>Task completion</span>
                      <span className={uiFonts.mono}>
                        {progress.completed_count}/{progress.base_count}
                        {progress.backlog_count ? ` +${progress.backlog_count}` : ''}
                      </span>
                    </div>
                    <div className="relative h-2 overflow-hidden rounded-full bg-white/6">
                      <div
                        className="absolute inset-y-0 left-0 rounded-full bg-[var(--vel-color-accent)]"
                        style={{ width: `${progress.completed_ratio * 100}%` }}
                      />
                      {progress.backlog_ratio > 0 ? (
                        <div
                          className="absolute inset-y-0 rounded-full bg-[var(--vel-color-offline)]/65"
                          style={{ left: `${progress.completed_ratio * 100}%`, width: `${Math.max(3, progress.backlog_ratio * 100)}%` }}
                        />
                      ) : null}
                    </div>
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
                              meta={item.meta ? <FilterDenseTag tone="muted">{item.meta}</FilterDenseTag> : null}
                            />
                            <p className="text-sm leading-6 text-[var(--vel-color-muted)]">{item.detail ?? 'Calendar event'}</p>
                          </ObjectRowLayout>
                        </ObjectRowFrame>
                      ) : (
                        renderTaskRow({
                          id: item.task?.id ?? item.id,
                          text: item.task?.text ?? item.title,
                          title: item.task?.title ?? item.title,
                          description: item.task?.description ?? null,
                          tags: item.task?.tags ?? [],
                          project: item.task?.project ?? null,
                          dueLabel: item.task?.due_label ?? null,
                          isOverdue: item.task?.is_overdue ?? false,
                          deadlineLabel: item.task?.deadline_label ?? null,
                          deadlinePassed: item.task?.deadline_passed ?? false,
                          threadId: item.task?.primary_thread_id ?? null,
                        }, 'next')
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
