import { useEffect, useMemo, useState } from 'react';
import { apiPatch } from '../api/client';
import { subscribeWs } from '../realtime/ws';
import type { RunSummaryData, SettingsData } from '../types';
import { setQueryData, useQuery } from '../data/query';
import { loadRecentRuns, loadSettings, queryKeys } from '../data/resources';

interface SettingsPageProps {
  onBack: () => void;
}

interface RetryDraft {
  reason: string;
  retryAfterSeconds: string;
  blockedReason: string;
}

interface RunActionState {
  status: 'success' | 'error';
  message: string;
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [saving, setSaving] = useState(false);
  const [actingRunId, setActingRunId] = useState<string | null>(null);
  const [retryDrafts, setRetryDrafts] = useState<Record<string, RetryDraft>>({});
  const [runActionState, setRunActionState] = useState<Record<string, RunActionState>>({});
  const runLimit = 6;
  const settingsKey = useMemo(() => queryKeys.settings(), []);
  const runsKey = useMemo(() => queryKeys.runs(runLimit), []);
  const {
    data: settings = {},
    loading,
  } = useQuery<SettingsData>(
    settingsKey,
    async () => {
      const response = await loadSettings();
      return response.ok && response.data ? response.data : {};
    },
  );
  const { data: runs = [], refetch: refetchRuns } = useQuery<RunSummaryData[]>(
    runsKey,
    async () => {
      const response = await loadRecentRuns(runLimit);
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWs((event) => {
      if (event.type === 'runs:updated') {
        setQueryData<RunSummaryData[]>(runsKey, (current = []) => {
          const next = [...current];
          const index = next.findIndex((run) => run.id === event.payload.id);
          if (index >= 0) {
            next[index] = event.payload;
            return next;
          }
          return [event.payload, ...next].slice(0, runLimit);
        });
      }
    });
  }, [runLimit, runsKey]);

