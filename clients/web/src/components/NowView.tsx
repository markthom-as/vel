import React, { Children, useEffect, useMemo, useState, type ReactNode } from 'react';
import {
  contextQueryKeys,
  loadActiveDailyLoopSession,
  updateCommitment,
  loadNow,
  startDailyLoopSession,
  submitDailyLoopTurn,
} from '../data/context';
import { operatorQueryKeys, runEvaluate, syncSource } from '../data/operator';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import type {
  AssistantEntryResponse,
  ActionItemData,
  DailyLoopPhaseData,
  DailyLoopSessionData,
  NowData,
  NowTaskData,
  ReviewSnapshotData,
} from '../types';
import { chatQueryKeys } from '../data/chat';
import { MessageComposer } from './MessageComposer';
import { SurfaceState } from './SurfaceState';

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
  onOpenSettings?: (target: { tab: 'integrations'; integrationId: SettingsIntegrationTarget }) => void;
}

export function NowView({ onOpenInbox, onOpenThread, onOpenSettings }: NowViewProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const currentContextKey = useMemo(() => contextQueryKeys.currentContext(), []);
  const commitmentsKey = useMemo(() => contextQueryKeys.commitments(25), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const inboxKey = useMemo(() => chatQueryKeys.inbox(), []);
  const conversationsKey = useMemo(() => chatQueryKeys.conversations(), []);
  const { data, loading, error, refetch } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const sessionDate = useMemo(
    () => (data ? formatSessionDate(data.computed_at, data.timezone) : null),
    [data],
  );
  const morningDailyLoopKey = useMemo(
    () => contextQueryKeys.dailyLoopActive(sessionDate ?? 'pending', 'morning_overview'),
    [sessionDate],
  );
  const standupDailyLoopKey = useMemo(
    () => contextQueryKeys.dailyLoopActive(sessionDate ?? 'pending', 'standup'),
    [sessionDate],
  );
  const { data: morningDailyLoop } = useQuery<DailyLoopSessionData | null>(
    morningDailyLoopKey,
    async () => {
      if (!sessionDate) {
        return null;
      }
      const response = await loadActiveDailyLoopSession(sessionDate, 'morning_overview');
      return response.ok ? response.data ?? null : null;
    },
    { enabled: Boolean(sessionDate) },
  );
  const { data: standupDailyLoop } = useQuery<DailyLoopSessionData | null>(
    standupDailyLoopKey,
    async () => {
      if (!sessionDate) {
        return null;
      }
      const response = await loadActiveDailyLoopSession(sessionDate, 'standup');
      return response.ok ? response.data ?? null : null;
    },
    { enabled: Boolean(sessionDate) },
  );
  const [pendingActions, setPendingActions] = useState<Record<string, true>>({});
  const [actionMessages, setActionMessages] = useState<Record<string, { status: 'success' | 'error'; message: string }>>({});
  const [dailyLoopPending, setDailyLoopPending] = useState(false);
  const [dailyLoopResponse, setDailyLoopResponse] = useState('');
  const [dailyLoopMessage, setDailyLoopMessage] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [assistantEntryMessage, setAssistantEntryMessage] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [assistantInlineResponse, setAssistantInlineResponse] = useState<AssistantEntryResponse | null>(null);
  const [assistantEntryThreadId, setAssistantEntryThreadId] = useState<string | null>(null);
  const [recentCompletedDailyLoop, setRecentCompletedDailyLoop] = useState<DailyLoopSessionData | null>(null);
  const [pendingCommitments, setPendingCommitments] = useState<Record<string, true>>({});
  const [commitmentMessages, setCommitmentMessages] = useState<
    Record<string, { status: 'success' | 'error'; message: string }>
  >({});
  const activeDailyLoop = standupDailyLoop ?? morningDailyLoop ?? null;

  const runFreshnessAction = async (source: NowData['freshness']['sources'][number]) => {
    const action = actionForFreshnessSource(source);
    if (!action) {
      return;
    }

    setPendingActions((current) => ({
      ...current,
      [source.key]: true,
    }));
    setActionMessages((current) => {
      const next = { ...current };
      delete next[source.key];
      return next;
    });

    try {
      if (action.type === 'evaluate') {
        const response = await runEvaluate();
        if (!response.ok) {
          throw new Error(response.error?.message ?? 'Failed to re-run evaluate');
        }
        invalidateQuery(currentContextKey, { refetch: true });
        invalidateQuery(integrationsKey, { refetch: true });
        await refetch();
        setActionMessages((current) => ({
          ...current,
          [source.key]: {
            status: 'success',
            message: 'Context refreshed.',
          },
        }));
      } else if (action.type === 'sync') {
        const response = await syncSource(action.source);
        if (!response.ok) {
          throw new Error(response.error?.message ?? `Failed to sync ${action.source}`);
        }
        invalidateQuery(integrationsKey, { refetch: true });
        invalidateQuery(currentContextKey, { refetch: true });
        await refetch();
        setActionMessages((current) => ({
          ...current,
          [source.key]: {
            status: 'success',
            message: `${action.successLabel} synced (${response.data?.signals_ingested ?? 0} signals).`,
          },
        }));
      } else {
        onOpenSettings?.({ tab: 'integrations', integrationId: action.integrationId });
      }
    } catch (actionError) {
      setActionMessages((current) => ({
        ...current,
        [source.key]: {
          status: 'error',
          message: actionError instanceof Error ? actionError.message : String(actionError),
        },
      }));
    } finally {
      setPendingActions((current) => {
        const next = { ...current };
        delete next[source.key];
        return next;
      });
    }
  };

  const startLoop = async (phase: DailyLoopPhaseData) => {
    if (!sessionDate) {
      return;
    }
    setDailyLoopPending(true);
    setDailyLoopMessage(null);
    try {
      const response = await startDailyLoopSession({
        phase,
        session_date: sessionDate,
        start: {
          source: 'manual',
          surface: 'web',
        },
      });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to start daily loop');
      }
      setRecentCompletedDailyLoop(null);
      setDailyLoopResponse('');
      setQueryData(
        phase === 'morning_overview' ? morningDailyLoopKey : standupDailyLoopKey,
        response.data,
      );
    } catch (actionError) {
      setDailyLoopMessage({
        status: 'error',
        message: actionError instanceof Error ? actionError.message : String(actionError),
      });
    } finally {
      setDailyLoopPending(false);
    }
  };

  const advanceLoop = async (action: 'submit' | 'skip') => {
    if (!activeDailyLoop) {
      return;
    }
    setDailyLoopPending(true);
    setDailyLoopMessage(null);
    try {
      const response = await submitDailyLoopTurn(
        activeDailyLoop.id,
        action,
        action === 'submit' ? dailyLoopResponse : null,
      );
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to advance daily loop');
      }
      const nextSession = response.data;
      const activeKey = nextSession.phase === 'standup' ? standupDailyLoopKey : morningDailyLoopKey;
      if (nextSession.status === 'completed' || nextSession.status === 'cancelled') {
        setRecentCompletedDailyLoop(nextSession);
        setQueryData(activeKey, null);
        invalidateQuery(activeKey, { refetch: true });
        invalidateQuery(nowKey, { refetch: true });
        invalidateQuery(commitmentsKey, { refetch: true });
        await refetch();
      } else {
        setRecentCompletedDailyLoop(null);
        setQueryData(activeKey, nextSession);
      }
      setDailyLoopResponse('');
    } catch (actionError) {
      setDailyLoopMessage({
        status: 'error',
        message: actionError instanceof Error ? actionError.message : String(actionError),
      });
    } finally {
      setDailyLoopPending(false);
    }
  };

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
    setDailyLoopMessage(null);
    setDailyLoopResponse('');
    setRecentCompletedDailyLoop(null);
  }, [sessionDate]);

  useEffect(() => {
    setDailyLoopResponse('');
  }, [activeDailyLoop?.id, activeDailyLoop?.current_prompt?.prompt_id]);

  const handleAssistantEntry = async (response: AssistantEntryResponse) => {
    invalidateQuery(conversationsKey, { refetch: true });
    invalidateQuery(inboxKey, { refetch: true });
    setAssistantEntryMessage(null);
    setAssistantInlineResponse(null);
    setAssistantEntryThreadId(response.conversation.id);

    if (response.route_target === 'threads') {
      onOpenThread?.(response.conversation.id);
      return;
    }
    if (response.route_target === 'inbox') {
      setAssistantEntryMessage({
        status: 'success',
        message: 'Saved to Inbox for follow-up.',
      });
        onOpenInbox?.();
      return;
    }
    setAssistantInlineResponse(response);
    if (response.assistant_error) {
      setAssistantEntryMessage({
        status: 'error',
        message: response.assistant_error,
      });
      return;
    }
    setAssistantEntryMessage({
      status: 'success',
        message: 'Handled here in Now.',
      });
  };

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
  const statusRow = data.status_row;
  const contextLine = data.context_line;
  const nudgeBars = data.nudge_bars ?? [];
  const taskLane = data.task_lane;
  const reviewSnapshot = data.review_snapshot ?? {
    open_action_count: 0,
    triage_count: 0,
    projects_needing_review: 0,
    pending_execution_reviews: 0,
  };
  const pendingWritebacks = data.pending_writebacks ?? [];
  const conflicts = data.conflicts ?? [];
  const peopleReview = peopleNeedingReview(data);
  const summarizedActionItems = actionItems.slice(0, 3);
  const threadAttentionCount = actionItems.filter((item) => item.thread_route !== null).length
    + (data.reflow_status?.thread_id ? 1 : 0);
  const overview = data.overview;
  const nowTs = data.computed_at;
  const activeEvent = findActiveEvent(data.schedule.upcoming_events, nowTs);
  const nextScheduledEvent = findNextEvent(data.schedule.upcoming_events, nowTs);
  const activeRoutineBlock = findActiveRoutineBlock(data.day_plan, nowTs);
  const commitmentRows = dedupeTasks([
    data.tasks.next_commitment,
    ...(data.tasks.other_open ?? []),
  ]);
  const pullableTasks = dedupeTasks(data.tasks.todoist ?? []);
  const currentStatus = buildCurrentStatus(
    data,
    nowTs,
    activeEvent,
    activeRoutineBlock,
    commitmentRows[0] ?? null,
    nextScheduledEvent,
  );
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

  const handleHeaderBucketRoute = (bucket: NonNullable<NowData['header']>['buckets'][number]) => {
    if (bucket.route_target.thread_id) {
      onOpenThread?.(bucket.route_target.thread_id);
    }
  };

  const handleNowBarAction = (
    bar: NowData['nudge_bars'][number],
    action: NowData['nudge_bars'][number]['actions'][number],
  ) => {
    if ((action.kind === 'open_thread' || action.kind === 'expand') && bar.primary_thread_id) {
      onOpenThread?.(bar.primary_thread_id);
      return;
    }
    if (action.kind === 'open_settings') {
      onOpenSettings?.({ tab: 'runtime' });
      return;
    }
    if (action.kind === 'open_inbox') {
      onOpenInbox?.();
    }
  };

  return (
    <div className="flex min-h-full flex-col bg-zinc-950">
      <div className="mx-auto w-full max-w-5xl flex-1 px-4 py-6 pb-32 sm:px-6">
        <section className="rounded-[28px] border border-zinc-800 bg-zinc-900/70 shadow-[0_0_0_1px_rgba(24,24,27,0.45)]">
          <header className="border-b border-zinc-800 px-5 py-5 sm:px-6">
            <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
              <div className="min-w-0">
                <p className="text-[11px] uppercase tracking-[0.22em] text-zinc-500">Now</p>
                <h1 className="mt-2 text-3xl font-semibold tracking-tight text-zinc-100">
                  {header?.title ?? 'Now'}
                </h1>
              </div>

              <div className="flex flex-wrap items-center gap-2 lg:max-w-[60%] lg:justify-end">
                {(header?.buckets ?? []).map((bucket) => (
                  <button
                    key={bucket.kind}
                    type="button"
                    onClick={() => handleHeaderBucketRoute(bucket)}
                    className={`inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs transition ${
                      bucket.urgent
                        ? 'border-amber-700/60 bg-amber-950/30 text-amber-100'
                        : 'border-zinc-700 bg-zinc-950/80 text-zinc-300 hover:border-zinc-500 hover:text-zinc-100'
                    }`}
                  >
                    <span>{formatNowHeaderBucketLabel(bucket.kind)}</span>
                    {formatNowHeaderBucketCount(bucket) ? (
                      <span className="rounded-full border border-current/20 px-1.5 py-0.5 text-[10px] leading-none">
                        {formatNowHeaderBucketCount(bucket)}
                      </span>
                    ) : null}
                  </button>
                ))}
                {meshSummary ? (
                  <div
                    className={`inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs ${
                      meshSummary.urgent
                        ? 'border-amber-700/60 bg-amber-950/30 text-amber-100'
                        : 'border-zinc-700 bg-zinc-950/80 text-zinc-400'
                    }`}
                  >
                    <span>{formatNowMeshSyncState(meshSummary.sync_state)}</span>
                    {meshSummary.queued_write_count > 0 ? (
                      <span className="rounded-full border border-current/20 px-1.5 py-0.5 text-[10px] leading-none">
                        {meshSummary.queued_write_count} queued
                      </span>
                    ) : null}
                  </div>
                ) : null}
              </div>
            </div>
          </header>

          {statusRow ? (
            <div className="grid gap-2 border-b border-zinc-800 px-5 py-3 text-sm text-zinc-400 sm:grid-cols-[auto_auto_minmax(0,1fr)_auto] sm:items-center sm:px-6">
              <span className="truncate">{statusRow.date_label}</span>
              <span className="truncate">{statusRow.time_label}</span>
              <span className="truncate">{statusRow.context_label}</span>
              <span className="truncate text-zinc-500">{statusRow.elapsed_label}</span>
            </div>
          ) : null}

          {contextLine ? (
            <div className="border-b border-zinc-800 px-5 py-3 sm:px-6">
              <p className="text-sm text-zinc-500">{contextLine.text}</p>
            </div>
          ) : null}

          {meshSummary ? (
            <div className="border-b border-zinc-800 px-5 py-3 sm:px-6">
              <div className="flex flex-wrap items-center gap-3 text-xs text-zinc-400">
                <span>{meshSummary.authority_label}</span>
                <span>{formatNowMeshSyncState(meshSummary.sync_state)}</span>
                <span>{meshSummary.linked_node_count} linked</span>
                {meshSummary.last_sync_at ? (
                  <span>Last sync {formatTime(meshSummary.last_sync_at, data.timezone)}</span>
                ) : null}
                {meshSummary.repair_route ? (
                  <span className={meshSummary.urgent ? 'text-amber-200' : 'text-zinc-500'}>
                    {meshSummary.repair_route.summary}
                  </span>
                ) : null}
              </div>
            </div>
          ) : null}

          {nudgeBars.length > 0 ? (
            <div className="space-y-2 border-b border-zinc-800 px-5 py-4 sm:px-6">
              {nudgeBars.map((bar) => (
                <div
                  key={bar.id}
                  className={`flex flex-col gap-3 rounded-xl border px-4 py-3 sm:flex-row sm:items-center sm:justify-between ${
                    bar.urgent
                      ? 'border-amber-700/50 bg-amber-950/20'
                      : 'border-zinc-800 bg-zinc-950/55'
                  }`}
                >
                  <div className="min-w-0">
                    <div className="flex flex-wrap items-center gap-2">
                      <span className="text-sm font-medium text-zinc-100">{bar.title}</span>
                      <span className="rounded-full border border-zinc-700 px-2 py-0.5 text-[10px] uppercase tracking-[0.16em] text-zinc-500">
                        {formatNowBarKind(bar.kind)}
                      </span>
                    </div>
                    <p className="mt-1 text-sm text-zinc-400">{bar.summary}</p>
                  </div>
                  {bar.actions.length > 0 ? (
                    <div className="flex flex-wrap gap-2">
                      {bar.actions.map((action) => (
                        <button
                          key={`${bar.id}-${action.kind}-${action.label}`}
                          type="button"
                          onClick={() => handleNowBarAction(bar, action)}
                          className="rounded-full border border-zinc-700 bg-zinc-950/80 px-3 py-1.5 text-xs text-zinc-300 transition hover:border-zinc-500 hover:text-zinc-100"
                        >
                          {action.label}
                        </button>
                      ))}
                    </div>
                  ) : null}
                </div>
              ))}
            </div>
          ) : null}

          {taskLane ? (
            <section className="px-5 py-4 sm:px-6">
              <div className="flex items-center justify-between gap-3">
                <h2 className="text-sm font-medium uppercase tracking-[0.18em] text-zinc-500">
                  Tasks
                </h2>
                {taskLane.overflow_count > 0 ? (
                  <span className="text-xs text-zinc-500">+{taskLane.overflow_count} more</span>
                ) : null}
              </div>

              <div className="mt-3 space-y-2">
                {taskLane.active ? (
                  <CompactTaskLaneRow
                    item={taskLane.active}
                    emphasis="active"
                    pending={Boolean(pendingCommitments[taskLane.active.id])}
                    feedback={commitmentMessages[taskLane.active.id]}
                    onOpenThread={taskLane.active.primary_thread_id ? () => onOpenThread?.(taskLane.active!.primary_thread_id!) : undefined}
                    onComplete={
                      commitmentRows.some((task) => task.id === taskLane.active?.id)
                        ? () => void completeCommitment(taskLane.active!.id)
                        : undefined
                    }
                  />
                ) : null}
                {taskLane.pending.map((item) => (
                  <CompactTaskLaneRow
                    key={item.id}
                    item={item}
                    pending={Boolean(pendingCommitments[item.id])}
                    feedback={commitmentMessages[item.id]}
                    onOpenThread={item.primary_thread_id ? () => onOpenThread?.(item.primary_thread_id!) : undefined}
                    onComplete={
                      commitmentRows.some((task) => task.id === item.id)
                        ? () => void completeCommitment(item.id)
                        : undefined
                    }
                  />
                ))}
                {taskLane.recent_completed.map((item) => (
                  <CompactTaskLaneRow
                    key={item.id}
                    item={item}
                    emphasis="completed"
                    onOpenThread={item.primary_thread_id ? () => onOpenThread?.(item.primary_thread_id!) : undefined}
                  />
                ))}
                {!taskLane.active && taskLane.pending.length === 0 && taskLane.recent_completed.length === 0 ? (
                  <SurfaceState message="No current tasks are surfaced right now." />
                ) : null}
              </div>
            </section>
          ) : null}
        </section>

        <details className="mt-6 rounded-2xl border border-zinc-800 bg-zinc-900/40 p-5">
          <summary className="cursor-pointer list-none text-sm font-medium text-zinc-100">
            More context and controls
          </summary>
          <div className="mt-5 space-y-6">
            <Panel
              title="Overview"
              subtitle="Backend-owned orientation decides the primary action, one visible nudge, and the explanation lane."
            >
              <div className="grid gap-4 lg:grid-cols-[minmax(0,1.7fr)_minmax(0,1fr)]">
                <div className="rounded-xl border border-zinc-800 bg-zinc-950/50 p-4">
                  <div className="flex flex-wrap items-center gap-2">
                    <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-400">
                      {overview.dominant_action ? 'Dominant action' : 'Suggestions'}
                    </span>
                    <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
                      Updated {formatTime(nowTs, data.timezone)}
                    </span>
                  </div>
                  <h2 className="mt-3 text-xl font-medium text-zinc-100">
                    {overview.dominant_action?.title ?? 'Choose the next bounded move'}
                  </h2>
                  <p className="mt-2 text-sm leading-6 text-zinc-300">
                    {overview.dominant_action?.summary
                      ?? overview.suggestions[0]?.summary
                      ?? 'No dominant action is active right now.'}
                  </p>
                  {overview.dominant_action ? null : overview.suggestions.length > 0 ? (
                    <div className="mt-4 space-y-3">
                      {overview.suggestions.map((suggestion) => (
                        <div
                          key={suggestion.id}
                          className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-3"
                        >
                          <p className="text-sm font-medium text-zinc-100">{suggestion.title}</p>
                          <p className="mt-1 text-xs leading-5 text-zinc-400">{suggestion.summary}</p>
                        </div>
                      ))}
                      <div className="flex flex-wrap gap-2">
                        {overview.decision_options.map((option) => (
                          <span
                            key={option}
                            className="rounded-full border border-zinc-800 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-400"
                          >
                            {option}
                          </span>
                        ))}
                      </div>
                    </div>
                  ) : null}
                </div>

                <div className="space-y-4">
                  {overview.visible_nudge ? (
                    <div className="rounded-xl border border-amber-900/50 bg-amber-950/20 p-4">
                      <p className="text-xs uppercase tracking-[0.18em] text-amber-200/70">
                        Visible nudge
                      </p>
                      <h3 className="mt-2 text-base font-medium text-amber-100">
                        {overview.visible_nudge.title}
                      </h3>
                      <p className="mt-2 text-sm leading-6 text-amber-50/85">
                        {overview.visible_nudge.summary}
                      </p>
                    </div>
                  ) : null}

                  <details className="rounded-xl border border-zinc-800 bg-zinc-900/40 p-4">
                    <summary className="cursor-pointer list-none text-sm font-medium text-zinc-100">
                      Why + state
                    </summary>
                    <div className="mt-3 space-y-2">
                      {overview.why_state.length > 0 ? (
                        overview.why_state.map((item) => (
                          <div
                            key={`${item.label}-${item.detail}`}
                            className="rounded-lg border border-zinc-900 bg-zinc-950/70 px-3 py-2"
                          >
                            <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                              {item.label}
                            </p>
                            <p className="mt-1 text-sm text-zinc-200">{item.detail}</p>
                          </div>
                        ))
                      ) : (
                        <SurfaceState message="No additional explanation is available yet." />
                      )}
                    </div>
                  </details>
                </div>
              </div>
            </Panel>

            <Panel
              title="Next event"
              subtitle="Calendar is authoritative here. Routine blocks and low-value schedule noise stay out of this slot."
            >
              {nextScheduledEvent ? (
                <EventSummaryCard
                  event={nextScheduledEvent}
                  timezone={data.timezone}
                  nowTs={nowTs}
                  emphasis="next"
                />
              ) : (
                <SurfaceState
                  message={currentStatus.fallbackEventMessage ?? 'No more calendar events are in play right now.'}
                />
              )}
            </Panel>

            <Panel
              title="Today"
              subtitle="Commitments stay primary. Pull from tasks only when today still has room."
            >
              <div className="space-y-4">
                <TodayLaneCard
                  label="Active"
                  emptyMessage="Nothing is actively in motion right now."
                >
                  {activeEvent ? (
                    <TodayEventRow event={activeEvent} timezone={data.timezone} nowTs={nowTs} />
                  ) : commitmentRows.length > 0 ? (
                    <TodayCommitmentRow
                      task={commitmentRows[0]}
                      timezone={data.timezone}
                      pending={Boolean(pendingCommitments[commitmentRows[0].id])}
                      feedback={commitmentMessages[commitmentRows[0].id]}
                      onComplete={() => void completeCommitment(commitmentRows[0].id)}
                    />
                  ) : (
                    <SurfaceState message={currentStatus.summary} />
                  )}
                </TodayLaneCard>

                <TodayLaneCard
                  label="Next up"
                  emptyMessage="Nothing time-bound is coming up next."
                >
                  {nextScheduledEvent ? (
                    <TodayEventRow event={nextScheduledEvent} timezone={data.timezone} nowTs={nowTs} />
                  ) : commitmentRows.length > 1 ? (
                    <TodayCommitmentRow
                      task={commitmentRows[1]}
                      timezone={data.timezone}
                      pending={Boolean(pendingCommitments[commitmentRows[1].id])}
                      feedback={commitmentMessages[commitmentRows[1].id]}
                      onComplete={() => void completeCommitment(commitmentRows[1].id)}
                    />
                  ) : (
                    <SurfaceState message="No next-up item is selected yet." />
                  )}
                </TodayLaneCard>

                <TodayLaneGroup
                  title="Commitments"
                  subtitle="Chosen or already in play for this day."
                  emptyMessage="No commitments are in play yet."
                >
                  {commitmentRows.map((task, index) => (
                    <TodayCommitmentRow
                      key={task.id}
                      task={task}
                      timezone={data.timezone}
                      pending={Boolean(pendingCommitments[task.id])}
                      feedback={commitmentMessages[task.id]}
                      emphasis={index === 0 ? 'primary' : 'default'}
                      onComplete={() => void completeCommitment(task.id)}
                    />
                  ))}
                </TodayLaneGroup>

                <TodayLaneGroup
                  title="Pull from tasks"
                  subtitle="Available work that has not earned more attention than commitments."
                  emptyMessage="No pullable tasks are surfaced right now."
                >
                  {pullableTasks.map((task) => (
                    <TodayTaskRow key={task.id} task={task} timezone={data.timezone} />
                  ))}
                </TodayLaneGroup>
              </div>
            </Panel>

            <FreshnessBanner
              freshness={data.freshness}
              pendingActions={pendingActions}
              actionMessages={actionMessages}
              onRunAction={runFreshnessAction}
            />

            <DailyLoopPanel
              sessionDate={sessionDate}
              activeSession={activeDailyLoop}
              completedSession={activeDailyLoop ? null : recentCompletedDailyLoop}
              responseText={dailyLoopResponse}
              pending={dailyLoopPending}
              message={dailyLoopMessage}
              onChangeResponse={setDailyLoopResponse}
              onStart={startLoop}
              onSubmit={() => void advanceLoop('submit')}
              onSkip={() => void advanceLoop('skip')}
            />

            {data.day_plan ? <DayPlanCardView dayPlan={data.day_plan} timezone={data.timezone} /> : null}
            {data.reflow ? <ReflowCardView reflow={data.reflow} /> : null}
            {data.commitment_scheduling_summary ? (
              <CommitmentSchedulingSummaryCardView summary={data.commitment_scheduling_summary} />
            ) : null}
            {data.planning_profile_summary ? (
              <PlanningProfileSummaryCardView summary={data.planning_profile_summary} />
            ) : null}
            {data.reflow_status ? <ReflowStatusView status={data.reflow_status} timezone={data.timezone} /> : null}
            {data.check_in ? <CheckInCardView checkIn={data.check_in} /> : null}
            <TrustReadinessPanel trustReadiness={data.trust_readiness} timezone={data.timezone} />
            <QueuePressureSummary
              reviewSnapshot={reviewSnapshot}
              actionItems={summarizedActionItems}
              threadAttentionCount={threadAttentionCount}
            />
            <DebugStatePanel debug={data.debug} />
          </div>
        </details>
      </div>

      <div className="sticky bottom-0 border-t border-zinc-800 bg-zinc-950/95 px-4 py-4 backdrop-blur sm:px-6">
        <div className="mx-auto max-w-5xl space-y-3">
          <p className="text-sm text-zinc-400">
            Type or hold the mic to talk locally, then let Vel route it inline, to Inbox, or into Threads.
          </p>
          {assistantEntryMessage ? (
            <p
              className={`rounded-xl border px-4 py-3 text-sm ${
                assistantEntryMessage.status === 'error'
                  ? 'border-red-900/70 bg-red-950/40 text-red-200'
                  : 'border-emerald-900/60 bg-emerald-950/30 text-emerald-200'
              }`}
              role={assistantEntryMessage.status === 'error' ? 'alert' : 'status'}
            >
              {assistantEntryMessage.message}
            </p>
          ) : null}
          {assistantEntryThreadId ? (
            <button
              type="button"
              onClick={() => onOpenThread?.(assistantEntryThreadId)}
              className="self-start rounded-full border border-zinc-700 bg-zinc-950/80 px-3 py-1.5 text-xs uppercase tracking-[0.16em] text-zinc-400 transition hover:border-zinc-500 hover:text-zinc-100"
            >
              Open thread
            </button>
          ) : null}
          {assistantInlineResponse?.assistant_message ? (
            <button
              type="button"
              onClick={() => onOpenThread?.(assistantInlineResponse.conversation.id)}
              className="rounded-xl border border-zinc-800 bg-zinc-900/70 px-4 py-3 text-left text-sm text-zinc-300 transition hover:border-zinc-600 hover:text-zinc-100"
            >
              <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Transcript / inline reply</p>
              <p className="mt-1">
                {extractMessageText(assistantInlineResponse.assistant_message) ?? 'Vel responded inline.'}
              </p>
            </button>
          ) : null}
          <MessageComposer
            onSent={(_, response) => {
              void handleAssistantEntry(response);
            }}
          />
        </div>
      </div>
    </div>
  );
}

