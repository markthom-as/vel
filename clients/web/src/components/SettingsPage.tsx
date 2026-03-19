import { useEffect, useMemo, useRef, useState } from 'react';
import { apiPost } from '../api/client';
import {
  CORE_DOCUMENTATION_ENTRIES,
  USER_DOCUMENTATION_ENTRIES,
} from '../data/documentationCatalog.generated';
import type {
  ClusterBootstrapData,
  ComponentData,
  ComponentLogEventData,
  DiagnosticsData,
  IntegrationLogEventData,
  IntegrationsData,
  LocalIntegrationData,
  LoopData,
  PairingTokenData,
  SettingsData,
  RunSummaryData,
} from '../types';
import { invalidateQuery, setQueryData, useQuery } from '../data/query';
import type { QueryKey } from '../data/query';
import { issuePairingToken } from '../data/operator';
import { subscribeWsQuerySync } from '../data/ws-sync';
import {
  disconnectGoogleCalendar,
  disconnectTodoist as disconnectTodoistIntegration,
  decodeGoogleCalendarAuthStartResponse,
  loadClusterBootstrap,
  loadComponentLogs,
  loadComponents,
  loadIntegrations,
  loadIntegrationLogs,
  loadLoops,
  loadRecentRuns,
  loadSettings,
  queryKeys,
  restartComponent,
  syncSource as syncSourceRequest,
  updateGoogleCalendarIntegration,
  updateLocalIntegrationSource,
  updateLoop,
  updateRun,
  updateSettings,
  updateTodoistIntegration,
} from '../data/resources';

interface SettingsPageProps {
  onBack: () => void;
  initialTab?: SettingsTab | LegacySettingsTab;
  initialIntegrationId?: IntegrationSectionKey;
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
  | 'activity-sync'
  | 'activity-save'
  | 'health-sync'
  | 'health-save'
  | 'git-sync'
  | 'git-save'
  | 'messaging-sync'
  | 'messaging-save'
  | 'notes-sync'
  | 'notes-save'
  | 'transcripts-sync'
  | 'transcripts-save'
  | 'todoist-save'
  | 'todoist-sync'
  | 'todoist-disconnect';
type IntegrationSectionKey =
  | 'google'
  | 'todoist'
  | 'activity'
  | 'health'
  | 'git'
  | 'messaging'
  | 'notes'
  | 'transcripts';
type LocalIntegrationSource = 'activity' | 'health' | 'git' | 'messaging' | 'notes' | 'transcripts';
type IntegrationLogSource = 'google-calendar' | 'todoist' | LocalIntegrationSource;

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

interface GuidanceActionButton {
  label: string;
  onClick: () => void;
  disabled?: boolean;
}

type LegacySettingsTab = 'components' | 'runs' | 'loops';
export type SettingsTab = 'general' | 'integrations' | 'runtime';

function normalizeSettingsTab(tab: SettingsTab | LegacySettingsTab): SettingsTab {
  if (tab === 'components' || tab === 'runs' || tab === 'loops') {
    return 'runtime';
  }
  return tab;
}

interface LoopDraft {
  intervalSeconds: string;
}

interface LoopActionState {
  status: 'success' | 'error';
  message: string;
}

type ComponentActionState =
  | {
      status: 'success' | 'error';
      message: string;
    }
  | { status: 'idle'; message: string };

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
    guidance: null,
  },
  todoist: {
    configured: false,
    connected: false,
    has_api_token: false,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  activity: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  health: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  git: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  messaging: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  notes: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
  transcripts: {
    configured: false,
    source_path: null,
    last_sync_at: null,
    last_sync_status: null,
    last_error: null,
    last_item_count: null,
    guidance: null,
  },
};

const LOCAL_INTEGRATION_SPECS: Array<{
  key: LocalIntegrationSource;
  title: string;
  description: string;
}> = [
  {
    key: 'activity',
    title: 'Computer Activity',
    description: 'Local workstation activity snapshots for attention and morning-state inference.',
  },
  {
    key: 'health',
    title: 'Health',
    description: 'Local health/activity snapshots for wellness-aware context and daily orientation.',
  },
  {
    key: 'git',
    title: 'Git Activity',
    description: 'Replay-safe git activity snapshots that improve coding context and explain surfaces.',
  },
  {
    key: 'messaging',
    title: 'Messaging',
    description: 'Local messaging thread snapshots for response debt and scheduling awareness.',
  },
  {
    key: 'notes',
    title: 'Obsidian Vault',
    description: 'Point Vel at your Obsidian vault root. Obsidian Sync handles replication; Vel ingests the synced markdown for recall and commitments.',
  },
  {
    key: 'transcripts',
    title: 'Transcripts',
    description: 'Assistant transcript snapshots for recall, synthesis, and thread continuity.',
  },
];

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

function integrationSectionForAction(key: IntegrationActionKey): IntegrationSectionKey {
  if (key.startsWith('google-')) {
    return 'google';
  }
  if (key.startsWith('todoist-')) {
    return 'todoist';
  }
  return key.replace(/-(sync|save)$/, '') as IntegrationSectionKey;
}

function integrationFeedbackForSection(
  feedback: Record<string, IntegrationFeedbackState>,
  section: IntegrationSectionKey,
) {
  return Object.values(feedback).filter((entry) => entry.section === section);
}

