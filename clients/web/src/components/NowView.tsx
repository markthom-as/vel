import { useMemo, type ReactNode } from 'react';
import { useQuery } from '../data/query';
import { loadNow, queryKeys } from '../data/resources';
import type { NowData, NowTaskData } from '../types';
import { SurfaceState } from './SurfaceState';

export function NowView() {
  const nowKey = useMemo(() => queryKeys.now(), []);
  const { data, loading, error } = useQuery<NowData | null>(
    nowKey,
    async () => {
      const response = await loadNow();
      return response.ok ? response.data ?? null : null;
    },
  );

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
            Updated {new Date(data.computed_at * 1000).toLocaleString()}
          </p>
        </header>

        <FreshnessBanner freshness={data.freshness} />

        <section className="grid gap-4 md:grid-cols-4">
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
              />
              {data.schedule.upcoming_events.length === 0 ? (
                <SurfaceState message="No upcoming calendar events in the current stream." />
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
          </div>
        </section>
      </div>
    </div>
  );
}

function FreshnessBanner({ freshness }: { freshness: NowData['freshness'] }) {
  const degraded = freshness.sources.filter((source) => isDegraded(source.status));
  if (degraded.length === 0) {
    return null;
  }

  const summary = degraded
    .map((source) => `${source.label}: ${labelFreshness(source.status)}`)
    .join(' • ');

  return (
    <div className="mb-6 rounded-2xl border border-amber-700/50 bg-amber-950/40 px-4 py-3">
      <p className="text-sm font-medium text-amber-100">
        Some inputs are degraded. Keep the current snapshot visible, but verify before acting.
      </p>
      <p className="mt-1 text-xs text-amber-200/80">{summary}</p>
    </div>
  );
}

function FreshnessNotice({
  source,
  message,
}: {
  source: NowData['freshness']['sources'][number] | undefined;
  message: Partial<Record<string, string>>;
}) {
  if (!source || !isDegraded(source.status)) {
    return null;
  }

  const copy = message[source.status] ?? `${source.label} is ${labelFreshness(source.status).toLowerCase()}.`;
  return (
    <div className="mb-4 rounded-xl border border-amber-700/40 bg-amber-950/30 px-3 py-2">
      <p className="text-sm text-amber-100">{copy}</p>
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

function findFreshnessSource(data: NowData, key: string) {
  return data.freshness.sources.find((source) => source.key === key);
}

function isDegraded(status: string): boolean {
  return ['aging', 'stale', 'error', 'disconnected', 'missing'].includes(status);
}

function freshnessClass(status: string): string {
  switch (status) {
    case 'fresh':
      return 'bg-emerald-900/40 text-emerald-200';
    case 'aging':
      return 'bg-yellow-900/40 text-yellow-200';
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
    default:
      return status;
  }
}