function DailyLoopPanel({
  sessionDate,
  activeSession,
  completedSession,
  responseText,
  pending,
  message,
  onChangeResponse,
  onStart,
  onSubmit,
  onSkip,
}: {
  sessionDate: string | null;
  activeSession: DailyLoopSessionData | null;
  completedSession: DailyLoopSessionData | null;
  responseText: string;
  pending: boolean;
  message: { status: 'success' | 'error'; message: string } | null;
  onChangeResponse: (value: string) => void;
  onStart: (phase: DailyLoopPhaseData) => void;
  onSubmit: () => void;
  onSkip: () => void;
}) {
  const session = activeSession ?? completedSession;
  const renderableSession = isRenderableDailyLoopSession(session) ? session : null;
  const state = renderableSession?.state ?? null;
  const outcome = renderableSession && completedSession ? completedSession.outcome : null;

  return (
    <Panel
      title="Daily loop"
      subtitle="Start or resume the backend-owned morning overview and standup flow for today."
    >
      {sessionDate ? (
        <p className="mb-4 text-xs uppercase tracking-[0.18em] text-zinc-500">
          Session date {sessionDate}
        </p>
      ) : null}

      {renderableSession ? (
        <div className="space-y-4">
          <div className="flex flex-wrap items-center gap-2">
            <span className="rounded-full border border-emerald-700/40 bg-emerald-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-emerald-200">
              {formatDailyLoopPhase(renderableSession.phase)}
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-400">
              {formatDailyLoopStatus(renderableSession.status)}
            </span>
            {renderableSession.current_prompt ? (
              <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
                Question {renderableSession.current_prompt.ordinal}
              </span>
            ) : null}
          </div>

          {state?.phase === 'morning_overview' ? (
            <div className="space-y-3 rounded-xl border border-zinc-800 bg-zinc-950/50 p-4">
              <p className="text-sm leading-6 text-zinc-200">{state.snapshot}</p>
              {state.friction_callouts.length > 0 ? (
                <div className="space-y-2">
                  {state.friction_callouts.map((callout) => (
                    <div key={`${callout.label}-${callout.detail}`} className="rounded-lg border border-amber-700/30 bg-amber-950/20 px-3 py-2">
                      <p className="text-sm font-medium text-amber-100">{callout.label}</p>
                      <p className="mt-1 text-xs text-amber-200/80">{callout.detail}</p>
                    </div>
                  ))}
                </div>
              ) : null}
              {state.signals.length > 0 ? (
                <ul className="space-y-2 text-sm text-zinc-300">
                  {state.signals.map((signal, index) => (
                    <li key={`${signal.kind}-${index}`} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                      {signal.text}
                    </li>
                  ))}
                </ul>
              ) : null}
            </div>
          ) : null}

          {state?.phase === 'standup' ? (
            <div className="space-y-4 rounded-xl border border-zinc-800 bg-zinc-950/50 p-4">
              <DailyLoopStandupSummary title="Commitments" emptyMessage="No commitments defined yet." items={state.commitments.map((item) => `${formatStandupBucket(item.bucket)} · ${item.title}`)} />
              <DailyLoopStandupSummary title="Deferred" emptyMessage="No deferrals recorded." items={state.deferred_tasks.map((item) => `${item.title} (${item.reason})`)} />
              <DailyLoopStandupSummary title="Calendar" emptyMessage="No calendar constraints confirmed yet." items={state.confirmed_calendar} />
              <DailyLoopStandupSummary title="Focus blocks" emptyMessage="No focus blocks proposed yet." items={state.focus_blocks.map((block) => `${block.label} · ${formatRfc3339(block.start_at, 'UTC')} to ${formatRfc3339(block.end_at, 'UTC')}`)} />
            </div>
          ) : null}

          {activeSession?.current_prompt ? (
            <div className="space-y-3 rounded-xl border border-zinc-800 bg-zinc-900/60 p-4">
              <p className="text-sm font-medium text-zinc-100">{activeSession.current_prompt.text}</p>
              <textarea
                value={responseText}
                onChange={(event) => onChangeResponse(event.target.value)}
                placeholder={activeSession.phase === 'morning_overview' ? 'Type a brief response, or skip.' : 'Type one concise answer, or skip.'}
                rows={3}
                className="w-full rounded-lg border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 outline-none transition focus:border-emerald-500"
              />
              <div className="flex flex-wrap gap-3">
                <button
                  type="button"
                  onClick={onSubmit}
                  disabled={pending || responseText.trim().length === 0}
                  className="rounded-md border border-emerald-700/70 px-3 py-1.5 text-sm font-medium text-emerald-100 transition hover:border-emerald-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-500"
                >
                  {pending ? 'Saving…' : 'Submit response'}
                </button>
                <button
                  type="button"
                  onClick={onSkip}
                  disabled={pending}
                  className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm font-medium text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-500"
                >
                  Skip
                </button>
              </div>
            </div>
          ) : null}

          {outcome ? (
            <div className="rounded-xl border border-emerald-700/30 bg-emerald-950/20 p-4">
              <p className="text-sm font-medium text-emerald-100">
                {outcome.phase === 'standup' ? 'Standup saved.' : 'Morning overview captured.'}
              </p>
              {outcome.phase === 'morning_overview' ? (
                <ul className="mt-3 space-y-2 text-sm text-emerald-50">
                  {outcome.signals.length === 0 ? (
                    <li>No intent signals were captured.</li>
                  ) : (
                    outcome.signals.map((signal, index) => (
                      <li key={`${signal.kind}-${index}`}>{signal.text}</li>
                    ))
                  )}
                </ul>
              ) : (
                <div className="mt-3 space-y-3 text-sm text-emerald-50">
                  <DailyLoopStandupSummary title="Committed" emptyMessage="No commitments were saved." items={outcome.commitments.map((item) => `${formatStandupBucket(item.bucket)} · ${item.title}`)} tone="success" />
                  <DailyLoopStandupSummary title="Deferred" emptyMessage="No deferrals recorded." items={outcome.deferred_tasks.map((item) => `${item.title} (${item.reason})`)} tone="success" />
                </div>
              )}
            </div>
          ) : null}
        </div>
      ) : (
        <div className="space-y-4">
          <p className="text-sm text-zinc-300">
            Start morning for a brief overview, or jump directly into standup if you already know the day.
          </p>
          <div className="flex flex-wrap gap-3">
            <button
              type="button"
              onClick={() => onStart('morning_overview')}
              disabled={pending || !sessionDate}
              className="rounded-md border border-emerald-700/70 px-3 py-1.5 text-sm font-medium text-emerald-100 transition hover:border-emerald-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-500"
            >
              Start morning
            </button>
            <button
              type="button"
              onClick={() => onStart('standup')}
              disabled={pending || !sessionDate}
              className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm font-medium text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-500"
            >
              Start standup
            </button>
          </div>
        </div>
      )}

      {message ? (
        <p className={`mt-4 text-sm ${message.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
          {message.message}
        </p>
      ) : null}
    </Panel>
  );
}

function CompactTaskLaneRow({
  item,
  emphasis = 'default',
  pending = false,
  feedback,
  onOpenThread,
  onComplete,
}: {
  item: NonNullable<NowData['task_lane']>['active'] extends infer T
    ? Exclude<T, null>
    : never;
  emphasis?: 'active' | 'default' | 'completed';
  pending?: boolean;
  feedback?: { status: 'success' | 'error'; message: string };
  onOpenThread?: () => void;
  onComplete?: () => void;
}) {
  const completed = emphasis === 'completed' || item.state === 'completed';
  const canComplete = Boolean(onComplete) && !completed;

  return (
    <div
      className={`rounded-xl border px-4 py-3 ${
        emphasis === 'active'
          ? 'border-emerald-700/40 bg-emerald-950/10'
          : completed
            ? 'border-zinc-800 bg-zinc-950/30'
            : 'border-zinc-800 bg-zinc-950/55'
      }`}
    >
      <div className="flex items-start gap-3">
        <button
          type="button"
          disabled={!canComplete || pending}
          onClick={onComplete}
          aria-label={completed ? `${item.text} completed` : `Complete ${item.text}`}
          className={`mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded border text-[11px] ${
            completed
              ? 'border-emerald-600 bg-emerald-600 text-zinc-950'
              : canComplete
                ? 'border-zinc-600 bg-zinc-950 text-zinc-500'
                : 'border-zinc-800 bg-zinc-900 text-zinc-700'
          }`}
        >
          {completed ? '✓' : ''}
        </button>
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <p className={`text-sm font-medium ${completed ? 'text-zinc-500 line-through' : 'text-zinc-100'}`}>
              {item.text}
            </p>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[10px] uppercase tracking-[0.16em] text-zinc-500">
              {item.task_kind}
            </span>
            {item.project ? (
              <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[10px] text-zinc-500">
                {item.project}
              </span>
            ) : null}
          </div>
          {feedback ? (
            <p className={`mt-2 text-xs ${feedback.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
              {feedback.message}
            </p>
          ) : null}
          {onOpenThread ? (
            <button
              type="button"
              onClick={onOpenThread}
              className="mt-2 rounded-full border border-zinc-700 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.16em] text-zinc-400 transition hover:border-zinc-500 hover:text-zinc-100"
            >
              Open thread
            </button>
          ) : null}
        </div>
      </div>
    </div>
  );
}

function ContextStripCard({
  label,
  value,
  detail,
}: {
  label: string;
  value: string;
  detail: string;
}) {
  return (
    <div className="rounded-2xl border border-zinc-800 bg-zinc-900/40 px-4 py-3">
      <p className="text-[11px] uppercase tracking-[0.18em] text-zinc-500">{label}</p>
      <p className="mt-2 text-sm font-medium text-zinc-100">{value}</p>
      <p className="mt-1 text-xs text-zinc-500">{detail}</p>
    </div>
  );
}

function CheckInCardView({ checkIn }: { checkIn: NowData['check_in'] }) {
  if (!checkIn) {
    return null;
  }

  return (
    <article className={`rounded-2xl border p-4 ${checkIn.blocking ? 'border-emerald-700/50 bg-emerald-950/20' : 'border-zinc-800 bg-zinc-950/50'}`}>
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-emerald-700/50 bg-emerald-950/40 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-emerald-200">
          Check-in
        </span>
        {checkIn.blocking ? (
          <span className="rounded-full border border-amber-700/50 bg-amber-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-amber-100">
            Blocking
          </span>
        ) : null}
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">{checkIn.title}</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">{checkIn.summary}</p>
      <div className="mt-3 rounded-xl border border-zinc-800 bg-zinc-950/70 p-3">
        <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Prompt</p>
        <p className="mt-2 text-sm text-zinc-100">{checkIn.prompt_text}</p>
      </div>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
        {checkIn.suggested_action_label ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
            Suggested: {checkIn.suggested_action_label}
          </span>
        ) : null}
        {checkIn.allow_skip ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
            Bypass allowed with note
          </span>
        ) : null}
        {checkIn.escalation ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
            {checkIn.escalation.label}
          </span>
        ) : null}
      </div>
    </article>
  );
}

