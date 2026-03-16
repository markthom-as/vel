import { useEffect, useMemo, useRef, useState } from 'react';
import { apiPatch, apiPost } from '../api/client';
import { subscribeWs } from '../realtime/ws';
import type {
  IntegrationsData,
  RunSummaryData,
  SettingsData,
} from '../types';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import type { QueryKey } from '../data/query';
import {
  decodeGoogleCalendarAuthStartResponse,
  loadIntegrations,
  loadRecentRuns,
  loadSettings,
  queryKeys,
} from '../data/resources';

interface SettingsPageProps {
  onBack: () => void;
}

interface RetryDraft {
  reason: string;
  retryAfterSeconds: string;
  blockedReason: string;
}

type RunActionKind = 'retry' | 'block';
type IntegrationActionKey =
  | 'google-save'
  | 'google-auth'
  | 'google-sync'
  | 'google-disconnect'
  | 'google-calendars'
  | 'todoist-save'
  | 'todoist-sync'
  | 'todoist-disconnect';
type IntegrationSectionKey = 'google' | 'todoist';

interface RunActionState {
  action: RunActionKind;
  status: 'success' | 'error';
  message: string;
  actionId: number;
}

interface IntegrationFeedbackState {
  section: IntegrationSectionKey;
  action: IntegrationActionKey;
  status: 'success' | 'error';
  message: string;
  actionId: number;
}

type SettingsTab = 'general' | 'integrations' | 'runs';

const DEFAULT_INTEGRATIONS: IntegrationsData = {
  google_calendar: {
    configured: false,
    connected: false,
    has_client_id: false,
    has_client_secret: false,
    calendars: [],
    all_calendars_selected: true,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
  },
  todoist: {
    configured: false,
    connected: false,
    has_api_token: false,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
  },
};

function updateRunsCache(
  runsKey: QueryKey,
  runLimit: number,
  run: RunSummaryData,
) {
  setQueryData<RunSummaryData[]>(runsKey, (current = []) => {
    const next = [...current];
    const index = next.findIndex((existingRun) => existingRun.id === run.id);
    if (index >= 0) {
      next[index] = run;
      return next;
    }
    return [run, ...next].slice(0, runLimit);
  });
}

function extractRunSummaryData(value: unknown): RunSummaryData | null {
  if (!value || typeof value !== 'object') {
    return null;
  }
  const record = value as { ok?: unknown; data?: unknown };
  if (record.ok !== true || !record.data || typeof record.data !== 'object') {
    return null;
  }
  const data = record.data as Partial<RunSummaryData>;
  if (typeof data.id !== 'string' || typeof data.kind !== 'string' || typeof data.status !== 'string') {
    return null;
  }
  return data as RunSummaryData;
}

function integrationSectionForAction(key: IntegrationActionKey): IntegrationSectionKey {
  return key.startsWith('google-') ? 'google' : 'todoist';
}

function integrationFeedbackForSection(
  feedback: Record<string, IntegrationFeedbackState>,
  section: IntegrationSectionKey,
) {
  return Object.values(feedback).filter((entry) => entry.section === section);
}

