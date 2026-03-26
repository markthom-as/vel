import { canonicalPatchMutation, canonicalPostMutation, canonicalQuery } from './canonicalTransport';
import {
  buildLinkingRequestValue,
  buildRemoteRoutesValue,
  normalizePairingTokenValue,
  type EmbeddedBridgeRoute,
} from './embeddedBridgeAdapter';
import {
  decodeApiResponse,
  decodeArray,
  decodeClusterBootstrapData,
  decodeClusterWorkersData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeDiagnosticsData,
  decodeExecutionHandoffRecordData,
  decodeGoogleCalendarAuthStartData,
  decodeIntegrationConnectionData,
  decodeIntegrationLogEventData,
  decodeIntegrationsData,
  decodeLocalIntegrationPathSelectionData,
  decodeLinkedNodeData,
  decodeLlmProfileHealthData,
  decodeLoopData,
  decodePairingTokenData,
  decodePlanningProfileResponseData,
  decodeProjectCreateResponseData,
  decodeProjectListResponseData,
  decodeRunSummaryData,
  decodeSettingsData,
  type ApiResponse,
  type BackupSettingsData,
  type ClusterBootstrapData,
  type ClusterWorkersData,
  type ComponentData,
  type ComponentLogEventData,
  type ConflictCaseData,
  type DiagnosticsData,
  type ExecutionHandoffRecordData,
  type ExecutionHandoffReviewStateData,
  type GoogleCalendarAuthStartData,
  type IntegrationConnectionData,
  type IntegrationLogEventData,
  type IntegrationsData,
  type LocalIntegrationPathSelectionData,
  type LinkScopeData,
  type LinkedNodeData,
  type LlmOpenAiOauthLaunchRequestData,
  type LlmProfileHandshakeRequestData,
  type LlmProfileHealthData,
  type LoopData,
  type NowData,
  type PairingTokenData,
  type PlanningProfileMutationRequestData,
  type PlanningProfileResponseData,
  type PersonRecordData,
  type ProjectCreateRequestData,
  type ProjectCreateResponseData,
  type ProjectListResponseData,
  type RunSummaryData,
  type SettingsData,
  type WebSettingsData,
  type WritebackOperationData,
} from '../types';

export interface SyncResultData {
  source: string;
  signals_ingested: number;
}

export interface EvaluateResultData {
  inferred_states: number;
  nudges_created_or_updated: number;
}

export interface OperatorReviewStatusData {
  writeback_enabled: boolean;
  pending_writebacks: WritebackOperationData[];
  open_conflicts: ConflictCaseData[];
  people_needing_review: PersonRecordData[];
  pending_execution_handoffs: ExecutionHandoffRecordData[];
}

export interface BackupTrustProjectionData {
  level: 'ok' | 'warn' | 'fail';
  statusLabel: string;
  freshnessLabel: string;
  outputRoot: string;
  lastBackupAt: string | null;
  artifactSummary: string;
  configSummary: string;
  warnings: string[];
  guidance: string[];
  commandHints: string[];
}

export interface OperatorOnboardingStepData {
  id: string;
  title: string;
  status: 'attention' | 'ready' | 'done';
  detail: string;
  supportPath: string;
}

export interface OperatorOnboardingGuideData {
  headline: string;
  nextAction: string;
  steps: OperatorOnboardingStepData[];
}

export interface CoreSetupStatusData {
  ready: boolean;
  missing: Array<'user_display_name' | 'node_display_name' | 'agent_profile' | 'llm_provider' | 'synced_provider'>;
  title: string;
  summary: string;
}

export interface EmbeddedLinkingRequestDraftData {
  token_code: string | null;
  target_base_url: string | null;
  route_candidates: EmbeddedBridgeRoute[];
}

function decodeSyncResultData(value: unknown): SyncResultData {
  const record = value as { source?: unknown; signals_ingested?: unknown };
  if (typeof record?.source !== 'string' || typeof record?.signals_ingested !== 'number') {
    throw new Error('Expected sync result payload with source and signals_ingested');
  }
  return {
    source: record.source,
    signals_ingested: record.signals_ingested,
  };
}

function decodeEvaluateResultData(value: unknown): EvaluateResultData {
  const record = value as { inferred_states?: unknown; nudges_created_or_updated?: unknown };
  if (
    typeof record?.inferred_states !== 'number'
    || typeof record?.nudges_created_or_updated !== 'number'
  ) {
    throw new Error('Expected evaluate result payload with inferred_states and nudges_created_or_updated');
  }
  return {
    inferred_states: record.inferred_states,
    nudges_created_or_updated: record.nudges_created_or_updated,
  };
}

