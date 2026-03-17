import { apiGet, apiPost } from '../api/client';
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
  decodeIntegrationsData,
  decodeInboxItemData,
  decodeMessageData,
  decodeNullable,
  decodeProvenanceData,
  decodeRunSummaryData,
  decodeSettingsData,
  type ApiResponse,
  type ConversationData,
  type ContextExplainData,
  type CommitmentData,
  type CurrentContextData,
  type DriftExplainData,
  type ComponentData,
  type ComponentLogEventData,
  type GoogleCalendarAuthStartData,
  type InboxItemData,
  type IntegrationsData,
  type MessageData,
  type ProvenanceData,
  type RunSummaryData,
  type SettingsData,
} from '../types';

export const queryKeys = {
  conversations: () => ['conversations'] as const,
  conversationMessages: (conversationId: string | null) => ['conversations', conversationId, 'messages'] as const,
  conversationInterventions: (conversationId: string | null) => ['conversations', conversationId, 'interventions'] as const,
  inbox: () => ['inbox'] as const,
  pendingInterventionActions: () => ['interventions', 'pending-actions'] as const,
  currentContext: () => ['context', 'current'] as const,
  contextExplain: () => ['context', 'explain'] as const,
  driftExplain: () => ['context', 'drift-explain'] as const,
  settings: () => ['settings'] as const,
  integrations: () => ['integrations'] as const,
  components: () => ['components'] as const,
  componentLogs: (componentId: string) => ['components', componentId, 'logs'] as const,
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

export function loadCurrentContext(): Promise<ApiResponse<CurrentContextData | null>> {
  return apiGet<ApiResponse<CurrentContextData | null>>(
    '/v1/context/current',
    (value) => decodeApiResponse(value, (data) => decodeNullable(data, decodeCurrentContextData)),
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

export function restartComponent(componentId: string): Promise<ApiResponse<ComponentData>> {
  return apiPost<ApiResponse<ComponentData>>(
    `/api/components/${encodeURIComponent(componentId.trim())}/restart`,
    {},
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