function DayPlanCardView({
  dayPlan,
  timezone,
}: {
  dayPlan: NowData['day_plan'];
  timezone: string;
}) {
  if (!dayPlan) {
    return null;
  }

  const operatorDeclaredRoutineCount = dayPlan.routine_blocks.filter(
    (block) => block.source === 'operator_declared',
  ).length;
  const inferredRoutineCount = dayPlan.routine_blocks.filter(
    (block) => block.source === 'inferred',
  ).length;

  return (
    <article className="rounded-2xl border border-sky-800/40 bg-sky-950/20 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-sky-700/50 bg-sky-950/40 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-sky-100">
          Day plan
        </span>
        {dayPlan.needs_judgment_count > 0 ? (
          <span className="rounded-full border border-amber-700/50 bg-amber-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-amber-100">
            {dayPlan.needs_judgment_count} need judgment
          </span>
        ) : null}
        {operatorDeclaredRoutineCount > 0 ? (
          <span className="rounded-full border border-emerald-700/50 bg-emerald-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-emerald-100">
            {operatorDeclaredRoutineCount} operator-managed
          </span>
        ) : inferredRoutineCount > 0 ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
            inferred routine blocks
          </span>
        ) : null}
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">{dayPlan.headline}</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">{dayPlan.summary}</p>
      <div className="mt-4 flex flex-wrap gap-2 text-xs text-zinc-300">
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {dayPlan.scheduled_count} scheduled
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {dayPlan.deferred_count} deferred
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {dayPlan.did_not_fit_count} did not fit
        </span>
      </div>
      {dayPlan.routine_blocks.length > 0 ? (
        <div className="mt-4 rounded-xl border border-zinc-800 bg-zinc-950/60 p-3">
          <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Routine blocks</p>
          <p className="mt-1 text-xs leading-5 text-zinc-400">
            {operatorDeclaredRoutineCount > 0
              ? 'These operator-managed blocks are shaping today from the durable planning profile before any recovery logic runs.'
              : 'These routine blocks are backend-inferred from current context until durable routines are configured in Settings.'}
          </p>
          <div className="mt-2 flex flex-wrap gap-2">
            {dayPlan.routine_blocks.slice(0, 3).map((block) => (
              <span
                key={block.id}
                className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] text-zinc-300"
              >
                {block.label} · {formatTimestamp(block.start_ts, timezone)} · {block.source === 'operator_declared' ? 'saved' : block.source.replaceAll('_', ' ')}
              </span>
            ))}
          </div>
        </div>
      ) : null}
      {dayPlan.changes.length > 0 ? (
        <div className="mt-4 space-y-2">
          {dayPlan.changes.slice(0, 3).map((change) => (
            <div
              key={`${change.kind}-${change.title}-${change.detail}`}
              className="rounded-lg border border-zinc-800 bg-zinc-900/70 px-3 py-2"
            >
              <div className="flex flex-wrap items-center gap-2">
                <p className="text-sm font-medium text-zinc-100">{change.title}</p>
                <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] uppercase tracking-[0.16em] text-zinc-400">
                  {change.kind.replaceAll('_', ' ')}
                </span>
                {change.project_label ? (
                  <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] text-zinc-400">
                    {change.project_label}
                  </span>
                ) : null}
                {change.scheduled_start_ts ? (
                  <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] text-zinc-400">
                    {formatTimestamp(change.scheduled_start_ts, timezone)}
                  </span>
                ) : null}
              </div>
              <p className="mt-1 text-xs leading-5 text-zinc-400">{change.detail}</p>
            </div>
          ))}
        </div>
      ) : null}
    </article>
  );
}

