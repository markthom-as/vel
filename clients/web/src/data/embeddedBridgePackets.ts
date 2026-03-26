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

function trimText(value: string): string {
  return value.trim();
}

function normalizePayload(value: string): string {
  return value.trim().replaceAll('\n', ' ').split(/\s+/).filter(Boolean).join(' ');
}

function normalizeOptionalTrimmed(value: string | null | undefined): string | null {
  if (value == null) return null;
  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
}

function normalizePositiveMinutes(value: number | null | undefined): number | null {
  if (value == null) return null;
  return Math.max(1, Math.trunc(value));
}

export function normalizePairingTokenPacket(input: string): EmbeddedBridgePacketResponse {
  const normalized = input
    .toUpperCase()
    .split('')
    .filter((character) => /[A-Z0-9]/.test(character))
    .slice(0, 6)
    .join('');

  const tokenCode =
    normalized.length <= 3 ? normalized : `${normalized.slice(0, 3)}-${normalized.slice(3)}`;

  return {
    kind: 'linking_settings_normalization',
    payloadJson: JSON.stringify({ tokenCode }),
  };
}

export function normalizeDomainHintPacket(input: string): EmbeddedBridgePacketResponse {
  const normalized = input.trim().toLowerCase().split(/\s+/).filter(Boolean).join(' ');
  return {
    kind: 'deterministic_domain_helpers',
    payloadJson: JSON.stringify({ normalized }),
  };
}

export function queuedActionPacket(
  kind: string,
  targetId?: string | null,
  text?: string | null,
  minutes?: number | null,
): EmbeddedBridgePacketResponse {
  const cleanKind = trimText(kind) || 'capture.create';
  const cleanTargetId = normalizeOptionalTrimmed(targetId);
  const cleanText = normalizeOptionalTrimmed(text);
  const cleanMinutes = normalizePositiveMinutes(minutes);
  const ready = [
    'capture.create',
    'commitment.create',
    'commitment.done',
    'nudge.done',
    'nudge.snooze',
  ].includes(cleanKind);

  return {
    kind: 'queued_action_packaging',
    payloadJson: JSON.stringify({
      queueKind: ready ? cleanKind : 'capture.create',
      targetId: cleanTargetId,
      text: cleanText,
      minutes: cleanMinutes,
      ready,
    }),
  };
}

export function voiceQuickActionPacket(
  intentStorageToken: string,
  primaryText: string,
  targetId?: string | null,
  minutes?: number | null,
): EmbeddedBridgePacketResponse {
  const cleanTargetId = normalizeOptionalTrimmed(targetId);
  const cleanMinutes = normalizePositiveMinutes(minutes);
  let payload: Record<string, unknown>;

  if (intentStorageToken === 'capture_create') {
    payload = {
      queueKind: 'capture.create',
      targetId: null,
      text: normalizePayload(primaryText),
      minutes: null,
      ready: true,
    };
  } else if (intentStorageToken === 'commitment_create') {
    payload = {
      queueKind: 'commitment.create',
      targetId: null,
      text: normalizePayload(primaryText),
      minutes: null,
      ready: true,
    };
  } else if (intentStorageToken === 'commitment_done') {
    payload = {
      queueKind: 'commitment.done',
      targetId: cleanTargetId,
      text: null,
      minutes: null,
      ready: true,
    };
  } else if (intentStorageToken === 'nudge_done') {
    payload = {
      queueKind: 'nudge.done',
      targetId: cleanTargetId,
      text: null,
      minutes: null,
      ready: true,
    };
  } else if (intentStorageToken.startsWith('nudge_snooze_')) {
    payload = {
      queueKind: 'nudge.snooze',
      targetId: cleanTargetId,
      text: null,
      minutes: cleanMinutes,
      ready: true,
    };
  } else {
    payload = {
      queueKind: 'capture.create',
      targetId: null,
      text: normalizePayload(primaryText),
      minutes: null,
      ready: false,
    };
  }

  return {
    kind: 'voice_quick_action_packaging',
    payloadJson: JSON.stringify(payload),
  };
}

