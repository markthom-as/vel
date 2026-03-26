import {
  getEmbeddedBridgePacketRuntime,
  installEmbeddedBridgePacketRuntime,
  type EmbeddedBridgePacketKind,
  type EmbeddedBridgePacketResponse,
  type EmbeddedBridgePacketRuntime,
} from './embeddedBridgePackets';

export interface EmbeddedBridgeWasmModule {
  velEmbeddedBrowserStatus?: () => string;
  velEmbeddedNormalizePairingTokenPacket(input: string): string;
  velEmbeddedNormalizeDomainHintPacket(input: string): string;
  velEmbeddedNormalizeSemanticLabelPacket(input: string): string;
  velEmbeddedNormalizeTaskDisplayPacket(tagsJson?: string | null, project?: string | null): string;
  velEmbeddedQueuedActionPacket(
    kind: string,
    targetId?: string | null,
    text?: string | null,
    minutes?: number | null,
  ): string;
  velEmbeddedVoiceQuickActionPacket(
    intentStorageToken: string,
    primaryText: string,
    targetId?: string | null,
    minutes?: number | null,
  ): string;
  velEmbeddedAssistantEntryFallbackPacket(
    text: string,
    requestedConversationId?: string | null,
  ): string;
  velEmbeddedCaptureMetadataPacket(
    text: string,
    captureType: string,
    sourceDevice: string,
  ): string;
  velEmbeddedThreadDraftPacket(
    text: string,
    requestedConversationId?: string | null,
  ): string;
  velEmbeddedVoiceCapturePacket(transcript: string, intentStorageToken: string): string;
  velEmbeddedLinkingRequestPacket(
    tokenCode?: string | null,
    targetBaseUrl?: string | null,
  ): string;
  velEmbeddedLinkingFeedbackPacket(
    scenario: string,
    nodeDisplayName?: string | null,
  ): string;
  velEmbeddedAppShellFeedbackPacket(
    scenario: string,
    detail?: string | null,
  ): string;
  velEmbeddedCollectRemoteRoutesPacket(
    syncBaseUrl?: string | null,
    tailscaleBaseUrl?: string | null,
    lanBaseUrl?: string | null,
    publicBaseUrl?: string | null,
  ): string;
  velEmbeddedVoiceContinuitySummaryPacket(
    draftExists?: boolean | null,
    threadedTranscript?: string | null,
    pendingRecoveryCount?: number | null,
    isReachable?: boolean | null,
    mergedTranscript?: string | null,
  ): string;
  velEmbeddedVoiceOfflineResponsePacket(
    scenario: string,
    primaryText?: string | null,
    matchedText?: string | null,
    options?: string | null,
    minutes?: number | null,
    isReachable?: boolean | null,
  ): string;
  velEmbeddedVoiceCachedQueryResponsePacket(
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
  ): string;
}

declare global {
  interface Window {
    __VEL_EMBEDDED_BRIDGE_WASM__?: EmbeddedBridgeWasmModule;
  }
}

function response(kind: EmbeddedBridgePacketKind, payloadJson: string): EmbeddedBridgePacketResponse {
  return { kind, payloadJson };
}

