import { canonicalPostMutation, canonicalQuery } from './canonicalTransport';
import { invalidateQuery } from './query';
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

/** Operator queue vs archived (resolved/dismissed) interventions from `/api/inbox`. */
export type InboxScope = 'queue' | 'archive';

export const chatQueryKeys = {
  conversations: () => ['conversations'] as const,
  conversationMessages: (conversationId: string | null) => ['conversations', conversationId, 'messages'] as const,
  conversationInterventions: (conversationId: string | null) => ['conversations', conversationId, 'interventions'] as const,
  inbox: (scope: InboxScope = 'queue') => ['inbox', scope] as const,
  pendingInterventionActions: () => ['interventions', 'pending-actions'] as const,
  provenance: (messageId: string | null) => ['messages', messageId, 'provenance'] as const,
};

/** Refetch both inbox scopes after mutations or realtime updates that affect either list. */
export function invalidateInboxQueries(): void {
  invalidateQuery(chatQueryKeys.inbox('queue'), { refetch: true });
  invalidateQuery(chatQueryKeys.inbox('archive'), { refetch: true });
}

export function loadConversationList(): Promise<ApiResponse<ConversationData[]>> {
  return canonicalQuery<ConversationData[]>(
    '/api/conversations',
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeConversationData)),
  );
}

export function loadConversationMessages(conversationId: string): Promise<ApiResponse<MessageData[]>> {
  return canonicalQuery<MessageData[]>(
    `/api/conversations/${conversationId}/messages`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeMessageData)),
  );
}

export function submitAssistantEntry(
  text: string,
  conversationId?: string | null,
  voice?: AssistantEntryVoiceProvenanceData | null,
): Promise<ApiResponse<AssistantEntryResponse>> {
  return canonicalPostMutation<AssistantEntryResponse>(
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
  return canonicalQuery<InboxItemData[]>(
    `/api/conversations/${conversationId}/interventions`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeInboxItemData)),
  );
}

export function loadInbox(scope: InboxScope = 'queue'): Promise<ApiResponse<InboxItemData[]>> {
  const suffix = scope === 'archive' ? '?scope=archive' : '';
  return canonicalQuery<InboxItemData[]>(
    `/api/inbox${suffix}`,
    (value) => decodeApiResponse(value, (data) => decodeArray(data, decodeInboxItemData)),
  );
}

export function loadProvenance(messageId: string): Promise<ApiResponse<ProvenanceData>> {
  return canonicalQuery<ProvenanceData>(
    `/api/messages/${messageId}/provenance`,
    (value) => decodeApiResponse(value, decodeProvenanceData),
  );
}

export function mutateIntervention(
  interventionId: string,
  action: 'acknowledge' | 'snooze' | 'resolve' | 'dismiss' | 'reactivate',
  body: Record<string, unknown>,
): Promise<ApiResponse<InterventionActionData>> {
  return canonicalPostMutation<InterventionActionData>(
    `/api/interventions/${encodeURIComponent(interventionId)}/${action}`,
    body,
    (value) => decodeApiResponse(value, decodeInterventionActionData),
  );
}

/** Stable intervention id for `/api/interventions/...` (global inbox uses synthetic `act_intervention_*` ids). */
export function getInterventionApiId(item: InboxItemData): string | null {
  if (item.id.startsWith('intv_')) {
    return item.id;
  }
  const fromEvidence = item.evidence.find(
    (e) => e.source_kind === 'intervention' || e.source_kind === 'assistant_proposal',
  );
  if (fromEvidence?.source_id) {
    return fromEvidence.source_id;
  }
  const prefix = 'act_intervention_';
  if (item.id.startsWith(prefix)) {
    return item.id.slice(prefix.length);
  }
  return null;
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

export function resolveInboxItem(
  interventionId: string,
): Promise<ApiResponse<InterventionActionData>> {
  return mutateIntervention(interventionId, 'resolve', {});
}

export function reactivateInboxItem(
  interventionId: string,
): Promise<ApiResponse<InterventionActionData>> {
  return mutateIntervention(interventionId, 'reactivate', {});
}

export function getInboxThreadPath(item: InboxItemData): string | null {
  if (!item.conversation_id || !item.available_actions.includes('open_thread')) {
    return null;
  }
  return `/api/conversations/${item.conversation_id}`;
}