function ReflowCardView({ reflow }: { reflow: NowData['reflow'] }) {
  if (!reflow) {
    return null;
  }

  const proposal = reflow.proposal;
  return (
    <article className={`rounded-2xl border p-4 ${reflow.severity === 'critical' ? 'border-rose-700/50 bg-rose-950/20' : 'border-amber-700/40 bg-amber-950/20'}`}>
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-rose-700/50 bg-rose-950/40 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-rose-100">
          Reflow
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          {reflow.severity}
        </span>
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">{reflow.title}</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">{reflow.summary}</p>
      {reflow.preview_lines.length > 0 ? (
        <ul className="mt-3 space-y-2">
          {reflow.preview_lines.map((line) => (
            <li key={line} className="rounded-lg border border-zinc-800 bg-zinc-950/70 px-3 py-2 text-sm text-zinc-100">
              {line}
            </li>
          ))}
        </ul>
      ) : null}
      {proposal ? (
        <div className="mt-4 space-y-3 rounded-xl border border-zinc-800/80 bg-zinc-950/60 p-3">
          <div className="flex flex-wrap gap-2 text-xs text-zinc-300">
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {proposal.moved_count} moved
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {proposal.unscheduled_count} unscheduled
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {proposal.needs_judgment_count} needs judgment
            </span>
          </div>
          <p className="text-sm leading-6 text-zinc-300">{proposal.summary}</p>
          {proposal.changes.length > 0 ? (
            <div className="space-y-2">
              {proposal.changes.slice(0, 3).map((change) => (
                <div key={`${change.kind}-${change.title}-${change.detail}`} className="rounded-lg border border-zinc-800 bg-zinc-900/70 px-3 py-2">
                  <div className="flex flex-wrap items-center gap-2">
                    <p className="text-sm font-medium text-zinc-100">{change.title}</p>
                    <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] uppercase tracking-[0.16em] text-zinc-400">
                      {change.kind.replace('_', ' ')}
                    </span>
                    {change.project_label ? (
                      <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] text-zinc-400">
                        {change.project_label}
                      </span>
                    ) : null}
                  </div>
                  <p className="mt-1 text-xs leading-5 text-zinc-400">{change.detail}</p>
                </div>
              ))}
            </div>
          ) : null}
          {proposal.rule_facets.length > 0 ? (
            <div className="flex flex-wrap gap-2">
              {proposal.rule_facets.slice(0, 4).map((facet) => (
                <span
                  key={`${facet.kind}-${facet.label}`}
                  className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] text-zinc-400"
                >
                  {facet.label}
                </span>
              ))}
            </div>
          ) : null}
        </div>
      ) : null}
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-300">
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {reflow.suggested_action_label}
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {reflow.edit_target.label}
        </span>
      </div>
    </article>
  );
}

