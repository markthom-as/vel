import { useEffect, useMemo, useState, type ReactNode } from 'react';
import {
  contextQueryKeys,
  loadActiveDailyLoopSession,
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
  const [recentCompletedDailyLoop, setRecentCompletedDailyLoop] = useState<DailyLoopSessionData | null>(null);
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

  const actionItems = [...(data.action_items ?? [])]
    .filter((item) => item.surface === 'now')
    .sort((left, right) => left.rank - right.rank);
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

  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-5xl px-6 py-8">
        <header className="mb-8">
          <div className="flex items-center justify-between gap-3">
            <div>
              <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Now</p>
              <h1 className="mt-2 text-3xl font-semibold text-zinc-100">What matters right now</h1>
            </div>
            <span className={`rounded-full px-3 py-1 text-xs ${freshnessClass(data.freshness.overall_status)}`}>
              {labelFreshness(data.freshness.overall_status)}
            </span>
          </div>
          <p className="mt-2 text-sm text-zinc-400">
            Updated {formatTimestamp(data.computed_at, data.timezone)}
          </p>
        </header>

        <FreshnessBanner
          freshness={data.freshness}
          pendingActions={pendingActions}
          actionMessages={actionMessages}
          onRunAction={runFreshnessAction}
        />

        <section className="mb-8 grid gap-4 md:grid-cols-2 xl:grid-cols-5">
          <ContextStripCard
            label="Mode"
            value={data.summary.mode.label}
            detail={data.summary.phase.label}
          />
          <ContextStripCard
            label="Next event"
            value={data.schedule.next_event?.title ?? 'No event'}
            detail={data.schedule.next_event?.location ?? 'No location attached'}
          />
          <ContextStripCard
            label="Next commitment"
            value={data.tasks.next_commitment?.text ?? 'Nothing selected'}
            detail={data.tasks.next_commitment?.project ?? 'No project'}
          />
          <ContextStripCard
            label="Inbox pressure"
            value={`${reviewSnapshot.triage_count} waiting`}
            detail={`${reviewSnapshot.open_action_count} open actions`}
          />
          <ContextStripCard
            label="Threads"
            value={threadAttentionCount > 0 ? `${threadAttentionCount} need context` : 'Quiet'}
            detail={data.reflow_status?.headline ?? 'Archive and follow-ups live here'}
          />
        </section>

        <Panel
          title="Ask, capture, or talk"
          subtitle="Start from Now. Type or hold the mic to talk locally, then let Vel decide whether it belongs inline, in Inbox, or in Threads."
        >
          <div className="space-y-4">
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
            {assistantInlineResponse?.assistant_message ? (
              <div className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
                <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">Inline reply</p>
                <p className="mt-2 text-sm leading-6 text-zinc-200">
                  {extractMessageText(assistantInlineResponse.assistant_message) ?? 'Vel responded inline.'}
                </p>
              </div>
            ) : null}
            <MessageComposer
              onSent={(_, response) => {
                void handleAssistantEntry(response);
              }}
            />
          </div>
        </Panel>

        <Panel
          title="Immediate pressure"
          subtitle="Keep this view minimal. Handle only what needs attention now, then go deeper elsewhere."
        >
          <div className="space-y-4">
            {data.reflow ? <ReflowCardView reflow={data.reflow} /> : null}
            {data.reflow_status ? <ReflowStatusView status={data.reflow_status} timezone={data.timezone} /> : null}
            {data.check_in ? <CheckInCardView checkIn={data.check_in} /> : null}
            <TrustReadinessPanel trustReadiness={data.trust_readiness} timezone={data.timezone} />
            <QueuePressureSummary
              reviewSnapshot={reviewSnapshot}
              actionItems={summarizedActionItems}
              threadAttentionCount={threadAttentionCount}
            />
          </div>
        </Panel>

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

        <section className="mt-8 grid gap-4 md:grid-cols-4">
          <FocusCard label="Mode" value={data.summary.mode.label} />
          <FocusCard label="Phase" value={data.summary.phase.label} />
          <FocusCard label="Meds" value={data.summary.meds.label} />
          <FocusCard label="Risk" value={data.summary.risk.label} />
        </section>

        <section className="mt-8 grid gap-6 xl:grid-cols-[1.3fr_1fr]">
          <div className="space-y-6">
            <Panel title="Upcoming events" subtitle="Current schedule pulled from persisted calendar signals">
              <FreshnessNotice
                source={findFreshnessSource(data, 'calendar')}
                message={{
                  aging: 'Calendar is a bit behind. Confirm event timing before acting on it.',
                  stale: 'Calendar needs a refresh. Upcoming events may be out of date.',
                  error: 'Calendar sync last failed. Keep this schedule visible, but confirm details before relying on it.',
                  disconnected: 'Calendar is disconnected. Events shown here may be incomplete.',
                  missing: 'Calendar has not synced yet. This schedule may be empty.',
                }}
                pendingActions={pendingActions}
                actionMessages={actionMessages}
                onRunAction={runFreshnessAction}
              />
              {data.schedule.upcoming_events.length === 0 ? (
                <SurfaceState
                  message={
                    data.schedule.empty_message ?? 'No upcoming calendar events in the current stream.'
                  }
                />
              ) : (
                <div className="space-y-3">
                  {data.schedule.upcoming_events.map((event) => (
                    <div key={`${event.title}-${event.start_ts}`} className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <p className="text-base font-medium text-zinc-100">{event.title}</p>
                          {event.location ? <p className="mt-1 text-sm text-zinc-400">{event.location}</p> : null}
                        </div>
                        <p className="text-sm text-zinc-400">
                          {formatTimestamp(event.start_ts, data.timezone)}
                        </p>
                      </div>
                      <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-400">
                        {event.end_ts ? (
                          <span>ends {formatTimestamp(event.end_ts, data.timezone)}</span>
                        ) : null}
                        {event.prep_minutes != null ? <span>prep {event.prep_minutes}m</span> : null}
                        {event.travel_minutes != null ? <span>travel {event.travel_minutes}m</span> : null}
                        {event.leave_by_ts ? (
                          <span>leave by {formatTimestamp(event.leave_by_ts, data.timezone)}</span>
                        ) : null}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </Panel>

            <Panel title="Todoist backlog" subtitle="Open commitments synced from Todoist">
              <FreshnessNotice
                source={findFreshnessSource(data, 'todoist')}
                message={{
                  aging: 'Todoist is a bit behind. Task ordering may lag recent changes.',
                  stale: 'Todoist needs a refresh. Open tasks may not reflect current urgency.',
                  error: 'Todoist sync last failed. Keep the backlog visible, but refresh before trusting it.',
                  disconnected: 'Todoist is disconnected. This backlog may be missing tasks.',
                  missing: 'Todoist has not synced yet. No backlog can be trusted yet.',
                }}
                pendingActions={pendingActions}
                actionMessages={actionMessages}
                onRunAction={runFreshnessAction}
              />
              {data.tasks.todoist.length === 0 ? (
                <SurfaceState message="No open Todoist-backed commitments found." />
              ) : (
                <div className="space-y-3">
                  {data.tasks.todoist.map((task) => (
                    <TaskCard key={task.id} task={task} timezone={data.timezone} />
                  ))}
                </div>
              )}
            </Panel>

            <Panel title="Recent source activity" subtitle="Latest non-calendar signals shaping current context">
              {hasSourceActivity(data) ? (
                <div className="space-y-3">
                  {data.sources.git_activity ? (
                    <SourceActivityCard
                      title={data.sources.git_activity.label}
                      timestamp={data.sources.git_activity.timestamp}
                      timezone={data.timezone}
                      lines={sourceSummaryLines(data.sources.git_activity.summary, ['repo', 'branch', 'operation'])}
                    />
                  ) : null}
                  {data.sources.health ? (
                    <SourceActivityCard
                      title={data.sources.health.label}
                      timestamp={data.sources.health.timestamp}
                      timezone={data.timezone}
                      lines={sourceSummaryLines(data.sources.health.summary, ['metric_type', 'value', 'unit', 'source_app', 'device'])}
                    />
                  ) : null}
                  {data.sources.note_document ? (
                    <SourceActivityCard
                      title={data.sources.note_document.label}
                      timestamp={data.sources.note_document.timestamp}
                      timezone={data.timezone}
                      lines={sourceSummaryLines(data.sources.note_document.summary, ['title', 'path'])}
                    />
                  ) : null}
                  {data.sources.assistant_message ? (
                    <SourceActivityCard
                      title={data.sources.assistant_message.label}
                      timestamp={data.sources.assistant_message.timestamp}
                      timezone={data.timezone}
                      lines={sourceSummaryLines(data.sources.assistant_message.summary, ['conversation_id', 'role', 'source'])}
                    />
                  ) : null}
                </div>
              ) : (
                <SurfaceState message="No recent git, health, note, or transcript activity is attached to this snapshot." />
              )}
            </Panel>
          </div>

          <div className="space-y-6">
            <Panel title="Operational state" subtitle="What Vel currently believes">
              <FreshnessNotice
                source={findFreshnessSource(data, 'context')}
                message={{
                  aging: 'Current context is a bit behind. Re-run evaluate if you need fresher state.',
                  stale: 'Current context needs a refresh. Re-run evaluate before trusting this view.',
                  error: 'Current context is degraded. Re-run evaluate and inspect logs.',
                  disconnected: 'Current context is disconnected from a required source.',
                  missing: 'Current context has not been computed yet.',
                }}
                pendingActions={pendingActions}
                actionMessages={actionMessages}
                onRunAction={runFreshnessAction}
              />
              <dl className="space-y-3 text-sm">
                <Row
                  label="Next event"
                  value={
                    data.schedule.next_event
                      ? formatTimestamp(data.schedule.next_event.start_ts, data.timezone)
                      : 'None'
                  }
                />
                <Row
                  label="Leave by"
                  value={
                    data.schedule.next_event?.leave_by_ts
                      ? formatTimestamp(data.schedule.next_event.leave_by_ts, data.timezone)
                      : 'None'
                  }
                />
                <Row label="Attention" value={data.attention.state.label} />
                <Row label="Drift" value={data.attention.drift.label} />
                <Row
                  label="Next commitment"
                  value={data.tasks.next_commitment?.text ?? 'None'}
                />
                <Row
                  label="Pending writebacks"
                  value={String(pendingWritebacks.length)}
                />
                <Row
                  label="Open conflicts"
                  value={String(conflicts.length)}
                />
                <Row
                  label="People needing review"
                  value={String(peopleReview.length)}
                />
              </dl>
            </Panel>

            <Panel title="Why Vel thinks this" subtitle="Top context and attention drivers">
              <ul className="space-y-2">
                {data.reasons.length === 0 ? (
                  <SurfaceState message="No explanation reasons available." />
                ) : (
                  data.reasons.map((reason) => (
                    <li key={reason} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2 text-sm text-zinc-200">
                      {reason}
                    </li>
                  ))
                )}
              </ul>
            </Panel>

            <Panel title="Freshness" subtitle="How current each source is">
              <div className="space-y-2">
                {data.freshness.sources.map((source) => (
                  <div key={source.key} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                    <div className="flex items-center justify-between gap-3">
                      <p className="text-sm text-zinc-100">{source.label}</p>
                      <span className={`rounded-full px-2 py-0.5 text-[11px] ${freshnessClass(source.status)}`}>
                        {labelFreshness(source.status)}
                      </span>
                    </div>
                    {source.last_sync_at ? (
                      <p className="mt-1 text-xs text-zinc-500">
                        Last sync {formatTimestamp(source.last_sync_at, data.timezone)}
                      </p>
                    ) : null}
                    {source.guidance ? (
                      <p className="mt-1 text-xs text-amber-300">{source.guidance}</p>
                    ) : null}
                    <FreshnessActionControls
                      source={source}
                      pendingActions={pendingActions}
                      actionMessages={actionMessages}
                      onRunAction={runFreshnessAction}
                      compact
                    />
                  </div>
                ))}
              </div>
            </Panel>

            <Panel title="Other open commitments" subtitle="Non-Todoist open items still in play">
              {data.tasks.other_open.length === 0 ? (
                <SurfaceState message="No additional open commitments surfaced." />
              ) : (
                <div className="space-y-2">
                  {data.tasks.other_open.map((task) => (
                    <div key={task.id} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                      <p className="text-sm text-zinc-100">{task.text}</p>
                      <p className="mt-1 text-xs text-zinc-500">{task.source_type}</p>
                    </div>
                  ))}
                </div>
              )}
            </Panel>

            <Panel title="People status" subtitle="People-linked writes and reviews currently in scope">
              {peopleReview.length === 0 ? (
                <SurfaceState message="No person-linked review items are currently open." />
              ) : (
                <div className="space-y-2">
                  {peopleReview.map((person) => (
                    <div key={person.id} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                      <p className="text-sm text-zinc-100">{person.display_name}</p>
                      <p className="mt-1 text-xs text-zinc-500">
                        {person.aliases.length > 0
                          ? person.aliases.map((alias) => `${alias.platform}:${alias.handle}`).join(' • ')
                          : 'No alias provenance attached yet'}
                      </p>
                    </div>
                  ))}
                </div>
              )}
            </Panel>

            <Panel title="Debug" subtitle="Raw inputs behind this snapshot">
              <details className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                <summary className="cursor-pointer text-sm text-zinc-100">Show raw fields</summary>
                <pre className="mt-3 overflow-x-auto whitespace-pre-wrap text-xs text-zinc-300">
                  {JSON.stringify(data.debug, null, 2)}
                </pre>
              </details>
            </Panel>
          </div>
        </section>
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
      <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-500">
        <span>Recorded {formatTimestamp(status.recorded_at, timezone)}</span>
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
          {threadAttentionCount} need thread context
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
