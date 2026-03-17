import { useMemo } from 'react';
import type { ContextExplainData, DriftExplainData, JsonObject, JsonValue, SignalExplainSummary } from '../types';
import { useQuery } from '../data/query';
import { loadContextExplain, loadDriftExplain, queryKeys } from '../data/resources';
import { SurfaceState } from './SurfaceState';

export function ContextPanel() {
  const contextExplainKey = useMemo(() => queryKeys.contextExplain(), []);
  const driftExplainKey = useMemo(() => queryKeys.driftExplain(), []);
  const {
    data: context,
    loading: contextLoading,
    error: contextError,
  } = useQuery<ContextExplainData | null>(
    contextExplainKey,
    async () => {
      const response = await loadContextExplain();
      return response.ok ? response.data ?? null : null;
    },
  );
  const {
    data: drift,
    loading: driftLoading,
    error: driftError,
  } = useQuery<DriftExplainData | null>(
    driftExplainKey,
    async () => {
      const response = await loadDriftExplain();
      return response.ok ? response.data ?? null : null;
    },
  );

  const loading = contextLoading || driftLoading;
  const error = contextError ?? driftError;
  const entries = context ? summarizeContext(context.context) : [];
  const signalSummaries = mergeSignalSummaries(
    context?.signal_summaries ?? [],
    drift?.signal_summaries ?? [],
  );

  if (loading) return <SurfaceState message="Loading context…" title="Context" />;
  if (error) return <SurfaceState message={error} title="Context" tone="warning" />;
  if (!context) {
    return <SurfaceState message="No context data. Run evaluate or start the engine." title="Context" />;
  }

  return (
    <div className="p-4 text-sm overflow-y-auto space-y-4">
      <div>
        <h3 className="font-medium text-zinc-400 mb-2">Context</h3>
        <p className="text-xs text-zinc-500">
          computed at {new Date(context.computed_at * 1000).toLocaleString()}
        </p>
      </div>

      <section className="grid gap-3">
        <div className="grid grid-cols-2 gap-3">
          <StatCard label="Mode" value={context.mode ?? 'unknown'} />
          <StatCard label="Morning state" value={context.morning_state ?? 'unknown'} />
        </div>
        {drift && hasDriftData(drift) && (
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-3">
            <p className="text-xs uppercase tracking-wide text-zinc-500">Attention</p>
            <div className="mt-2 grid grid-cols-2 gap-3">
              <StatCard label="State" value={drift.attention_state ?? 'unknown'} compact />
              <StatCard label="Drift" value={drift.drift_type ?? 'none'} compact />
              <StatCard label="Severity" value={drift.drift_severity ?? 'n/a'} compact />
              <StatCard
                label="Confidence"
                value={drift.confidence == null ? 'n/a' : `${Math.round(drift.confidence * 100)}%`}
                compact
              />
            </div>
          </div>
        )}
      </section>

      {context.reasons.length > 0 && (
        <section>
          <SectionHeading title="Why this context" />
          <ul className="mt-2 space-y-2">
            {context.reasons.map((reason) => (
              <li key={reason} className="rounded-md border border-zinc-800 bg-zinc-900/60 px-3 py-2 text-zinc-200">
                {reason}
              </li>
            ))}
            {drift?.reasons.map((reason) => (
              <li key={`drift-${reason}`} className="rounded-md border border-zinc-800 bg-zinc-900/40 px-3 py-2 text-zinc-300">
                {reason}
              </li>
            ))}
          </ul>
        </section>
      )}

      {entries.length > 0 && (
        <section>
          <SectionHeading title="Current state" />
          <dl className="mt-2 space-y-2">
            {entries.map(([label, value]) => (
              <div key={label} className="rounded-md border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                <dt className="text-zinc-500 text-xs">{label}</dt>
                <dd className="mt-1 text-zinc-200 break-words">{value}</dd>
              </div>
            ))}
          </dl>
        </section>
      )}

      {signalSummaries.length > 0 && (
        <section>
          <SectionHeading title="Signals used" />
          <div className="mt-2 space-y-2">
            {signalSummaries.map((signal) => (
              <div key={signal.signal_id} className="rounded-md border border-zinc-800 bg-zinc-900/60 px-3 py-2">
                <div className="flex items-start justify-between gap-3">
                  <div>
                    <p className="text-zinc-100">{signal.signal_type}</p>
                    <p className="text-xs text-zinc-500">{signal.source}</p>
                  </div>
                  <p className="text-xs text-zinc-500">
                    {new Date(signal.timestamp * 1000).toLocaleString()}
                  </p>
                </div>
                <p className="mt-2 text-zinc-300 text-xs whitespace-pre-wrap break-words">
                  {formatSummary(signal.summary)}
                </p>
              </div>
            ))}
          </div>
        </section>
      )}
    </div>
  );
}

function SectionHeading({ title }: { title: string }) {
  return <h4 className="font-medium text-zinc-400">{title}</h4>;
}

function StatCard({ label, value, compact = false }: { label: string; value: string; compact?: boolean }) {
  return (
    <div className={`rounded-lg border border-zinc-800 bg-zinc-900/70 ${compact ? 'p-2' : 'p-3'}`}>
      <p className="text-xs uppercase tracking-wide text-zinc-500">{label}</p>
      <p className="mt-1 text-zinc-100">{value}</p>
    </div>
  );
}

function hasDriftData(drift: DriftExplainData): boolean {
  return drift.attention_state !== null
    || drift.drift_type !== null
    || drift.drift_severity !== null
    || drift.confidence !== null
    || drift.reasons.length > 0;
}

function summarizeContext(context: JsonValue): Array<[string, string]> {
  if (!context || typeof context !== 'object' || Array.isArray(context)) {
    return [['value', formatSummary(context)]];
  }

  const record = context as JsonObject;
  const keys = [
    'next_commitment_id',
    'meds_status',
    'prep_window_active',
    'commute_window_active',
    'global_risk_level',
    'global_risk_score',
    'git_activity_summary',
    'message_waiting_on_me_count',
    'message_waiting_on_others_count',
    'message_scheduling_thread_count',
    'message_urgent_thread_count',
    'message_summary',
  ];

  return keys
    .filter((key) => record[key] !== undefined && record[key] !== null)
    .map((key) => [key.replace(/_/g, ' '), formatSummary(record[key])]);
}

function mergeSignalSummaries(
  left: SignalExplainSummary[],
  right: SignalExplainSummary[],
): SignalExplainSummary[] {
  const byId = new Map<string, SignalExplainSummary>();
  for (const signal of [...left, ...right]) {
    byId.set(signal.signal_id, signal);
  }
  return [...byId.values()];
}

function formatSummary(value: JsonValue): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean' || value === null) {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value.map((item) => formatSummary(item)).join(', ');
  }
  return Object.entries(value as JsonObject)
    .filter(([, next]) => next !== null && next !== undefined && next !== '')
    .map(([key, next]) => `${key.replace(/_/g, ' ')}: ${formatSummary(next)}`)
    .join(' · ');
}