function PlanningProfileSummaryCardView({
  summary,
}: {
  summary: NonNullable<NowData['planning_profile_summary']>;
}) {
  const latestPending = summary.latest_pending;
  const latestApplied = summary.latest_applied;
  const latestFailed = summary.latest_failed;

  return (
    <article className="rounded-2xl border border-emerald-800/30 bg-emerald-950/10 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-emerald-700/40 bg-emerald-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-emerald-100">
          Planning profile
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          {summary.pending_count} pending
        </span>
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">Routine edits stay review-gated</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">
        `Now` only shows whether a planning-profile change is waiting or recently applied. Approval and longer follow-up still live in `Threads`.
      </p>
      {latestPending ? (
        <div className="mt-4 rounded-lg border border-zinc-800 bg-zinc-950/60 px-3 py-2">
          <p className="text-sm font-medium text-zinc-100">Pending: {latestPending.title}</p>
          <p className="mt-1 text-xs leading-5 text-zinc-400">{latestPending.summary}</p>
        </div>
      ) : null}
      {latestApplied ? (
        <p className="mt-3 text-xs leading-5 text-zinc-400">
          Last applied: {latestApplied.title}
          {latestApplied.outcome_summary ? ` · ${latestApplied.outcome_summary}` : ''}
        </p>
      ) : null}
      {!latestApplied && latestFailed ? (
        <p className="mt-3 text-xs leading-5 text-rose-300">
          Last failed: {latestFailed.title}
          {latestFailed.outcome_summary ? ` · ${latestFailed.outcome_summary}` : ''}
        </p>
      ) : null}
    </article>
  );
}

function CommitmentSchedulingSummaryCardView({
  summary,
}: {
  summary: NonNullable<NowData['commitment_scheduling_summary']>;
}) {
  const latestPending = summary.latest_pending;
  const latestApplied = summary.latest_applied;
  const latestFailed = summary.latest_failed;

  return (
    <article className="rounded-2xl border border-sky-800/30 bg-sky-950/10 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-sky-700/40 bg-sky-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-sky-100">
          Same-day schedule
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          {summary.pending_count} pending
        </span>
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">Schedule edits stay supervised</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">
        `Now` only shows whether a bounded day-plan or reflow change is waiting or recently applied. Approval and longer schedule disagreement still live in `Threads`.
      </p>
      {latestPending ? (
        <div className="mt-4 rounded-lg border border-zinc-800 bg-zinc-950/60 px-3 py-2">
          <p className="text-sm font-medium text-zinc-100">Pending: {latestPending.title}</p>
          <p className="mt-1 text-xs leading-5 text-zinc-400">{latestPending.summary}</p>
        </div>
      ) : null}
      {latestApplied ? (
        <p className="mt-3 text-xs leading-5 text-zinc-400">
          Last applied: {latestApplied.title}
          {latestApplied.outcome_summary ? ` · ${latestApplied.outcome_summary}` : ''}
        </p>
      ) : null}
      {!latestApplied && latestFailed ? (
        <p className="mt-3 text-xs leading-5 text-rose-300">
          Last failed: {latestFailed.title}
          {latestFailed.outcome_summary ? ` · ${latestFailed.outcome_summary}` : ''}
        </p>
      ) : null}
    </article>
  );
}

function ReflowStatusView({
  status,
  timezone,
}: {
  status: NowData['reflow_status'];
  timezone: string;
}) {
  if (!status) {
    return null;
  }

  return (
    <article className="rounded-2xl border border-zinc-800 bg-zinc-950/50 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          Reflow status
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-500">
          {status.kind}
        </span>
      </div>
      <h3 className="mt-3 text-base font-medium text-zinc-100">{status.headline}</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">{status.detail}</p>
      {status.preview_lines.length > 0 ? (
        <ul className="mt-3 space-y-2">
          {status.preview_lines.map((line) => (
            <li
              key={line}
              className="rounded-lg border border-zinc-800 bg-zinc-950/70 px-3 py-2 text-sm text-zinc-100"
            >
              {line}
            </li>
          ))}
        </ul>
      ) : null}
      <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">
        <span>Recorded {formatTimestamp(status.recorded_at, timezone)}</span>
        {status.thread_id ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-zinc-300">
            Continue in Threads
          </span>
        ) : null}
        {status.thread_id ? <span>Thread {status.thread_id}</span> : null}
      </div>
    </article>
  );
}

