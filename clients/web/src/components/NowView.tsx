import { useMemo, type ReactNode } from 'react';
import { useQuery } from '../data/query';
import {
  loadCommitments,
  loadContextExplain,
  loadCurrentContext,
  loadDriftExplain,
  queryKeys,
} from '../data/resources';
import type {
  CommitmentData,
  ContextExplainData,
  CurrentContextData,
  DriftExplainData,
  JsonObject,
  JsonValue,
} from '../types';

const COMMITMENT_LIMIT = 12;

export function NowView() {
  const currentContextKey = useMemo(() => queryKeys.currentContext(), []);
  const contextExplainKey = useMemo(() => queryKeys.contextExplain(), []);
  const driftExplainKey = useMemo(() => queryKeys.driftExplain(), []);
  const commitmentsKey = useMemo(() => queryKeys.commitments(COMMITMENT_LIMIT), []);

  const { data: currentContext, loading: contextLoading, error: contextError } = useQuery<CurrentContextData | null>(
    currentContextKey,
    async () => {
      const response = await loadCurrentContext();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: explainedContext, error: explainError } = useQuery<ContextExplainData | null>(
    contextExplainKey,
    async () => {
      const response = await loadContextExplain();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: drift, error: driftError } = useQuery<DriftExplainData | null>(
    driftExplainKey,
    async () => {
      const response = await loadDriftExplain();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: commitments = [], error: commitmentsError } = useQuery<CommitmentData[]>(
    commitmentsKey,
    async () => {
      const response = await loadCommitments(COMMITMENT_LIMIT);
      return response.ok && response.data ? response.data : [];
    },
  );

  if (contextLoading) {
    return <div className="flex-1 p-6 text-sm text-zinc-500">Loading your current state…</div>;
  }

  const error = contextError ?? explainError ?? driftError ?? commitmentsError;
  if (error) {
    return <div className="flex-1 p-6 text-sm text-amber-400">{error}</div>;
  }

  if (!currentContext) {
    return (
      <div className="flex-1 p-6 text-sm text-zinc-500">
        No current context yet. Sync integrations or run an evaluation.
      </div>
    );
  }

  const context = asRecord(currentContext.context);
  const taskCommitments = commitments.filter((commitment) => commitment.status === 'open');
  const todoistTasks = taskCommitments
    .filter((commitment) => commitment.source_type === 'todoist')
    .slice(0, 6);
  const topCommitments = taskCommitments
    .filter((commitment) => commitment.source_type !== 'todoist')
    .slice(0, 4);
  const calendarSignals = (explainedContext?.signal_summaries ?? [])
    .filter((signal) => signal.signal_type === 'calendar_event')
    .sort((left, right) => left.timestamp - right.timestamp)
    .slice(0, 5);

  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-5xl px-6 py-8">
        <header className="mb-8">
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Now</p>
          <h1 className="mt-2 text-3xl font-semibold text-zinc-100">What matters right now</h1>
          <p className="mt-2 text-sm text-zinc-400">
            Updated {new Date(currentContext.computed_at * 1000).toLocaleString()}
          </p>
        </header>

        <section className="grid gap-4 md:grid-cols-4">
          <FocusCard label="Mode" value={stringValue(context.mode) ?? 'unknown'} />
          <FocusCard label="Morning state" value={stringValue(context.morning_state) ?? 'unknown'} />
          <FocusCard label="Meds" value={stringValue(context.meds_status) ?? 'unknown'} />
          <FocusCard label="Risk" value={riskLabel(context)} />
        </section>

        <section className="mt-8 grid gap-6 xl:grid-cols-[1.3fr_1fr]">
          <div className="space-y-6">
            <Panel title="Upcoming events" subtitle="Live from selected Google calendars">
              {calendarSignals.length === 0 ? (
                <EmptyState text="No upcoming calendar events in the current stream." />
              ) : (
                <div className="space-y-3">
                  {calendarSignals.map((signal) => {
                    const summary = asRecord(signal.summary);
                    const title = stringValue(summary.title) ?? 'Untitled event';
                    const location = stringValue(summary.location);
                    return (
                      <div key={signal.signal_id} className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
                        <div className="flex items-start justify-between gap-3">
                          <div>
                            <p className="text-base font-medium text-zinc-100">{title}</p>
                            {location ? <p className="mt-1 text-sm text-zinc-400">{location}</p> : null}
                          </div>
                          <p className="text-sm text-zinc-400">{formatTimestamp(signal.timestamp)}</p>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </Panel>

            <Panel title="Todoist backlog" subtitle="Open commitments synced from Todoist">
              {todoistTasks.length === 0 ? (
                <EmptyState text="No open Todoist-backed commitments found." />
              ) : (
                <div className="space-y-3">
                  {todoistTasks.map((commitment) => (
                    <div key={commitment.id} className="rounded-xl border border-zinc-800 bg-zinc-900/70 p-4">
                      <p className="text-sm font-medium text-zinc-100">{commitment.text}</p>
                      <div className="mt-2 flex flex-wrap gap-x-4 gap-y-1 text-xs text-zinc-400">
                        <span>{commitment.project ?? 'No project'}</span>
                        {commitment.due_at ? <span>due {formatDateTime(commitment.due_at)}</span> : null}
                        {commitment.commitment_kind ? <span>{commitment.commitment_kind}</span> : null}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </Panel>
          </div>

          <div className="space-y-6">
            <Panel title="Operational state" subtitle="What Vel currently believes">
              <dl className="space-y-3 text-sm">
                <Row label="Next event" value={formatUnix(context.next_event_start_ts)} />
                <Row label="Leave by" value={formatUnix(context.leave_by_ts)} />
                <Row label="Attention" value={stringValue(context.attention_state) ?? 'unknown'} />
                <Row label="Drift" value={stringValue(context.drift_type) ?? 'none'} />
                <Row
                  label="Waiting on me"
                  value={numberValue(context.message_waiting_on_me_count)?.toString() ?? '0'}
                />
                <Row
                  label="Waiting on others"
                  value={numberValue(context.message_waiting_on_others_count)?.toString() ?? '0'}
                />
              </dl>
            </Panel>

            <Panel title="Why Vel thinks this" subtitle="Top context and attention drivers">
              <ul className="space-y-2">
                {[...(explainedContext?.reasons ?? []), ...(drift?.reasons ?? [])].slice(0, 8).map((reason) => (
                  <li key={reason} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2 text-sm text-zinc-200">
                    {reason}
                  </li>
                ))}
                {(!explainedContext || explainedContext.reasons.length === 0) && (!drift || drift.reasons.length === 0) ? (
                  <EmptyState text="No explanation reasons available." />
                ) : null}
              </ul>
            </Panel>

            <Panel title="Other open commitments" subtitle="Non-Todoist open items still in play">
              {topCommitments.length === 0 ? (
                <EmptyState text="No additional open commitments surfaced." />
              ) : (
                <div className="space-y-2">
                  {topCommitments.map((commitment) => (
                    <div key={commitment.id} className="rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                      <p className="text-sm text-zinc-100">{commitment.text}</p>
                      <p className="mt-1 text-xs text-zinc-500">{commitment.source_type}</p>
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
      <p className="mt-2 text-lg font-medium text-zinc-100">{value}</p>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between gap-3 rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-2">
      <dt className="text-zinc-500">{label}</dt>
      <dd className="text-right text-zinc-100">{value}</dd>
    </div>
  );
}

function EmptyState({ text }: { text: string }) {
  return <p className="text-sm text-zinc-500">{text}</p>;
}

function asRecord(value: JsonValue): JsonObject {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return {};
  }
  return value as JsonObject;
}

function stringValue(value: JsonValue | undefined): string | null {
  return typeof value === 'string' ? value : null;
}

function numberValue(value: JsonValue | undefined): number | null {
  return typeof value === 'number' ? value : null;
}

function formatUnix(value: JsonValue | undefined): string {
  const numeric = numberValue(value);
  return numeric == null ? 'unknown' : formatTimestamp(numeric);
}

function formatTimestamp(unixSeconds: number): string {
  return new Date(unixSeconds * 1000).toLocaleString([], {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });
}

function formatDateTime(value: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }
  return date.toLocaleString([], {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });
}

function riskLabel(context: JsonObject): string {
  const level = stringValue(context.global_risk_level);
  const score = numberValue(context.global_risk_score);
  if (!level && score == null) {
    return 'unknown';
  }
  if (score == null) {
    return level ?? 'unknown';
  }
  return `${level ?? 'risk'} · ${Math.round(score * 100)}%`;
}
