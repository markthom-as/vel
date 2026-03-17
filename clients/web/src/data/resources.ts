import { apiGet, apiPatch, apiPost } from '../api/client';
import {
  decodeApiResponse,
  decodeCommitmentData,
  decodeArray,
  decodeConversationData,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeDriftExplainData,
  decodeComponentData,
  decodeComponentLogEventData,
  decodeGoogleCalendarAuthStartData,
  decodeIntegrationLogEventData,
  decodeIntegrationsData,
  decodeInboxItemData,
  decodeInterventionActionData,
  decodeLoopData,
  decodeMessageData,
  decodeNowData,
  decodeNullable,
  decodeProvenanceData,
  decodeRunSummaryData,
  decodeSettingsData,
  decodeSuggestionData,
  type ApiResponse,
  type ConversationData,
  type ContextExplainData,
  type CommitmentData,
  type CurrentContextData,
  type DriftExplainData,
  type ComponentData,
  type ComponentLogEventData,
  type GoogleCalendarAuthStartData,
  type IntegrationLogEventData,
  type InboxItemData,
  type IntegrationsData,
  type InterventionActionData,
  type LoopData,
  type MessageData,
  type NowData,
  type ProvenanceData,
  type RunSummaryData,
  type SettingsData,
  type SuggestionData,
} from '../types';

interface SyncResultData {
  source: string;
  signals_ingested: number;
}

interface EvaluateResultData {
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

export const queryKeys = {
  conversations: () => ['conversations'] as const,
  conversationMessages: (conversationId: string | null) => ['conversations', conversationId, 'messages'] as const,
  conversationInterventions: (conversationId: string | null) => ['conversations', conversationId, 'interventions'] as const,
  inbox: () => ['inbox'] as const,
  suggestions: (state: string) => ['suggestions', state] as const,
  suggestion: (suggestionId: string | null) => ['suggestions', suggestionId] as const,
  pendingInterventionActions: () => ['interventions', 'pending-actions'] as const,
  now: () => ['now'] as const,
  currentContext: () => ['context', 'current'] as const,
  contextExplain: () => ['context', 'explain'] as const,
  driftExplain: () => ['context', 'drift-explain'] as const,
  settings: () => ['settings'] as const,
  integrations: () => ['integrations'] as const,
  loops: () => ['loops'] as const,
  components: () => ['components'] as const,
  componentLogs: (componentId: string) => ['components', componentId, 'logs'] as const,
  integrationLogs: (integrationId: string) => ['integrations', integrationId, 'logs'] as const,
  commitments: (limit: number) => ['commitments', limit] as const,
  runs: (limit: number) => ['runs', limit] as const,
  provenance: (messageId: string | null) => ['messages', messageId, 'provenance'] as const,
};

export function loadConversationList(): Promise<ApiResponse<ConversationData[]>> {
  return apiGet<ApiResponse<ConversationData[]>>(
    '/api/conversations',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeConversationData)),
  );
}

export function loadConversationMessages(conversationId: string): Promise<ApiResponse<MessageData[]>> {
  return apiGet<ApiResponse<MessageData[]>>(
    `/api/conversations/${conversationId}/messages`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeMessageData)),
  );
}

export function loadConversationInterventions(conversationId: string): Promise<ApiResponse<InboxItemData[]>> {
  return apiGet<ApiResponse<InboxItemData[]>>(
    `/api/conversations/${conversationId}/interventions`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeInboxItemData)),
  );
}

export function loadInbox(): Promise<ApiResponse<InboxItemData[]>> {
  return apiGet<ApiResponse<InboxItemData[]>>(
    '/api/inbox',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeInboxItemData)),
  );
}

export function loadSuggestions(state = 'pending'): Promise<ApiResponse<SuggestionData[]>> {
  return apiGet<ApiResponse<SuggestionData[]>>(
    `/v1/suggestions?state=${encodeURIComponent(state)}&limit=50`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeSuggestionData)),
  );
}