export function SettingsPage({
  onBack,
  initialTab = 'general',
  initialIntegrationId,
}: SettingsPageProps) {
  const [activeTab, setActiveTab] = useState<SettingsTab>(normalizeSettingsTab(initialTab));
  const [saving, setSaving] = useState(false);
  const [pendingIntegrationActions, setPendingIntegrationActions] = useState<Record<string, true>>({});
  const [pendingComponentActions, setPendingComponentActions] = useState<Record<string, true>>({});
  const [googleClientId, setGoogleClientId] = useState('');
  const [googleClientSecret, setGoogleClientSecret] = useState('');
  const [todoistToken, setTodoistToken] = useState('');
  const [timezoneDraft, setTimezoneDraft] = useState('');
  const [nodeDisplayNameDraft, setNodeDisplayNameDraft] = useState('');
  const [tailscaleBaseUrlDraft, setTailscaleBaseUrlDraft] = useState('');
  const [lanBaseUrlDraft, setLanBaseUrlDraft] = useState('');
  const [syncNetworkFeedback, setSyncNetworkFeedback] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [pairingScopes, setPairingScopes] = useState({
    read_context: true,
    write_safe_actions: false,
    execute_repo_tasks: false,
  });
  const [pairingToken, setPairingToken] = useState<PairingTokenData | null>(null);
  const [pairingFeedback, setPairingFeedback] = useState<{
    status: 'success' | 'error';
    message: string;
  } | null>(null);
  const [issuingPairingToken, setIssuingPairingToken] = useState(false);
  const [localSourceDrafts, setLocalSourceDrafts] = useState<Record<LocalIntegrationSource, string>>({
    activity: '',
    health: '',
    git: '',
    messaging: '',
    notes: '',
    transcripts: '',
  });
  const [integrationFeedback, setIntegrationFeedback] = useState<Record<string, IntegrationFeedbackState>>({});
  const [componentActions, setComponentActions] = useState<Record<string, ComponentActionState>>({});
  const [expandedComponentLogs, setExpandedComponentLogs] = useState<Record<string, true>>({});
  const [expandedIntegrationLogs, setExpandedIntegrationLogs] = useState<Record<string, true>>({});
  const [actingRuns, setActingRuns] = useState<Record<string, true>>({});
  const [pendingOverrideRunId, setPendingOverrideRunId] = useState<string | null>(null);
  const [retryDrafts, setRetryDrafts] = useState<Record<string, RetryDraft>>({});
  const [runActionState, setRunActionState] = useState<Record<string, RunActionState>>({});
  const [actingLoops, setActingLoops] = useState<Record<string, true>>({});
  const [loopDrafts, setLoopDrafts] = useState<Record<string, LoopDraft>>({});
  const [loopActionState, setLoopActionState] = useState<Record<string, LoopActionState>>({});
  const [diagnostics, setDiagnostics] = useState<DiagnosticsData | null>(null);
  const nextIntegrationActionIdRef = useRef(0);
  const latestIntegrationActionIdByKeyRef = useRef<Record<IntegrationActionKey, number>>(
    {} as Record<IntegrationActionKey, number>,
  );
  const nextRunActionIdRef = useRef(0);
  const nextComponentActionIdRef = useRef(0);
  const latestComponentActionIdRef = useRef(0);
  const latestRunActionIdByRunRef = useRef<Record<string, number>>({});
  const localSourceInputRefs = useRef<Record<LocalIntegrationSource, HTMLInputElement | null>>({
    activity: null,
    health: null,
    git: null,
    messaging: null,
    notes: null,
    transcripts: null,
  });
  const integrationSectionRefs = useRef<Record<IntegrationSectionKey, HTMLDivElement | null>>({
    google: null,
    todoist: null,
    activity: null,
    health: null,
    git: null,
    messaging: null,
    notes: null,
    transcripts: null,
  });
  const runLimit = 6;
  const settingsKey = useMemo(() => queryKeys.settings(), []);
  const clusterBootstrapKey = useMemo(() => queryKeys.clusterBootstrap(), []);
  const integrationsKey = useMemo(() => queryKeys.integrations(), []);
  const componentsKey = useMemo(() => queryKeys.components(), []);
  const runsKey = useMemo(() => queryKeys.runs(runLimit), []);
  const loopsKey = useMemo(() => queryKeys.loops(), []);
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
  const { data: clusterBootstrap, error: clusterBootstrapError } = useQuery<ClusterBootstrapData>(
    clusterBootstrapKey,
    async () => {
      const response = await loadClusterBootstrap();
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to load cluster bootstrap');
      }
      return response.data;
    },
    { enabled: activeTab === 'general' },
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
  const activityFeedback = integrationFeedbackForSection(integrationFeedback, 'activity');
  const gitFeedback = integrationFeedbackForSection(integrationFeedback, 'git');
  const messagingFeedback = integrationFeedbackForSection(integrationFeedback, 'messaging');
  const notesFeedback = integrationFeedbackForSection(integrationFeedback, 'notes');
  const transcriptsFeedback = integrationFeedbackForSection(integrationFeedback, 'transcripts');
  const { data: runs = [] } = useQuery<RunSummaryData[]>(
    runsKey,
    async () => {
      const response = await loadRecentRuns(runLimit);
      return response.ok && response.data ? response.data : [];
    },
  );
  const { data: components = [], error: componentsLoadError } = useQuery<ComponentData[]>(
    componentsKey,
    async () => {
      const response = await loadComponents();
      return response.ok && response.data ? response.data : [];
    },
    { enabled: activeTab === 'runtime' },
  );
  const { data: loops = [] } = useQuery<LoopData[]>(
    loopsKey,
    async () => {
      const response = await loadLoops();
      return response.ok && response.data ? response.data : [];
    },
    { enabled: activeTab === 'runtime' },
  );

  useEffect(() => {
    return subscribeWsQuerySync();
  }, []);

  useEffect(() => {
    setActiveTab(normalizeSettingsTab(initialTab));
  }, [initialTab]);

  useEffect(() => {
    setTimezoneDraft(settings.timezone ?? '');
  }, [settings.timezone]);

  useEffect(() => {
    setNodeDisplayNameDraft(settings.node_display_name ?? '');
  }, [settings.node_display_name]);

  useEffect(() => {
    setTailscaleBaseUrlDraft(settings.tailscale_base_url ?? '');
  }, [settings.tailscale_base_url]);

  useEffect(() => {
    setLanBaseUrlDraft(settings.lan_base_url ?? '');
  }, [settings.lan_base_url]);

  useEffect(() => {
    setLocalSourceDrafts((current) => {
      const next = { ...current };
      let changed = false;
      for (const spec of LOCAL_INTEGRATION_SPECS) {
        const configuredPath = integrations[spec.key].source_path ?? '';
        if (current[spec.key] === '' || current[spec.key] === configuredPath) {
          if (current[spec.key] !== configuredPath) {
            next[spec.key] = configuredPath;
            changed = true;
          }
        }
      }
      return changed ? next : current;
    });
  }, [integrations]);

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

  useEffect(() => {
    if (loops.length === 0) {
      return;
    }
    setLoopDrafts((current) => {
      let changed = false;
      const next = { ...current };
      for (const loop of loops) {
        if (!next[loop.kind]) {
          next[loop.kind] = { intervalSeconds: String(loop.interval_seconds) };
          changed = true;
        }
      }
      return changed ? next : current;
    });
  }, [loops]);

  useEffect(() => {
    if (activeTab !== 'integrations' || !initialIntegrationId) {
      return;
    }
    integrationSectionRefs.current[initialIntegrationId]?.scrollIntoView({
      block: 'start',
      behavior: 'auto',
    });
  }, [activeTab, initialIntegrationId, integrations]);

  useEffect(() => {
    let cancelled = false;
    fetch('/api/diagnostics', {
      headers: { 'Content-Type': 'application/json' },
    })
      .then((res) => res.json())
      .then((body: { ok: boolean; data?: DiagnosticsData }) => {
        if (!cancelled && body.ok && body.data) {
          setDiagnostics(body.data);
        }
      })
      .catch(() => {
        // diagnostics fetch failure is non-critical — silently ignore
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const update = async (key: keyof SettingsData, value: boolean | unknown) => {
    setSaving(true);
    try {
      await updateSettings({ [key]: value });
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
    Object.keys(expandedIntegrationLogs).forEach((integrationId) => {
      invalidateQuery(queryKeys.integrationLogs(integrationId), { refetch: true });
    });
  };

  const saveSyncNetworkSettings = async () => {
    setSaving(true);
    setSyncNetworkFeedback(null);
    try {
      const response = await updateSettings({
        node_display_name: nodeDisplayNameDraft.trim() || null,
        tailscale_base_url: tailscaleBaseUrlDraft.trim() || null,
        lan_base_url: lanBaseUrlDraft.trim() || null,
      });
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to save sync settings');
      }
      if (response.data) {
        setQueryData<SettingsData>(settingsKey, () => response.data as SettingsData);
      }
      invalidateQuery(clusterBootstrapKey, { refetch: true });
      setSyncNetworkFeedback({
        status: 'success',
        message: 'Cross-client sync settings saved.',
      });
    } catch (error) {
      setSyncNetworkFeedback({
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    } finally {
      setSaving(false);
    }
  };

  const handleIssuePairingToken = async () => {
    if (!clusterBootstrap) {
      setPairingFeedback({
        status: 'error',
        message: 'Cluster bootstrap must load before Vel can issue a pairing token.',
      });
      return;
    }

    setIssuingPairingToken(true);
    setPairingFeedback(null);
    try {
      const response = await issuePairingToken({
        issued_by_node_id: clusterBootstrap.node_id,
        scopes: pairingScopes,
      });
      if (!response.ok || !response.data) {
        throw new Error(response.error?.message ?? 'Failed to issue pairing token');
      }
      setPairingToken(response.data);
      setPairingFeedback({
        status: 'success',
        message: 'Pairing token issued. Redeem it on the companion node, then refresh this page to confirm linked status.',
      });
    } catch (error) {
      setPairingFeedback({
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    } finally {
      setIssuingPairingToken(false);
    }
  };

  const refreshComponentViews = () => {
    invalidateQuery(componentsKey, { refetch: true });
    Object.keys(expandedComponentLogs).forEach((componentId) => {
      invalidateQuery(queryKeys.componentLogs(componentId), { refetch: true });
    });
  };

  const updateComponentLogsVisibility = (componentId: string, nextVisible: boolean) => {
    setExpandedComponentLogs((current) => {
      if (nextVisible) {
        return {
          ...current,
          [componentId]: true,
        };
      }
      const next = { ...current };
      delete next[componentId];
      return next;
    });
  };

  const updateIntegrationLogsVisibility = (integrationId: IntegrationLogSource, nextVisible: boolean) => {
    setExpandedIntegrationLogs((current) => {
      if (nextVisible) {
        return {
          ...current,
          [integrationId]: true,
        };
      }
      const next = { ...current };
      delete next[integrationId];
      return next;
    });
  };

  const beginComponentAction = (componentId: string): number => {
    const actionId = nextComponentActionIdRef.current + 1;
    nextComponentActionIdRef.current = actionId;
    latestComponentActionIdRef.current = actionId;
    setPendingComponentActions((current) => ({
      ...current,
      [componentId]: true,
    }));
    setComponentActions((current) => {
      const next = { ...current };
      delete next[componentId];
      return next;
    });
    return actionId;
  };

  const finishComponentAction = (
    componentId: string,
    actionId: number,
    message?: Omit<ComponentActionState, 'message'> & { message: string },
  ) => {
    setPendingComponentActions((current) => {
      if (!current[componentId]) {
        return current;
      }
      const next = { ...current };
      delete next[componentId];
      return next;
    });

    if (latestComponentActionIdRef.current !== actionId || !message) {
      return;
    }

    setComponentActions((current) => ({
      ...current,
      [componentId]: message,
    }));
  };

  const restartComponentAction = async (component: ComponentData) => {
    const actionId = beginComponentAction(component.id);
    try {
      const response = await restartComponent(component.id);
      if (!response.ok) {
        throw new Error(response.error?.message ?? 'Failed to restart component');
      }
      refreshComponentViews();
      finishComponentAction(component.id, actionId, {
        status: 'success',
        message: `${component.name} restarted (${response.data?.status ?? 'ok'}).`,
      });
    } catch (error) {
      finishComponentAction(component.id, actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const beginIntegrationAction = (key: IntegrationActionKey): number => {
    const actionId = nextIntegrationActionIdRef.current + 1;
    nextIntegrationActionIdRef.current = actionId;
    latestIntegrationActionIdByKeyRef.current[key] = actionId;
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

    if (latestIntegrationActionIdByKeyRef.current[key] !== actionId || !nextFeedback) {
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
      await updateGoogleCalendarIntegration({
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
      await disconnectGoogleCalendar();
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
      await updateTodoistIntegration({
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
      await disconnectTodoistIntegration();
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

  const syncSource = async (
    source: 'calendar' | 'todoist' | LocalIntegrationSource,
  ) => {
    const actionKey: IntegrationActionKey =
      source === 'calendar'
        ? 'google-sync'
        : source === 'todoist'
          ? 'todoist-sync'
          : `${source}-sync`;
    const actionId = beginIntegrationAction(actionKey);
    try {
      const response = await syncSourceRequest(source);
      refreshIntegrationViews();
      const label = source === 'calendar'
        ? 'Calendar'
        : source === 'todoist'
          ? 'Todoist'
          : LOCAL_INTEGRATION_SPECS.find((spec) => spec.key === source)?.title ?? source;
      finishIntegrationAction(actionKey, actionId, {
        status: 'success',
        message: `${label} synced (${response.data?.signals_ingested ?? 0} signals).`,
      });
    } catch (error) {
      finishIntegrationAction(actionKey, actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const focusLocalSourceInput = (source: LocalIntegrationSource) => {
    const input = localSourceInputRefs.current[source];
    if (!input) {
      return;
    }
    input.focus();
    input.select();
  };

  const saveLocalSourcePath = async (
    source: LocalIntegrationSource,
    sourcePathOverride?: string,
  ) => {
    const actionKey = `${source}-save` as IntegrationActionKey;
    const actionId = beginIntegrationAction(actionKey);
    const sourcePath = (sourcePathOverride ?? localSourceDrafts[source]).trim();
    try {
      const response = await updateLocalIntegrationSource(source, {
        source_path: sourcePath.length > 0 ? sourcePath : null,
      });
      const nextIntegrations = response.ok ? response.data ?? null : null;
      if (!nextIntegrations) {
        throw new Error(response.error?.message ?? 'Failed to save source path');
      }
      setQueryData<IntegrationsData>(integrationsKey, () => nextIntegrations);
      setLocalSourceDrafts((current) => ({
        ...current,
        [source]: nextIntegrations[source].source_path ?? '',
      }));
      refreshIntegrationViews();
      finishIntegrationAction(actionKey, actionId, {
        status: 'success',
        message:
          sourcePath.length > 0
            ? 'Source path saved.'
            : 'Source path cleared.',
      });
    } catch (error) {
      finishIntegrationAction(actionKey, actionId, {
        status: 'error',
        message: error instanceof Error ? error.message : String(error),
      });
    }
  };

  const openIntegrationHistory = (integrationId: IntegrationLogSource) => {
    updateIntegrationLogsVisibility(integrationId, true);
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
      await updateGoogleCalendarIntegration({
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
      await updateGoogleCalendarIntegration({
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
      const response = await updateRun(run.id, {
        status: 'retry_scheduled',
        reason,
        retry_after_seconds: retryAfterSeconds,
        allow_unsupported_retry: allowUnsupportedRetry,
      });
      const updatedRun = response.ok ? response.data ?? null : null;
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
      const response = await updateRun(run.id, {
        status: 'blocked',
        blocked_reason: blockedReason,
      });
      const updatedRun = response.ok ? response.data ?? null : null;
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

  const saveLoop = async (loop: LoopData, enabled: boolean) => {
    const draft = loopDrafts[loop.kind];
    const intervalSeconds = Number.parseInt(
      draft?.intervalSeconds?.trim() || String(loop.interval_seconds),
      10,
    );
    if (!Number.isFinite(intervalSeconds) || intervalSeconds <= 0) {
      setLoopActionState((current) => ({
        ...current,
        [loop.kind]: {
          status: 'error',
          message: 'Interval seconds must be a positive integer.',
        },
      }));
      return;
    }

    setActingLoops((current) => ({
      ...current,
      [loop.kind]: true,
    }));
    setLoopActionState((current) => {
      const next = { ...current };
      delete next[loop.kind];
      return next;
    });

    try {
      const response = await updateLoop(loop.kind, {
        enabled,
        interval_seconds: intervalSeconds,
      });
      const updatedLoop = response.ok ? response.data ?? null : null;
      if (updatedLoop) {
        setQueryData<LoopData[]>(loopsKey, (current = []) => current.map((entry) => (
          entry.kind === updatedLoop.kind ? updatedLoop : entry
        )));
        setLoopDrafts((current) => ({
          ...current,
          [updatedLoop.kind]: {
            intervalSeconds: String(updatedLoop.interval_seconds),
          },
        }));
      }
      setLoopActionState((current) => ({
        ...current,
        [loop.kind]: {
          status: 'success',
          message: 'Loop updated.',
        },
      }));
    } catch (error) {
      setLoopActionState((current) => ({
        ...current,
        [loop.kind]: {
          status: 'error',
          message: error instanceof Error ? error.message : String(error),
        },
      }));
    } finally {
      setActingLoops((current) => {
        const next = { ...current };
        delete next[loop.kind];
        return next;
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
        {(['general', 'integrations', 'runtime'] as const).map((tab) => (
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
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <div className="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
              <label className="flex-1 space-y-1">
                <span className="text-zinc-300">Timezone</span>
                <p className="text-sm text-zinc-500">
                  IANA timezone used for local day boundaries and Now timestamps.
                </p>
                <input
                  type="text"
                  value={timezoneDraft}
                  onChange={(event) => setTimezoneDraft(event.target.value)}
                  placeholder="America/Denver"
                  disabled={saving}
                  className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                />
              </label>
              <button
                type="button"
                onClick={() => update('timezone', timezoneDraft.trim())}
                disabled={saving || timezoneDraft.trim() === (settings.timezone ?? '')}
                className="rounded-md bg-emerald-700 px-3 py-2 text-sm text-white disabled:cursor-not-allowed disabled:bg-zinc-700"
              >
                Save timezone
              </button>
            </div>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-zinc-200">Cross-client Sync</h3>
                <p className="text-sm text-zinc-500">
                  Commitments are the global task authority across Vel clients. Prefer a Tailscale endpoint so Apple clients and other nodes resolve the same daemon consistently.
                </p>
              </div>
              <div className="grid gap-4 md:grid-cols-2">
                <label className="space-y-1">
                  <span className="text-xs uppercase tracking-wide text-zinc-500">Node display name</span>
                  <input
                    type="text"
                    value={nodeDisplayNameDraft}
                    onChange={(event) => setNodeDisplayNameDraft(event.target.value)}
                    placeholder="Vel Desktop"
                    disabled={saving}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                  />
                </label>
                <label className="space-y-1">
                  <span className="text-xs uppercase tracking-wide text-zinc-500">Tailscale URL</span>
                  <input
                    type="text"
                    value={tailscaleBaseUrlDraft}
                    onChange={(event) => setTailscaleBaseUrlDraft(event.target.value)}
                    placeholder="http://vel-desktop.tailnet.ts.net:4130"
                    disabled={saving}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                  />
                </label>
                <label className="space-y-1 md:col-span-2">
                  <span className="text-xs uppercase tracking-wide text-zinc-500">LAN fallback URL</span>
                  <input
                    type="text"
                    value={lanBaseUrlDraft}
                    onChange={(event) => setLanBaseUrlDraft(event.target.value)}
                    placeholder="http://192.168.1.50:4130"
                    disabled={saving}
                    className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-600"
                  />
                </label>
              </div>
              <div className="flex flex-wrap items-center gap-3">
                <button
                  type="button"
                  onClick={() => void saveSyncNetworkSettings()}
                  disabled={
                    saving
                    || (
                      nodeDisplayNameDraft.trim() === (settings.node_display_name ?? '')
                      && tailscaleBaseUrlDraft.trim() === (settings.tailscale_base_url ?? '')
                      && lanBaseUrlDraft.trim() === (settings.lan_base_url ?? '')
                    )
                  }
                  className="rounded-md bg-emerald-700 px-3 py-2 text-sm text-white disabled:cursor-not-allowed disabled:bg-zinc-700"
                >
                  Save sync settings
                </button>
                <p className="text-sm text-zinc-500">
                  Apple clients should set `vel_tailscale_url` to the same Tailscale URL shown here.
                </p>
              </div>
              {clusterBootstrap ? (
                <dl className="grid gap-2 text-sm text-zinc-300 md:grid-cols-2">
                  <div className="rounded-md border border-zinc-800 bg-zinc-950/70 p-3">
                    <dt className="text-zinc-500">Effective transport</dt>
                    <dd className="mt-1 text-base text-zinc-100">{clusterBootstrap.sync_transport}</dd>
                  </div>
                  <div className="rounded-md border border-zinc-800 bg-zinc-950/70 p-3">
                    <dt className="text-zinc-500">Effective sync base URL</dt>
                    <dd className="mt-1 break-all text-base text-zinc-100">{clusterBootstrap.sync_base_url}</dd>
                  </div>
                </dl>
              ) : null}
              {clusterBootstrapError ? (
                <p className="text-sm text-amber-300">
                  Cluster bootstrap unavailable: {clusterBootstrapError}
                </p>
              ) : null}
              {syncNetworkFeedback ? (
                <p className={`text-sm ${syncNetworkFeedback.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}>
                  {syncNetworkFeedback.message}
                </p>
              ) : null}
            </div>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-zinc-200">Linked devices</h3>
                <p className="text-sm text-zinc-500">
                  Guided linking stays linear: issue pairing token, disclose the granted scopes,
                  redeem the token on the companion node, then confirm continuity from the linked
                  status cards below.
                </p>
              </div>
              <div className="grid gap-3 md:grid-cols-3">
                <label className="flex items-start gap-3 rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                  <input
                    type="checkbox"
                    checked={pairingScopes.read_context}
                    onChange={(event) =>
                      setPairingScopes((current) => ({
                        ...current,
                        read_context: event.target.checked,
                      }))}
                    disabled={issuingPairingToken}
                    className="mt-1 rounded border-zinc-600 bg-zinc-900 text-emerald-600 focus:ring-emerald-500"
                  />
                  <span className="text-sm text-zinc-300">Read context</span>
                </label>
                <label className="flex items-start gap-3 rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                  <input
                    type="checkbox"
                    checked={pairingScopes.write_safe_actions}
                    onChange={(event) =>
                      setPairingScopes((current) => ({
                        ...current,
                        write_safe_actions: event.target.checked,
                      }))}
                    disabled={issuingPairingToken}
                    className="mt-1 rounded border-zinc-600 bg-zinc-900 text-emerald-600 focus:ring-emerald-500"
                  />
                  <span className="text-sm text-zinc-300">Write safe actions</span>
                </label>
                <label className="flex items-start gap-3 rounded-lg border border-zinc-800 bg-zinc-950/60 p-3">
                  <input
                    type="checkbox"
                    checked={pairingScopes.execute_repo_tasks}
                    onChange={(event) =>
                      setPairingScopes((current) => ({
                        ...current,
                        execute_repo_tasks: event.target.checked,
                      }))}
                    disabled={issuingPairingToken}
                    className="mt-1 rounded border-zinc-600 bg-zinc-900 text-emerald-600 focus:ring-emerald-500"
                  />
                  <span className="text-sm text-zinc-300">Execute repo tasks</span>
                </label>
              </div>
              <div className="flex flex-wrap items-center gap-3">
                <button
                  type="button"
                  onClick={() => void handleIssuePairingToken()}
                  disabled={issuingPairingToken || !clusterBootstrap}
                  className="min-h-[44px] rounded-md bg-emerald-600 px-3 py-2 text-sm font-medium text-zinc-950 disabled:cursor-not-allowed disabled:bg-zinc-700 disabled:text-zinc-300"
                >
                  {issuingPairingToken ? 'Issuing…' : 'Issue pairing token'}
                </button>
                <p className="text-sm text-zinc-500">
                  CLI fallback: `vel node link issue --scope-read-context --scope-write-safe-actions`
                </p>
              </div>
              {pairingToken ? (
                <div className="rounded-lg border border-emerald-500/40 bg-emerald-500/10 p-4">
                  <p className="text-sm font-medium text-emerald-200">Granted scopes</p>
                  <p className="mt-2 break-all font-mono text-sm text-zinc-100">{pairingToken.token_code}</p>
                  <p className="mt-2 text-sm text-zinc-300">
                    Expires {formatRuntimeTimestamp(pairingToken.expires_at)}.
                  </p>
                  <div className="mt-3 flex flex-wrap gap-2">
                    {scopeSummaryEntries(pairingToken.scopes).map((scope) => (
                      <span
                        key={scope.label}
                        className={`rounded-full px-2.5 py-1 text-xs ${
                          scope.enabled
                            ? 'bg-emerald-900/60 text-emerald-200'
                            : 'bg-zinc-800 text-zinc-500'
                        }`}
                      >
                        {scope.label}
                      </span>
                    ))}
                  </div>
                </div>
              ) : null}
              {pairingFeedback ? (
                <p className={`text-sm ${pairingFeedback.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}>
                  {pairingFeedback.message}
                </p>
              ) : null}
              <div className="space-y-3">
                <p className="text-xs uppercase tracking-[0.16em] text-zinc-500">linkedNodes</p>
                {clusterBootstrap?.linked_nodes?.length ? (
                  <div className="grid gap-3 md:grid-cols-2">
                    {clusterBootstrap.linked_nodes.map((node) => (
                      <article
                        key={node.node_id}
                        className="rounded-lg border border-zinc-800 bg-zinc-950/70 p-4"
                      >
                        <div className="flex items-start justify-between gap-3">
                          <div>
                            <h4 className="text-sm font-medium text-zinc-100">{node.node_display_name}</h4>
                            <p className="mt-1 text-xs text-zinc-500">{node.node_id}</p>
                          </div>
                          <span className={linkStatusClassName(node.status)}>
                            {node.status}
                          </span>
                        </div>
                        <div className="mt-3 space-y-2 text-sm text-zinc-400">
                          <p>Transport: {node.transport_hint ?? 'No transport hint'}</p>
                          <p>
                            Last seen:{' '}
                            {node.last_seen_at ? formatRuntimeTimestamp(node.last_seen_at) : 'Not observed yet'}
                          </p>
                        </div>
                        <div className="mt-3 flex flex-wrap gap-2">
                          {scopeSummaryEntries(node.scopes).map((scope) => (
                            <span
                              key={`${node.node_id}-${scope.label}`}
                              className={`rounded-full px-2.5 py-1 text-xs ${
                                scope.enabled
                                  ? 'bg-emerald-900/50 text-emerald-200'
                                  : 'bg-zinc-800 text-zinc-500'
                              }`}
                            >
                              {scope.label}
                            </span>
                          ))}
                        </div>
                      </article>
                    ))}
                  </div>
                ) : (
                  <p className="rounded-lg border border-dashed border-zinc-800 bg-zinc-950/60 px-4 py-3 text-sm text-zinc-500">
                    No linked devices yet. Issue a token here or use the CLI fallback to pair a
                    companion node.
                  </p>
                )}
              </div>
            </div>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <div className="space-y-2">
              <h3 className="text-sm font-medium text-zinc-200">Adaptive policy overrides</h3>
              <p className="text-sm text-zinc-500">
                Active runtime adjustments learned from accepted suggestions.
              </p>
              {settings.adaptive_policy_overrides?.commute_buffer_minutes == null
                && settings.adaptive_policy_overrides?.default_prep_minutes == null ? (
                  <p className="text-sm text-zinc-400">No adaptive overrides are active.</p>
                ) : (
                  <dl className="grid gap-2 text-sm text-zinc-300 md:grid-cols-2">
                    <div className="rounded-md border border-zinc-800 bg-zinc-950/70 p-3">
                      <dt className="text-zinc-500">Commute buffer</dt>
                      <dd className="mt-1 text-base text-zinc-100">
                        {settings.adaptive_policy_overrides?.commute_buffer_minutes == null
                          ? 'Default policy'
                          : `${settings.adaptive_policy_overrides.commute_buffer_minutes} min`}
                      </dd>
                      {settings.adaptive_policy_overrides?.commute_buffer_source_title
                        || settings.adaptive_policy_overrides?.commute_buffer_source_suggestion_id ? (
                          <p className="mt-2 text-xs text-zinc-500">
                            From{' '}
                            {settings.adaptive_policy_overrides?.commute_buffer_source_title
                              ?? settings.adaptive_policy_overrides?.commute_buffer_source_suggestion_id}
                          </p>
                        ) : null}
                    </div>
                    <div className="rounded-md border border-zinc-800 bg-zinc-950/70 p-3">
                      <dt className="text-zinc-500">Default prep window</dt>
                      <dd className="mt-1 text-base text-zinc-100">
                        {settings.adaptive_policy_overrides?.default_prep_minutes == null
                          ? 'Default policy'
                          : `${settings.adaptive_policy_overrides.default_prep_minutes} min`}
                      </dd>
                      {settings.adaptive_policy_overrides?.default_prep_source_title
                        || settings.adaptive_policy_overrides?.default_prep_source_suggestion_id ? (
                          <p className="mt-2 text-xs text-zinc-500">
                            From{' '}
                            {settings.adaptive_policy_overrides?.default_prep_source_title
                              ?? settings.adaptive_policy_overrides?.default_prep_source_suggestion_id}
                          </p>
                        ) : null}
                    </div>
                  </dl>
                )}
            </div>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-zinc-200">Documentation</h3>
                <p className="text-sm text-zinc-500">
                  Core Vel docs and user-specific operating docs are part of the product surface. Open these repo paths locally when you need authoritative guidance.
                </p>
              </div>
              <div className="grid gap-3 md:grid-cols-2">
                <DocumentationCard
                  title="Core documentation"
                  docs={CORE_DOCUMENTATION_ENTRIES}
                />
                <DocumentationCard
                  title="Your Vel documentation"
                  docs={USER_DOCUMENTATION_ENTRIES}
                />
              </div>
            </div>
          </div>
        </div>
      ) : null}

      {activeTab === 'integrations' ? (
        <section className="space-y-8">
          <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
            <h3 className="text-base font-medium text-zinc-100">Integration control plane</h3>
            <p className="mt-1 text-sm text-zinc-500">
              Configure participation and run sync from here. Use Stats for cross-integration diagnostics and long-form runtime inspection.
            </p>
          </div>
          {integrationsLoadError ? (
            <div className="rounded-lg border border-amber-900/80 bg-amber-950/30 p-4 text-sm text-amber-200">
              Integrations API unavailable: {integrationsLoadError}. Restart `veld` to pick up the new backend routes.
            </div>
          ) : null}
          <div
            ref={(node) => {
              integrationSectionRefs.current.google = node;
            }}
            className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5"
          >
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
              <button
                type="button"
                onClick={() => updateIntegrationLogsVisibility('google-calendar', !expandedIntegrationLogs['google-calendar'])}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white"
              >
                {expandedIntegrationLogs['google-calendar'] ? 'Hide history' : 'Show history'}
              </button>
            </div>
            <IntegrationMeta
              lastSyncAt={integrations.google_calendar.last_sync_at}
              lastSyncStatus={integrations.google_calendar.last_sync_status}
              lastItemCount={integrations.google_calendar.last_item_count}
              lastError={integrations.google_calendar.last_error}
              guidance={integrations.google_calendar.guidance}
              guidanceActions={googleGuidanceActions(
                integrations,
                expandedIntegrationLogs,
                pendingIntegrationActions,
                saveGoogleCredentials,
                startGoogleAuth,
                syncSource,
                openIntegrationHistory,
              )}
            />
            {expandedIntegrationLogs['google-calendar'] ? (
              <IntegrationLogPanel integrationId="google-calendar" />
            ) : null}
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

          <div
            ref={(node) => {
              integrationSectionRefs.current.todoist = node;
            }}
            className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5"
          >
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
              <button
                type="button"
                onClick={() => updateIntegrationLogsVisibility('todoist', !expandedIntegrationLogs.todoist)}
                className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white"
              >
                {expandedIntegrationLogs.todoist ? 'Hide history' : 'Show history'}
              </button>
            </div>
            <IntegrationMeta
              lastSyncAt={integrations.todoist.last_sync_at}
              lastSyncStatus={integrations.todoist.last_sync_status}
              lastItemCount={integrations.todoist.last_item_count}
              lastError={integrations.todoist.last_error}
              guidance={integrations.todoist.guidance}
              guidanceActions={todoistGuidanceActions(
                integrations,
                expandedIntegrationLogs,
                pendingIntegrationActions,
                saveTodoistToken,
                syncSource,
                openIntegrationHistory,
              )}
            />
            {expandedIntegrationLogs.todoist ? <IntegrationLogPanel integrationId="todoist" /> : null}
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

          {LOCAL_INTEGRATION_SPECS.map((spec) => {
            const integration = integrations[spec.key] as LocalIntegrationData;
            const feedback = ({
              activity: activityFeedback,
              health: integrationFeedbackForSection(integrationFeedback, 'health'),
              git: gitFeedback,
              messaging: messagingFeedback,
              notes: notesFeedback,
              transcripts: transcriptsFeedback,
            } as const)[spec.key] ?? [];
            const syncActionKey = `${spec.key}-sync` as IntegrationActionKey;
            const saveActionKey = `${spec.key}-save` as IntegrationActionKey;
            const sourceDraft = localSourceDrafts[spec.key];
            return (
              <div
                key={spec.key}
                ref={(node) => {
                  integrationSectionRefs.current[spec.key] = node;
                }}
                className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5"
              >
                <div className="flex items-start justify-between gap-4">
                  <div>
                    <h3 className="text-lg font-medium text-zinc-100">{spec.title}</h3>
                    <p className="mt-1 text-sm text-zinc-500">{spec.description}</p>
                  </div>
                  <IntegrationBadge
                    connected={integration.configured}
                    status={integration.last_sync_status}
                  />
                </div>
                <div className="mt-4 flex flex-wrap gap-3">
                  <label className="min-w-[18rem] flex-1 space-y-1">
                    <span className="text-xs uppercase tracking-wide text-zinc-500">Source path</span>
                    <input
                      ref={(node) => {
                        localSourceInputRefs.current[spec.key] = node;
                      }}
                      type="text"
                      value={sourceDraft}
                      onChange={(event) => {
                        const nextValue = event.target.value;
                        setLocalSourceDrafts((current) => ({
                          ...current,
                          [spec.key]: nextValue,
                        }));
                      }}
                      placeholder={
                        spec.key === 'notes'
                          ? 'Path to your Obsidian vault root or synced notes directory'
                          : 'Path to local snapshot file or directory'
                      }
                      className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none"
                    />
                  </label>
                </div>
                {spec.key === 'notes' ? (
                  <p className="mt-2 text-sm text-zinc-500">
                    Vel reads the vault from disk after Obsidian Sync lands the files locally. This keeps note sync local-first while the daemon ingests the same markdown across clients.
                  </p>
                ) : null}
                <div className="mt-4 flex flex-wrap gap-3">
                  <button
                    type="button"
                    onClick={() => void saveLocalSourcePath(spec.key)}
                    disabled={Boolean(pendingIntegrationActions[saveActionKey])}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {pendingIntegrationActions[saveActionKey] ? 'Saving…' : 'Save path'}
                  </button>
                  <button
                    type="button"
                    onClick={() => {
                      setLocalSourceDrafts((current) => ({
                        ...current,
                        [spec.key]: '',
                      }));
                      void saveLocalSourcePath(spec.key, '');
                    }}
                    disabled={Boolean(pendingIntegrationActions[saveActionKey]) || !integration.source_path}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    Clear path
                  </button>
                  <button
                    type="button"
                    onClick={() => void syncSource(spec.key)}
                    disabled={Boolean(pendingIntegrationActions[syncActionKey])}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                  >
                    {pendingIntegrationActions[syncActionKey] ? 'Syncing…' : 'Sync now'}
                  </button>
                  <button
                    type="button"
                    onClick={() => updateIntegrationLogsVisibility(spec.key, !expandedIntegrationLogs[spec.key])}
                    className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white"
                  >
                    {expandedIntegrationLogs[spec.key] ? 'Hide history' : 'Show history'}
                  </button>
                </div>
                <IntegrationMeta
                  sourcePath={integration.source_path}
                  lastSyncAt={integration.last_sync_at}
                  lastSyncStatus={integration.last_sync_status}
                  lastItemCount={integration.last_item_count}
                  lastError={integration.last_error}
                  guidance={integration.guidance}
                  guidanceActions={localGuidanceActions(
                    spec.key,
                    integration,
                    localSourceDrafts,
                    expandedIntegrationLogs,
                    pendingIntegrationActions,
                    saveLocalSourcePath,
                    focusLocalSourceInput,
                    syncSource,
                    openIntegrationHistory,
                  )}
                />
                {expandedIntegrationLogs[spec.key] ? <IntegrationLogPanel integrationId={spec.key} /> : null}
                {feedback.length > 0 ? (
                  <div className="mt-4 space-y-1">
                    {feedback.map((entry) => (
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
            );
          })}
        </section>
      ) : null}

      {activeTab === 'runtime' ? (
      <section className="space-y-8">
        <div className="rounded-lg border border-zinc-800 bg-zinc-900/60 p-4">
          <h3 className="text-base font-medium text-zinc-100">Runtime controls</h3>
          <p className="mt-1 text-sm text-zinc-500">
            This tab is for operator actions only: adjust loops, manage retries, and restart components. Use Stats for passive observability.
          </p>
        </div>
        {diagnostics ? (
          <section>
            <div className="mb-3">
              <h3 className="text-lg font-medium text-zinc-200">System Diagnostics</h3>
              <p className="text-sm text-zinc-500">
                Live sync status, active worker count, capabilities, and per-source freshness.
              </p>
            </div>
            <div className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-4 space-y-3">
              <div className="flex items-center gap-3">
                <span className="text-xs uppercase tracking-wide text-zinc-500 w-28">Node</span>
                <span className="text-sm text-zinc-200">{diagnostics.node_display_name}</span>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-xs uppercase tracking-wide text-zinc-500 w-28">Sync status</span>
                <span
                  className={`rounded-full px-2 py-0.5 text-xs ${
                    diagnostics.sync_status === 'ready'
                      ? 'bg-emerald-950 text-emerald-300'
                      : diagnostics.sync_status === 'degraded'
                      ? 'bg-amber-950 text-amber-300'
                      : diagnostics.sync_status === 'offline'
                      ? 'bg-rose-950 text-rose-300'
                      : 'bg-zinc-800 text-zinc-400'
                  }`}
                >
                  {diagnostics.sync_status}
                </span>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-xs uppercase tracking-wide text-zinc-500 w-28">Active workers</span>
                <span className="text-sm text-zinc-200">{diagnostics.active_workers}</span>
              </div>
              {diagnostics.capability_summary.length > 0 ? (
                <div className="flex items-start gap-3">
                  <span className="text-xs uppercase tracking-wide text-zinc-500 w-28 mt-0.5">Capabilities</span>
                  <span className="text-sm text-zinc-400">{diagnostics.capability_summary.join(', ')}</span>
                </div>
              ) : null}
              {diagnostics.freshness_entries.length > 0 ? (
                <div>
                  <p className="text-xs uppercase tracking-wide text-zinc-500 mb-2">Freshness</p>
                  <ul className="space-y-1">
                    {diagnostics.freshness_entries.map((entry) => (
                      <li key={entry.source} className="flex items-center justify-between text-sm">
                        <span className="text-zinc-400 truncate max-w-xs">{entry.source}</span>
                        <span
                          className={`ml-3 rounded-full px-2 py-0.5 text-xs ${
                            entry.status === 'fresh'
                              ? 'bg-emerald-950 text-emerald-300'
                              : entry.status === 'stale'
                              ? 'bg-amber-950 text-amber-300'
                              : 'bg-zinc-800 text-zinc-400'
                          }`}
                        >
                          {entry.status}
                        </span>
                      </li>
                    ))}
                  </ul>
                </div>
              ) : null}
            </div>
          </section>
        ) : null}

        <section>
          <div className="mb-3">
            <h3 className="text-lg font-medium text-zinc-200">Runtime loops</h3>
            <p className="text-sm text-zinc-500">
              Enable or slow down durable backend loops without dropping to the CLI.
            </p>
          </div>
          {loops.length === 0 ? (
            <p className="text-sm text-zinc-500">No loop records yet.</p>
          ) : (
            <div className="space-y-4">
              {loops.map((loop) => (
                <article key={loop.kind} className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-4">
                  <div className="flex items-start justify-between gap-4">
                    <div>
                      <p className="text-sm font-medium text-zinc-100">{loop.kind}</p>
                      <p className="mt-1 text-xs text-zinc-500">
                        Next due {loop.next_due_at ? formatUnixTimestamp(loop.next_due_at) : '—'}
                      </p>
                      {loop.last_status ? (
                        <p className="mt-1 text-xs text-zinc-500">
                          Last status {loop.last_status}
                          {loop.last_error ? ` · ${loop.last_error}` : ''}
                        </p>
                      ) : null}
                    </div>
                    <label className="flex items-center gap-2 text-sm text-zinc-300">
                      <input
                        type="checkbox"
                        checked={loop.enabled}
                        onChange={(event) => void saveLoop(loop, event.target.checked)}
                        disabled={Boolean(actingLoops[loop.kind])}
                        className="rounded border-zinc-600 bg-zinc-800 text-emerald-600 focus:ring-emerald-500"
                      />
                      Enabled
                    </label>
                  </div>
                  <div className="mt-4 flex flex-col gap-3 md:flex-row md:items-end">
                    <label className="flex-1 space-y-1">
                      <span className="text-xs uppercase tracking-wide text-zinc-500">Interval seconds</span>
                      <input
                        type="number"
                        min="1"
                        step="1"
                        value={loopDrafts[loop.kind]?.intervalSeconds ?? String(loop.interval_seconds)}
                        onChange={(event) => {
                          const nextValue = event.target.value;
                          setLoopDrafts((current) => ({
                            ...current,
                            [loop.kind]: {
                              intervalSeconds: nextValue,
                            },
                          }));
                        }}
                        disabled={Boolean(actingLoops[loop.kind])}
                        className="w-full rounded-md border border-zinc-700 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-600 focus:border-zinc-500 focus:outline-none disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                      />
                    </label>
                    <button
                      type="button"
                      onClick={() => void saveLoop(loop, loop.enabled)}
                      disabled={Boolean(actingLoops[loop.kind])}
                      className="rounded-md border border-zinc-700 px-3 py-2 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
                    >
                      {actingLoops[loop.kind] ? 'Saving…' : 'Save'}
                    </button>
                  </div>
                  {loopActionState[loop.kind] ? (
                    <p
                      className={`mt-3 text-sm ${
                        loopActionState[loop.kind]?.status === 'error' ? 'text-rose-400' : 'text-emerald-400'
                      }`}
                    >
                      {loopActionState[loop.kind]?.message}
                    </p>
                  ) : null}
                </article>
              ))}
            </div>
          )}
        </section>
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
                    Trace:{' '}
                    <span className="font-mono text-zinc-200">{run.trace_id}</span>
                  </p>
                  {run.parent_run_id ? (
                    <p>
                      Parent run:{' '}
                      <span className="font-mono text-zinc-200">{run.parent_run_id}</span>
                    </p>
                  ) : null}
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
        <section className="space-y-8">
          {componentsLoadError ? (
            <div className="rounded-lg border border-amber-900/80 bg-amber-950/30 p-4 text-sm text-amber-200">
              Components API unavailable: {componentsLoadError}. Restart `veld` to pick up the latest backend routes.
            </div>
          ) : null}

          <div>
            <h3 className="text-lg font-medium text-zinc-200">Runtime components</h3>
            <p className="text-sm text-zinc-500">
              Restart degraded components from the control plane. Open logs only when actively debugging.
            </p>
          </div>

          {components.length === 0 ? (
            <p className="text-sm text-zinc-500">No components loaded yet.</p>
          ) : null}

          {components.map((component) => (
            <ComponentCard
              key={component.id}
              component={component}
              action={componentActions[component.id]}
              isExpanded={Boolean(expandedComponentLogs[component.id])}
              isRestarting={Boolean(pendingComponentActions[component.id])}
              onRestart={() => void restartComponentAction(component)}
              onToggleLogs={() => updateComponentLogsVisibility(component.id, !expandedComponentLogs[component.id])}
            />
          ))}
        </section>
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

function formatUnixTimestamp(value: number): string {
  const date = new Date(value * 1000);
  if (Number.isNaN(date.getTime())) {
    return `${value}`;
  }
  return date.toLocaleString();
}

function formatTimestampMs(value: number): string {
  const date = new Date(value * 1000);
  if (Number.isNaN(date.getTime())) {
    return `${value}`;
  }
  return date.toLocaleString();
}

function ComponentCard({
  component,
  action,
  isExpanded,
  isRestarting,
  onRestart,
  onToggleLogs,
}: {
  component: ComponentData;
  action?: ComponentActionState;
  isExpanded: boolean;
  isRestarting: boolean;
  onRestart: () => void;
  onToggleLogs: () => void;
}) {
  const logsKey = useMemo(() => queryKeys.componentLogs(component.id), [component.id]);
  const { loading: logsLoading, data: logs = [] } = useQuery<ComponentLogEventData[]>(
    logsKey,
    async () => {
      const response = await loadComponentLogs(component.id);
      return response.ok && response.data ? response.data : [];
    },
    { enabled: isExpanded },
  );

  return (
    <article className="rounded-lg border border-zinc-800 bg-zinc-900/70 p-5">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h3 className="text-lg font-medium text-zinc-100">{component.name}</h3>
          <p className="mt-1 text-sm text-zinc-500">{component.description}</p>
        </div>
        <span
          className={`rounded-full px-2 py-1 text-xs ${
            component.status === 'ok' ? 'bg-emerald-950 text-emerald-300' : 'bg-zinc-800 text-zinc-400'
          }`}
        >
          {component.status}
        </span>
      </div>
      <div className="mt-4 grid gap-1 text-sm text-zinc-400">
        <p>Restarts: {component.restart_count}</p>
        <p>
          Last run:{' '}
          {component.last_restarted_at
            ? new Date(component.last_restarted_at * 1000).toLocaleString()
            : 'never'}
        </p>
        {component.last_error ? <p className="text-rose-400">Last error: {component.last_error}</p> : null}
      </div>
      <div className="mt-4 flex flex-wrap items-center gap-3">
        <button
          type="button"
          onClick={() => void onRestart()}
          disabled={isRestarting}
          className="rounded-md border border-emerald-900 px-3 py-1.5 text-sm text-emerald-200 transition hover:border-emerald-700 hover:text-white disabled:cursor-not-allowed disabled:border-zinc-800 disabled:text-zinc-600"
        >
          {isRestarting ? 'Restarting…' : 'Restart now'}
        </button>
        <button
          type="button"
          onClick={() => void onToggleLogs()}
          className="rounded-md border border-zinc-700 px-3 py-1.5 text-sm text-zinc-200 transition hover:border-zinc-500 hover:text-white"
        >
          {isExpanded ? 'Hide logs' : 'Show logs'}
        </button>
      </div>
      {action ? (
        <p className={`mt-3 text-sm ${action.status === 'error' ? 'text-rose-400' : 'text-emerald-400'}`}>
          {action.message}
        </p>
      ) : null}
      {isExpanded ? (
        <div className="mt-4 rounded-md border border-zinc-800 bg-zinc-950/60 p-4">
          <h4 className="text-sm font-medium text-zinc-300">Recent logs</h4>
          {logsLoading ? (
            <p className="mt-2 text-sm text-zinc-500">Loading logs…</p>
          ) : logs.length === 0 ? (
            <p className="mt-2 text-sm text-zinc-500">No logs yet.</p>
          ) : (
            <ul className="mt-2 space-y-2 text-sm text-zinc-300">
              {logs.slice(0, 10).map((entry) => (
                <li key={entry.id} className="rounded border border-zinc-800 bg-zinc-900/70 p-2">
                  <div className="flex items-center justify-between gap-3">
                    <span className="font-mono text-xs text-zinc-500">{formatTimestampMs(entry.created_at)}</span>
                    <span
                      className={`rounded px-2 py-0.5 text-[11px] ${
                        entry.status === 'success'
                          ? 'bg-emerald-950 text-emerald-300'
                          : entry.status === 'error'
                            ? 'bg-rose-950 text-rose-300'
                            : 'bg-zinc-800 text-zinc-400'
                      }`}
                    >
                      {entry.status}
                    </span>
                  </div>
                  <p className="mt-2 text-sm text-zinc-200">{entry.message}</p>
                  <p className="mt-1 text-xs text-zinc-500">{entry.event_name}</p>
                </li>
              ))}
            </ul>
          )}
        </div>
      ) : null}
    </article>
  );
}

function IntegrationLogPanel({
  integrationId,
}: {
  integrationId: IntegrationLogSource;
}) {
  const [showFailuresOnly, setShowFailuresOnly] = useState(false);
  const logsKey = useMemo(() => queryKeys.integrationLogs(integrationId), [integrationId]);
  const { loading: logsLoading, data: logs = [] } = useQuery<IntegrationLogEventData[]>(
    logsKey,
    async () => {
      const response = await loadIntegrationLogs(integrationId);
      return response.ok && response.data ? response.data : [];
    },
  );
  const visibleLogs = showFailuresOnly
    ? logs.filter((entry) => entry.status === 'error')
    : logs;

  return (
    <div className="mt-4 rounded-md border border-zinc-800 bg-zinc-950/60 p-4">
      <div className="flex items-center justify-between gap-3">
        <h4 className="text-sm font-medium text-zinc-300">Recent sync history</h4>
        <label className="flex items-center gap-2 text-xs text-zinc-400">
          <input
            type="checkbox"
            checked={showFailuresOnly}
            onChange={(event) => setShowFailuresOnly(event.target.checked)}
            className="rounded border-zinc-600 bg-zinc-800 text-rose-500 focus:ring-rose-500"
          />
          Failures only
        </label>
      </div>
      {logsLoading ? (
        <p className="mt-2 text-sm text-zinc-500">Loading history…</p>
      ) : visibleLogs.length === 0 ? (
        <p className="mt-2 text-sm text-zinc-500">
          {showFailuresOnly ? 'No failed syncs in recent history.' : 'No sync history yet.'}
        </p>
      ) : (
        <ul className="mt-2 space-y-2 text-sm text-zinc-300">
          {visibleLogs.map((entry) => (
            <li key={entry.id} className="rounded border border-zinc-800 bg-zinc-900/70 p-2">
              <div className="flex items-start justify-between gap-3">
                <div>
                  <p className={entry.status === 'error' ? 'text-rose-300' : 'text-zinc-200'}>
                    {entry.message}
                  </p>
                  <p className="mt-1 text-xs text-zinc-500">{entry.event_name}</p>
                  {typeof entry.payload === 'object' && entry.payload && !Array.isArray(entry.payload) ? (
                    <div className="mt-2 space-y-1 text-xs text-zinc-400">
                      {typeof entry.payload.item_count === 'number' ? (
                        <p>Items: {entry.payload.item_count}</p>
                      ) : null}
                      {typeof entry.payload.error === 'string' && entry.payload.error.trim().length > 0 ? (
                        <p className="text-rose-300">Error: {entry.payload.error}</p>
                      ) : null}
                      <details className="pt-1">
                        <summary className="cursor-pointer text-zinc-500 hover:text-zinc-300">
                          Payload
                        </summary>
                        <pre className="mt-2 overflow-x-auto rounded bg-zinc-950/80 p-2 text-[11px] text-zinc-400">
                          {JSON.stringify(entry.payload, null, 2)}
                        </pre>
                      </details>
                    </div>
                  ) : null}
                </div>
                <p className="font-mono text-xs text-zinc-500">{formatTimestampMs(entry.created_at)}</p>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
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
  sourcePath,
  lastSyncAt,
  lastSyncStatus,
  lastItemCount,
  lastError,
  guidance,
  guidanceActions = [],
}: {
  sourcePath?: string | null;
  lastSyncAt: number | null;
  lastSyncStatus: string | null;
  lastItemCount: number | null;
  lastError: string | null;
  guidance?: { title: string; detail: string; action: string } | null;
  guidanceActions?: GuidanceActionButton[];
}) {
  return (
    <div className="mt-4 space-y-2 text-sm text-zinc-400">
      {sourcePath ? <p>Source: {sourcePath}</p> : null}
      <p>Last sync: {lastSyncAt ? new Date(lastSyncAt * 1000).toLocaleString() : 'never'}</p>
      <p>Status: {lastSyncStatus ?? 'unknown'}</p>
      <p>Items ingested: {lastItemCount ?? 0}</p>
      {lastError ? <p className="text-rose-400">Last error: {lastError}</p> : null}
      {guidance ? (
        <div className="rounded-md border border-amber-900/70 bg-amber-950/30 p-3 text-amber-200">
          <p className="font-medium">{guidance.title}</p>
          <p className="mt-1 text-sm text-amber-100/90">{guidance.detail}</p>
          <p className="mt-2 text-xs uppercase tracking-wide text-amber-300">
            Recommended action: {guidance.action}
          </p>
          {guidanceActions.length > 0 ? (
            <div className="mt-3 flex flex-wrap gap-2">
              {guidanceActions.map((action) => (
                <button
                  key={action.label}
                  type="button"
                  onClick={action.onClick}
                  disabled={action.disabled}
                  className="rounded-md border border-amber-700/70 px-3 py-1.5 text-xs font-medium text-amber-100 transition hover:border-amber-500 hover:text-white disabled:cursor-not-allowed disabled:border-amber-900/40 disabled:text-amber-300/50"
                >
                  {action.label}
                </button>
              ))}
            </div>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

function googleGuidanceActions(
  integrations: IntegrationsData,
  expandedIntegrationLogs: Record<string, true>,
  pendingIntegrationActions: Record<string, true>,
  saveGoogleCredentials: () => Promise<void>,
  startGoogleAuth: () => Promise<void>,
  syncSource: (source: 'calendar' | 'todoist' | LocalIntegrationSource) => Promise<void>,
  openIntegrationHistory: (integrationId: IntegrationLogSource) => void,
): GuidanceActionButton[] {
  const guidance = integrations.google_calendar.guidance;
  if (!guidance) {
    return [];
  }

  switch (guidance.action) {
    case 'Save credentials':
      return [{
        label: 'Run: Save credentials',
        onClick: () => void saveGoogleCredentials(),
        disabled: Boolean(pendingIntegrationActions['google-save']),
      }];
    case 'Connect Google':
      return [{
        label: 'Run: Connect Google',
        onClick: () => void startGoogleAuth(),
        disabled: Boolean(pendingIntegrationActions['google-auth']) || !integrations.google_calendar.configured,
      }];
    case 'Sync now':
      return [{
        label: 'Run: Sync now',
        onClick: () => void syncSource('calendar'),
        disabled: Boolean(pendingIntegrationActions['google-sync']) || !integrations.google_calendar.connected,
      }];
    case 'Inspect history and retry sync':
      return [
        {
          label: expandedIntegrationLogs['google-calendar'] ? 'History open' : 'Open history',
          onClick: () => openIntegrationHistory('google-calendar'),
        },
        {
          label: 'Retry sync',
          onClick: () => void syncSource('calendar'),
          disabled: Boolean(pendingIntegrationActions['google-sync']) || !integrations.google_calendar.connected,
        },
      ];
    default:
      return [];
  }
}

function todoistGuidanceActions(
  integrations: IntegrationsData,
  expandedIntegrationLogs: Record<string, true>,
  pendingIntegrationActions: Record<string, true>,
  saveTodoistToken: () => Promise<void>,
  syncSource: (source: 'calendar' | 'todoist' | LocalIntegrationSource) => Promise<void>,
  openIntegrationHistory: (integrationId: IntegrationLogSource) => void,
): GuidanceActionButton[] {
  const guidance = integrations.todoist.guidance;
  if (!guidance) {
    return [];
  }

  switch (guidance.action) {
    case 'Save token':
      return [{
        label: 'Run: Save token',
        onClick: () => void saveTodoistToken(),
        disabled: Boolean(pendingIntegrationActions['todoist-save']),
      }];
    case 'Sync now':
      return [{
        label: 'Run: Sync now',
        onClick: () => void syncSource('todoist'),
        disabled: Boolean(pendingIntegrationActions['todoist-sync']) || !integrations.todoist.connected,
      }];
    case 'Inspect history and retry sync':
      return [
        {
          label: expandedIntegrationLogs.todoist ? 'History open' : 'Open history',
          onClick: () => openIntegrationHistory('todoist'),
        },
        {
          label: 'Retry sync',
          onClick: () => void syncSource('todoist'),
          disabled: Boolean(pendingIntegrationActions['todoist-sync']) || !integrations.todoist.connected,
        },
      ];
    default:
      return [];
  }
}

function localGuidanceActions(
  source: LocalIntegrationSource,
  integration: LocalIntegrationData,
  localSourceDrafts: Record<LocalIntegrationSource, string>,
  expandedIntegrationLogs: Record<string, true>,
  pendingIntegrationActions: Record<string, true>,
  saveLocalSourcePath: (source: LocalIntegrationSource) => Promise<void>,
  focusLocalSourceInput: (source: LocalIntegrationSource) => void,
  syncSource: (source: 'calendar' | 'todoist' | LocalIntegrationSource) => Promise<void>,
  openIntegrationHistory: (integrationId: IntegrationLogSource) => void,
): GuidanceActionButton[] {
  const guidance = integration.guidance;
  if (!guidance) {
    return [];
  }

  switch (guidance.action) {
    case 'Set source path': {
      const hasDraft = localSourceDrafts[source].trim().length > 0;
      return [
        {
          label: 'Edit source path',
          onClick: () => focusLocalSourceInput(source),
        },
        {
          label: 'Run: Save path',
          onClick: () => void saveLocalSourcePath(source),
          disabled: Boolean(pendingIntegrationActions[`${source}-save`]) || !hasDraft,
        },
      ];
    }
    case 'Sync now':
      return [{
        label: 'Run: Sync now',
        onClick: () => void syncSource(source),
        disabled: Boolean(pendingIntegrationActions[`${source}-sync`]),
      }];
    case 'Fix the source and retry sync':
      return [
        {
          label: expandedIntegrationLogs[source] ? 'History open' : 'Open history',
          onClick: () => openIntegrationHistory(source),
        },
        {
          label: 'Retry sync',
          onClick: () => void syncSource(source),
          disabled: Boolean(pendingIntegrationActions[`${source}-sync`]),
        },
      ];
    default:
      return [];
  }
}

function formatRuntimeTimestamp(timestamp: string): string {
  return new Date(timestamp).toLocaleString();
}

function scopeSummaryEntries(scopes: {
  read_context: boolean;
  write_safe_actions: boolean;
  execute_repo_tasks: boolean;
}) {
  return [
    { label: 'read_context', enabled: scopes.read_context },
    { label: 'write_safe_actions', enabled: scopes.write_safe_actions },
    { label: 'execute_repo_tasks', enabled: scopes.execute_repo_tasks },
  ];
}

function linkStatusClassName(status: string): string {
  switch (status) {
    case 'linked':
      return 'rounded-full bg-emerald-900/50 px-2.5 py-1 text-xs uppercase tracking-wide text-emerald-200';
    case 'pending':
      return 'rounded-full bg-amber-900/50 px-2.5 py-1 text-xs uppercase tracking-wide text-amber-200';
    case 'revoked':
    case 'expired':
      return 'rounded-full bg-rose-900/50 px-2.5 py-1 text-xs uppercase tracking-wide text-rose-200';
    default:
      return 'rounded-full bg-zinc-800 px-2.5 py-1 text-xs uppercase tracking-wide text-zinc-300';
  }
}

function DocumentationCard({
  title,
  docs,
}: {
  title: string;
  docs: Array<[string, string, string]>;
}) {
  return (
    <div className="rounded-md border border-zinc-800 bg-zinc-950/70 p-3">
      <h4 className="text-sm font-medium text-zinc-100">{title}</h4>
      <div className="mt-3 space-y-3">
        {docs.map(([label, path, summary]) => (
          <div key={path}>
            <p className="text-sm text-zinc-200">{label}</p>
            <p className="font-mono text-xs text-zinc-400">{path}</p>
            <p className="text-xs text-zinc-500">{summary}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
