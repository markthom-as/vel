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
  | 'capture_metadata_packaging';

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
