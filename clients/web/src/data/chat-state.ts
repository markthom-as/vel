import type { ConversationData, InboxItemData, MessageData } from '../types';

export interface PendingInterventionAction {
  state: string;
  confirmed: boolean;
}

function extractMessageText(content: MessageData['content']): string | null {
  if (!content || typeof content !== 'object' || Array.isArray(content)) {
    return null;
  }
  const text = (content as Record<string, unknown>).text;
  return typeof text === 'string' ? text.trim() : null;
}

export function appendUniqueMessages(existing: MessageData[], nextMessages: MessageData[]): MessageData[] {
  if (nextMessages.length === 0) {
    return existing;
  }

  const seen = new Set(existing.map((message) => message.id));
  const additions = nextMessages.filter((message) => {
    if (seen.has(message.id)) {
      return false;
    }
    seen.add(message.id);
    return true;
  });

  return additions.length > 0 ? [...existing, ...additions] : existing;
}

export function reconcileIncomingMessage(existing: MessageData[], incoming: MessageData): MessageData[] {
  const existingIndex = existing.findIndex((message) => message.id === incoming.id);
  if (existingIndex !== -1) {
    const next = [...existing];
    next[existingIndex] = incoming;
    return next;
  }

  const incomingText = incoming.kind === 'text' ? extractMessageText(incoming.content) : null;
  const pendingIndex = existing.findIndex((message) => {
    if (
      message.status !== 'sending'
      || message.conversation_id !== incoming.conversation_id
      || message.role !== incoming.role
      || message.kind !== incoming.kind
    ) {
      return false;
    }
    if (incomingText !== null && message.kind === 'text') {
      const pendingText = extractMessageText(message.content);
      return pendingText !== null && pendingText === incomingText;
    }
    return JSON.stringify(message.content) === JSON.stringify(incoming.content);
  });
  if (pendingIndex !== -1) {
    const next = [...existing];
    next[pendingIndex] = incoming;
    return next;
  }

  return [...existing, incoming];
}

export function reconcileConfirmedSend(
  existing: MessageData[],
  clientMessageId: string | undefined,
  userMessage: MessageData,
  assistantMessages: MessageData[],
): MessageData[] {
  let next = clientMessageId
    ? existing.filter((message) => message.id !== clientMessageId)
    : [...existing];
  next = reconcileIncomingMessage(next, userMessage);
  for (const assistantMessage of assistantMessages) {
    next = reconcileIncomingMessage(next, assistantMessage);
  }
  return next;
}

export function reconcileConversationFromMessage(
  conversations: ConversationData[],
  message: MessageData,
): ConversationData[] {
  const conversationIndex = conversations.findIndex((conversation) => conversation.id === message.conversation_id);
  if (conversationIndex === -1) {
    return conversations;
  }

  const updatedAt = Math.max(message.created_at, conversations[conversationIndex].updated_at);
  if (updatedAt === conversations[conversationIndex].updated_at) {
    return conversations;
  }

  const next = [...conversations];
  next[conversationIndex] = {
    ...next[conversationIndex],
    updated_at: updatedAt,
  };
  return next;
}

export function upsertInboxItem(
  items: InboxItemData[],
  nextItem: InboxItemData,
  pendingInterventionActions: Record<string, PendingInterventionAction>,
): InboxItemData[] {
  if (pendingInterventionActions[nextItem.id]) {
    return items;
  }

  const existingIndex = items.findIndex((item) => item.id === nextItem.id);
  if (existingIndex === -1) {
    return [nextItem, ...items];
  }
  const next = [...items];
  next[existingIndex] = nextItem;
  return next;
}

export function removeInterventionById(items: InboxItemData[], interventionId: string): InboxItemData[] {
  return items.filter((item) => item.id !== interventionId);
}

export function prunePendingInterventionActions(
  pendingActions: Record<string, PendingInterventionAction> | undefined,
  interventions: InboxItemData[],
): Record<string, PendingInterventionAction> {
  const next: Record<string, PendingInterventionAction> = {};
  let changed = false;

  for (const [interventionId, action] of Object.entries(pendingActions ?? {})) {
    if (!action.confirmed || interventions.some((intervention) => intervention.id === interventionId)) {
      next[interventionId] = action;
    } else {
      changed = true;
    }
  }

  return changed ? next : (pendingActions ?? {});
}

export function setPendingInterventionAction(
  pendingActions: Record<string, PendingInterventionAction> | undefined,
  interventionId: string,
  state: string,
): Record<string, PendingInterventionAction> {
  return {
    ...(pendingActions ?? {}),
    [interventionId]: {
      state,
      confirmed: false,
    },
  };
}

export function markPendingInterventionActionConfirmed(
  pendingActions: Record<string, PendingInterventionAction> | undefined,
  interventionId: string,
  state: string,
): Record<string, PendingInterventionAction> {
  const pendingAction = (pendingActions ?? {})[interventionId];
  if (!pendingAction || pendingAction.state !== state) {
    return pendingActions ?? {};
  }

  return {
    ...pendingActions,
    [interventionId]: {
      ...pendingAction,
      confirmed: true,
    },
  };
}
