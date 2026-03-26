const globalTarget = typeof globalThis !== 'undefined' ? globalThis : undefined;

function trimText(value) {
  return typeof value === 'string' ? value.trim() : '';
}

function normalizePayload(value) {
  return trimText(String(value ?? ''))
    .replace(/\n/g, ' ')
    .split(/\s+/)
    .filter(Boolean)
    .join(' ');
}

function normalizedOptionalTrimmed(value) {
  const trimmed = trimText(value ?? '');
  return trimmed.length > 0 ? trimmed : null;
}

function normalizeDomainHint(value) {
  return trimText(String(value ?? ''))
    .toLowerCase()
    .split(/\s+/)
    .filter(Boolean)
    .join(' ');
}

function normalizeSemanticLabel(value) {
  return trimText(String(value ?? ''))
    .toLowerCase()
    .split(/\s+/)
    .filter(Boolean)
    .join('_');
}

function normalizePairingTokenInput(value) {
  const normalized = String(value ?? '')
    .toUpperCase()
    .split('')
    .filter((character) => /[A-Z0-9]/.test(character))
    .slice(0, 6)
    .join('');

  if (normalized.length <= 3) {
    return normalized;
  }

  return `${normalized.slice(0, 3)}-${normalized.slice(3)}`;
}

function normalizePositiveMinutes(value) {
  if (value == null) {
    return null;
  }
  return Math.max(1, Number(value));
}