export const operatorQueryKeys = {
  agentInspect: () => ['agent', 'inspect'] as const,
  clusterBootstrap: () => ['cluster', 'bootstrap'] as const,
  clusterWorkers: () => ['cluster', 'workers'] as const,
  projects: () => ['projects'] as const,
  linkingStatus: () => ['linking', 'status'] as const,
  executionHandoffs: (state: ExecutionHandoffReviewStateData = 'pending_review') =>
    ['execution', 'handoffs', state] as const,
  settings: () => ['settings'] as const,
  integrations: () => ['integrations'] as const,
  loops: () => ['loops'] as const,
  planningProfile: () => ['planning-profile'] as const,
  components: () => ['components'] as const,
  componentLogs: (componentId: string) => ['components', componentId, 'logs'] as const,
  integrationLogs: (integrationId: string) => ['integrations', integrationId, 'logs'] as const,
  integrationConnections: (family?: string | null, providerKey?: string | null) =>
    ['integrations', 'connections', family ?? 'all', providerKey ?? 'all'] as const,
  runs: (limit: number) => ['runs', limit] as const,
};

export function buildOperatorReviewStatus(
  now: NowData | null | undefined,
  settings: SettingsData | null | undefined,
  handoffs: ExecutionHandoffRecordData[] | null | undefined = [],
): OperatorReviewStatusData {
  const peopleById = new Map<string, PersonRecordData>(
    (now?.people ?? []).map((person) => [person.id, person]),
  );
  const peopleNeedingReview = new Map<string, PersonRecordData>();

  for (const item of now?.action_items ?? []) {
    for (const evidence of item.evidence) {
      if (evidence.source_kind !== 'person') {
        continue;
      }
      const person = peopleById.get(evidence.source_id);
      if (person) {
        peopleNeedingReview.set(person.id, person);
      }
    }
  }

  return {
    writeback_enabled: settings?.writeback_enabled === true,
    pending_writebacks: now?.pending_writebacks ?? [],
    open_conflicts: now?.conflicts ?? [],
    people_needing_review: [...peopleNeedingReview.values()],
    pending_execution_handoffs: handoffs ?? [],
  };
}

function hasMeaningfulText(value: string | null | undefined): boolean {
  return typeof value === 'string' && value.trim().length > 0;
}

export function buildCoreSetupStatus(
  settings: SettingsData | null | undefined,
  integrations: IntegrationsData | null | undefined,
): CoreSetupStatusData {
  const core = settings?.core_settings;
  if (core?.bypass_setup_gate) {
    return {
      ready: true,
      missing: [],
      title: 'Core setup override is active',
      summary: 'Developer bypass is allowing the composer before minimum Core setup is complete.',
    };
  }

  const missing: CoreSetupStatusData['missing'] = [];
  if (!hasMeaningfulText(core?.user_display_name)) {
    missing.push('user_display_name');
  }
  if (!hasMeaningfulText(settings?.node_display_name)) {
    missing.push('node_display_name');
  }
  if (
    !hasMeaningfulText(core?.agent_profile?.role)
    && !hasMeaningfulText(core?.agent_profile?.preferences)
    && !hasMeaningfulText(core?.agent_profile?.constraints)
    && !hasMeaningfulText(core?.agent_profile?.freeform)
  ) {
    missing.push('agent_profile');
  }

  const defaultProfileId = settings?.llm?.default_chat_profile_id ?? null;
  const hasConfiguredLlm = Boolean(
    defaultProfileId
    && settings?.llm?.profiles.some((profile) => profile.enabled && profile.id === defaultProfileId),
  );
  if (!hasConfiguredLlm) {
    missing.push('llm_provider');
  }

  const hasSyncedProvider = Boolean(
    integrations?.google_calendar.configured
    || integrations?.todoist.configured,
  );
  if (!hasSyncedProvider) {
    missing.push('synced_provider');
  }

  if (missing.length === 0) {
    return {
      ready: true,
      missing,
      title: 'Core setup is complete',
      summary: 'The composer is available because Core identity, provider routing, and synced provider setup are in place.',
    };
  }

  const labels: Record<CoreSetupStatusData['missing'][number], string> = {
    user_display_name: 'your name',
    node_display_name: 'node name',
    agent_profile: 'agent profile',
    llm_provider: 'an LLM provider',
    synced_provider: 'a synced provider',
  };

  return {
    ready: false,
    missing,
    title: 'Finish Core setup to enable the composer',
    summary: `Missing ${missing.map((item) => labels[item]).join(', ')}. Vel will not be fully functional until required Core setup is submitted.`,
  };
}