export function SettingsPage({ onBack }: SettingsPageProps) {
  const [activeTab, setActiveTab] = useState<SettingsTab>('general');
  const [saving, setSaving] = useState(false);
  const [pendingIntegrationActions, setPendingIntegrationActions] = useState<Record<string, true>>({});
  const [googleClientId, setGoogleClientId] = useState('');
  const [googleClientSecret, setGoogleClientSecret] = useState('');
  const [todoistToken, setTodoistToken] = useState('');
  const [integrationFeedback, setIntegrationFeedback] = useState<Record<string, IntegrationFeedbackState>>({});
  const [actingRuns, setActingRuns] = useState<Record<string, true>>({});
  const [pendingOverrideRunId, setPendingOverrideRunId] = useState<string | null>(null);
  const [retryDrafts, setRetryDrafts] = useState<Record<string, RetryDraft>>({});
  const [runActionState, setRunActionState] = useState<Record<string, RunActionState>>({});
  const nextIntegrationActionIdRef = useRef(0);
  const latestIntegrationActionIdRef = useRef(0);
  const nextRunActionIdRef = useRef(0);
  const latestRunActionIdByRunRef = useRef<Record<string, number>>({});
  const runLimit = 6;
  const settingsKey = useMemo(() => queryKeys.settings(), []);
  const integrationsKey = useMemo(() => queryKeys.integrations(), []);
  const runsKey = useMemo(() => queryKeys.runs(runLimit), []);
  const currentContextKey = useMemo(() => queryKeys.currentContext(), []);
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
  const {
    data: integrationsData,
    error: integrationsLoadError,
  } = useQuery<IntegrationsData>(
    integrationsKey,
    async () => {
      const response = await loadIntegrations();
      return response.ok && response.data ? response.data : DEFAULT_INTEGRATIONS;
    },
  );
  const integrations = integrationsData ?? DEFAULT_INTEGRATIONS;
  const googleFeedback = integrationFeedbackForSection(integrationFeedback, 'google');
  const todoistFeedback = integrationFeedbackForSection(integrationFeedback, 'todoist');
  const { data: runs = [] } = useQuery<RunSummaryData[]>(
    runsKey,
    async () => {
      const response = await loadRecentRuns(runLimit);
      return response.ok && response.data ? response.data : [];
    },
  );

  useEffect(() => {
    return subscribeWs((event) => {
      if (event.type === 'runs:updated') {
        updateRunsCache(runsKey, runLimit, event.payload);
      }
    });
  }, [runLimit, runsKey]);

  useEffect(() => {
    if (!pendingOverrideRunId) {
      return;
    }
    const pendingRun = runs.find((run) => run.id === pendingOverrideRunId);
    if (!pendingRun || !shouldKeepPendingOverride(pendingRun)) {
      setPendingOverrideRunId(null);
    }
  }, [pendingOverrideRunId, runs]);

  useEffect(() => {
    if (runs.length === 0) {
      setRunActionState((current) => (Object.keys(current).length === 0 ? current : {}));
      return;
    }

    setRunActionState((current) => {
      let changed = false;
      const next: Record<string, RunActionState> = {};
      for (const [runId, actionState] of Object.entries(current)) {
        const run = runs.find((candidate) => candidate.id === runId);
        if (!run) {
          changed = true;
          continue;
        }
        if (!shouldKeepActionBanner(run, actionState)) {
          changed = true;
          continue;
        }
        next[runId] = actionState;
      }
      return changed ? next : current;
    });
  }, [runs]);

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

  const refreshIntegrationViews = () => {
    invalidateQuery(integrationsKey, { refetch: true });
    invalidateQuery(currentContextKey, { refetch: true });
  };

  const beginIntegrationAction = (key: IntegrationActionKey): number => {
    const actionId = nextIntegrationActionIdRef.current + 1;
    nextIntegrationActionIdRef.current = actionId;
    latestIntegrationActionIdRef.current = actionId;
    setPendingIntegrationActions((current) => ({
      ...current,
      [key]: true,
    }));
    setIntegrationFeedback((current) => {
      const next = { ...current };
      delete next[key];
      return next;
    });
    return actionId;
  };

  const finishIntegrationAction = (
    key: IntegrationActionKey,
    actionId: number,
    nextFeedback?: Omit<IntegrationFeedbackState, 'actionId' | 'section' | 'action'>,
  ) => {
    setPendingIntegrationActions((current) => {
      if (!current[key]) {
        return current;
      }
      const next = { ...current };
      delete next[key];
      return next;
    });

    if (latestIntegrationActionIdRef.current !== actionId || !nextFeedback) {
      return;
    }

    setIntegrationFeedback((current) => ({
      ...current,
      [key]: {
        ...nextFeedback,
        section: integrationSectionForAction(key),
        action: key,
        actionId,
      },
    }));
  };

  const saveGoogleCredentials = async () => {
    const actionId = beginIntegrationAction('google-save');
    try {
      await apiPatch('/api/integrations/google-calendar', {
        client_id: googleClientId,
        client_secret: googleClientSecret,
      });
      refreshIntegrationViews();
      setGoogleClientSecret('');
      finishIntegrationAction('google-save', actionId, {
        status: 'success',
        message: 'Google Calendar credentials saved.',
      });
    } catch (error) {
      finishIntegrationAction('google-save', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const startGoogleAuth = async () => {
    const actionId = beginIntegrationAction('google-auth');
    try {
      const response = await apiPost(
        '/api/integrations/google-calendar/auth/start',
        {},
        decodeGoogleCalendarAuthStartResponse,
      );
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to start Google auth');
      }
      window.open(response.data.auth_url, '_blank', 'noopener,noreferrer');
      refreshIntegrationViews();
      finishIntegrationAction('google-auth', actionId, {
        status: 'success',
        message: 'Google auth started. Complete it in the opened window, then sync.',
      });
    } catch (error) {
      finishIntegrationAction('google-auth', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const disconnectGoogle = async () => {
    const actionId = beginIntegrationAction('google-disconnect');
    try {
      await apiPost('/api/integrations/google-calendar/disconnect', {});
      refreshIntegrationViews();
      finishIntegrationAction('google-disconnect', actionId, {
        status: 'success',
        message: 'Google Calendar disconnected.',
      });
    } catch (error) {
      finishIntegrationAction('google-disconnect', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const saveTodoistToken = async () => {
    const actionId = beginIntegrationAction('todoist-save');
    try {
      await apiPatch('/api/integrations/todoist', {
        api_token: todoistToken,
      });
      refreshIntegrationViews();
      setTodoistToken('');
      finishIntegrationAction('todoist-save', actionId, {
        status: 'success',
        message: 'Todoist token saved.',
      });
    } catch (error) {
      finishIntegrationAction('todoist-save', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const disconnectTodoist = async () => {
    const actionId = beginIntegrationAction('todoist-disconnect');
    try {
      await apiPost('/api/integrations/todoist/disconnect', {});
      refreshIntegrationViews();
      finishIntegrationAction('todoist-disconnect', actionId, {
        status: 'success',
        message: 'Todoist disconnected.',
      });
    } catch (error) {
      finishIntegrationAction('todoist-disconnect', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const syncSource = async (source: 'calendar' | 'todoist') => {
    const actionKey = source === 'calendar' ? 'google-sync' : 'todoist-sync';
    const actionId = beginIntegrationAction(actionKey);
    try {
      await apiPost(`/v1/sync/${source}`, {});
      refreshIntegrationViews();
      finishIntegrationAction(actionKey, actionId, {
        status: 'success',
        message: `${source === 'calendar' ? 'Calendar' : 'Todoist'} synced.`,
      });
    } catch (error) {
      finishIntegrationAction(actionKey, actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const toggleCalendarSelection = async (calendarId: string, selected: boolean) => {
    if (!integrations) {
      return;
    }
    const nextSelectedIds = integrations.google_calendar.calendars
      .filter((calendar) => (calendar.id === calendarId ? selected : calendar.selected))
      .map((calendar) => calendar.id);
    const actionId = beginIntegrationAction('google-calendars');
    try {
      await apiPatch('/api/integrations/google-calendar', {
        selected_calendar_ids: nextSelectedIds,
        all_calendars_selected: false,
      });
      refreshIntegrationViews();
      finishIntegrationAction('google-calendars', actionId, {
        status: 'success',
        message: 'Google Calendar selection updated.',
      });
    } catch (error) {
      finishIntegrationAction('google-calendars', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const setAllCalendarsSelected = async (selected: boolean) => {
    const actionId = beginIntegrationAction('google-calendars');
    try {
      await apiPatch('/api/integrations/google-calendar', {
        all_calendars_selected: selected,
      });
      refreshIntegrationViews();
      finishIntegrationAction('google-calendars', actionId, {
        status: 'success',
        message: selected ? 'All Google calendars selected.' : 'Calendar selection unlocked.',
      });
    } catch (error) {
      finishIntegrationAction('google-calendars', actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const beginRunAction = (runId: string): number => {
    const actionId = nextRunActionIdRef.current + 1;
    nextRunActionIdRef.current = actionId;
    latestRunActionIdByRunRef.current = {
      ...latestRunActionIdByRunRef.current,
      [runId]: actionId,
    };
    setActingRuns((current) => ({
      ...current,
      [runId]: true,
    }));
    setRunActionState((current) => {
      const next = { ...current };
      delete next[runId];
      return next;
    });
    return actionId;
  };

  const finishRunAction = (
    runId: string,
    actionId: number,
    nextState?: Omit<RunActionState, 'actionId'>,
  ) => {
    setActingRuns((current) => {
      if (!current[runId]) {
        return current;
      }
      const next = { ...current };
      delete next[runId];
      return next;
    });

    if (latestRunActionIdByRunRef.current[runId] !== actionId || !nextState) {
      return;
    }

    setRunActionState((current) => ({
      ...current,
      [runId]: {
        ...nextState,
        actionId,
      },
    }));
  };

  const scheduleRetry = async (run: RunSummaryData, allowUnsupportedRetry: boolean) => {
    const draft = retryDrafts[run.id];
    const reason = draft?.reason.trim()
      || (allowUnsupportedRetry ? 'operator_ui_override_retry' : 'operator_ui_retry');
    const retryAfterSeconds = parseRetryAfterSeconds(draft?.retryAfterSeconds);
    const actionId = beginRunAction(run.id);
    try {
      const response = await apiPatch(`/v1/runs/${run.id}`, {
        status: 'retry_scheduled',
        reason,
        retry_after_seconds: retryAfterSeconds,
        allow_unsupported_retry: allowUnsupportedRetry,
      });
      const updatedRun = extractRunSummaryData(response);
      if (updatedRun) {
        updateRunsCache(runsKey, runLimit, updatedRun);
      }
      finishRunAction(run.id, actionId, {
        action: 'retry',
        status: 'success',
        message: 'Retry scheduled.',
      });
      setPendingOverrideRunId((current) => (current === run.id ? null : current));
    } catch (error) {
      finishRunAction(run.id, actionId, {
        action: 'retry',
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const beginScheduleRetry = async (run: RunSummaryData) => {
    if (run.automatic_retry_supported) {
      await scheduleRetry(run, false);
      return;
    }

    setRunActionState((current) => {
      const next = { ...current };
      delete next[run.id];
      return next;
    });
    setPendingOverrideRunId(run.id);
  };

  const blockRun = async (run: RunSummaryData) => {
    const blockedReason = retryDrafts[run.id]?.blockedReason.trim() || 'operator_ui_blocked';
    setPendingOverrideRunId((current) => (current === run.id ? null : current));
    const actionId = beginRunAction(run.id);
    try {
      const response = await apiPatch(`/v1/runs/${run.id}`, {
        status: 'blocked',
        blocked_reason: blockedReason,
      });
      const updatedRun = extractRunSummaryData(response);
      if (updatedRun) {
        updateRunsCache(runsKey, runLimit, updatedRun);
      }
      finishRunAction(run.id, actionId, {
        action: 'block',
        status: 'success',
        message: 'Run blocked.',
      });
    } catch (error) {
      finishRunAction(run.id, actionId, {
        action: 'block',
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  if (loading) return <div className="p-8 text-zinc-500">Loading settings…</div>;

  return (
    <div className="flex-1 overflow-y-auto p-8 max-w-4xl">
      <button
        type="button"
        onClick={onBack}
        className="text-zinc-500 hover:text-zinc-300 text-sm mb-6"
      >
        ← Back
      </button>
      <h2 className="text-xl font-medium text-zinc-200 mb-6">Settings</h2>
      <div className="mb-8 flex gap-2 border-b border-zinc-800 pb-3">
        {(['general', 'integrations', 'runs'] as const).map((tab) => (
          <button
            key={tab}
            type="button"
            onClick={() => setActiveTab(tab)}
            className={`rounded-md px-3 py-1.5 text-sm capitalize ${
              activeTab === tab
                ? 'bg-zinc-800 text-white'
                : 'text-zinc-500 hover:text-zinc-300'
            }`}
          >
            {tab}
          </button>
        ))}
      </div>

      {activeTab === 'general' ? (
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
      ) : null}

      {activeTab === 'integrations' ? (
        <section className="space-y-8">
          {integrationsLoadError ? (
            <div className="rounded-lg border border-amber-900/80 bg-amber-950/30 p-4 text-sm text-amber-200">
              Integrations API unavailable: {integrationsLoadError}. Restart `veld` to pick up the new backend routes.
            </div>
          ) : null}
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5">
            <div className="flex items-start justify-between gap-4">
              <div>
                <h3 className="text-lg font-medium text-zinc-100">Google Calendar</h3>
                <p className="mt-1 text-sm text-zinc-500">
                  OAuth-backed event sync. All calendars are included by default.
                </p>
              </div>
              <IntegrationBadge
                connected={integrations.google_calendar.connected}
                status={integrations.google_calendar.last_sync_status}
              />
            </div>
            <div className="mt-4 grid gap-4 md:grid-cols-2">
              <label className="space-y-1">
                <span className="text-xs uppercase tracking-wide text-zinc-500">Client ID</span>
                <input
                  type="text"
                  value={googleClientId}
                  onChange={(event) => setGoogleClientId(event.target.value)}
                  placeholder={integrations.google_calendar.has_client_id ? 'Saved locally' : 'Google OAuth client ID'}
                  className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none"
                />
              </label>
              <label className="space-y-1">
                <span className="text-xs uppercase tracking-wide text-zinc-500">Client secret</span>
                <input
                  type="password"
                  value={googleClientSecret}
                  onChange={(event) => setGoogleClientSecret(event.target.value)}
                  placeholder={integrations.google_calendar.has_client_secret ? 'Saved locally' : 'Google OAuth client secret'}
                  className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none"
                />
              </label>
            </div>
            <div className="mt-4 flex flex-wrap gap-3">
              <button
                type="button"
                onClick={() => void saveGoogleCredentials()}
                disabled={Boolean(pendingIntegrationActions['google-save'])}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['google-save'] ? 'Saving…' : 'Save credentials'}
              </button>
              <button
                type="button"
                onClick={() => void startGoogleAuth()}
                disabled={Boolean(pendingIntegrationActions['google-auth']) || !integrations.google_calendar.configured}
                className="rounded-md border border-emerald-800 px-3 py-1.5 text-sm text-emerald-200 transition hover:border-emerald-600 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['google-auth'] ? 'Connecting…' : 'Connect Google'}
              </button>
              <button
                type="button"
                onClick={() => void syncSource('calendar')}
                disabled={Boolean(pendingIntegrationActions['google-sync']) || !integrations.google_calendar.connected}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['google-sync'] ? 'Syncing…' : 'Sync now'}
              </button>
              <button
                type="button"
                onClick={() => void disconnectGoogle()}
                disabled={Boolean(pendingIntegrationActions['google-disconnect']) || !integrations.google_calendar.connected}
                className="rounded-md border border-rose-900 px-3 py-1.5 text-sm text-rose-200 transition hover:border-rose-700 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['google-disconnect'] ? 'Disconnecting…' : 'Disconnect'}
              </button>
            </div>
            <IntegrationMeta
              lastSyncAt={integrations.google_calendar.last_sync_at}
              lastSyncStatus={integrations.google_calendar.last_sync_status}
              lastItemCount={integrations.google_calendar.last_item_count}
              lastError={integrations.google_calendar.last_error}
            />
            <div className="mt-5 rounded-md border border-zinc-800 bg-zinc-950/60 p-4">
              <label className="flex items-center gap-3 text-sm text-zinc-200">
                <input
                  type="checkbox"
                  checked={integrations.google_calendar.all_calendars_selected}
                  onChange={(event) => void setAllCalendarsSelected(event.target.checked)}
                  disabled={Boolean(pendingIntegrationActions['google-calendars'])}
                  className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
                />
                Sync all calendars by default
              </label>
              <div className="mt-4 space-y-2">
                {integrations.google_calendar.calendars.length === 0 ? (
                  <p className="text-sm text-zinc-500">
                    No calendars loaded yet. Connect Google, then run sync.
                  </p>
                ) : (
                  integrations.google_calendar.calendars.map((calendar) => (
                    <label key={calendar.id} className="flex items-center justify-between gap-3 text-sm">
                      <span className="text-zinc-200">
                        {calendar.summary}
                        {calendar.primary ? ' · primary' : ''}
                      </span>
                      <input
                        type="checkbox"
                        checked={integrations.google_calendar.all_calendars_selected || calendar.selected}
                        onChange={(event) => void toggleCalendarSelection(calendar.id, event.target.checked)}
                        disabled={Boolean(pendingIntegrationActions['google-calendars']) || integrations.google_calendar.all_calendars_selected}
                        className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
                      />
                    </label>
                  ))
                )}
              </div>
            </div>
            {googleFeedback.length > 0 ? (
              <div className="mt-4 space-y-1">
                {googleFeedback.map((entry) => (
                  <p
                    key={entry.action}
                    className={`text-sm ${entry.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}
                  >
                    {entry.message}
                  </p>
                ))}
              </div>
            ) : null}
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5">
            <div className="flex items-start justify-between gap-4">
              <div>
                <h3 className="text-lg font-medium text-zinc-100">Todoist</h3>
                <p className="mt-1 text-sm text-zinc-500">
                  Live task sync using your Todoist API token.
                </p>
              </div>
              <IntegrationBadge
                connected={integrations.todoist.connected}
                status={integrations.todoist.last_sync_status}
              />
            </div>
            <label className="mt-4 block space-y-1">
              <span className="text-xs uppercase tracking-wide text-zinc-500">API token</span>
              <input
                type="password"
                value={todoistToken}
                onChange={(event) => setTodoistToken(event.target.value)}
                placeholder={integrations.todoist.has_api_token ? 'Saved locally' : 'Todoist API token'}
                className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none"
              />
            </label>
            <div className="mt-4 flex flex-wrap gap-3">
              <button
                type="button"
                onClick={() => void saveTodoistToken()}
                disabled={Boolean(pendingIntegrationActions['todoist-save'])}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['todoist-save'] ? 'Saving…' : 'Save token'}
              </button>
              <button
                type="button"
                onClick={() => void syncSource('todoist')}
                disabled={Boolean(pendingIntegrationActions['todoist-sync']) || !integrations.todoist.connected}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['todoist-sync'] ? 'Syncing…' : 'Sync now'}
              </button>
              <button
                type="button"
                onClick={() => void disconnectTodoist()}
                disabled={Boolean(pendingIntegrationActions['todoist-disconnect']) || !integrations.todoist.connected}
                className="rounded-md border border-rose-900 px-3 py-1.5 text-sm text-rose-200 transition hover:border-rose-700 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
              >
                {pendingIntegrationActions['todoist-disconnect'] ? 'Disconnecting…' : 'Disconnect'}
              </button>
            </div>
            <IntegrationMeta
              lastSyncAt={integrations.todoist.last_sync_at}
              lastSyncStatus={integrations.todoist.last_sync_status}
              lastItemCount={integrations.todoist.last_item_count}
              lastError={integrations.todoist.last_error}
            />
            {todoistFeedback.length > 0 ? (
              <div className="mt-4 space-y-1">
                {todoistFeedback.map((entry) => (
                  <p
                    key={entry.action}
                    className={`text-sm ${entry.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}
                  >
                    {entry.message}
                  </p>
                ))}
              </div>
            ) : null}
          </div>
        </section>
      ) : null}

      {activeTab === 'runs' ? (
      <section className="mt-2">
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
                      disabled={!canScheduleRetry(run) || Boolean(actingRuns[run.id])}
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
                      disabled={!canScheduleRetry(run) || Boolean(actingRuns[run.id])}
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
                      disabled={!canControlRun(run) || Boolean(actingRuns[run.id])}
                      placeholder="operator_ui_blocked"
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                    />
                  </label>
                </div>
                <div className="mt-4 flex items-center gap-3">
                  <button
                    type="button"
                    onClick={() => void beginScheduleRetry(run)}
                    disabled={!canScheduleRetry(run) || Boolean(actingRuns[run.id])}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {actingRuns[run.id] ? 'Working…' : 'Schedule retry'}
                  </button>
                  <button
                    type="button"
                    onClick={() => void blockRun(run)}
                    disabled={!canControlRun(run) || Boolean(actingRuns[run.id])}
                    className="rounded-md border border-rose-900 px-3 py-1.5 text-sm text-rose-200 transition hover:border-rose-700 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {actingRuns[run.id] ? 'Working…' : 'Block run'}
                  </button>
                  {!run.automatic_retry_supported ? (
                    <span className="text-xs text-amber-300">Requires override</span>
                  ) : null}
                </div>
                {pendingOverrideRunId === run.id ? (
                  <div className="mt-3 rounded-md border border-amber-900/80 bg-amber-950/40 p-3 text-sm text-amber-200">
                    <p>
                      Automatic retry is unsupported for {run.kind}. Confirm the manual override to
                      schedule this retry anyway.
                    </p>
                    <div className="mt-3 flex items-center gap-3">
                      <button
                        type="button"
                        onClick={() => void scheduleRetry(run, true)}
                        disabled={Boolean(actingRuns[run.id])}
                        className="rounded-md border border-amber-700 px-3 py-1.5 text-sm text-amber-100 transition hover:border-amber-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                      >
                        {actingRuns[run.id] ? 'Working…' : 'Confirm override retry'}
                      </button>
                      <button
                        type="button"
                        onClick={() => setPendingOverrideRunId((current) => (current === run.id ? null : current))}
                        disabled={Boolean(actingRuns[run.id])}
                        className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-300 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                ) : null}
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
      ) : null}
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

function shouldKeepPendingOverride(run: RunSummaryData): boolean {
  return !run.automatic_retry_supported && canScheduleRetry(run);
}

function shouldKeepActionBanner(run: RunSummaryData, actionState: RunActionState): boolean {
  const actionMatchesRun = actionState.action === 'retry'
    ? run.status === 'retry_scheduled'
    : run.status === 'blocked';

  if (actionState.status === 'success') {
    return actionMatchesRun;
  }

  return !actionMatchesRun;
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

function IntegrationBadge({
  connected,
  status,
}: {
  connected: boolean;
  status: string | null;
}) {
  const label = connected ? (status ?? 'connected') : 'disconnected';
  return (
    <span
      className={`rounded-full px-2 py-1 text-xs ${
        connected ? 'bg-emerald-950 text-emerald-300' : 'bg-zinc-800 text-zinc-400'
      }`}
    >
      {label}
    </span>
  );
}

function IntegrationMeta({
  lastSyncAt,
  lastSyncStatus,
  lastItemCount,
  lastError,
}: {
  lastSyncAt: number | null;
  lastSyncStatus: string | null;
  lastItemCount: number | null;
  lastError: string | null;
}) {
  return (
    <div className="mt-4 space-y-1 text-sm text-zinc-400">
      <p>Last sync: {lastSyncAt ? new Date(lastSyncAt * 1000).toLocaleString() : 'never'}</p>
      <p>Status: {lastSyncStatus ?? 'unknown'}</p>
      <p>Items ingested: {lastItemCount ?? 0}</p>
      {lastError ? <p className="text-rose-400">Last error: {lastError}</p> : null}
    </div>
  );
}