export function createEmbeddedBridgePacketRuntimeFromWasm(
  wasm: EmbeddedBridgeWasmModule,
): EmbeddedBridgePacketRuntime {
  return {
    normalizePairingTokenPacket(input) {
      return response('linking_settings_normalization', wasm.velEmbeddedNormalizePairingTokenPacket(input));
    },
    normalizeDomainHintPacket(input) {
      return response('deterministic_domain_helpers', wasm.velEmbeddedNormalizeDomainHintPacket(input));
    },
    normalizeSemanticLabelPacket(input) {
      return response('deterministic_domain_helpers', wasm.velEmbeddedNormalizeSemanticLabelPacket(input));
    },
    normalizeTaskDisplayPacket(tags, project) {
      return response(
        'deterministic_domain_helpers',
        wasm.velEmbeddedNormalizeTaskDisplayPacket(
          tags == null ? null : JSON.stringify(tags),
          project,
        ),
      );
    },
    queuedActionPacket(kind, targetId, text, minutes) {
      return response('queued_action_packaging', wasm.velEmbeddedQueuedActionPacket(kind, targetId, text, minutes));
    },
    voiceQuickActionPacket(intentStorageToken, primaryText, targetId, minutes) {
      return response(
        'voice_quick_action_packaging',
        wasm.velEmbeddedVoiceQuickActionPacket(intentStorageToken, primaryText, targetId, minutes),
      );
    },
    assistantEntryFallbackPacket(text, requestedConversationId) {
      return response(
        'assistant_entry_fallback_packaging',
        wasm.velEmbeddedAssistantEntryFallbackPacket(text, requestedConversationId),
      );
    },
    captureMetadataPacket(text, captureType, sourceDevice) {
      return response(
        'capture_metadata_packaging',
        wasm.velEmbeddedCaptureMetadataPacket(text, captureType, sourceDevice),
      );
    },
    threadDraftPacket(text, requestedConversationId) {
      return response('thread_draft_packaging', wasm.velEmbeddedThreadDraftPacket(text, requestedConversationId));
    },
    voiceCapturePacket(transcript, intentStorageToken) {
      return response(
        'voice_capture_packaging',
        wasm.velEmbeddedVoiceCapturePacket(transcript, intentStorageToken),
      );
    },
    linkingRequestPacket(tokenCode, targetBaseUrl) {
      return response(
        'linking_request_packaging',
        wasm.velEmbeddedLinkingRequestPacket(tokenCode, targetBaseUrl),
      );
    },
    linkingFeedbackPacket(scenario, nodeDisplayName) {
      return response(
        'linking_feedback_packaging',
        wasm.velEmbeddedLinkingFeedbackPacket(scenario, nodeDisplayName),
      );
    },
    appShellFeedbackPacket(scenario, detail) {
      return response(
        'app_shell_feedback_packaging',
        wasm.velEmbeddedAppShellFeedbackPacket(scenario, detail),
      );
    },
    collectRemoteRoutesPacket(syncBaseUrl, tailscaleBaseUrl, lanBaseUrl, publicBaseUrl) {
      return response(
        'linking_settings_normalization',
        wasm.velEmbeddedCollectRemoteRoutesPacket(
          syncBaseUrl,
          tailscaleBaseUrl,
          lanBaseUrl,
          publicBaseUrl,
        ),
      );
    },
    voiceContinuitySummaryPacket(
      draftExists,
      threadedTranscript,
      pendingRecoveryCount,
      isReachable,
      mergedTranscript,
    ) {
      return response(
        'voice_continuity_summary_packaging',
        wasm.velEmbeddedVoiceContinuitySummaryPacket(
          draftExists,
          threadedTranscript,
          pendingRecoveryCount,
          isReachable,
          mergedTranscript,
        ),
      );
    },
    voiceOfflineResponsePacket(scenario, primaryText, matchedText, options, minutes, isReachable) {
      return response(
        'voice_offline_response_packaging',
        wasm.velEmbeddedVoiceOfflineResponsePacket(
          scenario,
          primaryText,
          matchedText,
          options,
          minutes,
          isReachable,
        ),
      );
    },
    voiceCachedQueryResponsePacket(
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
    ) {
      return response(
        'voice_cached_query_packaging',
        wasm.velEmbeddedVoiceCachedQueryResponsePacket(
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
        ),
      );
    },
  };
}

export function installEmbeddedBridgePacketRuntimeFromWasm(
  wasm: EmbeddedBridgeWasmModule,
): EmbeddedBridgePacketRuntime {
  const runtime = createEmbeddedBridgePacketRuntimeFromWasm(wasm);
  installEmbeddedBridgePacketRuntime(runtime);
  return runtime;
}

export function maybeInstallEmbeddedBridgePacketRuntimeFromGlobal(): EmbeddedBridgePacketRuntime | null {
  try {
    return getEmbeddedBridgePacketRuntime();
  } catch {
    const wasm =
      typeof window !== 'undefined' ? window.__VEL_EMBEDDED_BRIDGE_WASM__ ?? null : null;
    if (wasm == null) {
      return null;
    }
    return installEmbeddedBridgePacketRuntimeFromWasm(wasm);
  }
}

export async function bootstrapEmbeddedBridgePacketRuntime(): Promise<EmbeddedBridgePacketRuntime | null> {
  const existing = maybeInstallEmbeddedBridgePacketRuntimeFromGlobal();
  if (existing) {
    return existing;
  }

  const wasmModuleUrl = (import.meta.env.VITE_VEL_EMBEDDED_BRIDGE_WASM_URL ?? '').trim();
  if (wasmModuleUrl.length === 0) {
    return null;
  }

  const browserModuleUrl =
    typeof window === 'undefined'
      ? wasmModuleUrl
      : new URL(wasmModuleUrl, window.location.origin).href;

  const imported = await import(/* @vite-ignore */ browserModuleUrl);
  if (typeof imported.default === 'function') {
    await imported.default();
  }

  const candidate = imported as Partial<EmbeddedBridgeWasmModule>;
  if (typeof candidate.velEmbeddedNormalizePairingTokenPacket !== 'function') {
    throw new Error(
      `Embedded bridge WASM module at ${browserModuleUrl} did not expose the expected packet runtime exports.`,
    );
  }

  const wasm = candidate as EmbeddedBridgeWasmModule;
  if (typeof window !== 'undefined') {
    window.__VEL_EMBEDDED_BRIDGE_WASM__ = wasm;
  }
  return installEmbeddedBridgePacketRuntimeFromWasm(wasm);
}