function formatCoverageSummary(
  label: string,
  coverage: BackupSettingsData['trust']['status']['artifact_coverage'] | BackupSettingsData['trust']['status']['config_coverage'],
): string {
  if (!coverage) {
    return `${label}: not recorded`;
  }
  return `${label}: ${coverage.included.length} included, ${coverage.omitted.length} omitted`;
}

function backupStatusLabel(level: BackupTrustProjectionData['level']): string {
  switch (level) {
    case 'ok':
      return 'Healthy backup trust';
    case 'warn':
      return 'Backup trust needs attention';
    case 'fail':
      return 'No trustworthy backup';
  }
}

function backupFreshnessLabel(backup: BackupSettingsData): string {
  const { freshness } = backup.trust;
  if (freshness.state === 'missing') {
    return 'Missing';
  }
  if (freshness.age_seconds == null) {
    return freshness.state;
  }
  const hours = Math.floor(freshness.age_seconds / 3600);
  if (hours < 1) {
    const minutes = Math.max(1, Math.floor(freshness.age_seconds / 60));
    return `${freshness.state} (${minutes}m old)`;
  }
  return `${freshness.state} (${hours}h old)`;
}

export function buildBackupTrustProjection(
  backup: BackupSettingsData | null | undefined,
): BackupTrustProjectionData | null {
  if (!backup) {
    return null;
  }
  const level = backup.trust.level;
  const outputRoot = backup.trust.status.output_root ?? backup.default_output_root;
  return {
    level,
    statusLabel: backupStatusLabel(level),
    freshnessLabel: backupFreshnessLabel(backup),
    outputRoot,
    lastBackupAt: backup.trust.status.last_backup_at,
    artifactSummary: formatCoverageSummary('Artifacts', backup.trust.status.artifact_coverage),
    configSummary: formatCoverageSummary('Config', backup.trust.status.config_coverage),
    warnings: backup.trust.status.warnings,
    guidance: backup.trust.guidance,
    commandHints: [
      'vel backup create',
      'vel backup inspect <backup_root>',
      'vel backup verify <backup_root>',
      'vel backup restore-check <backup_root>',
    ],
  };
}

