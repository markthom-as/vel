import { apiGet, apiPatch, apiPost } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeClusterBootstrapData,
  decodeClusterWorkersData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeExecutionHandoffRecordData,
  decodeGoogleCalendarAuthStartData,
  decodeIntegrationLogEventData,
  decodeIntegrationsData,
  decodeLocalIntegrationPathSelectionData,
  decodeLinkedNodeData,
  decodeLoopData,
  decodePairingTokenData,
  decodeProjectCreateResponseData,
  decodeProjectListResponseData,
  decodeRunSummaryData,
  decodeSettingsData,
  type ApiResponse,
  type ClusterBootstrapData,
  type ClusterWorkersData,
  type ComponentData,
  type ComponentLogEventData,
  type ConflictCaseData,
  type ExecutionHandoffRecordData,
  type ExecutionHandoffReviewStateData,
  type GoogleCalendarAuthStartData,
  type IntegrationLogEventData,
  type IntegrationsData,
  type LocalIntegrationPathSelectionData,
  type LinkScopeData,
  type LinkedNodeData,
  type LoopData,
  type NowData,
  type PairingTokenData,
  type PersonRecordData,
  type ProjectCreateRequestData,
  type ProjectCreateResponseData,
  type ProjectListResponseData,
  type RunSummaryData,
  type SettingsData,
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
  clusterBootstrap: () => ['cluster', 'bootstrap'] as const,
  clusterWorkers: () => ['cluster', 'workers'] as const,
  projects: () => ['projects'] as const,
  linkingStatus: () => ['linking', 'status'] as const,
  executionHandoffs: (state: ExecutionHandoffReviewStateData = 'pending_review') =>
    ['execution', 'handoffs', state] as const,
  settings: () => ['settings'] as const,
  integrations: () => ['integrations'] as const,
  loops: () => ['loops'] as const,
  components: () => ['components'] as const,
  componentLogs: (componentId: string) => ['components', componentId, 'logs'] as const,
  integrationLogs: (integrationId: string) => ['integrations', integrationId, 'logs'] as const,
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

export function loadExecutionHandoffs(
  state: ExecutionHandoffReviewStateData = 'pending_review',
): Promise<ApiResponse<ExecutionHandoffRecordData[]>> {
  return apiGet<ApiResponse<ExecutionHandoffRecordData[]>>(
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
  return apiPost<ApiResponse<ExecutionHandoffRecordData>>(
    `/v1/execution/handoffs/${handoffId}/approve`,
    payload,
    (value) => decodeApiResponse(value, decodeExecutionHandoffRecordData),
  );
}

export function rejectExecutionHandoff(
  handoffId: string,
  payload: { reviewed_by: string; decision_reason?: string | null },
): Promise<ApiResponse<ExecutionHandoffRecordData>> {
  return apiPost<ApiResponse<ExecutionHandoffRecordData>>(
    `/v1/execution/handoffs/${handoffId}/reject`,
    payload,
    (value) => decodeApiResponse(value, decodeExecutionHandoffRecordData),
  );
}

export function loadClusterBootstrap(): Promise<ApiResponse<ClusterBootstrapData>> {
  return apiGet<ApiResponse<ClusterBootstrapData>>(
    '/v1/cluster/bootstrap',
    (value) => decodeApiResponse(value, decodeClusterBootstrapData),
  );
}

export function loadClusterWorkers(): Promise<ApiResponse<ClusterWorkersData>> {
  return apiGet<ApiResponse<ClusterWorkersData>>(
    '/v1/cluster/workers',
    (value) => decodeApiResponse(value, decodeClusterWorkersData),
  );
}

export function loadProjects(): Promise<ApiResponse<ProjectListResponseData>> {
  return apiGet<ApiResponse<ProjectListResponseData>>(
    '/v1/projects',
    (value) => decodeApiResponse(value, decodeProjectListResponseData),
  );
}

export function createProject(
  payload: ProjectCreateRequestData,
): Promise<ApiResponse<ProjectCreateResponseData>> {
  return apiPost<ApiResponse<ProjectCreateResponseData>>(
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
  return apiPost<ApiResponse<PairingTokenData>>(
    '/v1/linking/tokens',
    payload,
    (value) => decodeApiResponse(value, decodePairingTokenData),
  );
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
  return apiPost<ApiResponse<LinkedNodeData>>(
    '/v1/linking/redeem',
    payload,
    (value) => decodeApiResponse(value, decodeLinkedNodeData),
  );
}

export function loadLinkingStatus(): Promise<ApiResponse<LinkedNodeData[]>> {
  return apiGet<ApiResponse<LinkedNodeData[]>>(
    '/v1/linking/status',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeLinkedNodeData)),
  );
}

