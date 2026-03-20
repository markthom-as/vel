import { apiGet, apiPost } from '../api/client';
import {
  decodeAssistantEntryResponse,
  decodeApiResponse,
  decodeArray,
  decodeConversationData,
  decodeInboxItemData,
  decodeInterventionActionData,
  decodeMessageData,
  decodeProvenanceData,
  type AssistantEntryResponse,
  type AssistantEntryVoiceProvenanceData,
  type ApiResponse,
  type ConversationData,
  type InboxItemData,
  type InterventionActionData,
  type MessageData,
  type ProvenanceData,
} from '../types';

export const chatQueryKeys = {
  conversations: () => ['conversations'] as const,
  conversationMessages: (conversationId: string | null) => ['conversations', conversationId, 'messages'] as const,
  conversationInterventions: (conversationId: string | null) => ['conversations', conversationId, 'interventions'] as const,
  inbox: () => ['inbox'] as const,
  pendingInterventionActions: () => ['interventions', 'pending-actions'] as const,
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

export function submitAssistantEntry(
  text: string,
  conversationId?: string | null,
  voice?: AssistantEntryVoiceProvenanceData | null,
): Promise<ApiResponse<AssistantEntryResponse>> {
  return apiPost<ApiResponse<AssistantEntryResponse>>(
    '/api/assistant/entry',
    {
      text,
      ...(conversationId ? { conversation_id: conversationId } : {}),
      ...(voice ? { voice } : {}),
    },
    (value) => decodeApiResponse(value, decodeAssistantEntryResponse),
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

export function loadProvenance(messageId: string): Promise<ApiResponse<ProvenanceData>> {
  return apiGet<ApiResponse<ProvenanceData>>(
    `/api/messages/${messageId}/provenance`,
    (value) => decodeApiResponse(value, decodeProvenanceData),
  );
}

export function mutateIntervention(
  interventionId: string,
  action: 'acknowledge' | 'snooze' | 'resolve' | 'dismiss',
  body: Record<string, unknown>,
): Promise<ApiResponse<InterventionActionData>> {
  return apiPost<ApiResponse<InterventionActionData>>(
    `/api/interventions/${interventionId}/${action}`,
    body,
    (value) => decodeApiResponse(value, decodeInterventionActionData),
  );
}

export function acknowledgeInboxItem(
  interventionId: string,
): Promise<ApiResponse<InterventionActionData>> {
  return mutateIntervention(interventionId, 'acknowledge', {});
}

export function snoozeInboxItem(
  interventionId: string,
  minutes: number,
): Promise<ApiResponse<InterventionActionData>> {
  return mutateIntervention(interventionId, 'snooze', { minutes });
}

export function dismissInboxItem(
  interventionId: string,
): Promise<ApiResponse<InterventionActionData>> {
  return mutateIntervention(interventionId, 'dismiss', {});
}

export function getInboxThreadPath(item: InboxItemData): string | null {
  if (!item.conversation_id || !item.available_actions.includes('open_thread')) {
    return null;
  }
  return `/api/conversations/${item.conversation_id}`;
}
