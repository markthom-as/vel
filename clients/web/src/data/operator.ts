import { apiGet, apiPatch, apiPost } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeClusterBootstrapData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeGoogleCalendarAuthStartData,
  decodeIntegrationLogEventData,
  decodeIntegrationsData,
  decodeLinkedNodeData,
  decodeLoopData,
  decodePairingTokenData,
  decodeProjectCreateResponseData,
  decodeProjectListResponseData,
  decodeRunSummaryData,
  decodeSettingsData,
  type ApiResponse,
  type ClusterBootstrapData,
  type ComponentData,
  type ComponentLogEventData,
  type GoogleCalendarAuthStartData,
  type IntegrationLogEventData,
  type IntegrationsData,
  type LinkScopeData,
  type LinkedNodeData,
  type LoopData,
  type PairingTokenData,
  type ProjectCreateRequestData,
  type ProjectCreateResponseData,
  type ProjectListResponseData,
  type RunSummaryData,
  type SettingsData,
} from '../types';

export interface SyncResultData {
  source: string;
  signals_ingested: number;
}

export interface EvaluateResultData {
  inferred_states: number;
  nudges_created_or_updated: number;
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
  projects: () => ['projects'] as const,
  linkingStatus: () => ['linking', 'status'] as const,
  settings: () => ['settings'] as const,
  integrations: () => ['integrations'] as const,
  loops: () => ['loops'] as const,
  components: () => ['components'] as const,
  componentLogs: (componentId: string) => ['components', componentId, 'logs'] as const,
  integrationLogs: (integrationId: string) => ['integrations', integrationId, 'logs'] as const,
  runs: (limit: number) => ['runs', limit] as const,
};

export function loadClusterBootstrap(): Promise<ApiResponse<ClusterBootstrapData>> {
  return apiGet<ApiResponse<ClusterBootstrapData>>(
    '/v1/cluster/bootstrap',
    (value) => decodeApiResponse(value, decodeClusterBootstrapData),
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