function TrustReadinessPanel({
  trustReadiness,
}: {
  trustReadiness: NowData['trust_readiness'];
  timezone: string;
}) {
  return (
    <article className="rounded-2xl border border-zinc-800 bg-zinc-950/50 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          Trust and readiness
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-500">
          {trustReadiness.level}
        </span>
      </div>
      <h3 className="mt-3 text-base font-medium text-zinc-100">{trustReadiness.headline}</h3>
      <p className="mt-2 text-sm leading-6 text-zinc-300">{trustReadiness.summary}</p>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {trustReadiness.review.pending_execution_reviews} execution reviews
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {trustReadiness.review.pending_writeback_count} pending writebacks
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {trustReadiness.review.conflict_count} conflicts
        </span>
      </div>
      {trustReadiness.follow_through.length > 0 ? (
        <div className="mt-3 space-y-2">
          {trustReadiness.follow_through.map((item) => (
            <div key={item.id} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
              <p className="text-sm font-medium text-zinc-100">{item.title}</p>
              <p className="mt-1 text-xs text-zinc-400">{item.summary}</p>
            </div>
          ))}
        </div>
      ) : null}
    </article>
  );
}

function QueuePressureSummary({
  reviewSnapshot,
  actionItems,
  threadAttentionCount,
}: {
  reviewSnapshot: ReviewSnapshotData;
  actionItems: ActionItemData[];
  threadAttentionCount: number;
}) {
  return (
    <article className="rounded-2xl border border-zinc-800 bg-zinc-950/50 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-300">
          Waiting elsewhere
        </span>
      </div>
      <p className="mt-3 text-sm leading-6 text-zinc-300">
        Keep `Now` lightweight. Use `Inbox` for explicit triage and `Threads` for longer context, history, and follow-up.
      </p>
      <div className="mt-3 flex flex-wrap gap-2 text-xs text-zinc-400">
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {reviewSnapshot.triage_count} waiting for Inbox triage
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {threadAttentionCount} need continuity
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
          {reviewSnapshot.projects_needing_review} projects need review
        </span>
      </div>
      {actionItems.length > 0 ? (
        <ul className="mt-3 space-y-2">
          {actionItems.map((item) => (
            <li key={item.id} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
              <div className="flex flex-wrap items-center gap-2">
                <span className="text-sm font-medium text-zinc-100">{item.title}</span>
                <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] uppercase tracking-[0.16em] text-zinc-500">
                  {formatActionKind(item.kind)}
                </span>
                {item.project_label ? (
                  <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2 py-0.5 text-[11px] text-emerald-300">
                    {item.project_label}
                  </span>
                ) : null}
              </div>
              <p className="mt-1 text-xs text-zinc-400">{item.summary}</p>
            </li>
          ))}
        </ul>
      ) : null}
    </article>
  );
}

function DebugStatePanel({ debug }: { debug: NowData['debug'] }) {
  return (
    <Panel
      title="Debug"
      subtitle="Secondary raw fields for verification and troubleshooting. Hidden from the main Now surface."
    >
      <details className="rounded-xl border border-zinc-800 bg-zinc-950/40 p-4">
        <summary className="cursor-pointer list-none text-sm font-medium text-zinc-100">
          Show raw fields
        </summary>
        <pre className="mt-4 overflow-x-auto rounded-lg bg-zinc-950 p-4 text-xs text-zinc-300">
          {JSON.stringify(debug, null, 2)}
        </pre>
      </details>
    </Panel>
  );
}