export function assistantEntryFallbackPacket(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgePacketResponse {
  const cleanConversationId = normalizeOptionalTrimmed(requestedConversationId);
  const payload = [
    'queued_assistant_entry:',
    cleanConversationId ? `requested_conversation_id: ${cleanConversationId}` : '',
    '',
    trimText(text),
  ]
    .filter((value) => value.trim().length > 0)
    .join('\n');

  return {
    kind: 'assistant_entry_fallback_packaging',
    payloadJson: JSON.stringify({ payload }),
  };
}

export function captureMetadataPacket(
  text: string,
  captureType: string,
  sourceDevice: string,
): EmbeddedBridgePacketResponse {
  const cleanType = trimText(captureType);
  const cleanSource = trimText(sourceDevice);
  const payload =
    cleanType === 'note' && cleanSource === 'apple'
      ? trimText(text)
      : [
          'queued_capture_metadata:',
          `requested_capture_type: ${cleanType}`,
          `requested_source_device: ${cleanSource}`,
          '',
          trimText(text),
        ].join('\n');

  return {
    kind: 'capture_metadata_packaging',
    payloadJson: JSON.stringify({ payload }),
  };
}

export function threadDraftPacket(
  text: string,
  requestedConversationId?: string | null,
): EmbeddedBridgePacketResponse {
  return {
    kind: 'thread_draft_packaging',
    payloadJson: JSON.stringify({
      payload: normalizePayload(text),
      requestedConversationId: normalizeOptionalTrimmed(requestedConversationId),
    }),
  };
}

export function voiceCapturePacket(
  transcript: string,
  intentStorageToken: string,
): EmbeddedBridgePacketResponse {
  const payload = [
    'voice_transcript:',
    trimText(transcript),
    '',
    `intent_candidate: ${normalizePayload(intentStorageToken)}`,
    'client_surface: ios_voice',
  ].join('\n');

  return {
    kind: 'voice_capture_packaging',
    payloadJson: JSON.stringify({ payload }),
  };
}

export function linkingRequestPacket(
  tokenCode?: string | null,
  targetBaseUrl?: string | null,
): EmbeddedBridgePacketResponse {
  return {
    kind: 'linking_request_packaging',
    payloadJson: JSON.stringify({
      tokenCode: normalizeOptionalTrimmed(tokenCode),
      targetBaseUrl: normalizeOptionalTrimmed(targetBaseUrl),
    }),
  };
}

export function linkingFeedbackPacket(
  scenario: string,
  nodeDisplayName?: string | null,
): EmbeddedBridgePacketResponse {
  let message: string | null = null;

  switch (scenario) {
    case 'issue_without_target':
      message = 'Pair nodes code created.';
      break;
    case 'issue_with_target':
      message = `Pair nodes code created. ${nodeDisplayName?.trim() || 'Remote client'} has been prompted to enter it on that client.`;
      break;
    case 'redeem_empty_token':
      message = 'Enter the pairing token shown on the issuing node.';
      break;
    case 'redeem_success':
      message = `Linked as ${nodeDisplayName?.trim() || 'linked node'}. The link has been saved locally and the issuing client has been notified.`;
      break;
    case 'renegotiate_success':
      message = `Pair nodes code created for ${nodeDisplayName?.trim() || 'linked node'}. That client has been prompted to approve the new access.`;
      break;
    case 'unpair_success':
      message = `Unpaired ${nodeDisplayName?.trim() || 'linked node'}.`;
      break;
  }

  return {
    kind: 'linking_feedback_packaging',
    payloadJson: JSON.stringify({ message }),
  };
}

export function appShellFeedbackPacket(
  scenario: string,
  detail?: string | null,
): EmbeddedBridgePacketResponse {
  const cleanDetail = normalizeOptionalTrimmed(detail);
  let message: string | null = null;

  switch (scenario) {
    case 'offline_cache_in_use':
      message = cleanDetail ? `Offline cache in use. ${cleanDetail}` : 'Offline cache in use.';
      break;
    case 'no_reachable_endpoint':
      message = 'No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url.';
      break;
    case 'refresh_signals_failed':
      message = cleanDetail
        ? `Could not refresh activity feed. ${cleanDetail}`
        : 'Could not refresh activity feed.';
      break;
    case 'queued_nudge_done':
      message = 'Queued nudge completion for sync.';
      break;
    case 'queued_nudge_snooze':
      message = 'Queued nudge snooze for sync.';
      break;
    case 'queued_commitment_done':
      message = 'Queued commitment completion for sync.';
      break;
    case 'queued_commitment_create':
      message = 'Queued commitment for sync.';
      break;
    case 'queued_capture_create':
      message = 'Queued capture for sync.';
      break;
    case 'assistant_entry_queued':
      message = 'Assistant message queued for sync.';
      break;
  }

  return {
    kind: 'app_shell_feedback_packaging',
    payloadJson: JSON.stringify({ message }),
  };
}

export function collectRemoteRoutesPacket(
  syncBaseUrl?: string | null,
  tailscaleBaseUrl?: string | null,
  lanBaseUrl?: string | null,
  publicBaseUrl?: string | null,
): EmbeddedBridgePacketResponse {
  const entries: Array<[string, string | null | undefined]> = [
    ['primary', syncBaseUrl],
    ['tailscale', tailscaleBaseUrl],
    ['lan', lanBaseUrl],
    ['public', publicBaseUrl],
  ];
  const seen = new Set<string>();
  const routes = entries.flatMap(([label, value]) => {
    const trimmed = normalizeOptionalTrimmed(value);
    if (
      !trimmed ||
      trimmed.includes('127.0.0.1') ||
      trimmed.includes('localhost') ||
      seen.has(trimmed)
    ) {
      return [];
    }
    seen.add(trimmed);
    return [{ label, baseUrl: trimmed }];
  });

  return {
    kind: 'linking_settings_normalization',
    payloadJson: JSON.stringify(routes),
  };
}

export function voiceContinuitySummaryPacket(
  draftExists?: boolean | null,
  threadedTranscript?: string | null,
  pendingRecoveryCount?: number | null,
  isReachable?: boolean | null,
  mergedTranscript?: string | null,
): EmbeddedBridgePacketResponse {
  let payload: Record<string, unknown>;

  if (draftExists) {
    payload = {
      headline: 'Voice draft ready to resume.',
      detail:
        'Your latest local transcript is still on device and can be resumed without reopening a separate thread.',
      ready: true,
    };
  } else if (normalizeOptionalTrimmed(threadedTranscript)) {
    payload = {
      headline: 'Voice follow-up saved in Threads.',
      detail: normalizeOptionalTrimmed(threadedTranscript),
      ready: true,
    };
  } else if ((pendingRecoveryCount ?? 0) > 0) {
    const count = Math.max(0, Math.trunc(pendingRecoveryCount ?? 0));
    payload = {
      headline: 'Voice recovery pending.',
      detail:
        isReachable
          ? 'Local voice recovery is waiting on canonical replay.'
          : `Reconnect to merge ${count} local voice entr${count === 1 ? 'y' : 'ies'} back into canonical state.`,
      ready: true,
    };
  } else if (normalizeOptionalTrimmed(mergedTranscript)) {
    payload = {
      headline: 'Local voice recovery merged.',
      detail: normalizeOptionalTrimmed(mergedTranscript),
      ready: true,
    };
  } else {
    payload = {
      headline: null,
      detail: null,
      ready: false,
    };
  }

  return {
    kind: 'voice_continuity_summary_packaging',
    payloadJson: JSON.stringify(payload),
  };
}

export function voiceOfflineResponsePacket(
  scenario: string,
  primaryText?: string | null,
  matchedText?: string | null,
  options?: string | null,
  minutes?: number | null,
  isReachable?: boolean | null,
): EmbeddedBridgePacketResponse {
  const cleanPrimaryText = normalizeOptionalTrimmed(primaryText);
  const cleanMatchedText = normalizeOptionalTrimmed(matchedText);
  const cleanOptions = normalizeOptionalTrimmed(options);
  const cleanMinutes = normalizePositiveMinutes(minutes) ?? 10;
  let payload: Record<string, unknown>;

  switch (trimText(scenario)) {
    case 'capture_shell':
      payload = {
        summary: isReachable ? 'Saved voice capture.' : 'Voice capture queued for sync.',
        detail: cleanPrimaryText,
        historyStatus: isReachable ? 'submitted' : 'queued',
        errorPrefix: isReachable ? '' : 'Voice transcript queued for sync.',
        ready: true,
      };
      break;
    case 'commitment_create_shell':
      payload = {
        summary: isReachable ? 'Created commitment.' : 'Commitment queued for sync.',
        detail: cleanPrimaryText,
        historyStatus: isReachable ? 'submitted' : 'queued',
        errorPrefix: isReachable ? '' : 'Commitment request queued for sync.',
        ready: true,
      };
      break;
    case 'backend_required_shell':
      payload = {
        summary: 'This voice action now requires the backend Apple route.',
        detail: 'Reconnect to Vel so the server can interpret and answer it.',
        historyStatus: 'backend_required',
        errorPrefix:
          'Transcript capture was preserved, but the action needs the backend-owned Apple route.',
        ready: true,
      };
      break;
    case 'capture_offline':
      payload = {
        summary: 'Voice capture queued for sync.',
        detail: cleanPrimaryText,
        historyStatus: 'queued',
        errorPrefix: 'Transcript capture queued for sync.',
        ready: true,
      };
      break;
    case 'commitment_target_missing':
      payload = {
        summary: 'Commitment target is missing.',
        detail: 'Try phrasing like "mark meds done."',
        historyStatus: 'needs_clarification',
        errorPrefix: 'Commitment target missing.',
        ready: true,
      };
      break;
    case 'commitment_no_match':
      payload = {
        summary: 'No open commitment matched.',
        detail: 'Transcript capture was queued for sync.',
        historyStatus: 'capture_only',
        errorPrefix: 'No local commitment match for offline queueing.',
        ready: true,
      };
      break;
    case 'commitment_ambiguous':
      payload = {
        summary: 'Ambiguous commitment target.',
        detail: cleanOptions ? `Could match: ${cleanOptions}` : null,
        historyStatus: 'needs_clarification',
        errorPrefix: 'Commitment target was ambiguous.',
        ready: true,
      };
      break;
    case 'commitment_done_queued':
      payload = {
        summary: 'Commitment completion queued.',
        detail: cleanMatchedText,
        historyStatus: 'queued',
        errorPrefix: 'Commitment completion queued for backend replay.',
        ready: true,
      };
      break;
    case 'nudge_missing':
      payload = {
        summary: 'No active nudge found.',
        detail: 'Transcript capture was queued for sync.',
        historyStatus: 'capture_only',
        errorPrefix: 'No active nudge available for offline queueing.',
        ready: true,
      };
      break;
    case 'nudge_done_queued':
      payload = {
        summary: 'Top nudge resolution queued.',
        detail: null,
        historyStatus: 'queued',
        errorPrefix: 'Top nudge resolution queued for backend replay.',
        ready: true,
      };
      break;
    case 'nudge_snooze_queued':
      payload = {
        summary: 'Top nudge snooze queued.',
        detail: `${cleanMinutes} minutes`,
        historyStatus: 'queued',
        errorPrefix: 'Top nudge snooze queued for backend replay.',
        ready: true,
      };
      break;
    case 'backend_required_offline':
      payload = {
        summary: 'Unavailable offline.',
        detail: 'This reply is backend-owned and is not synthesized from local Swift cache.',
        historyStatus: 'backend_required',
        errorPrefix:
          'Transcript capture queued, but this voice reply requires the backend route.',
        ready: true,
      };
      break;
    default:
      payload = {
        summary: null,
        detail: null,
        historyStatus: 'capture_only',
        errorPrefix: '',
        ready: false,
      };
      break;
  }

  return {
    kind: 'voice_offline_response_packaging',
    payloadJson: JSON.stringify(payload),
  };
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
  const cleanNextTitle = normalizeOptionalTrimmed(nextTitle);
  const cleanLeaveBy = normalizeOptionalTrimmed(leaveBy);
  const cleanEmptyMessage = normalizeOptionalTrimmed(emptyMessage);
  const cleanCachedNowSummary = normalizeOptionalTrimmed(cachedNowSummary);
  const cleanFirstReason = normalizeOptionalTrimmed(firstReason);
  const cleanNextCommitmentText = normalizeOptionalTrimmed(nextCommitmentText);
  const cleanNextCommitmentDueAt = normalizeOptionalTrimmed(nextCommitmentDueAt);
  const cleanBehaviorHeadline = normalizeOptionalTrimmed(behaviorHeadline);
  const cleanBehaviorReason = normalizeOptionalTrimmed(behaviorReason);
  let payload: Record<string, unknown>;

  switch (trimText(scenario)) {
    case 'schedule_with_event':
      payload = {
        summary: cleanNextTitle ? `Next event: ${cleanNextTitle}.` : null,
        detail: cleanLeaveBy ?? cleanCachedNowSummary ?? cleanEmptyMessage,
        ready: true,
      };
      break;
    case 'schedule_empty':
      payload = {
        summary: cleanEmptyMessage ?? 'No upcoming schedule is cached.',
        detail: cleanCachedNowSummary ?? cleanFirstReason,
        ready: true,
      };
      break;
    case 'next_commitment':
      payload = {
        summary: cleanNextCommitmentText ? `Next commitment: ${cleanNextCommitmentText}.` : null,
        detail: cleanNextCommitmentDueAt ?? cleanCachedNowSummary,
        ready: true,
      };
      break;
    case 'next_commitment_empty':
      payload = {
        summary: 'No next commitment is cached.',
        detail: cleanCachedNowSummary ?? cleanEmptyMessage,
        ready: true,
      };
      break;
    case 'behavior_cached':
      payload = {
        summary: cleanBehaviorHeadline,
        detail: cleanBehaviorReason,
        ready: true,
      };
      break;
    case 'backend_unavailable':
      payload = {
        summary: 'Unavailable offline.',
        detail: 'Reconnect to fetch a backend-owned reply.',
        ready: true,
      };
      break;
    case 'cached_now_missing':
      payload = {
        summary: 'Unavailable offline.',
        detail: 'No cached backend /v1/now payload is available yet.',
        ready: true,
      };
      break;
    case 'behavior_missing':
      payload = {
        summary: 'Unavailable offline.',
        detail: 'No cached backend behavior summary is available yet.',
        ready: true,
      };
      break;
    default:
      payload = {
        summary: null,
        detail: null,
        ready: false,
      };
      break;
  }

  return {
    kind: 'voice_cached_query_packaging',
    payloadJson: JSON.stringify(payload),
  };
}
