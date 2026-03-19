import { useMemo } from 'react';
import type {
  ComponentData,
  ContextExplainData,
  CurrentContextData,
  DriftExplainData,
  IntegrationsData,
  LoopData,
  RunSummaryData,
} from '../types';
import {
  contextQueryKeys,
  loadContextExplain,
  loadCurrentContext,
  loadDriftExplain,
} from '../data/context';
import {
  loadComponents,
  loadIntegrations,
  loadLoops,
  loadRecentRuns,
  operatorQueryKeys,
} from '../data/operator';
import { useQuery } from '../data/query';
import { SurfaceState } from './SurfaceState';

const RUN_LIMIT = 20;

export function StatsView() {
  const integrationsKey = useMemo(() => operatorQueryKeys.integrations(), []);
  const runsKey = useMemo(() => operatorQueryKeys.runs(RUN_LIMIT), []);
  const loopsKey = useMemo(() => operatorQueryKeys.loops(), []);
  const componentsKey = useMemo(() => operatorQueryKeys.components(), []);
  const contextKey = useMemo(() => contextQueryKeys.currentContext(), []);
  const contextExplainKey = useMemo(() => contextQueryKeys.contextExplain(), []);
  const driftExplainKey = useMemo(() => contextQueryKeys.driftExplain(), []);

  const { data: integrations, loading: integrationsLoading, error: integrationsError } = useQuery<IntegrationsData | null>(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: runs = [], loading: runsLoading, error: runsError } = useQuery<RunSummaryData[]>(
    runsKey,
    async () => {
      const response = await loadRecentRuns(RUN_LIMIT);
      return response.ok ? response.data ?? [] : [];
    },
  );
  const { data: loops = [], loading: loopsLoading, error: loopsError } = useQuery<LoopData[]>(
    loopsKey,
    async () => {
      const response = await loadLoops();
      return response.ok ? response.data ?? [] : [];
    },
  );
  const { data: components = [], loading: componentsLoading, error: componentsError } = useQuery<ComponentData[]>(
    componentsKey,
    async () => {
      const response = await loadComponents();
      return response.ok ? response.data ?? [] : [];
    },
  );
  const { data: currentContext, loading: currentContextLoading, error: currentContextError } = useQuery<CurrentContextData | null>(
    contextKey,
    async () => {
      const response = await loadCurrentContext();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: contextExplain, loading: contextExplainLoading, error: contextExplainError } = useQuery<ContextExplainData | null>(
    contextExplainKey,
    async () => {
      const response = await loadContextExplain();
      return response.ok ? response.data ?? null : null;
    },
  );
  const { data: driftExplain, loading: driftExplainLoading, error: driftExplainError } = useQuery<DriftExplainData | null>(
    driftExplainKey,
    async () => {
      const response = await loadDriftExplain();
      return response.ok ? response.data ?? null : null;
    },
  );

  const loading = integrationsLoading
    && runsLoading
    && loopsLoading
    && componentsLoading
    && currentContextLoading
    && contextExplainLoading
    && driftExplainLoading;
  const errors = [
    integrationsError,
    runsError,
    loopsError,
    componentsError,
    currentContextError,
    contextExplainError,
    driftExplainError,
  ].filter((error): error is string => Boolean(error));

  if (loading) {
    return <SurfaceState message="Loading stats…" layout="centered" />;
  }

  return (
    <div className="flex-1 overflow-y-auto bg-zinc-950">
      <div className="mx-auto max-w-6xl px-6 py-8 space-y-8">
        <header>
          <p className="text-xs uppercase tracking-[0.25em] text-zinc-500">Stats</p>
          <h1 className="mt-2 text-3xl font-semibold text-zinc-100">Passive detail and observability</h1>
          <p className="mt-2 text-sm text-zinc-400">
            Use this drill-down when you want richer context, runtime health, and execution detail
            without turning it into a first-contact daily-use surface.
          </p>
        </header>

        {errors.length > 0 ? (
          <div className="rounded-xl border border-amber-500/40 bg-amber-500/10 px-4 py-3 text-sm text-amber-100">
            Partial data: {errors.join(' | ')}
          </div>
        ) : null}

        <section className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          <StatCard
            label="Context mode"
            value={contextExplain?.mode ?? 'unknown'}
            detail={`Updated ${formatUnixTimestamp(contextExplain?.computed_at ?? null)}`}
          />
          <StatCard
            label="Morning state"
            value={contextExplain?.morning_state ?? 'unknown'}
            detail={`Current context ${formatUnixTimestamp(currentContext?.computed_at ?? null)}`}
          />
          <StatCard
            label="Attention"
            value={driftExplain?.attention_state ?? 'unknown'}
            detail={`Drift ${driftExplain?.drift_type ?? 'none'}`}
          />
          <StatCard
            label="Confidence"
            value={driftExplain?.confidence == null ? 'n/a' : `${Math.round(driftExplain.confidence * 100)}%`}
            detail={`Severity ${driftExplain?.drift_severity ?? 'n/a'}`}
          />
        </section>

        <section className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-5">
          <h2 className="text-lg font-medium text-zinc-100">Source health</h2>
          <p className="mt-1 text-sm text-zinc-400">
            Integration state and last observed sync health by source.
          </p>
          {integrations ? (
            <div className="mt-4 grid gap-3 md:grid-cols-2">
              {integrationRows(integrations).map((row) => (
                <div key={row.key} className="rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                  <div className="flex items-start justify-between gap-2">
                    <div>
                      <p className="text-sm font-medium text-zinc-100">{row.label}</p>
                      <p className="text-xs text-zinc-500">{row.status}</p>
                    </div>
                    <span className={`rounded-full px-2 py-1 text-[11px] ${statusClass(row.lastSyncStatus)}`}>
                      {row.lastSyncStatus ?? 'never'}
                    </span>
                  </div>
                  <p className="mt-2 text-xs text-zinc-400">
                    Last sync: {formatUnixTimestamp(row.lastSyncAt)}
                    {row.lastItemCount != null ? ` · ${row.lastItemCount} items` : ''}
                  </p>
                  {row.lastError ? (
                    <p className="mt-2 text-xs text-rose-300">Error: {row.lastError}</p>
                  ) : null}
                </div>
              ))}
            </div>
          ) : (
            <SurfaceState message="Integrations data unavailable." />
          )}
        </section>

        <section className="grid gap-4 xl:grid-cols-2">
          <div className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-5">
            <h2 className="text-lg font-medium text-zinc-100">Runtime loops</h2>
            <p className="mt-1 text-sm text-zinc-400">Scheduler loop state, cadence, and recent results.</p>
            {loops.length === 0 ? (
              <SurfaceState message="No loops reported." />
            ) : (
              <div className="mt-4 space-y-2">
                {loops.map((loop) => (
                  <div key={loop.kind} className="rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                    <div className="flex items-start justify-between gap-2">
                      <p className="text-sm font-medium text-zinc-100">{loop.kind}</p>
                      <span className={`rounded-full px-2 py-1 text-[11px] ${loop.enabled ? 'bg-emerald-500/20 text-emerald-200' : 'bg-zinc-700/40 text-zinc-300'}`}>
                        {loop.enabled ? 'enabled' : 'disabled'}
                      </span>
                    </div>
                    <p className="mt-2 text-xs text-zinc-400">
                      every {loop.interval_seconds}s · status {loop.last_status ?? 'unknown'} · next {formatUnixTimestamp(loop.next_due_at)}
                    </p>
                    {loop.last_error ? (
                      <p className="mt-2 text-xs text-rose-300">Error: {loop.last_error}</p>
                    ) : null}
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-5">
            <h2 className="text-lg font-medium text-zinc-100">Components</h2>
            <p className="mt-1 text-sm text-zinc-400">Backend component health and restart history.</p>
            {components.length === 0 ? (
              <SurfaceState message="No components reported." />
            ) : (
              <div className="mt-4 space-y-2">
                {components.map((component) => (
                  <div key={component.id} className="rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                    <div className="flex items-start justify-between gap-2">
                      <div>
                        <p className="text-sm font-medium text-zinc-100">{component.name}</p>
                        <p className="text-xs text-zinc-500">{component.description}</p>
                      </div>
                      <span className={`rounded-full px-2 py-1 text-[11px] ${componentStatusClass(component.status)}`}>
                        {component.status}
                      </span>
                    </div>
                    <p className="mt-2 text-xs text-zinc-400">
                      Restarts: {component.restart_count} · Last restart {formatUnixTimestamp(component.last_restarted_at)}
                    </p>
                    {component.last_error ? (
                      <p className="mt-2 text-xs text-rose-300">Error: {component.last_error}</p>
                    ) : null}
                  </div>
                ))}
              </div>
            )}
          </div>
        </section>

        <section className="rounded-2xl border border-zinc-800 bg-zinc-900/60 p-5">
          <h2 className="text-lg font-medium text-zinc-100">Recent runs</h2>
          <p className="mt-1 text-sm text-zinc-400">
            Latest run lifecycle outcomes and retry posture.
          </p>
          {runs.length === 0 ? (
            <SurfaceState message="No runs yet." />
          ) : (
            <div className="mt-4 overflow-x-auto">
              <table className="min-w-full text-sm">
                <thead className="text-left text-zinc-400">
                  <tr>
                    <th className="px-2 py-2">Kind</th>
                    <th className="px-2 py-2">Status</th>
                    <th className="px-2 py-2">Duration</th>
                    <th className="px-2 py-2">Created</th>
                    <th className="px-2 py-2">Retry policy</th>
                  </tr>
                </thead>
                <tbody>
                  {runs.map((run) => (
                    <tr key={run.id} className="border-t border-zinc-800">
                      <td className="px-2 py-2 text-zinc-100">{run.kind}</td>
                      <td className="px-2 py-2 text-zinc-300">{run.status}</td>
                      <td className="px-2 py-2 text-zinc-300">{formatDuration(run.duration_ms)}</td>
                      <td className="px-2 py-2 text-zinc-400">{formatRfcTimestamp(run.created_at)}</td>
                      <td className="px-2 py-2 text-zinc-400">
                        {run.automatic_retry_supported ? 'automatic' : 'manual'}
                        {run.blocked_reason ? ` · blocked: ${run.blocked_reason}` : ''}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>
      </div>
    </div>
  );
}

function StatCard({ label, value, detail }: { label: string; value: string; detail: string }) {
  return (
    <div className="rounded-xl border border-zinc-800 bg-zinc-900/70 px-4 py-3">
      <p className="text-xs uppercase tracking-wide text-zinc-500">{label}</p>
      <p className="mt-1 text-xl font-medium text-zinc-100">{value}</p>
      <p className="mt-1 text-xs text-zinc-400">{detail}</p>
    </div>
  );
}

function integrationRows(integrations: IntegrationsData): Array<{
  key: string;
  label: string;
  status: string;
  lastSyncStatus: string | null;
  lastSyncAt: number | null;
  lastItemCount: number | null;
  lastError: string | null;
}> {
  return [
    {
      key: 'google_calendar',
      label: 'Google Calendar',
      status: integrations.google_calendar.connected ? 'connected' : (integrations.google_calendar.configured ? 'configured' : 'not configured'),
      lastSyncStatus: integrations.google_calendar.last_sync_status,
      lastSyncAt: integrations.google_calendar.last_sync_at,
      lastItemCount: integrations.google_calendar.last_item_count,
      lastError: integrations.google_calendar.last_error,
    },
    {
      key: 'todoist',
      label: 'Todoist',
      status: integrations.todoist.connected ? 'connected' : (integrations.todoist.configured ? 'configured' : 'not configured'),
      lastSyncStatus: integrations.todoist.last_sync_status,
      lastSyncAt: integrations.todoist.last_sync_at,
      lastItemCount: integrations.todoist.last_item_count,
      lastError: integrations.todoist.last_error,
    },
    localIntegrationRow('activity', 'Computer Activity', integrations.activity),
    localIntegrationRow('health', 'Health', integrations.health),
    localIntegrationRow('git', 'Git Activity', integrations.git),
    localIntegrationRow('messaging', 'Messaging', integrations.messaging),
    localIntegrationRow('notes', 'Notes', integrations.notes),
    localIntegrationRow('transcripts', 'Transcripts', integrations.transcripts),
  ];
}

function localIntegrationRow(key: string, label: string, integration: IntegrationsData['activity']) {
  return {
    key,
    label,
    status: integration.configured ? 'configured' : 'not configured',
    lastSyncStatus: integration.last_sync_status,
    lastSyncAt: integration.last_sync_at,
    lastItemCount: integration.last_item_count,
    lastError: integration.last_error,
  };
}

function componentStatusClass(status: string): string {
  if (status === 'healthy' || status === 'running') {
    return 'bg-emerald-500/20 text-emerald-200';
  }
  if (status === 'degraded') {
    return 'bg-amber-500/20 text-amber-200';
  }
  if (status === 'failed' || status === 'error') {
    return 'bg-rose-500/20 text-rose-200';
  }
  return 'bg-zinc-700/40 text-zinc-300';
}

function statusClass(status: string | null): string {
  if (status === 'ok' || status === 'success') {
    return 'bg-emerald-500/20 text-emerald-200';
  }
  if (status === 'error' || status === 'failed') {
    return 'bg-rose-500/20 text-rose-200';
  }
  if (status === 'stale' || status === 'warning') {
    return 'bg-amber-500/20 text-amber-200';
  }
  return 'bg-zinc-700/40 text-zinc-300';
}

function formatDuration(durationMs: number | null): string {
  if (durationMs == null) {
    return 'n/a';
  }
  if (durationMs < 1000) {
    return `${durationMs}ms`;
  }
  return `${(durationMs / 1000).toFixed(1)}s`;
}

function formatUnixTimestamp(timestamp: number | null): string {
  if (timestamp == null) {
    return 'never';
  }
  return new Date(timestamp * 1000).toLocaleString();
}

function formatRfcTimestamp(timestamp: string | null): string {
  if (!timestamp) {
    return 'n/a';
  }
  return new Date(timestamp).toLocaleString();
}