export function buildSettingsOnboardingGuide({
  clusterBootstrap,
  clusterWorkers,
  linkedNodes,
  integrations,
}: {
  clusterBootstrap: ClusterBootstrapData | null | undefined;
  clusterWorkers: ClusterWorkersData | null | undefined;
  linkedNodes: LinkedNodeData[] | null | undefined;
  integrations: IntegrationsData | null | undefined;
}): OperatorOnboardingGuideData {
  const workers = clusterWorkers?.workers ?? [];
  const activeLinkedNodes = (linkedNodes ?? []).filter((node) => node.status === 'linked');
  const localWorker = clusterBootstrap
    ? workers.find((worker) => worker.node_id === clusterBootstrap.node_id) ?? null
    : null;
  const hasIncomingPrompt = localWorker?.incoming_linking_prompt != null;
  const discoveredCompanions = clusterBootstrap
    ? workers.filter(
      (worker) =>
        worker.node_id !== clusterBootstrap.node_id
        && !activeLinkedNodes.some((node) => node.node_id === worker.node_id),
    )
    : [];
  const localIntegrations = integrations
    ? [
      integrations.activity,
      integrations.health,
      integrations.git,
      integrations.messaging,
      integrations.reminders,
      integrations.notes,
      integrations.transcripts,
    ]
    : [];
  const configuredLocalSources = localIntegrations.filter((integration) => integration.configured).length;
  const discoverableLocalSources = localIntegrations.filter(
    (integration) =>
      (integration.available_paths?.length ?? 0) > 0
      || integration.suggested_paths.length > 0
      || (integration.internal_paths?.length ?? 0) > 0,
  ).length;
  const appleDiscoveryVisible = workers.some((worker) =>
    ['vel_macos', 'vel_ios', 'vel_watch'].includes(worker.client_kind ?? ''),
  )
    || localIntegrations.some((integration) =>
      [
        ...(integration.available_paths ?? []),
        ...(integration.suggested_paths ?? []),
        ...(integration.internal_paths ?? []),
      ].some((path) => path.includes('Application Support/Vel')),
    );

  const daemonStep: OperatorOnboardingStepData = clusterBootstrap
    ? {
      id: 'daemon',
      title: 'Reach the daemon',
      status: 'done',
      detail: `This shell is connected to ${clusterBootstrap.node_display_name}. Use the saved sync routes below when another client needs a stable endpoint.`,
      supportPath: 'docs/user/setup.md',
    }
    : {
      id: 'daemon',
      title: 'Reach the daemon',
      status: 'attention',
      detail: 'Cluster bootstrap is unavailable. Start `veld`, confirm the base URL, then retry linking or connector setup.',
      supportPath: 'docs/user/troubleshooting.md',
    };

  let linkingStep: OperatorOnboardingStepData;
  if (!clusterBootstrap) {
    linkingStep = {
      id: 'linking',
      title: 'Link a companion device',
      status: 'attention',
      detail: 'Linking stays blocked until the local daemon publishes bootstrap metadata.',
      supportPath: 'docs/api/runtime.md',
    };
  } else if (hasIncomingPrompt) {
    linkingStep = {
      id: 'linking',
      title: 'Link a companion device',
      status: 'ready',
      detail: 'This node already has an incoming pairing prompt. Enter the token here, then confirm the linked status card.',
      supportPath: 'docs/user/troubleshooting.md',
    };
  } else if (activeLinkedNodes.length > 0) {
    linkingStep = {
      id: 'linking',
      title: 'Link a companion device',
      status: 'done',
      detail: `${activeLinkedNodes.length} linked device${activeLinkedNodes.length === 1 ? '' : 's'} already share continuity with this node. Use scope renegotiation only when you need to widen or narrow access.`,
      supportPath: 'docs/api/runtime.md',
    };
  } else if (discoveredCompanions.length > 0) {
    linkingStep = {
      id: 'linking',
      title: 'Link a companion device',
      status: 'ready',
      detail: `Select one of the ${discoveredCompanions.length} discovered companion node${discoveredCompanions.length === 1 ? '' : 's'}, issue a token, then redeem it on that client before it expires.`,
      supportPath: 'docs/api/runtime.md',
    };
  } else {
    linkingStep = {
      id: 'linking',
      title: 'Link a companion device',
      status: 'attention',
      detail: 'No unlinked companion node is visible yet. Open Vel on the other client or use the CLI fallback to issue and redeem a token manually.',
      supportPath: 'docs/user/troubleshooting.md',
    };
  }

  const sourcesStep: OperatorOnboardingStepData = configuredLocalSources > 0
    ? {
      id: 'local-sources',
      title: 'Confirm local source paths',
      status: 'done',
      detail: `${configuredLocalSources} local source${configuredLocalSources === 1 ? '' : 's'} already have saved paths. Use the integration cards to validate sync freshness or swap paths when the host changes.`,
      supportPath: 'docs/user/integrations/local-sources.md',
    }
    : discoverableLocalSources > 0
      ? {
        id: 'local-sources',
        title: 'Confirm local source paths',
        status: 'ready',
        detail: 'Suggested or host-discovered paths are available in Integrations. Pick the real source path first, then run Sync now and evaluate.',
        supportPath: 'docs/user/integrations/local-sources.md',
      }
      : {
        id: 'local-sources',
        title: 'Confirm local source paths',
        status: 'attention',
        detail: 'No local source path is configured yet. Use Integrations to choose a path manually before expecting notes, activity, messaging, or reminders to refresh.',
        supportPath: 'docs/user/setup.md',
      };

  const appleStep: OperatorOnboardingStepData = appleDiscoveryVisible
    ? {
      id: 'apple',
      title: 'Validate Apple and macOS export paths',
      status: 'ready',
      detail: 'Apple-oriented routes or Application Support paths are visible. Confirm endpoint order on the device and keep snapshot exports in the daemon-side Vel folder.',
      supportPath: 'docs/user/integrations/apple-macos.md',
    }
    : {
      id: 'apple',
      title: 'Validate Apple and macOS export paths',
      status: 'attention',
      detail: 'No Apple bridge path is visible yet. Check endpoint resolution and the macOS Application Support export locations before assuming sync is broken.',
      supportPath: 'docs/user/integrations/apple-macos.md',
    };

  const steps = [daemonStep, linkingStep, sourcesStep, appleStep];
  const nextStep = steps.find((step) => step.status !== 'done') ?? steps[steps.length - 1];

  return {
    headline: 'Use the next unfinished step, not the whole checklist.',
    nextAction: `${nextStep.title}: ${nextStep.detail}`,
    steps,
  };
}

