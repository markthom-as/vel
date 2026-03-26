import {
  appShellFeedbackPacket,
  assistantEntryFallbackPacket,
  collectRemoteRoutesPacket,
  EmbeddedBridgePacketKind,
  linkingFeedbackPacket,
  linkingRequestPacket,
  normalizeDomainHintPacket,
  normalizePairingTokenPacket,
  queuedActionPacket,
  threadDraftPacket,
  voiceCachedQueryResponsePacket,
  voiceContinuitySummaryPacket,
  voiceOfflineResponsePacket,
  voiceQuickActionPacket,
} from './embeddedBridgePackets';

export const EMBEDDED_BRIDGE_WEB_MODE = 'scaffold_only' as const;

export type EmbeddedBridgeWebMode = typeof EMBEDDED_BRIDGE_WEB_MODE;

export type EmbeddedBridgeRoute = {
  label: string;
  baseUrl: string;
};

export type EmbeddedBridgeQueuePacket = {
  queueKind: string;
  targetId: string | null;
  text: string | null;
  minutes: number | null;
  ready: boolean;
};

export type EmbeddedBridgeThreadDraftPacket = {
  payload: string;
  requestedConversationId: string | null;
};

export type EmbeddedBridgeVoiceContinuitySummaryPacket = {
  headline: string | null;
  detail: string | null;
  ready: boolean;
};

export type EmbeddedBridgeVoiceOfflineResponsePacket = {
  summary: string | null;
  detail: string | null;
  historyStatus: string;
  errorPrefix: string;
  ready: boolean;
};

export type EmbeddedBridgeSimpleMessagePacket = {
  message: string | null;
};

export type EmbeddedBridgeCachedQueryPacket = {
  summary: string | null;
  detail: string | null;
  ready: boolean;
};

export type EmbeddedBridgePayloadPacket = {
  payload: string;
};

function parsePacket<T>(kind: EmbeddedBridgePacketKind, payloadJson: string): T {
  const parsed = JSON.parse(payloadJson) as { kind?: string } & T;
  void kind;
  return parsed;
}

export function normalizePairingTokenValue(input: string): { tokenCode: string } {
  const response = normalizePairingTokenPacket(input);
  return parsePacket(response.kind, response.payloadJson);
}

export function normalizeDomainHintValue(input: string): { normalized: string } {
  const response = normalizeDomainHintPacket(input);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildQueuedActionValue(
  kind: string,
  targetId?: string | null,
  text?: string | null,
  minutes?: number | null,
): EmbeddedBridgeQueuePacket {
  const response = queuedActionPacket(kind, targetId, text, minutes);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildThreadDraftValue(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgeThreadDraftPacket {
  const response = threadDraftPacket(text, requestedConversationId);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildVoiceQuickActionValue(
  intentStorageToken: string,
  primaryText: string,
  targetId?: string | null,
  minutes?: number | null,
): EmbeddedBridgeQueuePacket {
  const response = voiceQuickActionPacket(intentStorageToken, primaryText, targetId, minutes);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildVoiceContinuitySummaryValue(
  draftExists?: boolean | null,
  threadedTranscript?: string | null,
  pendingRecoveryCount?: number | null,
  isReachable?: boolean | null,
  mergedTranscript?: string | null,
): EmbeddedBridgeVoiceContinuitySummaryPacket {
  const response = voiceContinuitySummaryPacket(
    draftExists,
    threadedTranscript,
    pendingRecoveryCount,
    isReachable,
    mergedTranscript,
  );
  return parsePacket(response.kind, response.payloadJson);
}

export function buildVoiceOfflineResponseValue(
  scenario: string,
  primaryText?: string | null,
  matchedText?: string | null,
  options?: string | null,
  minutes?: number | null,
  isReachable?: boolean | null,
): EmbeddedBridgeVoiceOfflineResponsePacket {
  const response = voiceOfflineResponsePacket(
    scenario,
    primaryText,
    matchedText,
    options,
    minutes,
    isReachable,
  );
  return parsePacket(response.kind, response.payloadJson);
}

export function buildVoiceCachedQueryValue(
  scenario: string,
  nextTitle?: string | null,
  leaveBy?: string | null,
  emptyMessage?: string | null,
  cachedNowSummary?: string | null,
  firstReason?: string | null,
  nextCommitmentText?: string | null,
  nextCommitmentDueAt?: string | null,
  behaviorHeadline?: string | null,
  behaviorReason?: string | null,
): EmbeddedBridgeCachedQueryPacket {
  const response = voiceCachedQueryResponsePacket(
    scenario,
    nextTitle,
    leaveBy,
    emptyMessage,
    cachedNowSummary,
    firstReason,
    nextCommitmentText,
    nextCommitmentDueAt,
    behaviorHeadline,
    behaviorReason,
  );
  return parsePacket(response.kind, response.payloadJson);
}

export function buildAssistantEntryFallbackValue(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgePayloadPacket {
  const response = assistantEntryFallbackPacket(text, requestedConversationId);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildLinkingRequestValue(
  tokenCode?: string | null,
  targetBaseUrl?: string | null,
): { tokenCode: string | null; targetBaseUrl: string | null } {
  const response = linkingRequestPacket(tokenCode, targetBaseUrl);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildLinkingFeedbackValue(
  scenario: string,
  nodeDisplayName?: string | null,
): EmbeddedBridgeSimpleMessagePacket {
  const response = linkingFeedbackPacket(scenario, nodeDisplayName);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildAppShellFeedbackValue(
  scenario: string,
  detail?: string | null,
): EmbeddedBridgeSimpleMessagePacket {
  const response = appShellFeedbackPacket(scenario, detail);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildRemoteRoutesValue(
  syncBaseUrl?: string | null,
  tailscaleBaseUrl?: string | null,
  lanBaseUrl?: string | null,
  publicBaseUrl?: string | null,
): EmbeddedBridgeRoute[] {
  const response = collectRemoteRoutesPacket(
    syncBaseUrl,
    tailscaleBaseUrl,
    lanBaseUrl,
    publicBaseUrl,
  );
  return parsePacket(response.kind, response.payloadJson);
}