function DailyLoopStandupSummary({
  title,
  emptyMessage,
  items,
  tone = 'default',
}: {
  title: string;
  emptyMessage: string;
  items: string[];
  tone?: 'default' | 'success';
}) {
  return (
    <div>
      <p className={`text-xs uppercase tracking-[0.18em] ${tone === 'success' ? 'text-emerald-200/80' : 'text-zinc-500'}`}>
        {title}
      </p>
      {items.length === 0 ? (
        <p className={`mt-2 text-sm ${tone === 'success' ? 'text-emerald-50/80' : 'text-zinc-400'}`}>{emptyMessage}</p>
      ) : (
        <ul className={`mt-2 space-y-2 text-sm ${tone === 'success' ? 'text-emerald-50' : 'text-zinc-300'}`}>
          {items.map((item) => (
            <li key={`${title}-${item}`} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
              {item}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

function FreshnessBanner({
  freshness,
  pendingActions,
  actionMessages,
  onRunAction,
}: {
  freshness: NowData['freshness'];
  pendingActions: Record<string, true>;
  actionMessages: Record<string, { status: 'success' | 'error'; message: string }>;
  onRunAction: (source: NowData['freshness']['sources'][number]) => void;
}) {
  const degraded = freshness.sources.filter((source) => isDegraded(source.status));
  if (degraded.length === 0) {
    return null;
  }

  const summary = degraded
    .map((source) => {
      return `${source.label}: ${labelFreshness(source.status)}`
    })
    .join(' • ');

  return (
    <div className="mb-6 rounded-2xl border border-amber-700/40 bg-amber-950/25 px-4 py-3">
      <p className="text-sm font-medium text-amber-100">
        Some inputs need a refresh. Keep this snapshot visible, then refresh the source you need before acting.
      </p>
      <p className="mt-1 text-xs text-amber-200/80">{summary}</p>
      <div className="mt-3 space-y-2">
        {degraded.map((source) => (
          <FreshnessActionControls
            key={source.key}
            source={source}
            pendingActions={pendingActions}
            actionMessages={actionMessages}
            onRunAction={onRunAction}
          />
        ))}
      </div>
    </div>
  );
}

function FreshnessNotice({
  source,
  message,
  pendingActions,
  actionMessages,
  onRunAction,
}: {
  source: NowData['freshness']['sources'][number] | undefined;
  message: Partial<Record<string, string>>;
  pendingActions: Record<string, true>;
  actionMessages: Record<string, { status: 'success' | 'error'; message: string }>;
  onRunAction: (source: NowData['freshness']['sources'][number]) => void;
}) {
  if (!source || !isDegraded(source.status)) {
    return null;
  }

  const copy = message[source.status] ?? `${source.label} is ${labelFreshness(source.status).toLowerCase()}.`;
  return (
    <div className="mb-4 rounded-xl border border-amber-700/30 bg-amber-950/20 px-3 py-2">
      <p className="text-sm text-amber-100">{copy}</p>
      <FreshnessActionControls
        source={source}
        pendingActions={pendingActions}
        actionMessages={actionMessages}
        onRunAction={onRunAction}
      />
    </div>
  );
}

function FreshnessActionControls({
  source,
  pendingActions,
  actionMessages,
  onRunAction,
  compact = false,
}: {
  source: NowData['freshness']['sources'][number];
  pendingActions: Record<string, true>;
  actionMessages: Record<string, { status: 'success' | 'error'; message: string }>;
  onRunAction: (source: NowData['freshness']['sources'][number]) => void;
  compact?: boolean;
}) {
  const action = actionForFreshnessSource(source);
  const feedback = actionMessages[source.key];
  if (!action && !feedback) {
    return null;
  }

  return (
    <div className={compact ? 'mt-2' : 'mt-3'}>
      {action ? (
        <button
          type="button"
          onClick={() => onRunAction(source)}
          disabled={Boolean(pendingActions[source.key])}
          className="rounded-md border border-amber-700/70 px-3 py-1.5 text-xs font-medium text-amber-100 transition hover:border-amber-500 hover:text-white disabled:cursor-not-allowed disabled:border-amber-900/40 disabled:text-amber-300/50"
        >
          {pendingActions[source.key] && action.type !== 'open_settings' ? action.pendingLabel : action.label}
        </button>
      ) : null}
      {feedback ? (
        <p className={`mt-2 text-xs ${feedback.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
          {feedback.message}
        </p>
      ) : null}
    </div>
  );
}

function TaskCard({ task, timezone }: { task: NowTaskData; timezone: string }) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
      <p className="text-sm font-medium text-zinc-100">{task.text}</p>
      <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-400">
        <span>{task.project ?? 'No project'}</span>
        {task.due_at ? <span>due {formatDateTime(task.due_at, timezone)}</span> : null}
        {task.commitment_kind ? <span>{task.commitment_kind}</span> : null}
      </div>
    </div>
  );
}

function EventSummaryCard({
  event,
  timezone,
  nowTs,
  emphasis,
}: {
  event: NowData['schedule']['upcoming_events'][number];
  timezone: string;
  nowTs: number;
  emphasis: 'current' | 'next';
}) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-950/50 p-4">
      <div className="flex flex-wrap items-center gap-2">
        <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-400">
          {emphasis === 'current' ? 'In progress' : 'Calendar'}
        </span>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
          {formatEventSummary(event, timezone, nowTs)}
        </span>
      </div>
      <h3 className="mt-3 text-lg font-medium text-zinc-100">{event.title}</h3>
      <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-400">
        {event.location ? <span>{event.location}</span> : null}
        {event.end_ts ? <span>ends {formatTime(event.end_ts, timezone)}</span> : null}
        {event.prep_minutes != null ? <span>prep {event.prep_minutes}m</span> : null}
        {event.travel_minutes != null ? <span>travel {event.travel_minutes}m</span> : null}
        {event.leave_by_ts ? <span>leave by {formatTime(event.leave_by_ts, timezone)}</span> : null}
      </div>
    </div>
  );
}

function TodayLaneCard({
  label,
  children,
  emptyMessage,
}: {
  label: string;
  children: ReactNode;
  emptyMessage: string;
}) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-950/40 p-4">
      <p className="text-[11px] uppercase tracking-[0.18em] text-zinc-500">{label}</p>
      <div className="mt-3">{children ?? <SurfaceState message={emptyMessage} />}</div>
    </div>
  );
}

function TodayLaneGroup({
  title,
  subtitle,
  emptyMessage,
  children,
}: {
  title: string;
  subtitle: string;
  emptyMessage: string;
  children: ReactNode;
}) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-950/40 p-4">
      <div className="mb-3">
        <p className="text-sm font-medium text-zinc-100">{title}</p>
        <p className="mt-1 text-xs text-zinc-500">{subtitle}</p>
      </div>
      <div className="space-y-3">
        {children && Children.count(children) > 0 ? children : <SurfaceState message={emptyMessage} />}
      </div>
    </div>
  );
}

function TodayEventRow({
  event,
  timezone,
  nowTs,
}: {
  event: NowData['schedule']['upcoming_events'][number];
  timezone: string;
  nowTs: number;
}) {
  return (
    <div className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-4">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium text-zinc-100">{event.title}</p>
          <p className="mt-1 text-xs text-zinc-500">{formatEventSummary(event, timezone, nowTs)}</p>
        </div>
        {event.location ? <span className="text-xs text-zinc-500">{event.location}</span> : null}
      </div>
    </div>
  );
}

function TodayCommitmentRow({
  task,
  timezone,
  pending,
  feedback,
  onComplete,
  emphasis = 'default',
}: {
  task: NowTaskData;
  timezone: string;
  pending: boolean;
  feedback?: { status: 'success' | 'error'; message: string };
  onComplete: () => void;
  emphasis?: 'primary' | 'default';
}) {
  return (
    <div
      className={`rounded-lg border p-4 ${
        emphasis === 'primary'
          ? 'border-emerald-700/40 bg-emerald-950/10'
          : 'border-zinc-800 bg-zinc-900/70'
      }`}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium text-zinc-100">{task.text}</p>
          <div className="mt-1 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">
            <span>{task.project ?? 'No project'}</span>
            {task.due_at ? <span>due {formatDateTime(task.due_at, timezone)}</span> : null}
            {task.commitment_kind ? <span>{task.commitment_kind}</span> : null}
          </div>
        </div>
        <button
          type="button"
          onClick={onComplete}
          disabled={pending}
          className="rounded-md border border-emerald-700/70 px-3 py-1.5 text-xs font-medium text-emerald-100 transition hover:border-emerald-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-500"
        >
          {pending ? 'Saving…' : 'Complete'}
        </button>
      </div>
      {feedback ? (
        <p className={`mt-2 text-xs ${feedback.status === 'error' ? 'text-rose-300' : 'text-emerald-300'}`}>
          {feedback.message}
        </p>
      ) : null}
    </div>
  );
}

function TodayTaskRow({ task, timezone }: { task: NowTaskData; timezone: string }) {
  return (
    <div className="rounded-lg border border-zinc-800 bg-zinc-900/40 p-4">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium text-zinc-200">{task.text}</p>
          <div className="mt-1 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">
            <span>{task.project ?? 'No project'}</span>
            {task.due_at ? <span>due {formatDateTime(task.due_at, timezone)}</span> : null}
            {task.commitment_kind ? <span>{task.commitment_kind}</span> : null}
          </div>
        </div>
        <span className="rounded-full border border-zinc-800 bg-zinc-950/70 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-500">
          Task
        </span>
      </div>
    </div>
  );
}

function ActionItemRow({ item, timezone }: { item: ActionItemData; timezone: string }) {
  const executionHandoffEvidence = item.evidence.find(
    (evidence) => evidence.source_kind === 'execution_handoff',
  );
  return (
    <article className="rounded-xl border border-zinc-800 bg-zinc-950/70 p-4">
      <div className="flex flex-col gap-3 lg:flex-row lg:items-start lg:justify-between">
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <span className="rounded-full border border-zinc-700 bg-zinc-950 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-zinc-400">
              {formatActionKind(item.kind)}
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-zinc-500">
              Rank {item.rank}
            </span>
            {executionHandoffEvidence ? (
              <span className="rounded-full border border-amber-700/60 bg-amber-950/30 px-2.5 py-1 text-[11px] uppercase tracking-[0.18em] text-amber-100">
                Execution review
              </span>
            ) : null}
            {item.project_id ? (
              <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-emerald-300">
                {item.project_id}
              </span>
            ) : null}
          </div>
          <h3 className="mt-3 text-lg font-medium text-zinc-100">{item.title}</h3>
          <p className="mt-2 text-sm leading-6 text-zinc-300">{item.summary}</p>
          {executionHandoffEvidence?.detail ? (
            <p className="mt-2 text-sm text-amber-200">{executionHandoffEvidence.detail}</p>
          ) : null}
        </div>
        <div className="shrink-0 text-sm text-zinc-500">
          Surfaced {formatRfc3339(item.surfaced_at, timezone)}
        </div>
      </div>
      <div className="mt-3 flex flex-wrap gap-2">
        {item.evidence.length === 0 ? (
          <span className="rounded-full border border-zinc-800 bg-zinc-900/70 px-2.5 py-1 text-xs text-zinc-500">
            Evidence pending
          </span>
        ) : (
          item.evidence.map((evidence) => (
            <span
              key={`${item.id}-${evidence.source_id}-${evidence.label}`}
              className={`rounded-full border px-2.5 py-1 text-xs ${
                evidence.source_kind === 'person'
                  ? 'border-emerald-700/60 bg-emerald-950/30 text-emerald-200'
                  : evidence.source_kind === 'execution_handoff'
                    ? 'border-amber-700/60 bg-amber-950/30 text-amber-100'
                  : 'border-zinc-800 bg-zinc-900/70 text-zinc-400'
              }`}
            >
              {evidence.label}
            </span>
          ))
        )}
      </div>
    </article>
  );
}

function SourceActivityCard({
  title,
  timestamp,
  timezone,
  lines,
}: {
  title: string;
  timestamp: number;
  timezone: string;
  lines: string[];
}) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
      <div className="flex items-start justify-between gap-3">
        <p className="text-sm font-medium text-zinc-100">{title}</p>
        <p className="text-xs text-zinc-500">{formatTimestamp(timestamp, timezone)}</p>
      </div>
      <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-400">
        {lines.map((line) => (
          <span key={line}>{line}</span>
        ))}
      </div>
    </div>
  );
}

function Panel({
  title,
  subtitle,
  children,
}: {
  title: string;
  subtitle: string;
  children: ReactNode;
}) {
  return (
    <section className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-5">
      <div className="mb-4">
        <h2 className="text-lg font-medium text-zinc-100">{title}</h2>
        <p className="mt-1 text-sm text-zinc-500">{subtitle}</p>
      </div>
      {children}
    </section>
  );
}

function formatNowHeaderBucketLabel(kind: string): string {
  switch (kind) {
    case 'threads_by_type':
      return 'Threads';
    case 'needs_input':
      return 'Needs input';
    case 'new_nudges':
      return 'Nudges';
    case 'search_filter':
      return 'Search';
    case 'snoozed':
      return 'Snoozed';
    case 'review_apply':
      return 'Review';
    case 'reflow':
      return 'Reflow';
    default:
      return 'Follow-up';
  }
}

function formatNowHeaderBucketCount(bucket: NonNullable<NowData['header']>['buckets'][number]): string {
  if (bucket.count_display === 'always_show') {
    return String(bucket.count);
  }
  if (bucket.count_display === 'show_nonzero') {
    return bucket.count > 0 ? String(bucket.count) : '';
  }
  if (bucket.count_display === 'hidden_until_active') {
    return bucket.urgent || bucket.count > 0 ? String(bucket.count) : '';
  }
  return '';
}

function formatNowMeshSyncState(state: NonNullable<NowData['mesh_summary']>['sync_state']): string {
  switch (state) {
    case 'local_only':
      return 'Local only';
    case 'offline':
      return 'Offline';
    case 'stale':
      return 'Stale';
    default:
      return 'Synced';
  }
}

function formatNowBarKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}

function FocusCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-2xl border border-zinc-800 bg-zinc-900/70 p-4">
      <p className="text-xs uppercase tracking-wide text-zinc-500">{label}</p>
      <p className="mt-2 text-xl font-medium text-zinc-100">{value}</p>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-start justify-between gap-3">
      <dt className="text-zinc-500">{label}</dt>
      <dd className="text-right text-zinc-100">{value}</dd>
    </div>
  );
}

function formatTimestamp(timestamp: number, timezone: string): string {
  return new Date(timestamp * 1000).toLocaleString(undefined, { timeZone: timezone });
}

function formatTime(timestamp: number, timezone: string): string {
  return new Date(timestamp * 1000).toLocaleTimeString(undefined, {
    timeZone: timezone,
    hour: 'numeric',
    minute: '2-digit',
  });
}

function formatDateTime(value: string, timezone: string): string {
  return new Date(value).toLocaleString(undefined, { timeZone: timezone });
}

function formatRfc3339(value: string, timezone: string): string {
  return new Date(value).toLocaleString(undefined, { timeZone: timezone });
}

function formatSessionDate(timestamp: number, timezone: string): string {
  return new Intl.DateTimeFormat('en-CA', {
    timeZone: timezone,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  }).format(new Date(timestamp * 1000));
}

function formatDailyLoopPhase(phase: DailyLoopPhaseData | string | undefined): string {
  return phase === 'morning_overview' ? 'Morning overview' : 'Standup';
}

function formatDailyLoopStatus(status: DailyLoopSessionData['status'] | string | undefined): string {
  return (status ?? 'unknown').replaceAll('_', ' ');
}

function formatStandupBucket(bucket: string): string {
  return bucket.toUpperCase();
}

function formatActionKind(kind: string): string {
  return kind.replaceAll('_', ' ');
}

function findFreshnessSource(data: NowData, key: string) {
  return data.freshness.sources.find((source) => source.key === key);
}

function dedupeTasks(tasks: Array<NowTaskData | null | undefined>): NowTaskData[] {
  const seen = new Set<string>();
  return tasks.filter((task): task is NowTaskData => {
    if (!task || seen.has(task.id)) {
      return false;
    }
    seen.add(task.id);
    return true;
  });
}

function dedupeActionItems(items: ActionItemData[]): ActionItemData[] {
  const seen = new Set<string>();
  return items.filter((item) => {
    const dedupeKey = [
      item.kind,
      item.title.trim().toLowerCase(),
      item.summary.trim().toLowerCase(),
      item.project_label?.trim().toLowerCase() ?? '',
      item.thread_route?.thread_id ?? '',
      item.thread_route?.label.trim().toLowerCase() ?? '',
    ].join('::');
    if (seen.has(dedupeKey)) {
      return false;
    }
    seen.add(dedupeKey);
    return true;
  });
}

function findActiveEvent(events: NowData['schedule']['upcoming_events'], nowTs: number) {
  return events.find((event) => {
    const endTs = event.end_ts ?? event.start_ts;
    return event.start_ts <= nowTs && endTs >= nowTs;
  }) ?? null;
}

function findNextEvent(events: NowData['schedule']['upcoming_events'], nowTs: number) {
  return events.find((event) => event.start_ts > nowTs) ?? null;
}

function findActiveRoutineBlock(dayPlan: NowData['day_plan'], nowTs: number) {
  return dayPlan?.routine_blocks.find((block) => block.start_ts <= nowTs && block.end_ts >= nowTs) ?? null;
}

function deriveInferredActivity(data: NowData): { title: string; detail: string } | null {
  if (data.sources.git_activity) {
    return {
      title: 'Likely working from recent activity',
      detail: sourceSummaryLines(data.sources.git_activity.summary, ['repo', 'operation'])[0] ?? 'Git activity is the strongest recent signal.',
    };
  }
  if (data.sources.note_document) {
    return {
      title: 'Likely in note work',
      detail: sourceSummaryLines(data.sources.note_document.summary, ['title', 'path'])[0] ?? 'A recent note is the strongest signal.',
    };
  }
  return null;
}

function buildCurrentStatus(
  data: NowData,
  nowTs: number,
  activeEvent: NowData['schedule']['upcoming_events'][number] | null,
  activeRoutineBlock: RoutineBlockData | null,
  currentCommitment: NowTaskData | null,
  nextEvent: NowData['schedule']['upcoming_events'][number] | null,
) {
  if (activeEvent) {
    return {
      kind: 'Calendar',
      title: activeEvent.title,
      detail: activeEvent.location ?? formatTime(activeEvent.start_ts, data.timezone),
      subtitle: 'What is happening now takes precedence over everything else.',
      summary: `You are in ${activeEvent.title}${activeEvent.location ? ` at ${activeEvent.location}` : ''}.`,
      fallbackEventMessage: null,
    };
  }
  if (currentCommitment) {
    return {
      kind: 'Commitment',
      title: currentCommitment.text,
      detail: currentCommitment.project ?? 'No project',
      subtitle: 'No calendar event is active, so the current commitment becomes the execution anchor.',
      summary: `Current commitment: ${currentCommitment.text}.`,
      fallbackEventMessage: null,
    };
  }
  if (activeRoutineBlock) {
    return {
      kind: 'Routine',
      title: activeRoutineBlock.label,
      detail: activeRoutineBlock.source.replaceAll('_', ' '),
      subtitle: 'Routine stays visible when it is active, but it does not replace calendar truth.',
      summary: `Routine block in progress: ${activeRoutineBlock.label}.`,
      fallbackEventMessage: null,
    };
  }
  const inferred = deriveInferredActivity(data);
  if (inferred) {
    return {
      kind: 'Inference',
      title: inferred.title,
      detail: inferred.detail,
      subtitle: 'Inference only shows when no stronger declared structure is active.',
      summary: inferred.detail,
      fallbackEventMessage: null,
    };
  }
  if (nextEvent) {
    return {
      kind: 'Free',
      title: `Free until ${formatTime(nextEvent.start_ts, data.timezone)}`,
      detail: nextEvent.title,
      subtitle: 'Nothing explicit is active right now, so the next event sets the edge of free time.',
      summary: `Free until ${nextEvent.title} at ${formatTime(nextEvent.start_ts, data.timezone)}.`,
      fallbackEventMessage: `Free until ${nextEvent.title} at ${formatTime(nextEvent.start_ts, data.timezone)}.`,
    };
  }
  return {
    kind: 'Between blocks',
    title: 'Between blocks',
    detail: 'No event, commitment, or strong routine signal is active.',
    subtitle: 'When Vel has no stronger current-day structure, it should say so plainly.',
    summary: 'Nothing stronger is active right now.',
    fallbackEventMessage: 'No more calendar events are scheduled right now.',
  };
}

function formatEventSummary(
  event: NowData['schedule']['upcoming_events'][number],
  timezone: string,
  nowTs: number,
): string {
  const active = event.start_ts <= nowTs && (event.end_ts ?? event.start_ts) >= nowTs;
  if (active) {
    return `Now · started ${formatTime(event.start_ts, timezone)}`;
  }
  return formatTime(event.start_ts, timezone);
}

function hasSourceActivity(data: NowData): boolean {
  return Boolean(
    data.sources.git_activity
      || data.sources.health
      || data.sources.mood
      || data.sources.pain
      || data.sources.note_document
      || data.sources.assistant_message,
  );
}

function peopleNeedingReview(data: NowData) {
  const personIds = new Set(
    (data.action_items ?? [])
      .flatMap((item) => item.evidence)
      .filter((evidence) => evidence.source_kind === 'person')
      .map((evidence) => evidence.source_id),
  );
  return (data.people ?? []).filter((person) => personIds.has(person.id));
}

function sourceSummaryLines(summary: unknown, keys: string[]): string[] {
  if (!summary || typeof summary !== 'object') {
    return [];
  }
  const record = summary as Record<string, unknown>;
  return keys
    .map((key) => {
      const value = record[key];
      if (typeof value === 'string' && value.length > 0) {
        return `${key.replaceAll('_', ' ')}: ${value}`;
      }
      if (typeof value === 'number' || typeof value === 'boolean') {
        return `${key.replaceAll('_', ' ')}: ${value}`;
      }
      return null;
    })
    .filter((value): value is string => value !== null);
}

function isDegraded(status: string): boolean {
  return ['aging', 'stale', 'error', 'disconnected', 'missing'].includes(status);
}

function actionForFreshnessSource(source: NowData['freshness']['sources'][number]):
  | { type: 'evaluate'; label: string; pendingLabel: string }
  | {
      type: 'sync';
      source: 'calendar' | 'todoist' | 'activity' | 'messaging';
      label: string;
      pendingLabel: string;
      successLabel: string;
    }
  | {
      type: 'open_settings';
      integrationId: SettingsIntegrationTarget;
      label: string;
      pendingLabel: string;
    }
  | null {
  const guidance = source.guidance?.toLowerCase() ?? '';
  if (source.key === 'context') {
    return { type: 'evaluate', label: 'Re-run evaluate', pendingLabel: 'Re-running…' };
  }

  if (guidance.includes('configure a source path')) {
    return {
      type: 'open_settings',
      integrationId: source.key as Extract<SettingsIntegrationTarget, 'activity' | 'git' | 'messaging' | 'notes' | 'transcripts'>,
      label: 'Open source settings',
      pendingLabel: 'Open source settings',
    };
  }

  if (
    guidance.includes('save a todoist api token')
    || guidance.includes('save a todoist token')
    || guidance.includes('save a todoist')
  ) {
    return {
      type: 'open_settings',
      integrationId: 'todoist',
      label: 'Open Todoist settings',
      pendingLabel: 'Open Todoist settings',
    };
  }

  if (
    guidance.includes('save a google')
    || guidance.includes('save credentials')
    || guidance.includes('connect google')
  ) {
    return {
      type: 'open_settings',
      integrationId: 'google',
      label: 'Open Google settings',
      pendingLabel: 'Open Google settings',
    };
  }

  switch (source.key) {
    case 'calendar':
      if (source.status === 'disconnected') {
        return {
          type: 'open_settings',
          integrationId: 'google',
          label: 'Open Google settings',
          pendingLabel: 'Open Google settings',
        };
      }
      return {
        type: 'sync',
        source: 'calendar',
        label: 'Sync calendar',
        pendingLabel: 'Syncing calendar…',
        successLabel: 'Calendar',
      };
    case 'todoist':
      if (source.status === 'disconnected') {
        return {
          type: 'open_settings',
          integrationId: 'todoist',
          label: 'Open Todoist settings',
          pendingLabel: 'Open Todoist settings',
        };
      }
      return {
        type: 'sync',
        source: 'todoist',
        label: 'Sync Todoist',
        pendingLabel: 'Syncing Todoist…',
        successLabel: 'Todoist',
      };
    case 'activity':
      return {
        type: 'sync',
        source: 'activity',
        label: 'Sync activity',
        pendingLabel: 'Syncing activity…',
        successLabel: 'Activity',
      };
    case 'messaging':
      return {
        type: 'sync',
        source: 'messaging',
        label: 'Sync messaging',
        pendingLabel: 'Syncing messaging…',
        successLabel: 'Messaging',
      };
    default:
      return null;
  }
}

function freshnessClass(status: string): string {
  switch (status) {
    case 'fresh':
      return 'bg-emerald-900/40 text-emerald-200';
    case 'aging':
      return 'bg-yellow-900/40 text-yellow-200';
    case 'unchecked':
      return 'bg-zinc-700/60 text-zinc-100';
    case 'error':
    case 'stale':
      return 'bg-rose-900/40 text-rose-200';
    default:
      return 'bg-zinc-800 text-zinc-200';
  }
}

function labelFreshness(status: string): string {
  switch (status) {
    case 'fresh':
      return 'Fresh';
    case 'aging':
      return 'Aging';
    case 'stale':
      return 'Stale';
    case 'error':
      return 'Error';
    case 'disconnected':
      return 'Disconnected';
    case 'missing':
      return 'Missing';
    case 'unchecked':
      return 'Unchecked';
    default:
      return status;
  }
}

function extractMessageText(message: AssistantEntryResponse['assistant_message']): string | null {
  if (!message || typeof message.content !== 'object' || message.content === null || Array.isArray(message.content)) {
    return null;
  }
  return typeof message.content.text === 'string' ? message.content.text : null;
}

function isRenderableDailyLoopSession(value: DailyLoopSessionData | null): value is DailyLoopSessionData {
  return Boolean(
    value
      && typeof value.phase === 'string'
      && typeof value.status === 'string'
      && value.state
      && typeof value.state === 'object'
      && 'phase' in value.state,
  );
}