export function loadExecutionHandoffs(
  state: ExecutionHandoffReviewStateData = 'pending_review',
): Promise<ApiResponse<ExecutionHandoffRecordData[]>> {
  return canonicalQuery<ExecutionHandoffRecordData[]>(
    `/v1/execution/handoffs?state=${encodeURIComponent(state)}`,
    (value) =>
      decodeApiResponse(
        value,
        (data) => decodeArray(data, decodeExecutionHandoffRecordData),
      ),
  );
}

export function approveExecutionHandoff(
  handoffId: string,
  payload: { reviewed_by: string; decision_reason?: string | null },
): Promise<ApiResponse<ExecutionHandoffRecordData>> {
  return canonicalPostMutation<ExecutionHandoffRecordData>(
    `/v1/execution/handoffs/${handoffId}/approve`,
    payload,
    (value) => decodeApiResponse(value, decodeExecutionHandoffRecordData),
  );
}

export function rejectExecutionHandoff(
  handoffId: string,
  payload: { reviewed_by: string; decision_reason?: string | null },
): Promise<ApiResponse<ExecutionHandoffRecordData>> {
  return canonicalPostMutation<ExecutionHandoffRecordData>(
    `/v1/execution/handoffs/${handoffId}/reject`,
    payload,
    (value) => decodeApiResponse(value, decodeExecutionHandoffRecordData),
  );
}

export function loadClusterBootstrap(): Promise<ApiResponse<ClusterBootstrapData>> {
  return canonicalQuery<ClusterBootstrapData>(
    '/v1/cluster/bootstrap',
    (value) => decodeApiResponse(value, decodeClusterBootstrapData),
  );
}

export function loadClusterWorkers(): Promise<ApiResponse<ClusterWorkersData>> {
  return canonicalQuery<ClusterWorkersData>(
    '/v1/cluster/workers',
    (value) => decodeApiResponse(value, decodeClusterWorkersData),
  );
}

export function loadProjects(): Promise<ApiResponse<ProjectListResponseData>> {
  return canonicalQuery<ProjectListResponseData>(
    '/v1/projects',
    (value) => decodeApiResponse(value, decodeProjectListResponseData),
  );
}

export function createProject(
  payload: ProjectCreateRequestData,
): Promise<ApiResponse<ProjectCreateResponseData>> {
  return canonicalPostMutation<ProjectCreateResponseData>(
    '/v1/projects',
    payload,
    (value) => decodeApiResponse(value, decodeProjectCreateResponseData),
  );
}

export function issuePairingToken(payload: {
  issued_by_node_id: string;
  ttl_seconds?: number;
  scopes: LinkScopeData;
  target_node_id?: string;
  target_node_display_name?: string | null;
  target_base_url?: string | null;
}): Promise<ApiResponse<PairingTokenData>> {
  return canonicalPostMutation<PairingTokenData>(
    '/v1/linking/tokens',
    payload,
    (value) => decodeApiResponse(value, decodePairingTokenData),
  );
}

export function buildEmbeddedPairingTokenPreview(rawInput: string): string {
  return normalizePairingTokenValue(rawInput).tokenCode;
}

export function buildEmbeddedLinkingRequestDraft(input: {
  token_code?: string | null;
  target_base_url?: string | null;
  sync_base_url?: string | null;
  tailscale_base_url?: string | null;
  lan_base_url?: string | null;
  public_base_url?: string | null;
}): EmbeddedLinkingRequestDraftData {
  const linkingRequest = buildLinkingRequestValue(input.token_code, input.target_base_url);
  const routeCandidates = buildRemoteRoutesValue(
    input.sync_base_url,
    input.tailscale_base_url,
    input.lan_base_url,
    input.public_base_url,
  );

  return {
    token_code: linkingRequest.tokenCode,
    target_base_url: linkingRequest.targetBaseUrl,
    route_candidates: routeCandidates,
  };
}