export function revokeLinkedNode(nodeId: string): Promise<ApiResponse<LinkedNodeData>> {
  return apiPost<ApiResponse<LinkedNodeData>>(
    `/v1/linking/revoke/${encodeURIComponent(nodeId)}`,
    {},
    (value) => decodeApiResponse(value, decodeLinkedNodeData),
  );
}

export function loadSettings(): Promise<ApiResponse<SettingsData>> {
  return apiGet<ApiResponse<SettingsData>>(
    '/api/settings',
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function updateSettings(
  patch: Partial<SettingsData>,
): Promise<ApiResponse<SettingsData>> {
  return apiPatch<ApiResponse<SettingsData>>(
    '/api/settings',
    patch,
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function loadRecentRuns(limit: number): Promise<ApiResponse<RunSummaryData[]>> {
  return apiGet<ApiResponse<RunSummaryData[]>>(
    `/v1/runs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeRunSummaryData)),
  );
}

export function updateRun(
  runId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<RunSummaryData>> {
  return apiPatch<ApiResponse<RunSummaryData>>(
    `/v1/runs/${runId}`,
    patch,
    (value) => decodeApiResponse(value, decodeRunSummaryData),
  );
}

export function loadIntegrations(): Promise<ApiResponse<IntegrationsData>> {
  return apiGet<ApiResponse<IntegrationsData>>(
    '/api/integrations',
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function updateGoogleCalendarIntegration(
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return apiPatch<ApiResponse<IntegrationsData>>(
    '/api/integrations/google-calendar',
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function disconnectGoogleCalendar(): Promise<ApiResponse<IntegrationsData>> {
  return apiPost<ApiResponse<IntegrationsData>>(
    '/api/integrations/google-calendar/disconnect',
    {},
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function updateTodoistIntegration(
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return apiPatch<ApiResponse<IntegrationsData>>(
    '/api/integrations/todoist',
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function disconnectTodoist(): Promise<ApiResponse<IntegrationsData>> {
  return apiPost<ApiResponse<IntegrationsData>>(
    '/api/integrations/todoist/disconnect',
    {},
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function updateLocalIntegrationSource(
  integrationId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<IntegrationsData>> {
  return apiPatch<ApiResponse<IntegrationsData>>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/source`,
    patch,
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function chooseLocalIntegrationSourcePath(
  integrationId: string,
): Promise<ApiResponse<LocalIntegrationPathSelectionData>> {
  return apiPost<ApiResponse<LocalIntegrationPathSelectionData>>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/path-dialog`,
    {},
    (value) => decodeApiResponse(value, decodeLocalIntegrationPathSelectionData),
  );
}

export function syncSource(source: string): Promise<ApiResponse<SyncResultData>> {
  return apiPost<ApiResponse<SyncResultData>>(
    `/v1/sync/${source}`,
    {},
    (value) => decodeApiResponse(value, decodeSyncResultData),
  );
}

export function runEvaluate(): Promise<ApiResponse<EvaluateResultData>> {
  return apiPost<ApiResponse<EvaluateResultData>>(
    '/v1/evaluate',
    {},
    (value) => decodeApiResponse(value, decodeEvaluateResultData),
  );
}

export function loadLoops(): Promise<ApiResponse<LoopData[]>> {
  return apiGet<ApiResponse<LoopData[]>>(
    '/v1/loops',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeLoopData)),
  );
}

export function updateLoop(
  loopKind: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<LoopData>> {
  return apiPatch<ApiResponse<LoopData>>(
    `/v1/loops/${encodeURIComponent(loopKind.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeLoopData),
  );
}

export function loadComponents(): Promise<ApiResponse<ComponentData[]>> {
  return apiGet<ApiResponse<ComponentData[]>>(
    '/api/components',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeComponentData)),
  );
}

export function restartComponent(componentId: string): Promise<ApiResponse<ComponentData>> {
  return apiPost<ApiResponse<ComponentData>>(
    `/api/components/${encodeURIComponent(componentId.trim())}/restart`,
    {},
    (value) => decodeApiResponse(value, decodeComponentData),
  );
}

export function loadComponentLogs(
  componentId: string,
  limit = 50,
): Promise<ApiResponse<ComponentLogEventData[]>> {
  return apiGet<ApiResponse<ComponentLogEventData[]>>(
    `/api/components/${encodeURIComponent(componentId.trim())}/logs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeComponentLogEventData)),
  );
}

export function loadIntegrationLogs(
  integrationId: string,
  limit = 10,
): Promise<ApiResponse<IntegrationLogEventData[]>> {
  return apiGet<ApiResponse<IntegrationLogEventData[]>>(
    `/api/integrations/${encodeURIComponent(integrationId.trim())}/logs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeIntegrationLogEventData)),
  );
}

export function decodeGoogleCalendarAuthStartResponse(value: unknown): ApiResponse<GoogleCalendarAuthStartData> {
  return decodeApiResponse(value, decodeGoogleCalendarAuthStartData);
}
