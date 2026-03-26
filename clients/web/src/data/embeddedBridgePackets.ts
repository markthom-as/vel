export type EmbeddedBridgePacketKind =
  | 'deterministic_domain_helpers'
  | 'linking_settings_normalization'
  | 'linking_request_packaging'
  | 'linking_feedback_packaging'
  | 'app_shell_feedback_packaging'
  | 'queued_action_packaging'
  | 'thread_draft_packaging'
  | 'voice_capture_packaging'
  | 'voice_quick_action_packaging'
  | 'assistant_entry_fallback_packaging'
  | 'capture_metadata_packaging'
  | 'voice_continuity_summary_packaging'
  | 'voice_offline_response_packaging'
  | 'voice_cached_query_packaging';

export type EmbeddedBridgePacketResponse = {
  kind: EmbeddedBridgePacketKind;
  payloadJson: string;
};

export interface EmbeddedBridgePacketRuntime {
  normalizePairingTokenPacket(input: string): EmbeddedBridgePacketResponse;
  normalizeDomainHintPacket(input: string): EmbeddedBridgePacketResponse;
  normalizeSemanticLabelPacket(input: string): EmbeddedBridgePacketResponse;
  queuedActionPacket(
    kind: string,
    targetId?: string | null,
    text?: string | null,
    minutes?: number | null,
  ): EmbeddedBridgePacketResponse;
  voiceQuickActionPacket(
    intentStorageToken: string,
    primaryText: string,
    targetId?: string | null,
    minutes?: number | null,
  ): EmbeddedBridgePacketResponse;
  assistantEntryFallbackPacket(
    text: string,
    requestedConversationId?: string | null,
  ): EmbeddedBridgePacketResponse;
  captureMetadataPacket(
    text: string,
    captureType: string,
    sourceDevice: string,
  ): EmbeddedBridgePacketResponse;
  threadDraftPacket(
    text: string,
    requestedConversationId?: string | null,
  ): EmbeddedBridgePacketResponse;
  voiceCapturePacket(
    transcript: string,
    intentStorageToken: string,
  ): EmbeddedBridgePacketResponse;
  linkingRequestPacket(
    tokenCode?: string | null,
    targetBaseUrl?: string | null,
  ): EmbeddedBridgePacketResponse;
  linkingFeedbackPacket(
    scenario: string,
    nodeDisplayName?: string | null,
  ): EmbeddedBridgePacketResponse;
  appShellFeedbackPacket(
    scenario: string,
    detail?: string | null,
  ): EmbeddedBridgePacketResponse;
  collectRemoteRoutesPacket(
    syncBaseUrl?: string | null,
    tailscaleBaseUrl?: string | null,
    lanBaseUrl?: string | null,
    publicBaseUrl?: string | null,
  ): EmbeddedBridgePacketResponse;
  voiceContinuitySummaryPacket(
    draftExists?: boolean | null,
    threadedTranscript?: string | null,
    pendingRecoveryCount?: number | null,
    isReachable?: boolean | null,
    mergedTranscript?: string | null,
  ): EmbeddedBridgePacketResponse;
  voiceOfflineResponsePacket(
    scenario: string,
    primaryText?: string | null,
    matchedText?: string | null,
    options?: string | null,
    minutes?: number | null,
    isReachable?: boolean | null,
  ): EmbeddedBridgePacketResponse;
  voiceCachedQueryResponsePacket(
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
  ): EmbeddedBridgePacketResponse;
}

let installedRuntime: EmbeddedBridgePacketRuntime | null = null;

export function installEmbeddedBridgePacketRuntime(runtime: EmbeddedBridgePacketRuntime): void {
  installedRuntime = runtime;
}

export function getEmbeddedBridgePacketRuntime(): EmbeddedBridgePacketRuntime {
  if (installedRuntime == null) {
    throw new Error(
      'Embedded bridge Rust runtime is not installed. Web packet shaping no longer has a TypeScript fallback.',
    );
  }
  return installedRuntime;
}

export function normalizePairingTokenPacket(input: string): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().normalizePairingTokenPacket(input);
}

export function normalizeDomainHintPacket(input: string): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().normalizeDomainHintPacket(input);
}

export function normalizeSemanticLabelPacket(input: string): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().normalizeSemanticLabelPacket(input);
}

export function queuedActionPacket(
  kind: string,
  targetId?: string | null,
  text?: string | null,
  minutes?: number | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().queuedActionPacket(kind, targetId, text, minutes);
}

export function voiceQuickActionPacket(
  intentStorageToken: string,
  primaryText: string,
  targetId?: string | null,
  minutes?: number | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().voiceQuickActionPacket(
    intentStorageToken,
    primaryText,
    targetId,
    minutes,
  );
}

export function assistantEntryFallbackPacket(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().assistantEntryFallbackPacket(
    text,
    requestedConversationId,
  );
}

export function captureMetadataPacket(
  text: string,
  captureType: string,
  sourceDevice: string,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().captureMetadataPacket(text, captureType, sourceDevice);
}

export function threadDraftPacket(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().threadDraftPacket(text, requestedConversationId);
}

export function voiceCapturePacket(
  transcript: string,
  intentStorageToken: string,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().voiceCapturePacket(transcript, intentStorageToken);
}

export function linkingRequestPacket(
  tokenCode?: string | null,
  targetBaseUrl?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().linkingRequestPacket(tokenCode, targetBaseUrl);
}

export function linkingFeedbackPacket(
  scenario: string,
  nodeDisplayName?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().linkingFeedbackPacket(scenario, nodeDisplayName);
}

export function appShellFeedbackPacket(
  scenario: string,
  detail?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().appShellFeedbackPacket(scenario, detail);
}

export function collectRemoteRoutesPacket(
  syncBaseUrl?: string | null,
  tailscaleBaseUrl?: string | null,
  lanBaseUrl?: string | null,
  publicBaseUrl?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().collectRemoteRoutesPacket(
    syncBaseUrl,
    tailscaleBaseUrl,
    lanBaseUrl,
    publicBaseUrl,
  );
}

export function voiceContinuitySummaryPacket(
  draftExists?: boolean | null,
  threadedTranscript?: string | null,
  pendingRecoveryCount?: number | null,
  isReachable?: boolean | null,
  mergedTranscript?: string | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().voiceContinuitySummaryPacket(
    draftExists,
    threadedTranscript,
    pendingRecoveryCount,
    isReachable,
    mergedTranscript,
  );
}

export function voiceOfflineResponsePacket(
  scenario: string,
  primaryText?: string | null,
  matchedText?: string | null,
  options?: string | null,
  minutes?: number | null,
  isReachable?: boolean | null,
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().voiceOfflineResponsePacket(
    scenario,
    primaryText,
    matchedText,
    options,
    minutes,
    isReachable,
  );
}

export function voiceCachedQueryResponsePacket(
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
): EmbeddedBridgePacketResponse {
  return getEmbeddedBridgePacketRuntime().voiceCachedQueryResponsePacket(
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
}