  useEffect(() => {
    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        void refetchRuns();
      }
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    return () => document.removeEventListener('visibilitychange', handleVisibilityChange);
  }, [refetchRuns]);

  const update = async (key: keyof SettingsData, value: boolean | unknown) => {
    setSaving(true);
    try {
      await apiPatch('/api/settings', { [key]: value });
      setQueryData<SettingsData>(settingsKey, (prev = {}) => ({
        ...prev,
        [key]: value,
      }));
    } finally {
      setSaving(false);
    }
  };

  const scheduleRetry = async (run: RunSummaryData) => {
    const allowUnsupportedRetry = !run.automatic_retry_supported;
    const draft = retryDrafts[run.id];
    const reason = draft?.reason.trim()
      || (allowUnsupportedRetry ? 'operator_ui_override_retry' : 'operator_ui_retry');
    const retryAfterSeconds = parseRetryAfterSeconds(draft?.retryAfterSeconds);

    if (allowUnsupportedRetry) {
      const confirmed = window.confirm(
        `Automatic retry is unsupported for ${run.kind}. Schedule a manual override retry anyway?`,
      );
      if (!confirmed) {
        return;
      }
    }

    setActingRunId(run.id);
    setRunActionState((current) => {
      const next = { ...current };
      delete next[run.id];
      return next;
    });
    try {
      await apiPatch(`/v1/runs/${run.id}`, {
        status: 'retry_scheduled',
        reason,
        retry_after_seconds: retryAfterSeconds,
        allow_unsupported_retry: allowUnsupportedRetry,
      });
      setRunActionState((current) => ({
        ...current,
        [run.id]: {
          status: 'success',
          message: 'Retry scheduled.',
        },
      }));
    } catch (error) {
      setRunActionState((current) => ({
        ...current,
        [run.id]: {
          status: 'error',
          message: error instanceof Error ? error.message : String(error),
        },
      }));
    } finally {
      setActingRunId(null);
    }
  };

  const blockRun = async (run: RunSummaryData) => {
    const blockedReason = retryDrafts[run.id]?.blockedReason.trim() || 'operator_ui_blocked';
    setActingRunId(run.id);
    setRunActionState((current) => {
      const next = { ...current };
      delete next[run.id];
      return next;
    });
    try {
      await apiPatch(`/v1/runs/${run.id}`, {
        status: 'blocked',
        blocked_reason: blockedReason,
      });
      setRunActionState((current) => ({
        ...current,
        [run.id]: {
          status: 'success',
          message: 'Run blocked.',
        },
      }));
    } catch (error) {
      setRunActionState((current) => ({
        ...current,
        [run.id]: {
          status: 'error',
          message: error instanceof Error ? error.message : String(error),
        },
      }));
    } finally {
      setActingRunId(null);
    }
  };

  if (loading) return <div className="p-8 text-zinc-500">Loading settings…</div>;

  return (
    <div className="flex-1 overflow-y-auto p-8 max-w-lg">
      <button
        type="button"
        onClick={onBack}
        className="text-zinc-500 hover:text-zinc-300 text-sm mb-6"
      >
        ← Back
      </button>
      <h2 className="text-xl font-medium text-zinc-200 mb-6">Settings</h2>
      <div className="space-y-4">
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Disable proactive interventions</span>
          <input
            type="checkbox"
            checked={settings.disable_proactive ?? false}
            onChange={(e) => update('disable_proactive', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show risks</span>
          <input
            type="checkbox"
            checked={settings.toggle_risks !== false}
            onChange={(e) => update('toggle_risks', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
        <label className="flex items-center justify-between gap-4">
          <span className="text-zinc-300">Show reminders</span>
          <input
            type="checkbox"
            checked={settings.toggle_reminders !== false}
            onChange={(e) => update('toggle_reminders', e.target.checked)}
            disabled={saving}
            className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
          />
        </label>
      </div>
      <section className="mt-10">
        <div className="mb-3">
          <h3 className="text-lg font-medium text-zinc-200">Recent runs</h3>
          <p className="text-sm text-zinc-500">
            Automatic retry policy and manual override status for the most recent backend runs.
          </p>
        </div>
        <div className="space-y-3">
          {runs.length === 0 ? (
            <p className="text-sm text-zinc-500">No runs yet.</p>
          ) : (
            runs.map((run) => (
              <article key={run.id} className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-4">
                <div className="flex items-start justify-between gap-3">
                  <div>
                    <p className="text-sm font-medium text-zinc-100">{run.kind}</p>
                    <p className="text-xs text-zinc-500">{run.id}</p>
                  </div>
                  <div className="text-right">
                    <p className="text-sm text-zinc-200">{run.status}</p>
                    <p className="text-xs text-zinc-500">{formatTimestamp(run.created_at)}</p>
                  </div>
                </div>
                <div className="mt-3 space-y-1 text-sm text-zinc-300">
                  <p>
                    Auto retry:{' '}
                    <span className={run.automatic_retry_supported ? 'text-emerald-400' : 'text-amber-300'}>
                      {run.automatic_retry_supported ? 'supported' : 'unsupported'}
                    </span>
                  </p>
                  {run.automatic_retry_reason ? (
                    <p className="text-zinc-400">{run.automatic_retry_reason}</p>
                  ) : null}
                  {run.retry_scheduled_at ? (
                    <p>Retry at: {formatTimestamp(run.retry_scheduled_at)}</p>
                  ) : null}
                  {run.retry_reason ? <p>Retry reason: {run.retry_reason}</p> : null}
                  {run.blocked_reason ? <p>Blocked reason: {run.blocked_reason}</p> : null}
                  {run.unsupported_retry_override ? (
                    <p className="text-amber-300">
                      Manual override active
                      {run.unsupported_retry_override_reason
                        ? `: ${run.unsupported_retry_override_reason}`
                        : ''}
                    </p>
                  ) : null}
                </div>
                <div className="mt-4 grid gap-3 md:grid-cols-[minmax(0,1fr)_9rem]">
                  <label className="space-y-1">
                    <span className="text-xs uppercase tracking-wide text-zinc-500">Retry reason</span>
                    <input
                      type="text"
                      value={retryDrafts[run.id]?.reason ?? ''}
                      onChange={(event) => {
                        const nextReason = event.target.value;
                        setRetryDrafts((current) => ({
                          ...current,
                          [run.id]: {
                            reason: nextReason,
                            retryAfterSeconds: current[run.id]?.retryAfterSeconds ?? '',
                            blockedReason: current[run.id]?.blockedReason ?? '',
                          },
                        }));
                      }}
                      disabled={!canScheduleRetry(run) || actingRunId !== null}
                      placeholder={run.automatic_retry_supported ? 'operator_ui_retry' : 'operator_ui_override_retry'}
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                    />
                  </label>
                  <label className="space-y-1">
                    <span className="text-xs uppercase tracking-wide text-zinc-500">Delay seconds</span>
                    <input
                      type="number"
                      min="0"
                      step="1"
                      value={retryDrafts[run.id]?.retryAfterSeconds ?? ''}
                      onChange={(event) => {
                        const nextDelay = event.target.value;
                        setRetryDrafts((current) => ({
                          ...current,
                          [run.id]: {
                            reason: current[run.id]?.reason ?? '',
                            retryAfterSeconds: nextDelay,
                            blockedReason: current[run.id]?.blockedReason ?? '',
                          },
                        }));
                      }}
                      disabled={!canScheduleRetry(run) || actingRunId !== null}
                      placeholder="0"
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                    />
                  </label>
                </div>
                <div className="mt-3">
                  <label className="space-y-1">
                    <span className="text-xs uppercase tracking-wide text-zinc-500">Blocked reason</span>
                    <input
                      type="text"
                      value={retryDrafts[run.id]?.blockedReason ?? ''}
                      onChange={(event) => {
                        const nextBlockedReason = event.target.value;
                        setRetryDrafts((current) => ({
                          ...current,
                          [run.id]: {
                            reason: current[run.id]?.reason ?? '',
                            retryAfterSeconds: current[run.id]?.retryAfterSeconds ?? '',
                            blockedReason: nextBlockedReason,
                          },
                        }));
                      }}
                      disabled={!canControlRun(run) || actingRunId !== null}
                      placeholder="operator_ui_blocked"
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                    />
                  </label>
                </div>
                <div className="mt-4 flex items-center gap-3">
                  <button
                    type="button"
                    onClick={() => void scheduleRetry(run)}
                    disabled={!canScheduleRetry(run) || actingRunId !== null}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {actingRunId === run.id ? 'Working…' : 'Schedule retry'}
                  </button>
                  <button
                    type="button"
                    onClick={() => void blockRun(run)}
                    disabled={!canControlRun(run) || actingRunId !== null}
                    className="rounded-md border border-rose-900 px-3 py-1.5 text-sm text-rose-200 transition hover:border-rose-700 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {actingRunId === run.id ? 'Working…' : 'Block run'}
                  </button>
                  {!run.automatic_retry_supported ? (
                    <span className="text-xs text-amber-300">Requires override</span>
                  ) : null}
                </div>
                {runActionState[run.id] ? (
                  <p
                    className={`mt-3 text-sm ${
                      runActionState[run.id]?.status === 'error' ? 'text-rose-400' : 'text-emerald-400'
                    }`}
                  >
                    {runActionState[run.id]?.message}
                  </p>
                ) : null}
              </article>
            ))
          )}
        </div>
      </section>
      {saving && <p className="text-zinc-500 text-sm mt-4">Saving…</p>}
    </div>
  );
}

function formatTimestamp(value: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }
  return date.toLocaleString();
}

function canScheduleRetry(run: RunSummaryData): boolean {
  return run.status !== 'running' && run.status !== 'retry_scheduled';
}

function canControlRun(run: RunSummaryData): boolean {
  return run.status !== 'running' && run.status !== 'blocked';
}

function parseRetryAfterSeconds(value: string | undefined): number | undefined {
  if (!value) {
    return undefined;
  }
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    return undefined;
  }
  return parsed;
}
