import { apiGet } from '../api/client';
import {
  decodeApiResponse,
  decodeArray,
  decodeConversationData,
  decodeCurrentContextData,
  decodeInboxItemData,
  decodeMessageData,
  decodeNullable,
  decodeProvenanceData,
  decodeSettingsData,
  type ApiResponse,
  type ConversationData,
  type CurrentContextData,
  type InboxItemData,
  type MessageData,
  type ProvenanceData,
  type SettingsData,
} from '../types';

export const queryKeys = {
  conversations: () => ['conversations'] as const,
  conversationMessages: (conversationId: string | null) => ['conversations', conversationId, 'messages'] as const,
  conversationInterventions: (conversationId: string | null) => ['conversations', conversationId, 'interventions'] as const,
  inbox: () => ['inbox'] as const,
  currentContext: () => ['context', 'current'] as const,
  settings: () => ['settings'] as const,
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

export function loadSettings(): Promise<ApiResponse<SettingsData>> {
  return apiGet<ApiResponse<SettingsData>>(
    '/api/settings',
    (value) => decodeApiResponse(value, decodeSettingsData),
  );
}

export function loadProvenance(messageId: string): Promise<ApiResponse<ProvenanceData>> {
  return apiGet<ApiResponse<ProvenanceData>>(
    `/api/messages/${messageId}/provenance`,
    (value) => decodeApiResponse(value, decodeProvenanceData),
  );
}
