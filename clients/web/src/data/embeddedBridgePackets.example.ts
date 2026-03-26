import {
  appShellFeedbackPacket,
  assistantEntryFallbackPacket,
  collectRemoteRoutesPacket,
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

export function buildEmbeddedBridgePacketExamples() {
  return {
    token: normalizePairingTokenPacket(' abc 123 '),
    domain: normalizeDomainHintPacket('  Focus   Rituals  '),
    queuedAction: queuedActionPacket('commitment.done', 'cmt_123'),
    threadDraft: threadDraftPacket('Follow up with Dr. Lee tomorrow', 'conv_42'),
    voiceQuickAction: voiceQuickActionPacket('nudge_snooze_15', '', 'nudge_7', 15),
    voiceContinuitySummary: voiceContinuitySummaryPacket(false, null, 2, false, null),
    voiceOfflineResponse: voiceOfflineResponsePacket(
      'commitment_done_queued',
      null,
      'Morning meds',
    ),
    voiceCachedQuery: voiceCachedQueryResponsePacket(
      'schedule_with_event',
      'Therapy',
      'Leave by 2:10 PM',
    ),
    assistantFallback: assistantEntryFallbackPacket('Please summarize my priorities.', 'conv_42'),
    linkingRequest: linkingRequestPacket('ABC-123', 'https://vel.example'),
    linkingFeedback: linkingFeedbackPacket('redeem_success', 'Desk Mac'),
    shellFeedback: appShellFeedbackPacket('offline_cache_in_use', 'Showing cached orientation.'),
    remoteRoutes: collectRemoteRoutesPacket(
      'https://vel-primary.example',
      'https://vel-tailnet.example',
    ),
  };
}
