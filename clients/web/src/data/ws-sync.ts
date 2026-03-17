import type {
  ComponentData,
  ConversationData,
  CurrentContextData,
  InboxItemData,
  MessageData,
  RunSummaryData,
  WsEvent,
} from '../types';
import { subscribeWs } from '../realtime/ws';
import {
  getQueryData,
  invalidateQuery,
  listLoadedQueryKeys,
  setQueryData,
  type QueryKey,
} from './query';
import {
  markPendingInterventionActionConfirmed,
  reconcileConversationFromMessage,
  reconcileIncomingMessage,
  upsertInboxItem,
  type PendingInterventionAction,
} from './chat-state';
import { queryKeys } from './resources';

let refCount = 0;
let unsubscribe: (() => void) | null = null;

function isConversationMessagesKey(key: QueryKey): key is readonly [string, string | null, string] {
  return key.length === 3 && key[0] === 'conversations' && key[2] === 'messages';
}

function isConversationInterventionsKey(
  key: QueryKey,
): key is readonly [string, string | null, string] {
  return key.length === 3 && key[0] === 'conversations' && key[2] === 'interventions';
}

function isRunsKey(key: QueryKey): boolean {
  return key.length === 2 && key[0] === 'runs';
}

function isComponentsKey(key: QueryKey): boolean {
  return key.length === 1 && key[0] === 'components';
}

function pendingInterventionActions() {
  return getQueryData<Record<string, PendingInterventionAction>>(queryKeys.pendingInterventionActions()) ?? {};
}

function updateRunsCache(run: RunSummaryData) {
  for (const key of listLoadedQueryKeys()) {
    if (!isRunsKey(key)) {
      continue;
    }
    const limit = typeof key[1] === 'number' ? key[1] : null;
    setQueryData<RunSummaryData[]>(key, (current = []) => {
      const next = [...current];
      const index = next.findIndex((existingRun) => existingRun.id === run.id);
      if (index >= 0) {
        next[index] = run;
        return next;
      }
      const updated = [run, ...next];
      return typeof limit === 'number' ? updated.slice(0, limit) : updated;
    });
  }
}

function updateComponentsCache(component: ComponentData) {
  for (const key of listLoadedQueryKeys()) {
    if (!isComponentsKey(key)) {
      continue;
    }
    setQueryData<ComponentData[]>(key, (current = []) => {
      const next = [...current];
      const index = next.findIndex((existing) => existing.id === component.id);
      if (index >= 0) {
        next[index] = component;
        return next;
      }
      return [...next, component];
    });
  }
}

function applyMessageEvent(message: MessageData) {
  setQueryData<ConversationData[]>(queryKeys.conversations(), (current = []) =>
    reconcileConversationFromMessage(current, message),
  );

  for (const key of listLoadedQueryKeys()) {
    if (!isConversationMessagesKey(key) || key[1] !== message.conversation_id) {
      continue;
    }
    setQueryData<MessageData[]>(key, (current = []) => reconcileIncomingMessage(current, message));
  }
}

function applyInterventionCreated(inboxItem: InboxItemData) {
  const pendingActions = pendingInterventionActions();
  setQueryData<InboxItemData[]>(queryKeys.inbox(), (current = []) =>
    upsertInboxItem(current, inboxItem, pendingActions),
  );

  for (const key of listLoadedQueryKeys()) {
    if (!isConversationInterventionsKey(key)) {
      continue;
    }
    const conversationId = key[1];
    if (!conversationId) {
      continue;
    }
    const messages = getQueryData<MessageData[]>(queryKeys.conversationMessages(conversationId)) ?? [];
    if (!messages.some((message) => message.id === inboxItem.message_id)) {
      continue;
    }
    setQueryData<InboxItemData[]>(key, (current = []) =>
      upsertInboxItem(current, inboxItem, pendingActions),
    );
  }
}

function applyInterventionUpdated(id: string, state: string) {
  setQueryData<Record<string, PendingInterventionAction>>(
    queryKeys.pendingInterventionActions(),
    (current = {}) => markPendingInterventionActionConfirmed(current, id, state),
  );
  invalidateQuery(queryKeys.inbox(), { refetch: true });
  for (const key of listLoadedQueryKeys()) {
    if (isConversationInterventionsKey(key)) {
      invalidateQuery(key, { refetch: true });
    }
  }
}

function applyContextUpdated(currentContext: CurrentContextData) {
  setQueryData(queryKeys.currentContext(), currentContext);
  invalidateQuery(queryKeys.now(), { refetch: true });
  invalidateQuery(queryKeys.contextExplain(), { refetch: true });
  invalidateQuery(queryKeys.driftExplain(), { refetch: true });

  for (const key of listLoadedQueryKeys()) {
    if (key.length === 2 && key[0] === 'commitments') {
      invalidateQuery(key, { refetch: true });
    }
  }
}

function applyWsEvent(event: WsEvent) {
  switch (event.type) {
    case 'messages:new':
      applyMessageEvent(event.payload);
      break;
    case 'interventions:new':
      applyInterventionCreated(event.payload);
      break;
    case 'interventions:updated':
      applyInterventionUpdated(event.payload.id, event.payload.state);
      break;
    case 'context:updated':
      applyContextUpdated(event.payload);
      break;
    case 'runs:updated':
      updateRunsCache(event.payload);
      break;
    case 'components:updated':
      updateComponentsCache(event.payload);
      break;
  }
}

export function subscribeWsQuerySync(): () => void {
  refCount += 1;
  if (!unsubscribe) {
    unsubscribe = subscribeWs(applyWsEvent);
  }

  return () => {
    refCount -= 1;
    if (refCount <= 0) {
      refCount = 0;
      unsubscribe?.();
      unsubscribe = null;
    }
  };
}

export function resetWsQuerySyncForTests(): void {
  refCount = 0;
  unsubscribe?.();
  unsubscribe = null;
}
