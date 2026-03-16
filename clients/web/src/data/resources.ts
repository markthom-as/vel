import { apiGet } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeConversationData,
  decodeContextExplainData,
  decodeCurrentContextData,
  decodeDriftExplainData,
  decodeInboxItemData,
  decodeMessageData,
  decodeNullable,
  decodeProvenanceData,
  decodeRunSummaryData,
  decodeSettingsData,
  type ApiResponse,
  type ConversationData,
  type ContextExplainData,
  type CurrentContextData,
  type DriftExplainData,
  type InboxItemData,
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

export function loadProvenance(messageId: string): Promise<ApiResponse<ProvenanceData>> {
  return apiGet<ApiResponse<ProvenanceData>>(
    `/api/messages/${messageId}/provenance`,
    (value) => decodeApiResponse(value, decodeProvenanceData),
  );
}