export function redeemPairingToken(payload: {
  token_code: string;
  node_id: string;
  node_display_name: string;
  transport_hint?: string | null;
  requested_scopes?: LinkScopeData | null;
  sync_base_url?: string | null;
  tailscale_base_url?: string | null;
  lan_base_url?: string | null;
  localhost_base_url?: string | null;
  public_base_url?: string | null;
}): Promise<ApiResponse<LinkedNodeData>> {
  return canonicalPostMutation<LinkedNodeData>(
    '/v1/linking/redeem',
    payload,
    (value) => decodeApiResponse(value, decodeLinkedNodeData),
  );
}

export function loadLinkingStatus(): Promise<ApiResponse<LinkedNodeData[]>> {
  return canonicalQuery<LinkedNodeData[]>(
    '/v1/linking/status',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeLinkedNodeData)),
  );
}

export function revokeLinkedNode(nodeId: string): Promise<ApiResponse<LinkedNodeData>> {
  return canonicalPostMutation<LinkedNodeData>(
    `/v1/linking/revoke/${encodeURIComponent(nodeId)}`,
    {},
    (value) => decodeApiResponse(value, decodeLinkedNodeData),
  );
}

export function loadSettings(): Promise<ApiResponse<SettingsData>> {
  return canonicalQuery<SettingsData>(
    '/api/settings',
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function loadPlanningProfile(): Promise<ApiResponse<PlanningProfileResponseData>> {
  return canonicalQuery<PlanningProfileResponseData>(
    '/v1/planning-profile',
    (value) => decodeApiResponse(value, decodePlanningProfileResponseData),
  );
}

export function applyPlanningProfileMutation(
  payload: PlanningProfileMutationRequestData,
): Promise<ApiResponse<PlanningProfileResponseData>> {
  return canonicalPatchMutation<PlanningProfileResponseData>(
    '/v1/planning-profile',
    payload,
    (value) => decodeApiResponse(value, decodePlanningProfileResponseData),
  );
}

export function updateSettings(
  patch: Record<string, unknown>,
): Promise<ApiResponse<SettingsData>> {
  return canonicalPatchMutation<SettingsData>(
    '/api/settings',
    patch,
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function updateWebSettings(
  patch: Partial<WebSettingsData>,
): Promise<ApiResponse<SettingsData>> {
  return updateSettings({
    web_settings: patch,
  });
}

export function loadLlmProfileHealth(
  profileId: string,
): Promise<ApiResponse<LlmProfileHealthData>> {
  return canonicalQuery<LlmProfileHealthData>(
    `/api/llm/profiles/${encodeURIComponent(profileId)}/health`,
    (value) => decodeApiResponse(value, decodeLlmProfileHealthData),
  );
}

export function runLlmProfileHandshake(
  payload: LlmProfileHandshakeRequestData,
): Promise<ApiResponse<LlmProfileHealthData>> {
  return canonicalPostMutation<LlmProfileHealthData>(
    '/api/llm/handshake',
    payload,
    (value) => decodeApiResponse(value, decodeLlmProfileHealthData),
  );
}

export function launchOpenAiOauthProxy(
  payload: LlmOpenAiOauthLaunchRequestData,
): Promise<ApiResponse<LlmProfileHealthData>> {
  return canonicalPostMutation<LlmProfileHealthData>(
    '/api/llm/openai-oauth/launch',
    payload,
    (value) => decodeApiResponse(value, decodeLlmProfileHealthData),
  );
}

export function loadRecentRuns(limit: number): Promise<ApiResponse<RunSummaryData[]>> {
  return canonicalQuery<RunSummaryData[]>(
    `/v1/runs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeRunSummaryData)),
  );
}

export function updateRun(
  runId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<RunSummaryData>> {
  return canonicalPatchMutation<RunSummaryData>(
    `/v1/runs/${runId}`,
    patch,
    (value) => decodeApiResponse(value, decodeRunSummaryData),
  );
}

export function loadIntegrations(): Promise<ApiResponse<IntegrationsData>> {
  return canonicalQuery<IntegrationsData>(
    '/api/integrations',
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function loadIntegrationConnections(
  query: {
    family?: string;
    providerKey?: string;
    includeDisabled?: boolean;
  } = {},
): Promise<ApiResponse<IntegrationConnectionData[]>> {
  const params = new URLSearchParams();
  if (query.family) {
    params.set('family', query.family);
  }
  if (query.providerKey) {
    params.set('provider_key', query.providerKey);
  }
  if (query.includeDisabled) {
    params.set('include_disabled', 'true');
  }
  const suffix = params.size > 0 ? `?${params.toString()}` : '';
  return canonicalQuery<IntegrationConnectionData[]>(
    `/api/integrations/connections${suffix}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeIntegrationConnectionData)),
  );
}

export function updateGoogleCalendarIntegration(
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return canonicalPatchMutation<IntegrationsData>(
    '/api/integrations/google-calendar',
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function disconnectGoogleCalendar(): Promise<ApiResponse<IntegrationsData>> {
  return canonicalPostMutation<IntegrationsData>(
    '/api/integrations/google-calendar/disconnect',
    {},
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function updateTodoistIntegration(
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return canonicalPatchMutation<IntegrationsData>(
    '/api/integrations/todoist',
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function disconnectTodoist(): Promise<ApiResponse<IntegrationsData>> {
  return canonicalPostMutation<IntegrationsData>(
    '/api/integrations/todoist/disconnect',
    {},
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function updateLocalIntegrationSource(
  integrationId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return canonicalPatchMutation<IntegrationsData>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/source`,
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function chooseLocalIntegrationSourcePath(
  integrationId: string,
): Promise<ApiResponse<LocalIntegrationPathSelectionData>> {
  return canonicalPostMutation<LocalIntegrationPathSelectionData>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/path-dialog`,
    {},
    (value) => decodeApiResponse(value, decodeLocalIntegrationPathSelectionData),
  );
}

export function syncSource(source: string): Promise<ApiResponse<SyncResultData>> {
  return canonicalPostMutation<SyncResultData>(
    `/v1/sync/${source}`,
    {},
    (value) => decodeApiResponse(value, decodeSyncResultData),
  );
}

export function runEvaluate(): Promise<ApiResponse<EvaluateResultData>> {
  return canonicalPostMutation<EvaluateResultData>(
    '/v1/evaluate',
    {},
    (value) => decodeApiResponse(value, decodeEvaluateResultData),
  );
}

export function loadLoops(): Promise<ApiResponse<LoopData[]>> {
  return canonicalQuery<LoopData[]>(
    '/v1/loops',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeLoopData)),
  );
}

export function updateLoop(
  loopKind: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<LoopData>> {
  return canonicalPatchMutation<LoopData>(
    `/v1/loops/${encodeURIComponent(loopKind.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeLoopData),
  );
}

export function loadComponents(): Promise<ApiResponse<ComponentData[]>> {
  return canonicalQuery<ComponentData[]>(
    '/api/components',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeComponentData)),
  );
}

export function restartComponent(componentId: string): Promise<ApiResponse<ComponentData>> {
  return canonicalPostMutation<ComponentData>(
    `/api/components/${encodeURIComponent(componentId.trim())}/restart`,
    {},
    (value) => decodeApiResponse(value, decodeComponentData),
  );
}

export function loadComponentLogs(
  componentId: string,
  limit = 50,
): Promise<ApiResponse<ComponentLogEventData[]>> {
  return canonicalQuery<ComponentLogEventData[]>(
    `/api/components/${encodeURIComponent(componentId.trim())}/logs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeComponentLogEventData)),
  );
}

export function loadIntegrationLogs(
  integrationId: string,
  limit = 10,
): Promise<ApiResponse<IntegrationLogEventData[]>> {
  return canonicalQuery<IntegrationLogEventData[]>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/logs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeIntegrationLogEventData)),
  );
}

export function decodeGoogleCalendarAuthStartResponse(value: unknown): ApiResponse<GoogleCalendarAuthStartData> {
  return decodeApiResponse(value, decodeGoogleCalendarAuthStartData);
}

export function startGoogleCalendarAuth(): Promise<ApiResponse<GoogleCalendarAuthStartData>> {
  return canonicalPostMutation<GoogleCalendarAuthStartData>(
    '/api/integrations/google-calendar/auth/start',
    {},
    decodeGoogleCalendarAuthStartResponse,
  );
}

export function loadDiagnostics(): Promise<ApiResponse<DiagnosticsData>> {
  return canonicalQuery<DiagnosticsData>(
    '/api/diagnostics',
    (value) => decodeApiResponse(value, decodeDiagnosticsData),
    { allowDegraded: true },
  );
}
