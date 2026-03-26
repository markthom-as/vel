import {
  actionItemDedupeKeyPacket,
  appShellFeedbackPacket,
  assistantEntryFallbackPacket,
  collectRemoteRoutesPacket,
  linkingFeedbackPacket,
  linkingRequestPacket,
  normalizeDomainHintPacket,
  normalizePairingTokenPacket,
  normalizeSemanticLabelPacket,
  normalizeTaskDisplayBatchPacket,
  normalizeTaskDisplayPacket,
  queuedActionPacket,
  shortClientKindLabelPacket,
  threadDraftPacket,
  voiceCachedQueryResponsePacket,
  voiceContinuitySummaryPacket,
  voiceOfflineResponsePacket,
  voiceQuickActionPacket,
} from './embeddedBridgePackets';
import type { EmbeddedBridgePacketKind } from './embeddedBridgePackets';
import { maybeInstallEmbeddedBridgePacketRuntimeFromGlobal } from './embeddedBridgeWasmRuntime';

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

export type EmbeddedBridgeTaskDisplayPacket = {
  tags: string[];
  project: string | null;
};

export type EmbeddedBridgeClientKindPacket = {
  shortLabel: string | null;
};

export type EmbeddedBridgeActionItemDedupeKeyPacket = {
  key: string;
};

type EmbeddedBridgeTaskDisplayBatchPacket = {
  items: EmbeddedBridgeTaskDisplayPacket[];
};

const semanticLabelCache = new Map<string, { normalized: string }>();
const taskDisplayCache = new Map<string, EmbeddedBridgeTaskDisplayPacket>();
const clientKindCache = new Map<string, EmbeddedBridgeClientKindPacket>();
const actionItemDedupeKeyCache = new Map<string, EmbeddedBridgeActionItemDedupeKeyPacket>();

function parsePacket<T>(kind: EmbeddedBridgePacketKind, payloadJson: string): T {
  const parsed = JSON.parse(payloadJson) as { kind?: string } & T;
  void kind;
  return parsed;
}

function ensureEmbeddedBridgeRuntime(): void {
  maybeInstallEmbeddedBridgePacketRuntimeFromGlobal();
}

export function normalizePairingTokenValue(input: string): { tokenCode: string } {
  ensureEmbeddedBridgeRuntime();
  const response = normalizePairingTokenPacket(input);
  return parsePacket(response.kind, response.payloadJson);
}

export function normalizeDomainHintValue(input: string): { normalized: string } {
  ensureEmbeddedBridgeRuntime();
  const response = normalizeDomainHintPacket(input);
  return parsePacket(response.kind, response.payloadJson);
}

export function normalizeSemanticLabelValue(input: string): { normalized: string } {
  ensureEmbeddedBridgeRuntime();
  const cached = semanticLabelCache.get(input);
  if (cached) {
    return cached;
  }
  const response = normalizeSemanticLabelPacket(input);
  const parsed = parsePacket<{ normalized: string }>(response.kind, response.payloadJson);
  semanticLabelCache.set(input, parsed);
  return parsed;
}

export function normalizeTaskDisplayValue(
  tags?: string[] | null,
  project?: string | null,
): EmbeddedBridgeTaskDisplayPacket {
  ensureEmbeddedBridgeRuntime();
  const cacheKey = JSON.stringify([tags ?? null, project ?? null]);
  const cached = taskDisplayCache.get(cacheKey);
  if (cached) {
    return cached;
  }
  const response = normalizeTaskDisplayPacket(tags, project);
  const parsed = parsePacket<EmbeddedBridgeTaskDisplayPacket>(response.kind, response.payloadJson);
  taskDisplayCache.set(cacheKey, parsed);
  return parsed;
}