function parseTagsJson(tagsJson) {
  if (typeof tagsJson !== 'string' || tagsJson.trim().length === 0) {
    return [];
  }
  try {
    const parsed = JSON.parse(tagsJson);
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function normalizeTaskDisplay(tagsJson, project) {
  const normalizedProject = normalizedOptionalTrimmed(project);
  const projectKey = normalizedProject?.toLowerCase() ?? null;
  const normalizedTags = [];

  for (const rawTag of parseTagsJson(tagsJson)) {
    const trimmed = trimText(rawTag);
    if (trimmed.length === 0) {
      continue;
    }
    const normalized = trimmed.toLowerCase();
    if (projectKey != null && projectKey === normalized) {
      continue;
    }
    if (normalizedTags.some((existing) => existing.toLowerCase() === normalized)) {
      continue;
    }
    normalizedTags.push(trimmed);
  }

  return { tags: normalizedTags, project: normalizedProject };
}

function shortClientKindLabel(clientKind) {
  const trimmed = normalizedOptionalTrimmed(clientKind);
  if (trimmed == null) {
    return null;
  }

  const normalized = trimmed.toLowerCase();
  if (normalized.includes('web')) return 'Web';
  if (normalized.includes('mac')) return 'macOS';
  if (normalized.includes('ios') || normalized.includes('iphone') || normalized.includes('ipad')) return 'iOS';
  if (normalized.includes('watch')) return 'watchOS';
  if (normalized.includes('veld') || normalized.includes('daemon') || normalized.includes('server')) return 'Authority';
  return trimmed;
}

function actionItemDedupeKey(kind, title, summary, projectLabel, threadId, threadLabel) {
  return [
    normalizeSemanticLabel(kind),
    normalizePayload(title),
    normalizePayload(summary),
    normalizedOptionalTrimmed(projectLabel) ?? '',
    normalizedOptionalTrimmed(threadId) ?? '',
    normalizedOptionalTrimmed(threadLabel) ?? '',
  ].join('::');
}

function prepareVoiceQuickActionPacket(intentStorageToken, primaryText, targetId, minutes) {
  if (intentStorageToken === 'capture_create') {
    return { queueKind: 'capture.create', targetId: null, text: normalizePayload(primaryText), minutes: null, ready: true };
  }
  if (intentStorageToken === 'commitment_create') {
    return { queueKind: 'commitment.create', targetId: null, text: normalizePayload(primaryText), minutes: null, ready: true };
  }
  if (intentStorageToken === 'commitment_done') {
    return { queueKind: 'commitment.done', targetId: normalizedOptionalTrimmed(targetId), text: null, minutes: null, ready: true };
  }
  if (intentStorageToken === 'nudge_done') {
    return { queueKind: 'nudge.done', targetId: normalizedOptionalTrimmed(targetId), text: null, minutes: null, ready: true };
  }
  if (String(intentStorageToken ?? '').startsWith('nudge_snooze_')) {
    return {
      queueKind: 'nudge.snooze',
      targetId: normalizedOptionalTrimmed(targetId),
      text: null,
      minutes: normalizePositiveMinutes(minutes),
      ready: true,
    };
  }

  return { queueKind: 'capture.create', targetId: null, text: normalizePayload(primaryText), minutes: null, ready: false };
}

function prepareQueuedActionPacket(kind, targetId, text, minutes) {
  const ready = ['capture.create', 'commitment.create', 'commitment.done', 'nudge.done', 'nudge.snooze'].includes(kind);
  return {
    queueKind: ready ? kind : 'capture.create',
    targetId: normalizedOptionalTrimmed(targetId),
    text: normalizedOptionalTrimmed(text),
    minutes: normalizePositiveMinutes(minutes),
    ready,
  };
}

function prepareAssistantEntryFallbackPayload(text, requestedConversationId) {
  return [
    'queued_assistant_entry:',
    requestedConversationId ? `requested_conversation_id: ${requestedConversationId}` : '',
    '',
    trimText(text),
  ].filter((value) => value.trim().length > 0).join('\n');
}

function prepareCaptureMetadataPayload(text, captureType, sourceDevice) {
  const trimmedText = trimText(text);
  const trimmedCaptureType = trimText(captureType);
  const trimmedSourceDevice = trimText(sourceDevice);

  if (trimmedCaptureType === 'note' && trimmedSourceDevice === 'apple') {
    return trimmedText;
  }

  return [
    'queued_capture_metadata:',
    `requested_capture_type: ${trimmedCaptureType}`,
    `requested_source_device: ${trimmedSourceDevice}`,
    '',
    trimmedText,
  ].join('\n');
}

function prepareVoiceCapturePayload(transcript, intentStorageToken) {
  return [
    'voice_transcript:',
    trimText(transcript),
    '',
    `intent_candidate: ${normalizePayload(intentStorageToken)}`,
    'client_surface: ios_voice',
  ].join('\n');
}

function prepareLinkingFeedbackPacket(scenario, nodeDisplayName) {
  const displayName = nodeDisplayName ?? 'linked node';
  switch (scenario) {
    case 'issue_without_target':
      return { message: 'Pair nodes code created.' };
    case 'issue_with_target':
      return { message: `Pair nodes code created. ${nodeDisplayName ?? 'Remote client'} has been prompted to enter it on that client.` };
    case 'redeem_empty_token':
      return { message: 'Enter the pairing token shown on the issuing node.' };
    case 'redeem_success':
      return { message: `Linked as ${displayName}. The link has been saved locally and the issuing client has been notified.` };
    case 'renegotiate_success':
      return { message: `Pair nodes code created for ${displayName}. That client has been prompted to approve the new access.` };
    case 'unpair_success':
      return { message: `Unpaired ${displayName}.` };
    default:
      return { message: 'Linking status updated.' };
  }
}

function prepareAppShellFeedbackPacket(scenario, detail) {
  const trimmedDetail = normalizedOptionalTrimmed(detail);
  switch (scenario) {
    case 'offline_cache_in_use':
      return { message: trimmedDetail ? `Offline cache in use. ${trimmedDetail}` : 'Offline cache in use.' };
    case 'no_reachable_endpoint':
      return { message: 'No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url.' };
    case 'refresh_signals_failed':
      return { message: trimmedDetail ? `Could not refresh activity feed. ${trimmedDetail}` : 'Could not refresh activity feed.' };
    case 'queued_nudge_done':
      return { message: 'Queued nudge completion for sync.' };
    case 'queued_nudge_snooze':
      return { message: 'Queued nudge snooze for sync.' };
    case 'queued_commitment_done':
      return { message: 'Queued commitment completion for sync.' };
    case 'queued_commitment_create':
      return { message: 'Queued commitment for sync.' };
    case 'queued_capture_create':
      return { message: 'Queued capture for sync.' };
    case 'assistant_entry_queued':
      return { message: 'Assistant message queued for sync.' };
    default:
      return { message: trimmedDetail ?? 'Application state updated.' };
  }
}

function prepareVoiceContinuitySummaryPacket(draftExists, threadedTranscript, pendingRecoveryCount, isReachable, mergedTranscript) {
  const threaded = normalizedOptionalTrimmed(threadedTranscript);
  const merged = normalizedOptionalTrimmed(mergedTranscript);
  const pending = Number(pendingRecoveryCount ?? 0);

  if (draftExists === true) {
    return {
      headline: 'Voice draft ready to resume.',
      detail: 'Your latest local transcript is still on device and can be resumed without reopening a separate thread.',
      ready: true,
    };
  }
  if (threaded) {
    return { headline: 'Voice follow-up saved in Threads.', detail: threaded, ready: true };
  }
  if (pending > 0) {
    return {
      headline: 'Voice recovery pending.',
      detail: isReachable
        ? 'Local voice recovery is waiting on canonical replay.'
        : `Reconnect to merge ${pending} local voice entr${pending === 1 ? 'y' : 'ies'} back into canonical state.`,
      ready: true,
    };
  }
  if (merged) {
    return { headline: 'Local voice recovery merged.', detail: merged, ready: true };
  }
  return { headline: null, detail: null, ready: false };
}

function prepareVoiceOfflineResponsePacket(scenario, primaryText, matchedText, options, minutes, isReachable) {
  const normalizedPrimaryText = normalizedOptionalTrimmed(primaryText);
  const normalizedMatchedText = normalizedOptionalTrimmed(matchedText);
  const normalizedOptions = normalizedOptionalTrimmed(options);
  const normalizedMinutes = Math.max(1, Number(minutes ?? 10));
  const reachable = Boolean(isReachable);

  switch (trimText(scenario)) {
    case 'capture_shell':
      return {
        summary: reachable ? 'Saved voice capture.' : 'Voice capture queued for sync.',
        detail: normalizedPrimaryText,
        historyStatus: reachable ? 'submitted' : 'queued',
        errorPrefix: reachable ? '' : 'Voice transcript queued for sync.',
        ready: true,
      };
    case 'commitment_create_shell':
      return {
        summary: reachable ? 'Created commitment.' : 'Commitment queued for sync.',
        detail: normalizedPrimaryText,
        historyStatus: reachable ? 'submitted' : 'queued',
        errorPrefix: reachable ? '' : 'Commitment request queued for sync.',
        ready: true,
      };
    case 'backend_required_shell':
      return {
        summary: 'This voice action now requires the backend Apple route.',
        detail: 'Reconnect to Vel so the server can interpret and answer it.',
        historyStatus: 'backend_required',
        errorPrefix: 'Transcript capture was preserved, but the action needs the backend-owned Apple route.',
        ready: true,
      };
    case 'capture_offline':
      return {
        summary: 'Voice capture queued for sync.',
        detail: normalizedPrimaryText,
        historyStatus: 'queued',
        errorPrefix: 'Transcript capture queued for sync.',
        ready: true,
      };
    case 'commitment_target_missing':
      return {
        summary: 'Commitment target is missing.',
        detail: 'Try phrasing like "mark meds done."',
        historyStatus: 'needs_clarification',
        errorPrefix: 'Commitment target missing.',
        ready: true,
      };
    case 'commitment_no_match':
      return {
        summary: 'No open commitment matched.',
        detail: 'Transcript capture was queued for sync.',
        historyStatus: 'capture_only',
        errorPrefix: 'No local commitment match for offline queueing.',
        ready: true,
      };
    case 'commitment_ambiguous':
      return {
        summary: 'Ambiguous commitment target.',
        detail: normalizedOptions ? `Could match: ${normalizedOptions}` : null,
        historyStatus: 'needs_clarification',
        errorPrefix: 'Commitment target was ambiguous.',
        ready: true,
      };
    case 'commitment_done_queued':
      return {
        summary: 'Commitment completion queued.',
        detail: normalizedMatchedText,
        historyStatus: 'queued',
        errorPrefix: 'Commitment completion queued for backend replay.',
        ready: true,
      };
    case 'nudge_missing':
      return {
        summary: 'No active nudge found.',
        detail: 'Transcript capture was queued for sync.',
        historyStatus: 'capture_only',
        errorPrefix: 'No active nudge available for offline queueing.',
        ready: true,
      };
    case 'nudge_done_queued':
      return {
        summary: 'Top nudge resolution queued.',
        detail: null,
        historyStatus: 'queued',
        errorPrefix: 'Top nudge resolution queued for backend replay.',
        ready: true,
      };
    case 'nudge_snooze_queued':
      return {
        summary: 'Top nudge snooze queued.',
        detail: `${normalizedMinutes} minutes`,
        historyStatus: 'queued',
        errorPrefix: 'Top nudge snooze queued for backend replay.',
        ready: true,
      };
    case 'backend_required_offline':
      return {
        summary: 'Unavailable offline.',
        detail: 'This reply is backend-owned and is not synthesized from local Swift cache.',
        historyStatus: 'backend_required',
        errorPrefix: 'Transcript capture queued, but this voice reply requires the backend route.',
        ready: true,
      };
    default:
      return {
        summary: null,
        detail: null,
        historyStatus: 'capture_only',
        errorPrefix: '',
        ready: false,
      };
  }
}

function prepareVoiceCachedQueryResponsePacket(scenario, nextTitle, leaveBy, emptyMessage, cachedNowSummary, firstReason, nextCommitmentText, nextCommitmentDueAt, behaviorHeadline, behaviorReason) {
  const normalizedLeaveBy = normalizedOptionalTrimmed(leaveBy);
  const normalizedEmptyMessage = normalizedOptionalTrimmed(emptyMessage);
  const normalizedCachedNowSummary = normalizedOptionalTrimmed(cachedNowSummary);
  const normalizedFirstReason = normalizedOptionalTrimmed(firstReason);
  const normalizedNextCommitmentText = normalizedOptionalTrimmed(nextCommitmentText);
  const normalizedNextCommitmentDueAt = normalizedOptionalTrimmed(nextCommitmentDueAt);
  const normalizedBehaviorHeadline = normalizedOptionalTrimmed(behaviorHeadline);
  const normalizedBehaviorReason = normalizedOptionalTrimmed(behaviorReason);

  switch (trimText(scenario)) {
    case 'schedule_with_event':
      return {
        summary: normalizedOptionalTrimmed(nextTitle) ? `Next event: ${trimText(nextTitle)}.` : null,
        detail: normalizedLeaveBy ?? normalizedCachedNowSummary ?? normalizedEmptyMessage,
        ready: true,
      };
    case 'schedule_empty':
      return {
        summary: normalizedEmptyMessage ?? 'No upcoming schedule is cached.',
        detail: normalizedCachedNowSummary ?? normalizedFirstReason,
        ready: true,
      };
    case 'next_commitment':
      return {
        summary: normalizedNextCommitmentText ? `Next commitment: ${normalizedNextCommitmentText}.` : null,
        detail: normalizedNextCommitmentDueAt ?? normalizedCachedNowSummary,
        ready: true,
      };
    case 'next_commitment_empty':
      return {
        summary: 'No next commitment is cached.',
        detail: normalizedCachedNowSummary ?? normalizedEmptyMessage,
        ready: true,
      };
    case 'behavior_cached':
      return {
        summary: normalizedBehaviorHeadline,
        detail: normalizedBehaviorReason,
        ready: true,
      };
    case 'backend_unavailable':
      return {
        summary: 'Unavailable offline.',
        detail: 'Reconnect to fetch a backend-owned reply.',
        ready: true,
      };
    case 'cached_now_missing':
      return {
        summary: 'Unavailable offline.',
        detail: 'No cached backend /v1/now payload is available yet.',
        ready: true,
      };
    case 'behavior_missing':
      return {
        summary: 'Unavailable offline.',
        detail: 'No cached backend behavior summary is available yet.',
        ready: true,
      };
    default:
      return { summary: null, detail: null, ready: false };
  }
}

function collectRemoteRoutes(syncBaseUrl, tailscaleBaseUrl, lanBaseUrl, publicBaseUrl) {
  const seen = new Set();
  const routes = [];
  for (const [label, value] of [
    ['primary', syncBaseUrl],
    ['tailscale', tailscaleBaseUrl],
    ['lan', lanBaseUrl],
    ['public', publicBaseUrl],
  ]) {
    const trimmed = normalizedOptionalTrimmed(value);
    if (!trimmed || trimmed.includes('127.0.0.1') || trimmed.includes('localhost') || seen.has(trimmed)) {
      continue;
    }
    seen.add(trimmed);
    routes.push({ label, baseUrl: trimmed });
  }
  return routes;
}

export function velEmbeddedBrowserStatus() {
  return 'dev_fallback';
}

export function velEmbeddedNormalizePairingTokenPacket(input) {
  return JSON.stringify({ tokenCode: normalizePairingTokenInput(input) });
}

export function velEmbeddedNormalizeDomainHintPacket(input) {
  return JSON.stringify({ normalized: normalizeDomainHint(input) });
}

export function velEmbeddedNormalizeSemanticLabelPacket(input) {
  return JSON.stringify({ normalized: normalizeSemanticLabel(input) });
}

export function velEmbeddedNormalizeTaskDisplayPacket(tagsJson, project) {
  return JSON.stringify(normalizeTaskDisplay(tagsJson, project));
}

export function velEmbeddedNormalizeTaskDisplayBatchPacket(entriesJson) {
  let entries = [];
  try {
    entries = JSON.parse(entriesJson);
  } catch {
    entries = [];
  }
  return JSON.stringify({
    items: Array.isArray(entries)
      ? entries.map((entry) => normalizeTaskDisplay(JSON.stringify(entry?.tags ?? []), entry?.project ?? null))
      : [],
  });
}

export function velEmbeddedShortClientKindLabelPacket(clientKind) {
  return JSON.stringify({ shortLabel: shortClientKindLabel(clientKind) });
}

export function velEmbeddedActionItemDedupeKeyPacket(kind, title, summary, projectLabel, threadId, threadLabel) {
  return JSON.stringify({ key: actionItemDedupeKey(kind, title, summary, projectLabel, threadId, threadLabel) });
}

export function velEmbeddedActionItemDedupeBatchPacket(entriesJson) {
  let entries = [];
  try {
    entries = JSON.parse(entriesJson);
  } catch {
    entries = [];
  }
  return JSON.stringify({
    keys: Array.isArray(entries)
      ? entries.map((entry) => actionItemDedupeKey(
        entry?.kind ?? '',
        entry?.title ?? '',
        entry?.summary ?? '',
        entry?.projectLabel ?? null,
        entry?.threadId ?? null,
        entry?.threadLabel ?? null,
      ))
      : [],
  });
}

export function velEmbeddedQueuedActionPacket(kind, targetId, text, minutes) {
  return JSON.stringify(prepareQueuedActionPacket(trimText(kind), targetId, text, minutes));
}

export function velEmbeddedVoiceQuickActionPacket(intentStorageToken, primaryText, targetId, minutes) {
  return JSON.stringify(prepareVoiceQuickActionPacket(trimText(intentStorageToken), primaryText, targetId, minutes));
}

export function velEmbeddedAssistantEntryFallbackPacket(text, requestedConversationId) {
  return JSON.stringify({ payload: prepareAssistantEntryFallbackPayload(text, normalizedOptionalTrimmed(requestedConversationId)) });
}

export function velEmbeddedCaptureMetadataPacket(text, captureType, sourceDevice) {
  return JSON.stringify({ payload: prepareCaptureMetadataPayload(text, captureType, sourceDevice) });
}

export function velEmbeddedThreadDraftPacket(text, requestedConversationId) {
  return JSON.stringify({
    payload: normalizePayload(text),
    requestedConversationId: normalizedOptionalTrimmed(requestedConversationId),
  });
}

export function velEmbeddedVoiceCapturePacket(transcript, intentStorageToken) {
  return JSON.stringify({ payload: prepareVoiceCapturePayload(transcript, intentStorageToken) });
}

export function velEmbeddedLinkingRequestPacket(tokenCode, targetBaseUrl) {
  return JSON.stringify({
    tokenCode: normalizedOptionalTrimmed(tokenCode),
    targetBaseUrl: normalizedOptionalTrimmed(targetBaseUrl),
  });
}

export function velEmbeddedLinkingFeedbackPacket(scenario, nodeDisplayName) {
  return JSON.stringify(prepareLinkingFeedbackPacket(scenario, nodeDisplayName));
}

export function velEmbeddedAppShellFeedbackPacket(scenario, detail) {
  return JSON.stringify(prepareAppShellFeedbackPacket(scenario, detail));
}

export function velEmbeddedCollectRemoteRoutesPacket(syncBaseUrl, tailscaleBaseUrl, lanBaseUrl, publicBaseUrl) {
  return JSON.stringify(collectRemoteRoutes(syncBaseUrl, tailscaleBaseUrl, lanBaseUrl, publicBaseUrl));
}

export function velEmbeddedVoiceContinuitySummaryPacket(draftExists, threadedTranscript, pendingRecoveryCount, isReachable, mergedTranscript) {
  return JSON.stringify(prepareVoiceContinuitySummaryPacket(draftExists, threadedTranscript, pendingRecoveryCount, isReachable, mergedTranscript));
}

export function velEmbeddedVoiceOfflineResponsePacket(scenario, primaryText, matchedText, options, minutes, isReachable) {
  return JSON.stringify(prepareVoiceOfflineResponsePacket(scenario, primaryText, matchedText, options, minutes, isReachable));
}

export function velEmbeddedVoiceCachedQueryResponsePacket(scenario, nextTitle, leaveBy, emptyMessage, cachedNowSummary, firstReason, nextCommitmentText, nextCommitmentDueAt, behaviorHeadline, behaviorReason) {
  return JSON.stringify(
    prepareVoiceCachedQueryResponsePacket(
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
}

const exportedRuntime = {
  velEmbeddedBrowserStatus,
  velEmbeddedNormalizePairingTokenPacket,
  velEmbeddedNormalizeDomainHintPacket,
  velEmbeddedNormalizeSemanticLabelPacket,
  velEmbeddedNormalizeTaskDisplayPacket,
  velEmbeddedNormalizeTaskDisplayBatchPacket,
  velEmbeddedShortClientKindLabelPacket,
  velEmbeddedActionItemDedupeKeyPacket,
  velEmbeddedActionItemDedupeBatchPacket,
  velEmbeddedQueuedActionPacket,
  velEmbeddedVoiceQuickActionPacket,
  velEmbeddedAssistantEntryFallbackPacket,
  velEmbeddedCaptureMetadataPacket,
  velEmbeddedThreadDraftPacket,
  velEmbeddedVoiceCapturePacket,
  velEmbeddedLinkingRequestPacket,
  velEmbeddedLinkingFeedbackPacket,
  velEmbeddedAppShellFeedbackPacket,
  velEmbeddedCollectRemoteRoutesPacket,
  velEmbeddedVoiceContinuitySummaryPacket,
  velEmbeddedVoiceOfflineResponsePacket,
  velEmbeddedVoiceCachedQueryResponsePacket,
};

export default async function init() {
  if (globalTarget) {
    globalTarget.__VEL_EMBEDDED_BRIDGE_WASM__ = exportedRuntime;
  }
  return exportedRuntime;
}