export function loadSuggestion(suggestionId: string): Promise<ApiResponse<SuggestionData>> {
  return apiGet<ApiResponse<SuggestionData>>(
    `/v1/suggestions/${encodeURIComponent(suggestionId.trim())}`,
    (value) => decodeApiResponse(value, decodeSuggestionData),
  );
}

export function loadCurrentContext(): Promise<ApiResponse<CurrentContextData | null>> {
  return apiGet<ApiResponse<CurrentContextData | null>>(
    '/v1/context/current',
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeCurrentContextData)),
  );
}

export function loadNow(): Promise<ApiResponse<NowData>> {
  return apiGet<ApiResponse<NowData>>(
    '/v1/now',
    (value) => decodeApiResponse(value, decodeNowData),
  );
}

export function loadContextExplain(): Promise<ApiResponse<ContextExplainData>> {
  return apiGet<ApiResponse<ContextExplainData>>(
    '/v1/explain/context',
    (value) => decodeApiResponse(value, decodeContextExplainData),
  );
}

export function loadDriftExplain(): Promise<ApiResponse<DriftExplainData>> {
  return apiGet<ApiResponse<DriftExplainData>>(
    '/v1/explain/drift',
    (value) => decodeApiResponse(value, decodeDriftExplainData),
  );
}

export function loadSettings(): Promise<ApiResponse<SettingsData>> {
  return apiGet<ApiResponse<SettingsData>>(
    '/api/settings',
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function loadRecentRuns(limit: number): Promise<ApiResponse<RunSummaryData[]>> {
  return apiGet<ApiResponse<RunSummaryData[]>>(
    `/v1/runs?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeRunSummaryData)),
  );
}

export function loadIntegrations(): Promise<ApiResponse<IntegrationsData>> {
  return apiGet<ApiResponse<IntegrationsData>>(
    '/api/integrations',
    (value) => decodeApiResponse(value, decodeIntegrationsData),
  );
}

export function loadLoops(): Promise<ApiResponse<LoopData[]>> {
  return apiGet<ApiResponse<LoopData[]>>(
    '/v1/loops',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeLoopData)),
  );
}

export function loadComponents(): Promise<ApiResponse<ComponentData[]>> {
  return apiGet<ApiResponse<ComponentData[]>>(
    '/api/components',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeComponentData)),
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

export function restartComponent(componentId: string): Promise<ApiResponse<ComponentData>> {
  return apiPost<ApiResponse<ComponentData>>(
    `/api/components/${encodeURIComponent(componentId.trim())}/restart`,
    {},
    (value) => decodeApiResponse(value, decodeComponentData),
  );
}

export function loadCommitments(limit: number): Promise<ApiResponse<CommitmentData[]>> {
  return apiGet<ApiResponse<CommitmentData[]>>(
    `/v1/commitments?limit=${limit}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeCommitmentData)),
  );
}

export function decodeGoogleCalendarAuthStartResponse(value: unknown): ApiResponse<GoogleCalendarAuthStartData> {
  return decodeApiResponse(value, decodeGoogleCalendarAuthStartData);
}

export function loadProvenance(messageId: string): Promise<ApiResponse<ProvenanceData>> {
  return apiGet<ApiResponse<ProvenanceData>>(
    `/api/messages/${messageId}/provenance`,
    (value) => decodeApiResponse(value, decodeProvenanceData),
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

export function syncSource(
  source: string,
): Promise<ApiResponse<SyncResultData>> {
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

export function mutateIntervention(
  interventionId: string,
  action: 'snooze' | 'resolve' | 'dismiss',
  body: Record<string, unknown>,
): Promise<ApiResponse<InterventionActionData>> {
  return apiPost<ApiResponse<InterventionActionData>>(
    `/api/interventions/${interventionId}/${action}`,
    body,
    (value) => decodeApiResponse(value, decodeInterventionActionData),
  );
}

export function updateSuggestion(
  suggestionId: string,
  patch: Record<string, unknown>,
): Promise<ApiResponse<SuggestionData>> {
  return apiPatch<ApiResponse<SuggestionData>>(
    `/v1/suggestions/${encodeURIComponent(suggestionId.trim())}`,
    patch,
    (value) => decodeApiResponse(value, decodeSuggestionData),
  );
}