export function normalizeTaskDisplayBatchValue(
  entries: Array<{ tags?: string[] | null; project?: string | null }>,
): EmbeddedBridgeTaskDisplayPacket[] {
  ensureEmbeddedBridgeRuntime();
  const response = normalizeTaskDisplayBatchPacket(
    JSON.stringify(entries.map((entry) => ({
      tags: entry.tags ?? null,
      project: entry.project ?? null,
    }))),
  );
  return parsePacket<EmbeddedBridgeTaskDisplayBatchPacket>(response.kind, response.payloadJson).items;
}

export function shortClientKindLabelValue(
  clientKind?: string | null,
): EmbeddedBridgeClientKindPacket {
  ensureEmbeddedBridgeRuntime();
  const cacheKey = clientKind ?? '__null__';
  const cached = clientKindCache.get(cacheKey);
  if (cached) {
    return cached;
  }
  const response = shortClientKindLabelPacket(clientKind);
  const parsed = parsePacket<EmbeddedBridgeClientKindPacket>(response.kind, response.payloadJson);
  clientKindCache.set(cacheKey, parsed);
  return parsed;
}

export function actionItemDedupeKeyValue(
  kind: string,
  title: string,
  summary: string,
  projectLabel?: string | null,
  threadId?: string | null,
  threadLabel?: string | null,
): EmbeddedBridgeActionItemDedupeKeyPacket {
  ensureEmbeddedBridgeRuntime();
  const cacheKey = JSON.stringify([
    kind,
    title,
    summary,
    projectLabel ?? null,
    threadId ?? null,
    threadLabel ?? null,
  ]);
  const cached = actionItemDedupeKeyCache.get(cacheKey);
  if (cached) {
    return cached;
  }
  const response = actionItemDedupeKeyPacket(
    kind,
    title,
    summary,
    projectLabel,
    threadId,
    threadLabel,
  );
  const parsed = parsePacket<EmbeddedBridgeActionItemDedupeKeyPacket>(
    response.kind,
    response.payloadJson,
  );
  actionItemDedupeKeyCache.set(cacheKey, parsed);
  return parsed;
}

export function buildQueuedActionValue(
  kind: string,
  targetId?: string | null,
  text?: string | null,
  minutes?: number | null,
): EmbeddedBridgeQueuePacket {
  ensureEmbeddedBridgeRuntime();
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
  ensureEmbeddedBridgeRuntime();
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
  ensureEmbeddedBridgeRuntime();
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
  ensureEmbeddedBridgeRuntime();
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
  ensureEmbeddedBridgeRuntime();
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
  ensureEmbeddedBridgeRuntime();
  const response = assistantEntryFallbackPacket(text, requestedConversationId);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildLinkingRequestValue(
  tokenCode?: string | null,
  targetBaseUrl?: string | null,
): { tokenCode: string | null; targetBaseUrl: string | null } {
  ensureEmbeddedBridgeRuntime();
  const response = linkingRequestPacket(tokenCode, targetBaseUrl);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildLinkingFeedbackValue(
  scenario: string,
  nodeDisplayName?: string | null,
): EmbeddedBridgeSimpleMessagePacket {
  ensureEmbeddedBridgeRuntime();
  const response = linkingFeedbackPacket(scenario, nodeDisplayName);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildAppShellFeedbackValue(
  scenario: string,
  detail?: string | null,
): EmbeddedBridgeSimpleMessagePacket {
  ensureEmbeddedBridgeRuntime();
  const response = appShellFeedbackPacket(scenario, detail);
  return parsePacket(response.kind, response.payloadJson);
}

export function buildRemoteRoutesValue(
  syncBaseUrl?: string | null,
  tailscaleBaseUrl?: string | null,
  lanBaseUrl?: string | null,
  publicBaseUrl?: string | null,
): EmbeddedBridgeRoute[] {
  ensureEmbeddedBridgeRuntime();
  const response = collectRemoteRoutesPacket(
    syncBaseUrl,
    tailscaleBaseUrl,
    lanBaseUrl,
    publicBaseUrl,
  );
  return parsePacket(response.kind, response.payloadJson);
}
