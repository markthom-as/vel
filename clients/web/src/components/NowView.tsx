import { useEffect, useMemo, useState, type ReactNode } from 'react';
import { contextQueryKeys, loadNow } from '../data/context';
import { operatorQueryKeys, runEvaluate, syncSource } from '../data/operator';
import { invalidateQuery, useQuery } from '../data/query';
import type { ActionItemData, NowData, NowTaskData } from '../types';
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
  onOpenSettings?: (target: { tab: 'integrations'; integrationId: SettingsIntegrationTarget }) => void;
}

export function NowView({ onOpenSettings }: NowViewProps) {
  const nowKey = useMemo(() => contextQueryKeys.now(), []);
  const currentContextKey = useMemo(() => contextQueryKeys.currentContext(), []);
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const { data, loading, error, refetch } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );
  const [pendingActions, setPendingActions] = useState<Record<string, true>>({});
  const [actionMessages, setActionMessages] = useState<Record<string, { status: 'success' | 'error'; message: string }>>({});

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
  };
  const pendingWritebacks = data.pending_writebacks ?? [];
  const conflicts = data.conflicts ?? [];
  const peopleReview = peopleNeedingReview(data);

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

        <Panel
          title="Action stack"
          subtitle="Ranked actions derived from persisted evidence and the current review snapshot"
        >
          <div className="mb-4 flex flex-wrap gap-2 text-xs text-zinc-400">
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {reviewSnapshot.open_action_count} open actions
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {reviewSnapshot.triage_count} waiting for Inbox triage
            </span>
            <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1">
              {reviewSnapshot.projects_needing_review} projects need review
            </span>
          </div>
          {actionItems.length === 0 ? (
            <SurfaceState message="No ranked actions yet. Re-run evaluate after syncing sources." />
          ) : (
            <div className="space-y-3">
              {actionItems.map((item) => (
                <ActionItemRow key={item.id} item={item} timezone={data.timezone} />
              ))}
            </div>
          )}
        </Panel>

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
                  aging: 'Calendar is aging. Confirm event timing before acting on it.',
                  stale: 'Calendar is stale. Upcoming events may be out of date.',
                  error: 'Calendar sync last failed. Treat this schedule as degraded.',
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
                  aging: 'Todoist is aging. Task ordering may lag behind recent changes.',
                  stale: 'Todoist is stale. Open tasks may not reflect current urgency.',
                  error: 'Todoist sync last failed. Backlog state may be incomplete.',
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
                  aging: 'Current context is aging. Evaluate soon if you need fresher state.',
                  stale: 'Current context is stale. Re-run evaluate before trusting this view.',
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
      const guidance = source.guidance ? ` · ${source.guidance}` : ''
      return `${source.label}: ${labelFreshness(source.status)}${guidance}`
    })
    .join(' • ');

  return (
    <div className="mb-6 rounded-2xl border border-amber-700/50 bg-amber-950/40 px-4 py-3">
      <p className="text-sm font-medium text-amber-100">
        Some inputs are degraded. Keep the current snapshot visible, but verify before acting.
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
    <div className="mb-4 rounded-xl border border-amber-700/40 bg-amber-950/30 px-3 py-2">
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
            {item.project_id ? (
              <span className="rounded-full border border-zinc-800 bg-zinc-950/80 px-2.5 py-1 text-xs text-emerald-300">
                {item.project_id}
              </span>
            ) : null}
          </div>
          <h3 className="mt-3 text-lg font-medium text-zinc-100">{item.title}</h3>
          <p className="mt-2 text-sm leading-6 text-zinc-300">{item.summary}</p>
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
