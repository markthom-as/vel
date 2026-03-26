import Foundation
import VelDomain
import VelFeatureFlags
#if canImport(Darwin)
import Darwin
#endif

public enum EmbeddedAppleFlow: String, Sendable, CaseIterable {
    case cachedNowHydration = "cached_now_hydration"
    case localQuickActionPreparation = "local_quick_action_preparation"
    case offlineRequestPackaging = "offline_request_packaging"
    case deterministicDomainHelpers = "deterministic_domain_helpers"
    case localThreadDraftPackaging = "local_thread_draft_packaging"
    case localVoiceCapturePackaging = "local_voice_capture_packaging"
    case localVoiceQuickActionPackaging = "local_voice_quick_action_packaging"
    case localVoiceContinuityPackaging = "local_voice_continuity_packaging"
    case localQueuedActionPackaging = "local_queued_action_packaging"
    case localLinkingSettingsNormalization = "local_linking_settings_normalization"
    case localAssistantEntryFallbackPackaging = "local_assistant_entry_fallback_packaging"
    case localLinkingRequestPackaging = "local_linking_request_packaging"
    case localCaptureMetadataPackaging = "local_capture_metadata_packaging"
    case localVoiceContinuitySummaryPackaging = "local_voice_continuity_summary_packaging"
    case localVoiceOfflineResponsePackaging = "local_voice_offline_response_packaging"
}

public struct EmbeddedBridgeRuntimeStatus: Sendable {
    public let resolvedSource: String?
    public let attemptedPaths: [String]
    public let freeBufferAvailable: Bool
    public let cachedNowHydrationSymbolAvailable: Bool
    public let localQuickActionPreparationSymbolAvailable: Bool
    public let offlineRequestPackagingSymbolAvailable: Bool
    public let deterministicDomainHelpersSymbolAvailable: Bool
    public let localThreadDraftPackagingSymbolAvailable: Bool
    public let localVoiceCapturePackagingSymbolAvailable: Bool
    public let localVoiceQuickActionPackagingSymbolAvailable: Bool
    public let localVoiceContinuityPackagingSymbolAvailable: Bool
    public let localQueuedActionPackagingSymbolAvailable: Bool
    public let localLinkingSettingsNormalizationSymbolAvailable: Bool
    public let localAssistantEntryFallbackPackagingSymbolAvailable: Bool
    public let localLinkingRequestPackagingSymbolAvailable: Bool
    public let localCaptureMetadataPackagingSymbolAvailable: Bool
    public let localVoiceContinuitySummaryPackagingSymbolAvailable: Bool
    public let localVoiceOfflineResponsePackagingSymbolAvailable: Bool

    public init(
        resolvedSource: String?,
        attemptedPaths: [String],
        freeBufferAvailable: Bool,
        cachedNowHydrationSymbolAvailable: Bool,
        localQuickActionPreparationSymbolAvailable: Bool,
        offlineRequestPackagingSymbolAvailable: Bool,
        deterministicDomainHelpersSymbolAvailable: Bool,
        localThreadDraftPackagingSymbolAvailable: Bool,
        localVoiceCapturePackagingSymbolAvailable: Bool,
        localVoiceQuickActionPackagingSymbolAvailable: Bool,
        localVoiceContinuityPackagingSymbolAvailable: Bool,
        localQueuedActionPackagingSymbolAvailable: Bool,
        localLinkingSettingsNormalizationSymbolAvailable: Bool,
        localAssistantEntryFallbackPackagingSymbolAvailable: Bool,
        localLinkingRequestPackagingSymbolAvailable: Bool,
        localCaptureMetadataPackagingSymbolAvailable: Bool,
        localVoiceContinuitySummaryPackagingSymbolAvailable: Bool,
        localVoiceOfflineResponsePackagingSymbolAvailable: Bool
    ) {
        self.resolvedSource = resolvedSource
        self.attemptedPaths = attemptedPaths
        self.freeBufferAvailable = freeBufferAvailable
        self.cachedNowHydrationSymbolAvailable = cachedNowHydrationSymbolAvailable
        self.localQuickActionPreparationSymbolAvailable = localQuickActionPreparationSymbolAvailable
        self.offlineRequestPackagingSymbolAvailable = offlineRequestPackagingSymbolAvailable
        self.deterministicDomainHelpersSymbolAvailable = deterministicDomainHelpersSymbolAvailable
        self.localThreadDraftPackagingSymbolAvailable = localThreadDraftPackagingSymbolAvailable
        self.localVoiceCapturePackagingSymbolAvailable = localVoiceCapturePackagingSymbolAvailable
        self.localVoiceQuickActionPackagingSymbolAvailable = localVoiceQuickActionPackagingSymbolAvailable
        self.localVoiceContinuityPackagingSymbolAvailable = localVoiceContinuityPackagingSymbolAvailable
        self.localQueuedActionPackagingSymbolAvailable = localQueuedActionPackagingSymbolAvailable
        self.localLinkingSettingsNormalizationSymbolAvailable = localLinkingSettingsNormalizationSymbolAvailable
        self.localAssistantEntryFallbackPackagingSymbolAvailable = localAssistantEntryFallbackPackagingSymbolAvailable
        self.localLinkingRequestPackagingSymbolAvailable = localLinkingRequestPackagingSymbolAvailable
        self.localCaptureMetadataPackagingSymbolAvailable = localCaptureMetadataPackagingSymbolAvailable
        self.localVoiceContinuitySummaryPackagingSymbolAvailable = localVoiceContinuitySummaryPackagingSymbolAvailable
        self.localVoiceOfflineResponsePackagingSymbolAvailable = localVoiceOfflineResponsePackagingSymbolAvailable
    }

    public static let unavailable = EmbeddedBridgeRuntimeStatus(
        resolvedSource: nil,
        attemptedPaths: [],
        freeBufferAvailable: false,
        cachedNowHydrationSymbolAvailable: false,
        localQuickActionPreparationSymbolAvailable: false,
        offlineRequestPackagingSymbolAvailable: false,
        deterministicDomainHelpersSymbolAvailable: false,
        localThreadDraftPackagingSymbolAvailable: false,
        localVoiceCapturePackagingSymbolAvailable: false,
        localVoiceQuickActionPackagingSymbolAvailable: false,
        localVoiceContinuityPackagingSymbolAvailable: false,
        localQueuedActionPackagingSymbolAvailable: false,
        localLinkingSettingsNormalizationSymbolAvailable: false,
        localAssistantEntryFallbackPackagingSymbolAvailable: false,
        localLinkingRequestPackagingSymbolAvailable: false,
        localCaptureMetadataPackagingSymbolAvailable: false,
        localVoiceContinuitySummaryPackagingSymbolAvailable: false,
        localVoiceOfflineResponsePackagingSymbolAvailable: false
    )

    public var isBridgeLoaded: Bool {
        resolvedSource != nil && freeBufferAvailable
    }

    public var hasUsableSymbols: Bool {
        cachedNowHydrationSymbolAvailable
            || localQuickActionPreparationSymbolAvailable
            || offlineRequestPackagingSymbolAvailable
            || deterministicDomainHelpersSymbolAvailable
            || localThreadDraftPackagingSymbolAvailable
            || localVoiceCapturePackagingSymbolAvailable
            || localVoiceQuickActionPackagingSymbolAvailable
            || localVoiceContinuityPackagingSymbolAvailable
            || localQueuedActionPackagingSymbolAvailable
            || localLinkingSettingsNormalizationSymbolAvailable
            || localAssistantEntryFallbackPackagingSymbolAvailable
            || localLinkingRequestPackagingSymbolAvailable
            || localCaptureMetadataPackagingSymbolAvailable
            || localVoiceContinuitySummaryPackagingSymbolAvailable
            || localVoiceOfflineResponsePackagingSymbolAvailable
    }

    public var isOperational: Bool {
        isBridgeLoaded && hasUsableSymbols
    }

    public func isOperational(for configuration: EmbeddedBridgeConfiguration) -> Bool {
        guard isBridgeLoaded else { return false }
        return configuration.approvedFlows.allSatisfy { symbolAvailable(for: $0) }
    }

    public func missingApprovedFlows(for configuration: EmbeddedBridgeConfiguration) -> [EmbeddedAppleFlow] {
        configuration.approvedFlows
            .filter { !symbolAvailable(for: $0) }
            .sorted { $0.rawValue < $1.rawValue }
    }

    public var discoveredSymbolCount: Int {
        [cachedNowHydrationSymbolAvailable, localQuickActionPreparationSymbolAvailable, offlineRequestPackagingSymbolAvailable, deterministicDomainHelpersSymbolAvailable, localThreadDraftPackagingSymbolAvailable, localVoiceCapturePackagingSymbolAvailable, localVoiceQuickActionPackagingSymbolAvailable, localVoiceContinuityPackagingSymbolAvailable, localQueuedActionPackagingSymbolAvailable, localLinkingSettingsNormalizationSymbolAvailable, localAssistantEntryFallbackPackagingSymbolAvailable, localLinkingRequestPackagingSymbolAvailable, localCaptureMetadataPackagingSymbolAvailable, localVoiceContinuitySummaryPackagingSymbolAvailable, localVoiceOfflineResponsePackagingSymbolAvailable]
            .filter(\.self)
            .count
    }

    public func symbolAvailable(for flow: EmbeddedAppleFlow) -> Bool {
        switch flow {
        case .cachedNowHydration:
            cachedNowHydrationSymbolAvailable
        case .localQuickActionPreparation:
            localQuickActionPreparationSymbolAvailable
        case .offlineRequestPackaging:
            offlineRequestPackagingSymbolAvailable
        case .deterministicDomainHelpers:
            deterministicDomainHelpersSymbolAvailable
        case .localThreadDraftPackaging:
            localThreadDraftPackagingSymbolAvailable
        case .localVoiceCapturePackaging:
            localVoiceCapturePackagingSymbolAvailable
        case .localVoiceQuickActionPackaging:
            localVoiceQuickActionPackagingSymbolAvailable
        case .localVoiceContinuityPackaging:
            localVoiceContinuityPackagingSymbolAvailable
        case .localQueuedActionPackaging:
            localQueuedActionPackagingSymbolAvailable
        case .localLinkingSettingsNormalization:
            localLinkingSettingsNormalizationSymbolAvailable
        case .localAssistantEntryFallbackPackaging:
            localAssistantEntryFallbackPackagingSymbolAvailable
        case .localLinkingRequestPackaging:
            localLinkingRequestPackagingSymbolAvailable
        case .localCaptureMetadataPackaging:
            localCaptureMetadataPackagingSymbolAvailable
        case .localVoiceContinuitySummaryPackaging:
            localVoiceContinuitySummaryPackagingSymbolAvailable
        case .localVoiceOfflineResponsePackaging:
            localVoiceOfflineResponsePackagingSymbolAvailable
        }
    }
}

public struct EmbeddedBridgeConfiguration: Sendable {
    public let isBridgeAvailableInBuild: Bool
    public let mode: EmbeddedRuntimeMode
    public let target: EmbeddedRuntimeTarget
    public let approvedFlows: Set<EmbeddedAppleFlow>

    public init(
        isBridgeAvailableInBuild: Bool,
        mode: EmbeddedRuntimeMode,
        target: EmbeddedRuntimeTarget,
        approvedFlows: Set<EmbeddedAppleFlow>
    ) {
        self.isBridgeAvailableInBuild = isBridgeAvailableInBuild
        self.mode = mode
        self.target = target
        self.approvedFlows = approvedFlows
    }

    public func permits(_ flow: EmbeddedAppleFlow) -> Bool {
        isBridgeAvailableInBuild
            && mode == .embeddedCapable
            && target == .iphoneOnly
            && approvedFlows.contains(flow)
    }

    public static func daemonBackedDefault() -> EmbeddedBridgeConfiguration {
        EmbeddedBridgeConfiguration(
            isBridgeAvailableInBuild: false,
            mode: .daemonBacked,
            target: .iphoneOnly,
            approvedFlows: []
        )
    }
}

public protocol EmbeddedNowBridge: Sendable {
    func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String]
}

public protocol EmbeddedQuickActionBridge: Sendable {
    func prepareQuickCapture(_ text: String) -> String
}

public protocol EmbeddedOfflineRequestBridge: Sendable {
    func packageOfflineRequest(_ payload: String) -> String
}

public protocol EmbeddedDomainHelpersBridge: Sendable {
    func normalizeDomainHint(_ input: String) -> String
}

public struct EmbeddedThreadDraftPacket: Sendable {
    public let payload: String
    public let requestedConversationID: String?

    public init(payload: String, requestedConversationID: String?) {
        self.payload = payload
        self.requestedConversationID = requestedConversationID
    }
}

public protocol EmbeddedThreadDraftBridge: Sendable {
    func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket
}

public protocol EmbeddedVoiceCaptureBridge: Sendable {
    func prepareVoiceCapturePayload(transcript: String, intentStorageToken: String) -> String
}

public struct EmbeddedVoiceQuickActionPacket: Sendable {
    public let queueKind: String
    public let targetID: String?
    public let text: String?
    public let minutes: Int?

    public init(queueKind: String, targetID: String?, text: String?, minutes: Int?) {
        self.queueKind = queueKind
        self.targetID = targetID
        self.text = text
        self.minutes = minutes
    }
}

public protocol EmbeddedVoiceQuickActionBridge: Sendable {
    func packageVoiceQuickAction(
        intentStorageToken: String,
        primaryText: String,
        targetID: String?,
        minutes: Int?
    ) -> EmbeddedVoiceQuickActionPacket?
}

public struct EmbeddedVoiceDraftPacket: Sendable {
    public let transcript: String
    public let suggestedIntentStorageToken: String
    public let suggestedText: String

    public init(transcript: String, suggestedIntentStorageToken: String, suggestedText: String) {
        self.transcript = transcript
        self.suggestedIntentStorageToken = suggestedIntentStorageToken
        self.suggestedText = suggestedText
    }
}

public struct EmbeddedVoiceContinuityEntryPacket: Sendable {
    public let transcript: String
    public let suggestedIntentStorageToken: String
    public let committedIntentStorageToken: String?
    public let status: String
    public let threadID: String?

    public init(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) {
        self.transcript = transcript
        self.suggestedIntentStorageToken = suggestedIntentStorageToken
        self.committedIntentStorageToken = committedIntentStorageToken
        self.status = status
        self.threadID = threadID
    }
}

public protocol EmbeddedVoiceContinuityBridge: Sendable {
    func prepareVoiceDraft(
        transcript: String,
        suggestedIntentStorageToken: String,
        suggestedText: String
    ) -> EmbeddedVoiceDraftPacket

    func prepareVoiceContinuityEntry(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) -> EmbeddedVoiceContinuityEntryPacket
}

public struct EmbeddedQueuedActionPacket: Sendable {
    public let queueKind: String
    public let targetID: String?
    public let text: String?
    public let minutes: Int?

    public init(queueKind: String, targetID: String?, text: String?, minutes: Int?) {
        self.queueKind = queueKind
        self.targetID = targetID
        self.text = text
        self.minutes = minutes
    }
}

public protocol EmbeddedQueuedActionBridge: Sendable {
    func packageQueuedAction(
        kind: String,
        targetID: String?,
        text: String?,
        minutes: Int?
    ) -> EmbeddedQueuedActionPacket?
}

public struct EmbeddedRemoteRouteSummary: Sendable, Equatable {
    public let label: String
    public let baseURL: String

    public init(label: String, baseURL: String) {
        self.label = label
        self.baseURL = baseURL
    }
}

public protocol EmbeddedLinkingSettingsBridge: Sendable {
    func normalizePairingTokenInput(_ value: String) -> String
    func collectRemoteRoutes(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> [EmbeddedRemoteRouteSummary]
}

public struct EmbeddedAssistantEntryFallbackPacket: Sendable {
    public let payload: String

    public init(payload: String) {
        self.payload = payload
    }
}

public protocol EmbeddedAssistantEntryFallbackBridge: Sendable {
    func prepareAssistantEntryFallback(
        text: String,
        conversationID: String?
    ) -> EmbeddedAssistantEntryFallbackPacket
}

public struct EmbeddedLinkingRequestPacket: Sendable {
    public let tokenCode: String?
    public let targetBaseURL: String?

    public init(tokenCode: String?, targetBaseURL: String?) {
        self.tokenCode = tokenCode
        self.targetBaseURL = targetBaseURL
    }
}

public protocol EmbeddedLinkingRequestBridge: Sendable {
    func prepareLinkingRequest(tokenCode: String?, targetBaseURL: String?) -> EmbeddedLinkingRequestPacket
    func preparePairingTokenIssueRequest(
        issuedByNodeID: String,
        targetNodeID: String?,
        targetNodeDisplayName: String?,
        targetBaseURL: String?
    ) -> EmbeddedPairingTokenIssueRequestPacket
    func preparePairingTokenRedeemRequest(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) -> EmbeddedPairingTokenRedeemRequestPacket
}

public struct EmbeddedPairingTokenIssueRequestPacket: Sendable {
    public let issuedByNodeID: String
    public let targetNodeID: String?
    public let targetNodeDisplayName: String?
    public let targetBaseURL: String?

    public init(issuedByNodeID: String, targetNodeID: String?, targetNodeDisplayName: String?, targetBaseURL: String?) {
        self.issuedByNodeID = issuedByNodeID
        self.targetNodeID = targetNodeID
        self.targetNodeDisplayName = targetNodeDisplayName
        self.targetBaseURL = targetBaseURL
    }
}

public struct EmbeddedPairingTokenRedeemRequestPacket: Sendable {
    public let tokenCode: String
    public let nodeID: String
    public let nodeDisplayName: String
    public let transportHint: String?
    public let syncBaseURL: String?
    public let tailscaleBaseURL: String?
    public let lanBaseURL: String?
    public let localhostBaseURL: String?
    public let publicBaseURL: String?

    public init(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) {
        self.tokenCode = tokenCode
        self.nodeID = nodeID
        self.nodeDisplayName = nodeDisplayName
        self.transportHint = transportHint
        self.syncBaseURL = syncBaseURL
        self.tailscaleBaseURL = tailscaleBaseURL
        self.lanBaseURL = lanBaseURL
        self.localhostBaseURL = localhostBaseURL
        self.publicBaseURL = publicBaseURL
    }
}

public protocol EmbeddedCaptureMetadataBridge: Sendable {
    func prepareQueuedCaptureText(text: String, type: String, source: String) -> String
}

public struct EmbeddedVoiceContinuitySummaryPacket: Sendable {
    public let headline: String
    public let detail: String?

    public init(headline: String, detail: String?) {
        self.headline = headline
        self.detail = detail
    }
}

public struct EmbeddedVoiceOfflineResponsePacket: Sendable {
    public let summary: String
    public let detail: String?
    public let historyStatus: String
    public let errorPrefix: String

    public init(summary: String, detail: String?, historyStatus: String, errorPrefix: String) {
        self.summary = summary
        self.detail = detail
        self.historyStatus = historyStatus
        self.errorPrefix = errorPrefix
    }
}

public protocol EmbeddedVoiceContinuitySummaryBridge: Sendable {
    func prepareVoiceContinuitySummary(
        draftExists: Bool,
        threadedTranscript: String?,
        pendingRecoveryCount: Int,
        isReachable: Bool,
        mergedTranscript: String?
    ) -> EmbeddedVoiceContinuitySummaryPacket?
}

public protocol EmbeddedVoiceOfflineResponseBridge: Sendable {
    func prepareVoiceOfflineResponse(
        scenario: String,
        primaryText: String?,
        matchedText: String?,
        options: String?,
        minutes: Int?,
        isReachable: Bool
    ) -> EmbeddedVoiceOfflineResponsePacket?
}

private struct OfflineBridgeEnvelope: Decodable {
    let kind: String?
    let payload: String?
}

public protocol EmbeddedBridgeSurface: Sendable {
    var configuration: EmbeddedBridgeConfiguration { get }
    var runtimeStatus: EmbeddedBridgeRuntimeStatus { get }
    var nowBridge: any EmbeddedNowBridge { get }
    var quickActionBridge: any EmbeddedQuickActionBridge { get }
    var offlineRequestBridge: any EmbeddedOfflineRequestBridge { get }
    var domainHelpersBridge: any EmbeddedDomainHelpersBridge { get }
    var threadDraftBridge: any EmbeddedThreadDraftBridge { get }
    var voiceCaptureBridge: any EmbeddedVoiceCaptureBridge { get }
    var voiceQuickActionBridge: any EmbeddedVoiceQuickActionBridge { get }
    var voiceContinuityBridge: any EmbeddedVoiceContinuityBridge { get }
    var queuedActionBridge: any EmbeddedQueuedActionBridge { get }
    var linkingSettingsBridge: any EmbeddedLinkingSettingsBridge { get }
    var assistantEntryFallbackBridge: any EmbeddedAssistantEntryFallbackBridge { get }
    var linkingRequestBridge: any EmbeddedLinkingRequestBridge { get }
    var captureMetadataBridge: any EmbeddedCaptureMetadataBridge { get }
    var voiceContinuitySummaryBridge: any EmbeddedVoiceContinuitySummaryBridge { get }
    var voiceOfflineResponseBridge: any EmbeddedVoiceOfflineResponseBridge { get }
}

public struct NoopEmbeddedNowBridge: EmbeddedNowBridge {
    public init() {}

    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        [
            "Mode: \(context.mode ?? "unknown")",
            "Next: \(context.nextEventTitle ?? "none")",
            "Nudges: \(context.nudgeCount)"
        ]
    }
}

public struct NoopEmbeddedQuickActionBridge: EmbeddedQuickActionBridge {
    public init() {}

    public func prepareQuickCapture(_ text: String) -> String {
        text.trimmingCharacters(in: .whitespacesAndNewlines)
    }
}

public struct NoopEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge {
    public init() {}

    public func packageOfflineRequest(_ payload: String) -> String {
        guard let data = payload.data(using: .utf8),
              let envelope = try? JSONDecoder().decode(OfflineBridgeEnvelope.self, from: data),
              let envelopePayload = envelope.payload else {
            return payload.trimmingCharacters(in: .whitespacesAndNewlines)
        }
        return envelopePayload
    }
}

public struct NoopEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge {
    public init() {}

    public func normalizeDomainHint(_ input: String) -> String {
        input
            .trimmingCharacters(in: .whitespacesAndNewlines)
            .lowercased()
    }
}

public struct NoopEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge {
    public init() {}

    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        let normalizedConversationID = conversationID?
            .trimmingCharacters(in: .whitespacesAndNewlines)
        return EmbeddedThreadDraftPacket(
            payload: text.trimmingCharacters(in: .whitespacesAndNewlines),
            requestedConversationID: normalizedConversationID?.isEmpty == true ? nil : normalizedConversationID
        )
    }
}

public struct NoopEmbeddedVoiceCaptureBridge: EmbeddedVoiceCaptureBridge {
    public init() {}

    public func prepareVoiceCapturePayload(transcript: String, intentStorageToken: String) -> String {
        [
            "voice_transcript:",
            transcript.trimmingCharacters(in: .whitespacesAndNewlines),
            "",
            "intent_candidate: \(intentStorageToken.trimmingCharacters(in: .whitespacesAndNewlines))",
            "client_surface: ios_voice"
        ]
        .joined(separator: "\n")
    }
}

public struct NoopEmbeddedVoiceQuickActionBridge: EmbeddedVoiceQuickActionBridge {
    public init() {}

    public func packageVoiceQuickAction(
        intentStorageToken: String,
        primaryText: String,
        targetID: String?,
        minutes: Int?
    ) -> EmbeddedVoiceQuickActionPacket? {
        let normalizedText = primaryText.trimmingCharacters(in: .whitespacesAndNewlines)
        let normalizedTargetID = targetID?.trimmingCharacters(in: .whitespacesAndNewlines)

        if intentStorageToken == "capture_create" {
            return EmbeddedVoiceQuickActionPacket(
                queueKind: "capture.create",
                targetID: nil,
                text: normalizedText,
                minutes: nil
            )
        }

        if intentStorageToken == "commitment_create" {
            return EmbeddedVoiceQuickActionPacket(
                queueKind: "commitment.create",
                targetID: nil,
                text: normalizedText,
                minutes: nil
            )
        }

        if intentStorageToken == "commitment_done" {
            return EmbeddedVoiceQuickActionPacket(
                queueKind: "commitment.done",
                targetID: normalizedTargetID,
                text: nil,
                minutes: nil
            )
        }

        if intentStorageToken == "nudge_done" {
            return EmbeddedVoiceQuickActionPacket(
                queueKind: "nudge.done",
                targetID: normalizedTargetID,
                text: nil,
                minutes: nil
            )
        }

        if intentStorageToken.hasPrefix("nudge_snooze_") {
            return EmbeddedVoiceQuickActionPacket(
                queueKind: "nudge.snooze",
                targetID: normalizedTargetID,
                text: nil,
                minutes: minutes ?? 10
            )
        }

        return nil
    }
}

public struct NoopEmbeddedVoiceContinuityBridge: EmbeddedVoiceContinuityBridge {
    public init() {}

    public func prepareVoiceDraft(
        transcript: String,
        suggestedIntentStorageToken: String,
        suggestedText: String
    ) -> EmbeddedVoiceDraftPacket {
        EmbeddedVoiceDraftPacket(
            transcript: transcript.trimmingCharacters(in: .whitespacesAndNewlines),
            suggestedIntentStorageToken: suggestedIntentStorageToken.trimmingCharacters(in: .whitespacesAndNewlines),
            suggestedText: suggestedText.trimmingCharacters(in: .whitespacesAndNewlines)
        )
    }

    public func prepareVoiceContinuityEntry(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) -> EmbeddedVoiceContinuityEntryPacket {
        let normalizedThreadID = threadID?.trimmingCharacters(in: .whitespacesAndNewlines)
        return EmbeddedVoiceContinuityEntryPacket(
            transcript: transcript.trimmingCharacters(in: .whitespacesAndNewlines),
            suggestedIntentStorageToken: suggestedIntentStorageToken.trimmingCharacters(in: .whitespacesAndNewlines),
            committedIntentStorageToken: committedIntentStorageToken?.trimmingCharacters(in: .whitespacesAndNewlines),
            status: status.trimmingCharacters(in: .whitespacesAndNewlines),
            threadID: normalizedThreadID?.isEmpty == true ? nil : normalizedThreadID
        )
    }
}

public struct NoopEmbeddedQueuedActionBridge: EmbeddedQueuedActionBridge {
    public init() {}

    public func packageQueuedAction(
        kind: String,
        targetID: String?,
        text: String?,
        minutes: Int?
    ) -> EmbeddedQueuedActionPacket? {
        let normalizedKind = kind.trimmingCharacters(in: .whitespacesAndNewlines)
        let normalizedTargetID = targetID?.trimmingCharacters(in: .whitespacesAndNewlines)
        let normalizedText = text?.trimmingCharacters(in: .whitespacesAndNewlines)
        let normalizedMinutes = minutes.map { max($0, 1) }
        guard !normalizedKind.isEmpty else { return nil }
        return EmbeddedQueuedActionPacket(
            queueKind: normalizedKind,
            targetID: normalizedTargetID?.isEmpty == true ? nil : normalizedTargetID,
            text: normalizedText?.isEmpty == true ? nil : normalizedText,
            minutes: normalizedMinutes
        )
    }
}

public struct NoopEmbeddedLinkingSettingsBridge: EmbeddedLinkingSettingsBridge {
    public init() {}

    public func normalizePairingTokenInput(_ value: String) -> String {
        let normalized = value.uppercased().filter { character in
            character.isASCII && (character.isLetter || character.isNumber)
        }.prefix(6)
        let text = String(normalized)
        if text.count <= 3 { return text }
        let splitIndex = text.index(text.startIndex, offsetBy: 3)
        return "\(text[..<splitIndex])-\(text[splitIndex...])"
    }

    public func collectRemoteRoutes(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> [EmbeddedRemoteRouteSummary] {
        let entries: [(String, String?)] = [
            ("primary", syncBaseURL),
            ("tailscale", tailscaleBaseURL),
            ("lan", lanBaseURL),
            ("public", publicBaseURL),
        ]
        var seen = Set<String>()
        var routes: [EmbeddedRemoteRouteSummary] = []
        for (label, value) in entries {
            let trimmed = value?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
            if trimmed.isEmpty || trimmed.contains("127.0.0.1") || trimmed.contains("localhost") || seen.contains(trimmed) {
                continue
            }
            seen.insert(trimmed)
            routes.append(EmbeddedRemoteRouteSummary(label: label, baseURL: trimmed))
        }
        return routes
    }
}

public struct NoopEmbeddedAssistantEntryFallbackBridge: EmbeddedAssistantEntryFallbackBridge {
    public init() {}

    public func prepareAssistantEntryFallback(
        text: String,
        conversationID: String?
    ) -> EmbeddedAssistantEntryFallbackPacket {
        let normalizedConversationID = conversationID?.trimmingCharacters(in: .whitespacesAndNewlines)
        let payload = [
            "queued_assistant_entry:",
            normalizedConversationID.map { "requested_conversation_id: \($0)" },
            "",
            text.trimmingCharacters(in: .whitespacesAndNewlines)
        ]
            .compactMap { $0 }
            .filter { !$0.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty }
            .joined(separator: "\n")
        return EmbeddedAssistantEntryFallbackPacket(payload: payload)
    }
}

public struct NoopEmbeddedLinkingRequestBridge: EmbeddedLinkingRequestBridge {
    public init() {}

    public func prepareLinkingRequest(tokenCode: String?, targetBaseURL: String?) -> EmbeddedLinkingRequestPacket {
        let normalizedToken = tokenCode?.trimmingCharacters(in: .whitespacesAndNewlines)
        let normalizedBaseURL = targetBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines)
        return EmbeddedLinkingRequestPacket(
            tokenCode: normalizedToken?.isEmpty == true ? nil : normalizedToken,
            targetBaseURL: normalizedBaseURL?.isEmpty == true ? nil : normalizedBaseURL
        )
    }

    public func preparePairingTokenIssueRequest(
        issuedByNodeID: String,
        targetNodeID: String?,
        targetNodeDisplayName: String?,
        targetBaseURL: String?
    ) -> EmbeddedPairingTokenIssueRequestPacket {
        EmbeddedPairingTokenIssueRequestPacket(
            issuedByNodeID: issuedByNodeID.trimmingCharacters(in: .whitespacesAndNewlines),
            targetNodeID: targetNodeID?.trimmingCharacters(in: .whitespacesAndNewlines),
            targetNodeDisplayName: targetNodeDisplayName?.trimmingCharacters(in: .whitespacesAndNewlines),
            targetBaseURL: targetBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines)
        )
    }

    public func preparePairingTokenRedeemRequest(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) -> EmbeddedPairingTokenRedeemRequestPacket {
        EmbeddedPairingTokenRedeemRequestPacket(
            tokenCode: tokenCode.trimmingCharacters(in: .whitespacesAndNewlines),
            nodeID: nodeID.trimmingCharacters(in: .whitespacesAndNewlines),
            nodeDisplayName: nodeDisplayName.trimmingCharacters(in: .whitespacesAndNewlines),
            transportHint: transportHint?.trimmingCharacters(in: .whitespacesAndNewlines),
            syncBaseURL: syncBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines),
            tailscaleBaseURL: tailscaleBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines),
            lanBaseURL: lanBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines),
            localhostBaseURL: localhostBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines),
            publicBaseURL: publicBaseURL?.trimmingCharacters(in: .whitespacesAndNewlines)
        )
    }
}

public struct NoopEmbeddedCaptureMetadataBridge: EmbeddedCaptureMetadataBridge {
    public init() {}

    public func prepareQueuedCaptureText(text: String, type: String, source: String) -> String {
        let cleanType = type.trimmingCharacters(in: .whitespacesAndNewlines)
        let cleanSource = source.trimmingCharacters(in: .whitespacesAndNewlines)
        guard cleanType != "note" || cleanSource != "apple" else {
            return text
        }

        return [
            "queued_capture_metadata:",
            "requested_capture_type: \(cleanType)",
            "requested_source_device: \(cleanSource)",
            "",
            text
        ].joined(separator: "\n")
    }
}

public struct NoopEmbeddedVoiceContinuitySummaryBridge: EmbeddedVoiceContinuitySummaryBridge {
    public init() {}

    public func prepareVoiceContinuitySummary(
        draftExists: Bool,
        threadedTranscript: String?,
        pendingRecoveryCount: Int,
        isReachable: Bool,
        mergedTranscript: String?
    ) -> EmbeddedVoiceContinuitySummaryPacket? {
        if draftExists {
            return EmbeddedVoiceContinuitySummaryPacket(
                headline: "Voice draft ready to resume.",
                detail: "Your latest local transcript is still on device and can be resumed without reopening a separate thread."
            )
        }

        if let threadedTranscript, !threadedTranscript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            return EmbeddedVoiceContinuitySummaryPacket(
                headline: "Voice follow-up saved in Threads.",
                detail: threadedTranscript
            )
        }

        if pendingRecoveryCount > 0 {
            let detail = isReachable
                ? "Local voice recovery is waiting on canonical replay."
                : "Reconnect to merge \(pendingRecoveryCount) local voice entr\(pendingRecoveryCount == 1 ? "y" : "ies") back into canonical state."
            return EmbeddedVoiceContinuitySummaryPacket(
                headline: "Voice recovery pending.",
                detail: detail
            )
        }

        if let mergedTranscript, !mergedTranscript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            return EmbeddedVoiceContinuitySummaryPacket(
                headline: "Local voice recovery merged.",
                detail: mergedTranscript
            )
        }

        return nil
    }
}

public struct NoopEmbeddedVoiceOfflineResponseBridge: EmbeddedVoiceOfflineResponseBridge {
    public init() {}

    public func prepareVoiceOfflineResponse(
        scenario: String,
        primaryText: String?,
        matchedText: String?,
        options: String?,
        minutes: Int?,
        isReachable: Bool
    ) -> EmbeddedVoiceOfflineResponsePacket? {
        switch scenario {
        case "capture_shell":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: isReachable ? "Saved voice capture." : "Voice capture queued for sync.",
                detail: primaryText,
                historyStatus: isReachable ? "submitted" : "queued",
                errorPrefix: isReachable ? "" : "Voice transcript queued for sync."
            )
        case "commitment_create_shell":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: isReachable ? "Created commitment." : "Commitment queued for sync.",
                detail: primaryText,
                historyStatus: isReachable ? "submitted" : "queued",
                errorPrefix: isReachable ? "" : "Commitment request queued for sync."
            )
        case "backend_required_shell":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "This voice action now requires the backend Apple route.",
                detail: "Reconnect to Vel so the server can interpret and answer it.",
                historyStatus: "backend_required",
                errorPrefix: "Transcript capture was preserved, but the action needs the backend-owned Apple route."
            )
        case "capture_offline":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Voice capture queued for sync.",
                detail: primaryText,
                historyStatus: "queued",
                errorPrefix: "Transcript capture queued for sync."
            )
        case "commitment_target_missing":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Commitment target is missing.",
                detail: "Try phrasing like \"mark meds done.\"",
                historyStatus: "needs_clarification",
                errorPrefix: "Commitment target missing."
            )
        case "commitment_no_match":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "No open commitment matched.",
                detail: "Transcript capture was queued for sync.",
                historyStatus: "capture_only",
                errorPrefix: "No local commitment match for offline queueing."
            )
        case "commitment_ambiguous":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Ambiguous commitment target.",
                detail: options.map { "Could match: \($0)" },
                historyStatus: "needs_clarification",
                errorPrefix: "Commitment target was ambiguous."
            )
        case "commitment_done_queued":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Commitment completion queued.",
                detail: matchedText,
                historyStatus: "queued",
                errorPrefix: "Commitment completion queued for backend replay."
            )
        case "nudge_missing":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "No active nudge found.",
                detail: "Transcript capture was queued for sync.",
                historyStatus: "capture_only",
                errorPrefix: "No active nudge available for offline queueing."
            )
        case "nudge_done_queued":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Top nudge resolution queued.",
                detail: nil,
                historyStatus: "queued",
                errorPrefix: "Top nudge resolution queued for backend replay."
            )
        case "nudge_snooze_queued":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Top nudge snooze queued.",
                detail: "\(minutes ?? 10) minutes",
                historyStatus: "queued",
                errorPrefix: "Top nudge snooze queued for backend replay."
            )
        case "backend_required_offline":
            return EmbeddedVoiceOfflineResponsePacket(
                summary: "Unavailable offline.",
                detail: "This reply is backend-owned and is not synthesized from local Swift cache.",
                historyStatus: "backend_required",
                errorPrefix: "Transcript capture queued, but this voice reply requires the backend route."
            )
        default:
            return nil
        }
    }
}

public struct NoopEmbeddedBridgeSurface: EmbeddedBridgeSurface {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge
    public let voiceCaptureBridge: any EmbeddedVoiceCaptureBridge
    public let voiceQuickActionBridge: any EmbeddedVoiceQuickActionBridge
    public let voiceContinuityBridge: any EmbeddedVoiceContinuityBridge
    public let queuedActionBridge: any EmbeddedQueuedActionBridge
    public let linkingSettingsBridge: any EmbeddedLinkingSettingsBridge
    public let assistantEntryFallbackBridge: any EmbeddedAssistantEntryFallbackBridge
    public let linkingRequestBridge: any EmbeddedLinkingRequestBridge
    public let captureMetadataBridge: any EmbeddedCaptureMetadataBridge
    public let voiceContinuitySummaryBridge: any EmbeddedVoiceContinuitySummaryBridge
    public let voiceOfflineResponseBridge: any EmbeddedVoiceOfflineResponseBridge

    public init(configuration: EmbeddedBridgeConfiguration = .daemonBackedDefault()) {
        self.configuration = configuration
        self.runtimeStatus = .unavailable
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
        self.voiceCaptureBridge = NoopEmbeddedVoiceCaptureBridge()
        self.voiceQuickActionBridge = NoopEmbeddedVoiceQuickActionBridge()
        self.voiceContinuityBridge = NoopEmbeddedVoiceContinuityBridge()
        self.queuedActionBridge = NoopEmbeddedQueuedActionBridge()
        self.linkingSettingsBridge = NoopEmbeddedLinkingSettingsBridge()
        self.assistantEntryFallbackBridge = NoopEmbeddedAssistantEntryFallbackBridge()
        self.linkingRequestBridge = NoopEmbeddedLinkingRequestBridge()
        self.captureMetadataBridge = NoopEmbeddedCaptureMetadataBridge()
        self.voiceContinuitySummaryBridge = NoopEmbeddedVoiceContinuitySummaryBridge()
        self.voiceOfflineResponseBridge = NoopEmbeddedVoiceOfflineResponseBridge()
    }
}

#if canImport(Darwin)
private typealias VelEmbeddedCachedNowSummaryFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareQuickCaptureFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPackageOfflineRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedNormalizeDomainHelpersFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareThreadDraftFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareVoiceCapturePayloadFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPackageVoiceQuickActionFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareVoiceDraftFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareVoiceContinuityEntryFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPackageQueuedActionFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedNormalizePairingTokenFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedCollectRemoteRoutesFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareAssistantEntryFallbackFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareLinkingRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareCaptureMetadataFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPreparePairingTokenIssueRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPreparePairingTokenRedeemRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareVoiceContinuitySummaryFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareVoiceOfflineResponseFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedFreeBufferFn = @convention(c) (UnsafeMutablePointer<CChar>?) -> Void

private struct VelEmbeddedRustBindings: @unchecked Sendable {
    let handle: UnsafeMutableRawPointer
    let cachedNowSummary: VelEmbeddedCachedNowSummaryFn?
    let prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn?
    let packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn?
    let normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn?
    let prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn?
    let prepareVoiceCapturePayload: VelEmbeddedPrepareVoiceCapturePayloadFn?
    let packageVoiceQuickAction: VelEmbeddedPackageVoiceQuickActionFn?
    let prepareVoiceDraft: VelEmbeddedPrepareVoiceDraftFn?
    let prepareVoiceContinuityEntry: VelEmbeddedPrepareVoiceContinuityEntryFn?
    let packageQueuedAction: VelEmbeddedPackageQueuedActionFn?
    let normalizePairingToken: VelEmbeddedNormalizePairingTokenFn?
    let collectRemoteRoutes: VelEmbeddedCollectRemoteRoutesFn?
    let prepareAssistantEntryFallback: VelEmbeddedPrepareAssistantEntryFallbackFn?
    let prepareLinkingRequest: VelEmbeddedPrepareLinkingRequestFn?
    let prepareCaptureMetadata: VelEmbeddedPrepareCaptureMetadataFn?
    let preparePairingTokenIssueRequest: VelEmbeddedPreparePairingTokenIssueRequestFn?
    let preparePairingTokenRedeemRequest: VelEmbeddedPreparePairingTokenRedeemRequestFn?
    let prepareVoiceContinuitySummary: VelEmbeddedPrepareVoiceContinuitySummaryFn?
    let prepareVoiceOfflineResponse: VelEmbeddedPrepareVoiceOfflineResponseFn?
    let freeBuffer: VelEmbeddedFreeBufferFn
}

private enum VelEmbeddedRustBridge {
    private static let symbolNames = (
        cachedNowSummary: "vel_embedded_cached_now_summary",
        prepareQuickCapture: "vel_embedded_prepare_quick_capture",
        packageOfflineRequest: "vel_embedded_package_offline_request",
        normalizeDomainHelpers: "vel_embedded_normalize_domain_helpers",
        prepareThreadDraft: "vel_embedded_prepare_thread_draft",
        prepareVoiceCapturePayload: "vel_embedded_prepare_voice_capture_payload",
        packageVoiceQuickAction: "vel_embedded_package_voice_quick_action",
        prepareVoiceDraft: "vel_embedded_prepare_voice_draft",
        prepareVoiceContinuityEntry: "vel_embedded_prepare_voice_continuity_entry",
        packageQueuedAction: "vel_embedded_package_queued_action",
        normalizePairingToken: "vel_embedded_normalize_pairing_token",
        collectRemoteRoutes: "vel_embedded_collect_remote_routes",
        prepareAssistantEntryFallback: "vel_embedded_prepare_assistant_entry_fallback",
        prepareLinkingRequest: "vel_embedded_prepare_linking_request",
        prepareCaptureMetadata: "vel_embedded_prepare_capture_metadata",
        preparePairingTokenIssueRequest: "vel_embedded_prepare_pairing_token_issue_request",
        preparePairingTokenRedeemRequest: "vel_embedded_prepare_pairing_token_redeem_request",
        prepareVoiceContinuitySummary: "vel_embedded_prepare_voice_continuity_summary",
        prepareVoiceOfflineResponse: "vel_embedded_prepare_voice_offline_response",
        freeBuffer: "vel_embedded_free_buffer"
    )

    private typealias BindingResolution = (
        bindings: VelEmbeddedRustBindings?,
        status: EmbeddedBridgeRuntimeStatus
    )

    static let resolution: BindingResolution = {
        let flags = RTLD_NOW | RTLD_LOCAL
        var attemptedPaths: [String] = []

        func makeStatus(
            source: String?,
            freeBuffer: VelEmbeddedFreeBufferFn?,
            cachedNowSummary: VelEmbeddedCachedNowSummaryFn?,
            prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn?,
            packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn?,
            normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn?,
            prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn?,
            prepareVoiceCapturePayload: VelEmbeddedPrepareVoiceCapturePayloadFn?,
            packageVoiceQuickAction: VelEmbeddedPackageVoiceQuickActionFn?,
            prepareVoiceDraft: VelEmbeddedPrepareVoiceDraftFn?,
            prepareVoiceContinuityEntry: VelEmbeddedPrepareVoiceContinuityEntryFn?,
            packageQueuedAction: VelEmbeddedPackageQueuedActionFn?,
            normalizePairingToken: VelEmbeddedNormalizePairingTokenFn?,
            collectRemoteRoutes: VelEmbeddedCollectRemoteRoutesFn?,
            prepareAssistantEntryFallback: VelEmbeddedPrepareAssistantEntryFallbackFn?,
            prepareLinkingRequest: VelEmbeddedPrepareLinkingRequestFn?,
            prepareCaptureMetadata: VelEmbeddedPrepareCaptureMetadataFn?,
            preparePairingTokenIssueRequest: VelEmbeddedPreparePairingTokenIssueRequestFn?,
            preparePairingTokenRedeemRequest: VelEmbeddedPreparePairingTokenRedeemRequestFn?,
            prepareVoiceContinuitySummary: VelEmbeddedPrepareVoiceContinuitySummaryFn?,
            prepareVoiceOfflineResponse: VelEmbeddedPrepareVoiceOfflineResponseFn?
        ) -> EmbeddedBridgeRuntimeStatus {
            EmbeddedBridgeRuntimeStatus(
                resolvedSource: source,
                attemptedPaths: attemptedPaths,
                freeBufferAvailable: freeBuffer != nil,
                cachedNowHydrationSymbolAvailable: cachedNowSummary != nil,
                localQuickActionPreparationSymbolAvailable: prepareQuickCapture != nil,
                offlineRequestPackagingSymbolAvailable: packageOfflineRequest != nil,
                deterministicDomainHelpersSymbolAvailable: normalizeDomainHelpers != nil,
                localThreadDraftPackagingSymbolAvailable: prepareThreadDraft != nil,
                localVoiceCapturePackagingSymbolAvailable: prepareVoiceCapturePayload != nil,
                localVoiceQuickActionPackagingSymbolAvailable: packageVoiceQuickAction != nil,
                localVoiceContinuityPackagingSymbolAvailable: prepareVoiceDraft != nil && prepareVoiceContinuityEntry != nil,
                localQueuedActionPackagingSymbolAvailable: packageQueuedAction != nil,
                localLinkingSettingsNormalizationSymbolAvailable: normalizePairingToken != nil && collectRemoteRoutes != nil,
                localAssistantEntryFallbackPackagingSymbolAvailable: prepareAssistantEntryFallback != nil,
                localLinkingRequestPackagingSymbolAvailable: prepareLinkingRequest != nil && preparePairingTokenIssueRequest != nil && preparePairingTokenRedeemRequest != nil,
                localCaptureMetadataPackagingSymbolAvailable: prepareCaptureMetadata != nil,
                localVoiceContinuitySummaryPackagingSymbolAvailable: prepareVoiceContinuitySummary != nil,
                localVoiceOfflineResponsePackagingSymbolAvailable: prepareVoiceOfflineResponse != nil
            )
        }

        func bindingsIfUsable(
            from handle: UnsafeMutableRawPointer,
            source: String
        ) -> BindingResolution {
            let freeBuffer = lookup(candidate: symbolNames.freeBuffer, from: handle)
            let cachedNowSummary: VelEmbeddedCachedNowSummaryFn? = lookup(
                candidate: symbolNames.cachedNowSummary,
                from: handle
            )
            let prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn? = lookup(
                candidate: symbolNames.prepareQuickCapture,
                from: handle
            )
            let packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn? = lookup(
                candidate: symbolNames.packageOfflineRequest,
                from: handle
            )
            let normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn? = lookup(
                candidate: symbolNames.normalizeDomainHelpers,
                from: handle
            )
            let prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn? = lookup(
                candidate: symbolNames.prepareThreadDraft,
                from: handle
            )
            let prepareVoiceCapturePayload: VelEmbeddedPrepareVoiceCapturePayloadFn? = lookup(
                candidate: symbolNames.prepareVoiceCapturePayload,
                from: handle
            )
            let packageVoiceQuickAction: VelEmbeddedPackageVoiceQuickActionFn? = lookup(
                candidate: symbolNames.packageVoiceQuickAction,
                from: handle
            )
            let prepareVoiceDraft: VelEmbeddedPrepareVoiceDraftFn? = lookup(
                candidate: symbolNames.prepareVoiceDraft,
                from: handle
            )
            let prepareVoiceContinuityEntry: VelEmbeddedPrepareVoiceContinuityEntryFn? = lookup(
                candidate: symbolNames.prepareVoiceContinuityEntry,
                from: handle
            )
            let packageQueuedAction: VelEmbeddedPackageQueuedActionFn? = lookup(
                candidate: symbolNames.packageQueuedAction,
                from: handle
            )
            let normalizePairingToken: VelEmbeddedNormalizePairingTokenFn? = lookup(
                candidate: symbolNames.normalizePairingToken,
                from: handle
            )
            let collectRemoteRoutes: VelEmbeddedCollectRemoteRoutesFn? = lookup(
                candidate: symbolNames.collectRemoteRoutes,
                from: handle
            )
            let prepareAssistantEntryFallback: VelEmbeddedPrepareAssistantEntryFallbackFn? = lookup(
                candidate: symbolNames.prepareAssistantEntryFallback,
                from: handle
            )
            let prepareLinkingRequest: VelEmbeddedPrepareLinkingRequestFn? = lookup(
                candidate: symbolNames.prepareLinkingRequest,
                from: handle
            )
            let prepareCaptureMetadata: VelEmbeddedPrepareCaptureMetadataFn? = lookup(
                candidate: symbolNames.prepareCaptureMetadata,
                from: handle
            )
            let preparePairingTokenIssueRequest: VelEmbeddedPreparePairingTokenIssueRequestFn? = lookup(
                candidate: symbolNames.preparePairingTokenIssueRequest,
                from: handle
            )
            let preparePairingTokenRedeemRequest: VelEmbeddedPreparePairingTokenRedeemRequestFn? = lookup(
                candidate: symbolNames.preparePairingTokenRedeemRequest,
                from: handle
            )
            let prepareVoiceContinuitySummary: VelEmbeddedPrepareVoiceContinuitySummaryFn? = lookup(
                candidate: symbolNames.prepareVoiceContinuitySummary,
                from: handle
            )
            let prepareVoiceOfflineResponse: VelEmbeddedPrepareVoiceOfflineResponseFn? = lookup(
                candidate: symbolNames.prepareVoiceOfflineResponse,
                from: handle
            )

            let status = makeStatus(
                source: freeBuffer == nil ? nil : source,
                freeBuffer: freeBuffer,
                cachedNowSummary: cachedNowSummary,
                prepareQuickCapture: prepareQuickCapture,
                packageOfflineRequest: packageOfflineRequest,
                normalizeDomainHelpers: normalizeDomainHelpers,
                prepareThreadDraft: prepareThreadDraft,
                prepareVoiceCapturePayload: prepareVoiceCapturePayload,
                packageVoiceQuickAction: packageVoiceQuickAction,
                prepareVoiceDraft: prepareVoiceDraft,
                prepareVoiceContinuityEntry: prepareVoiceContinuityEntry,
                packageQueuedAction: packageQueuedAction,
                normalizePairingToken: normalizePairingToken,
                collectRemoteRoutes: collectRemoteRoutes,
                prepareAssistantEntryFallback: prepareAssistantEntryFallback,
                prepareLinkingRequest: prepareLinkingRequest,
                prepareCaptureMetadata: prepareCaptureMetadata,
                preparePairingTokenIssueRequest: preparePairingTokenIssueRequest,
                preparePairingTokenRedeemRequest: preparePairingTokenRedeemRequest,
                prepareVoiceContinuitySummary: prepareVoiceContinuitySummary,
                prepareVoiceOfflineResponse: prepareVoiceOfflineResponse
            )

            guard freeBuffer != nil else {
                return (nil, status)
            }

            if cachedNowSummary == nil
                && prepareQuickCapture == nil
                && packageOfflineRequest == nil
                && normalizeDomainHelpers == nil
                && prepareThreadDraft == nil
                && prepareVoiceCapturePayload == nil
                && packageVoiceQuickAction == nil
                && prepareVoiceDraft == nil
                && prepareVoiceContinuityEntry == nil
                && packageQueuedAction == nil
                && normalizePairingToken == nil
                && collectRemoteRoutes == nil
                && prepareAssistantEntryFallback == nil
                && prepareLinkingRequest == nil
                && prepareCaptureMetadata == nil
                && preparePairingTokenIssueRequest == nil
                && preparePairingTokenRedeemRequest == nil
                && prepareVoiceContinuitySummary == nil
                && prepareVoiceOfflineResponse == nil
            {
                return (nil, status)
            }

            return (
                VelEmbeddedRustBindings(
                    handle: handle,
                    cachedNowSummary: cachedNowSummary,
                    prepareQuickCapture: prepareQuickCapture,
                    packageOfflineRequest: packageOfflineRequest,
                    normalizeDomainHelpers: normalizeDomainHelpers,
                    prepareThreadDraft: prepareThreadDraft,
                    prepareVoiceCapturePayload: prepareVoiceCapturePayload,
                    packageVoiceQuickAction: packageVoiceQuickAction,
                    prepareVoiceDraft: prepareVoiceDraft,
                    prepareVoiceContinuityEntry: prepareVoiceContinuityEntry,
                    packageQueuedAction: packageQueuedAction,
                    normalizePairingToken: normalizePairingToken,
                    collectRemoteRoutes: collectRemoteRoutes,
                    prepareAssistantEntryFallback: prepareAssistantEntryFallback,
                    prepareLinkingRequest: prepareLinkingRequest,
                    prepareCaptureMetadata: prepareCaptureMetadata,
                    preparePairingTokenIssueRequest: preparePairingTokenIssueRequest,
                    preparePairingTokenRedeemRequest: preparePairingTokenRedeemRequest,
                    prepareVoiceContinuitySummary: prepareVoiceContinuitySummary,
                    prepareVoiceOfflineResponse: prepareVoiceOfflineResponse,
                    freeBuffer: freeBuffer
                ),
                status
            )
        }

        if let handle = dlopen(nil, flags) {
            attemptedPaths.append("main process")
            let primary = bindingsIfUsable(from: handle, source: "main process")
            if let primaryBindings = primary.bindings {
                return (primaryBindings, primary.status)
            }

            _ = dlclose(handle)
        }

        let candidates = resolveRustLibraryPaths()
        for candidate in candidates {
            attemptedPaths.append(candidate)
            guard let handle = dlopen(candidate, flags) else {
                continue
            }

            let discovered = bindingsIfUsable(from: handle, source: candidate)
            guard let rustBindings = discovered.bindings else {
                _ = dlclose(handle)
                continue
            }

            return (rustBindings, discovered.status)
        }

        return (
            nil,
            EmbeddedBridgeRuntimeStatus(
                resolvedSource: nil,
                attemptedPaths: attemptedPaths,
                freeBufferAvailable: false,
                cachedNowHydrationSymbolAvailable: false,
                localQuickActionPreparationSymbolAvailable: false,
                offlineRequestPackagingSymbolAvailable: false,
                deterministicDomainHelpersSymbolAvailable: false,
                localThreadDraftPackagingSymbolAvailable: false,
                localVoiceCapturePackagingSymbolAvailable: false,
                localVoiceQuickActionPackagingSymbolAvailable: false,
                localVoiceContinuityPackagingSymbolAvailable: false,
                localQueuedActionPackagingSymbolAvailable: false,
                localLinkingSettingsNormalizationSymbolAvailable: false,
                localAssistantEntryFallbackPackagingSymbolAvailable: false,
                localLinkingRequestPackagingSymbolAvailable: false,
                localCaptureMetadataPackagingSymbolAvailable: false,
                localVoiceContinuitySummaryPackagingSymbolAvailable: false,
                localVoiceOfflineResponsePackagingSymbolAvailable: false
            )
        )
    }()

    static var bindings: VelEmbeddedRustBindings? {
        resolution.bindings
    }

    static var runtimeStatus: EmbeddedBridgeRuntimeStatus {
        resolution.status
    }

    static func resolveRustLibraryPaths() -> [String] {
        var candidates = [
            "@rpath/libvel_embedded_bridge.dylib",
            "@rpath/VelEmbeddedBridge.framework/VelEmbeddedBridge",
            "libvel_embedded_bridge.dylib",
            "libvel_embedded_bridge.so",
            "/usr/lib/libvel_embedded_bridge.dylib"
        ]

        if let executableURL = Bundle.main.executableURL {
            let executableDirectory = executableURL.deletingLastPathComponent()
            candidates.append(executableDirectory.appendingPathComponent("libvel_embedded_bridge.dylib").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/libvel_embedded_bridge.dylib").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/vel_embedded_bridge.framework/vel_embedded_bridge").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/VelEmbeddedBridge.framework/VelEmbeddedBridge").path)
        }

        let bundlePath = Bundle.main.bundlePath
        let bundleURL = URL(fileURLWithPath: bundlePath)
        candidates.append(bundleURL.appendingPathComponent("libvel_embedded_bridge.dylib").path)
        candidates.append(bundleURL.appendingPathComponent("Frameworks/libvel_embedded_bridge.dylib").path)
        candidates.append(bundleURL.appendingPathComponent("Frameworks/VelEmbeddedBridge.framework/VelEmbeddedBridge").path)

        if let appSupport = NSSearchPathForDirectoriesInDomains(.applicationSupportDirectory, .userDomainMask, true).first {
            candidates.append((appSupport as NSString).appendingPathComponent("libvel_embedded_bridge.dylib"))
        }

        return candidates
    }

    static func lookup<T>(candidate: String, from handle: UnsafeMutableRawPointer) -> T? {
        guard let symbol = dlsym(handle, candidate) else { return nil }
        return unsafeBitCast(symbol, to: T.self)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedCachedNowSummaryFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareQuickCaptureFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPackageOfflineRequestFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedNormalizeDomainHelpersFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareThreadDraftFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareVoiceCapturePayloadFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPackageVoiceQuickActionFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareVoiceDraftFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareVoiceContinuityEntryFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPackageQueuedActionFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedNormalizePairingTokenFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedCollectRemoteRoutesFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareAssistantEntryFallbackFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareLinkingRequestFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareCaptureMetadataFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPreparePairingTokenIssueRequestFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPreparePairingTokenRedeemRequestFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareVoiceContinuitySummaryFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    @inline(__always)
    static func encodeContextPayload(_ context: VelContextSnapshot) -> String {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.sortedKeys]
        return (try? encoder.encode(context)).flatMap { String(data: $0, encoding: .utf8) } ?? "{}"
    }

    @inline(__always)
    static func splitSummary(_ value: String) -> [String] {
        guard let data = value.data(using: .utf8) else { return [] }
        return (try? JSONDecoder().decode([String].self, from: data)) ?? [value]
    }

    struct OfflineRequestPacket: Decodable {
        let kind: String
        let payload: String
        let ready: Bool
        let reason: String?
    }

    struct DomainHintPacket: Decodable {
        let normalized: String
        let kind: String?
        let ready: Bool?
    }

    struct ThreadDraftInput: Encodable {
        let text: String
        let requestedConversationID: String?

        enum CodingKeys: String, CodingKey {
            case text
            case requestedConversationID = "requestedConversationId"
        }
    }

    struct ThreadDraftPacket: Decodable {
        let payload: String
        let requestedConversationID: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case payload
            case requestedConversationID = "requestedConversationId"
            case ready
        }
    }

    struct VoiceCaptureInput: Encodable {
        let transcript: String
        let intentStorageToken: String

        enum CodingKeys: String, CodingKey {
            case transcript
            case intentStorageToken = "intentStorageToken"
        }
    }

    struct VoiceQuickActionInput: Encodable {
        let intentStorageToken: String
        let primaryText: String
        let targetID: String?
        let minutes: Int?

        enum CodingKeys: String, CodingKey {
            case intentStorageToken
            case primaryText
            case targetID = "targetId"
            case minutes
        }
    }

    struct VoiceQuickActionPacket: Decodable {
        let queueKind: String
        let targetID: String?
        let text: String?
        let minutes: Int?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case queueKind
            case targetID = "targetId"
            case text
            case minutes
            case ready
        }
    }

    struct VoiceDraftInput: Encodable {
        let transcript: String
        let suggestedIntentStorageToken: String
        let suggestedText: String

        enum CodingKeys: String, CodingKey {
            case transcript
            case suggestedIntentStorageToken = "suggestedIntentStorageToken"
            case suggestedText = "suggestedText"
        }
    }

    struct VoiceDraftPacket: Decodable {
        let transcript: String
        let suggestedIntentStorageToken: String
        let suggestedText: String
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case transcript
            case suggestedIntentStorageToken = "suggestedIntentStorageToken"
            case suggestedText = "suggestedText"
            case ready
        }
    }

    struct VoiceContinuityEntryInput: Encodable {
        let transcript: String
        let suggestedIntentStorageToken: String
        let committedIntentStorageToken: String?
        let status: String
        let threadID: String?

        enum CodingKeys: String, CodingKey {
            case transcript
            case suggestedIntentStorageToken = "suggestedIntentStorageToken"
            case committedIntentStorageToken = "committedIntentStorageToken"
            case status
            case threadID = "threadId"
        }
    }

    struct VoiceContinuityEntryPacket: Decodable {
        let transcript: String
        let suggestedIntentStorageToken: String
        let committedIntentStorageToken: String?
        let status: String
        let threadID: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case transcript
            case suggestedIntentStorageToken = "suggestedIntentStorageToken"
            case committedIntentStorageToken = "committedIntentStorageToken"
            case status
            case threadID = "threadId"
            case ready
        }
    }

    struct QueuedActionInput: Encodable {
        let kind: String
        let targetID: String?
        let text: String?
        let minutes: Int?

        enum CodingKeys: String, CodingKey {
            case kind
            case targetID = "targetId"
            case text
            case minutes
        }
    }

    struct QueuedActionPacket: Decodable {
        let queueKind: String
        let targetID: String?
        let text: String?
        let minutes: Int?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case queueKind
            case targetID = "targetId"
            case text
            case minutes
            case ready
        }
    }

    struct RemoteRoutesInput: Encodable {
        let syncBaseURL: String?
        let tailscaleBaseURL: String?
        let lanBaseURL: String?
        let publicBaseURL: String?

        enum CodingKeys: String, CodingKey {
            case syncBaseURL = "syncBaseUrl"
            case tailscaleBaseURL = "tailscaleBaseUrl"
            case lanBaseURL = "lanBaseUrl"
            case publicBaseURL = "publicBaseUrl"
        }
    }

    struct RemoteRoutePacket: Decodable {
        let label: String
        let baseURL: String

        enum CodingKeys: String, CodingKey {
            case label
            case baseURL = "baseUrl"
        }
    }

    struct AssistantEntryFallbackInput: Encodable {
        let text: String
        let requestedConversationID: String?

        enum CodingKeys: String, CodingKey {
            case text
            case requestedConversationID = "requestedConversationId"
        }
    }

    struct AssistantEntryFallbackPacket: Decodable {
        let payload: String
        let ready: Bool
    }

    struct LinkingRequestInput: Encodable {
        let tokenCode: String?
        let targetBaseURL: String?

        enum CodingKeys: String, CodingKey {
            case tokenCode = "tokenCode"
            case targetBaseURL = "targetBaseUrl"
        }
    }

    struct LinkingRequestPacket: Decodable {
        let tokenCode: String?
        let targetBaseURL: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case tokenCode = "tokenCode"
            case targetBaseURL = "targetBaseUrl"
            case ready
        }
    }

    struct CaptureMetadataInput: Encodable {
        let text: String
        let captureType: String
        let sourceDevice: String

        enum CodingKeys: String, CodingKey {
            case text
            case captureType = "captureType"
            case sourceDevice = "sourceDevice"
        }
    }

    struct CaptureMetadataPacket: Decodable {
        let payload: String
        let ready: Bool
    }

    struct PairingTokenIssueRequestInput: Encodable {
        let issuedByNodeID: String
        let targetNodeID: String?
        let targetNodeDisplayName: String?
        let targetBaseURL: String?

        enum CodingKeys: String, CodingKey {
            case issuedByNodeID = "issuedByNodeId"
            case targetNodeID = "targetNodeId"
            case targetNodeDisplayName = "targetNodeDisplayName"
            case targetBaseURL = "targetBaseUrl"
        }
    }

    struct PairingTokenIssueRequestPacket: Decodable {
        let issuedByNodeID: String
        let targetNodeID: String?
        let targetNodeDisplayName: String?
        let targetBaseURL: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case issuedByNodeID = "issuedByNodeId"
            case targetNodeID = "targetNodeId"
            case targetNodeDisplayName = "targetNodeDisplayName"
            case targetBaseURL = "targetBaseUrl"
            case ready
        }
    }

    struct PairingTokenRedeemRequestInput: Encodable {
        let tokenCode: String
        let nodeID: String
        let nodeDisplayName: String
        let transportHint: String?
        let syncBaseURL: String?
        let tailscaleBaseURL: String?
        let lanBaseURL: String?
        let localhostBaseURL: String?
        let publicBaseURL: String?

        enum CodingKeys: String, CodingKey {
            case tokenCode = "tokenCode"
            case nodeID = "nodeId"
            case nodeDisplayName = "nodeDisplayName"
            case transportHint = "transportHint"
            case syncBaseURL = "syncBaseUrl"
            case tailscaleBaseURL = "tailscaleBaseUrl"
            case lanBaseURL = "lanBaseUrl"
            case localhostBaseURL = "localhostBaseUrl"
            case publicBaseURL = "publicBaseUrl"
        }
    }

    struct PairingTokenRedeemRequestPacket: Decodable {
        let tokenCode: String
        let nodeID: String
        let nodeDisplayName: String
        let transportHint: String?
        let syncBaseURL: String?
        let tailscaleBaseURL: String?
        let lanBaseURL: String?
        let localhostBaseURL: String?
        let publicBaseURL: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case tokenCode = "tokenCode"
            case nodeID = "nodeId"
            case nodeDisplayName = "nodeDisplayName"
            case transportHint = "transportHint"
            case syncBaseURL = "syncBaseUrl"
            case tailscaleBaseURL = "tailscaleBaseUrl"
            case lanBaseURL = "lanBaseUrl"
            case localhostBaseURL = "localhostBaseUrl"
            case publicBaseURL = "publicBaseUrl"
            case ready
        }
    }

    struct VoiceContinuitySummaryInput: Encodable {
        let draftExists: Bool
        let threadedTranscript: String?
        let pendingRecoveryCount: Int
        let isReachable: Bool
        let mergedTranscript: String?

        enum CodingKeys: String, CodingKey {
            case draftExists = "draftExists"
            case threadedTranscript = "threadedTranscript"
            case pendingRecoveryCount = "pendingRecoveryCount"
            case isReachable = "isReachable"
            case mergedTranscript = "mergedTranscript"
        }
    }

    struct VoiceContinuitySummaryPacket: Decodable {
        let headline: String?
        let detail: String?
        let ready: Bool
    }

    struct VoiceOfflineResponseInput: Encodable {
        let scenario: String
        let primaryText: String?
        let matchedText: String?
        let options: String?
        let minutes: Int?
        let isReachable: Bool

        enum CodingKeys: String, CodingKey {
            case scenario
            case primaryText = "primaryText"
            case matchedText = "matchedText"
            case options
            case minutes
            case isReachable = "isReachable"
        }
    }

    struct VoiceOfflineResponsePacket: Decodable {
        let summary: String?
        let detail: String?
        let historyStatus: String
        let errorPrefix: String
        let ready: Bool
    }

    static func decodeOfflineRequest(_ value: String) -> OfflineRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(OfflineRequestPacket.self, from: data)
    }

    static func decodeDomainHint(_ value: String) -> DomainHintPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(DomainHintPacket.self, from: data)
    }

    static func encodeThreadDraftPayload(text: String, conversationID: String?) -> String {
        let payload = ThreadDraftInput(
            text: text,
            requestedConversationID: conversationID
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"text\":\"\"}"
        }
        return value
    }

    static func decodeThreadDraft(_ value: String) -> ThreadDraftPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(ThreadDraftPacket.self, from: data)
    }

    static func encodeVoiceCapturePayload(transcript: String, intentStorageToken: String) -> String {
        let payload = VoiceCaptureInput(
            transcript: transcript,
            intentStorageToken: intentStorageToken
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"transcript\":\"\"}"
        }
        return value
    }

    static func encodeVoiceQuickActionPayload(
        intentStorageToken: String,
        primaryText: String,
        targetID: String?,
        minutes: Int?
    ) -> String {
        let payload = VoiceQuickActionInput(
            intentStorageToken: intentStorageToken,
            primaryText: primaryText,
            targetID: targetID,
            minutes: minutes
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"intentStorageToken\":\"capture_create\"}"
        }
        return value
    }

    static func decodeVoiceQuickAction(_ value: String) -> VoiceQuickActionPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(VoiceQuickActionPacket.self, from: data)
    }

    static func encodeVoiceDraftPayload(
        transcript: String,
        suggestedIntentStorageToken: String,
        suggestedText: String
    ) -> String {
        let payload = VoiceDraftInput(
            transcript: transcript,
            suggestedIntentStorageToken: suggestedIntentStorageToken,
            suggestedText: suggestedText
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"transcript\":\"\"}"
        }
        return value
    }

    static func decodeVoiceDraft(_ value: String) -> VoiceDraftPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(VoiceDraftPacket.self, from: data)
    }

    static func encodeVoiceContinuityEntryPayload(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) -> String {
        let payload = VoiceContinuityEntryInput(
            transcript: transcript,
            suggestedIntentStorageToken: suggestedIntentStorageToken,
            committedIntentStorageToken: committedIntentStorageToken,
            status: status,
            threadID: threadID
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"transcript\":\"\"}"
        }
        return value
    }

    static func decodeVoiceContinuityEntry(_ value: String) -> VoiceContinuityEntryPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(VoiceContinuityEntryPacket.self, from: data)
    }

    static func encodeQueuedActionPayload(kind: String, targetID: String?, text: String?, minutes: Int?) -> String {
        let payload = QueuedActionInput(kind: kind, targetID: targetID, text: text, minutes: minutes)
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"kind\":\"capture.create\"}"
        }
        return value
    }

    static func decodeQueuedAction(_ value: String) -> QueuedActionPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(QueuedActionPacket.self, from: data)
    }

    static func encodeRemoteRoutesPayload(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> String {
        let payload = RemoteRoutesInput(
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            publicBaseURL: publicBaseURL
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodeRemoteRoutes(_ value: String) -> [RemoteRoutePacket]? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode([RemoteRoutePacket].self, from: data)
    }

    static func encodeAssistantEntryFallbackPayload(text: String, conversationID: String?) -> String {
        let payload = AssistantEntryFallbackInput(text: text, requestedConversationID: conversationID)
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"text\":\"\"}"
        }
        return value
    }

    static func decodeAssistantEntryFallback(_ value: String) -> AssistantEntryFallbackPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(AssistantEntryFallbackPacket.self, from: data)
    }

    static func encodeLinkingRequestPayload(tokenCode: String?, targetBaseURL: String?) -> String {
        let payload = LinkingRequestInput(tokenCode: tokenCode, targetBaseURL: targetBaseURL)
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodeLinkingRequest(_ value: String) -> LinkingRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(LinkingRequestPacket.self, from: data)
    }

    static func encodeCaptureMetadataPayload(text: String, type: String, source: String) -> String {
        let payload = CaptureMetadataInput(text: text, captureType: type, sourceDevice: source)
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"text\":\"\"}"
        }
        return value
    }

    static func decodeCaptureMetadata(_ value: String) -> CaptureMetadataPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(CaptureMetadataPacket.self, from: data)
    }

    static func encodePairingTokenIssueRequestPayload(
        issuedByNodeID: String,
        targetNodeID: String?,
        targetNodeDisplayName: String?,
        targetBaseURL: String?
    ) -> String {
        let payload = PairingTokenIssueRequestInput(
            issuedByNodeID: issuedByNodeID,
            targetNodeID: targetNodeID,
            targetNodeDisplayName: targetNodeDisplayName,
            targetBaseURL: targetBaseURL
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodePairingTokenIssueRequest(_ value: String) -> PairingTokenIssueRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(PairingTokenIssueRequestPacket.self, from: data)
    }

    static func encodePairingTokenRedeemRequestPayload(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) -> String {
        let payload = PairingTokenRedeemRequestInput(
            tokenCode: tokenCode,
            nodeID: nodeID,
            nodeDisplayName: nodeDisplayName,
            transportHint: transportHint,
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            localhostBaseURL: localhostBaseURL,
            publicBaseURL: publicBaseURL
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodePairingTokenRedeemRequest(_ value: String) -> PairingTokenRedeemRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(PairingTokenRedeemRequestPacket.self, from: data)
    }

    static func encodeVoiceContinuitySummaryPayload(
        draftExists: Bool,
        threadedTranscript: String?,
        pendingRecoveryCount: Int,
        isReachable: Bool,
        mergedTranscript: String?
    ) -> String {
        let payload = VoiceContinuitySummaryInput(
            draftExists: draftExists,
            threadedTranscript: threadedTranscript,
            pendingRecoveryCount: pendingRecoveryCount,
            isReachable: isReachable,
            mergedTranscript: mergedTranscript
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodeVoiceContinuitySummary(_ value: String) -> VoiceContinuitySummaryPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(VoiceContinuitySummaryPacket.self, from: data)
    }

    static func encodeVoiceOfflineResponsePayload(
        scenario: String,
        primaryText: String?,
        matchedText: String?,
        options: String?,
        minutes: Int?,
        isReachable: Bool
    ) -> String {
        let payload = VoiceOfflineResponseInput(
            scenario: scenario,
            primaryText: primaryText,
            matchedText: matchedText,
            options: options,
            minutes: minutes,
            isReachable: isReachable
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{}"
        }
        return value
    }

    static func decodeVoiceOfflineResponse(_ value: String) -> VoiceOfflineResponsePacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(VoiceOfflineResponsePacket.self, from: data)
    }
}

public struct RustEmbeddedNowBridge: EmbeddedNowBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.cachedNowSummary != nil else { return nil }
        self.bindings = bindings
    }

    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        guard let response = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.cachedNowSummary,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeContextPayload(context)
        ) else {
            return []
        }
        return VelEmbeddedRustBridge.splitSummary(response)
    }
}

public struct RustEmbeddedQuickActionBridge: EmbeddedQuickActionBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareQuickCapture != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareQuickCapture(_ text: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareQuickCapture,
            freeBuffer: bindings.freeBuffer,
            payload: text
        ) else {
            return text
        }
        return output
    }
}

public struct RustEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.packageOfflineRequest != nil else { return nil }
        self.bindings = bindings
    }

    public func packageOfflineRequest(_ payload: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.packageOfflineRequest,
            freeBuffer: bindings.freeBuffer,
            payload: payload
        ) else {
            return payload
        }

        guard let parsed = VelEmbeddedRustBridge.decodeOfflineRequest(output),
              parsed.ready else {
            return output
        }

        return parsed.payload
    }
}

public struct RustEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.normalizeDomainHelpers != nil else { return nil }
        self.bindings = bindings
    }

    public func normalizeDomainHint(_ input: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.normalizeDomainHelpers,
            freeBuffer: bindings.freeBuffer,
            payload: input
        ) else {
            return input
        }

        guard let parsed = VelEmbeddedRustBridge.decodeDomainHint(output),
              parsed.ready ?? false else {
            return input
        }

        return parsed.normalized
    }
}

public struct RustEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareThreadDraft != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareThreadDraft,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeThreadDraftPayload(text: text, conversationID: conversationID)
        ),
        let parsed = VelEmbeddedRustBridge.decodeThreadDraft(output),
        parsed.ready else {
            return NoopEmbeddedThreadDraftBridge().prepareThreadDraft(text, conversationID: conversationID)
        }

        return EmbeddedThreadDraftPacket(
            payload: parsed.payload,
            requestedConversationID: parsed.requestedConversationID
        )
    }
}

public struct RustEmbeddedVoiceCaptureBridge: EmbeddedVoiceCaptureBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareVoiceCapturePayload != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareVoiceCapturePayload(transcript: String, intentStorageToken: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareVoiceCapturePayload,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceCapturePayload(
                transcript: transcript,
                intentStorageToken: intentStorageToken
            )
        ) else {
            return NoopEmbeddedVoiceCaptureBridge().prepareVoiceCapturePayload(
                transcript: transcript,
                intentStorageToken: intentStorageToken
            )
        }

        return output
    }
}

public struct RustEmbeddedVoiceQuickActionBridge: EmbeddedVoiceQuickActionBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.packageVoiceQuickAction != nil else { return nil }
        self.bindings = bindings
    }

    public func packageVoiceQuickAction(
        intentStorageToken: String,
        primaryText: String,
        targetID: String?,
        minutes: Int?
    ) -> EmbeddedVoiceQuickActionPacket? {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.packageVoiceQuickAction,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceQuickActionPayload(
                intentStorageToken: intentStorageToken,
                primaryText: primaryText,
                targetID: targetID,
                minutes: minutes
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeVoiceQuickAction(output),
        parsed.ready else {
            return NoopEmbeddedVoiceQuickActionBridge().packageVoiceQuickAction(
                intentStorageToken: intentStorageToken,
                primaryText: primaryText,
                targetID: targetID,
                minutes: minutes
            )
        }

        return EmbeddedVoiceQuickActionPacket(
            queueKind: parsed.queueKind,
            targetID: parsed.targetID,
            text: parsed.text,
            minutes: parsed.minutes
        )
    }
}

public struct RustEmbeddedVoiceContinuityBridge: EmbeddedVoiceContinuityBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareVoiceDraft != nil,
              bindings.prepareVoiceContinuityEntry != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareVoiceDraft(
        transcript: String,
        suggestedIntentStorageToken: String,
        suggestedText: String
    ) -> EmbeddedVoiceDraftPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareVoiceDraft,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceDraftPayload(
                transcript: transcript,
                suggestedIntentStorageToken: suggestedIntentStorageToken,
                suggestedText: suggestedText
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeVoiceDraft(output),
        parsed.ready else {
            return NoopEmbeddedVoiceContinuityBridge().prepareVoiceDraft(
                transcript: transcript,
                suggestedIntentStorageToken: suggestedIntentStorageToken,
                suggestedText: suggestedText
            )
        }

        return EmbeddedVoiceDraftPacket(
            transcript: parsed.transcript,
            suggestedIntentStorageToken: parsed.suggestedIntentStorageToken,
            suggestedText: parsed.suggestedText
        )
    }

    public func prepareVoiceContinuityEntry(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) -> EmbeddedVoiceContinuityEntryPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareVoiceContinuityEntry,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceContinuityEntryPayload(
                transcript: transcript,
                suggestedIntentStorageToken: suggestedIntentStorageToken,
                committedIntentStorageToken: committedIntentStorageToken,
                status: status,
                threadID: threadID
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeVoiceContinuityEntry(output),
        parsed.ready else {
            return NoopEmbeddedVoiceContinuityBridge().prepareVoiceContinuityEntry(
                transcript: transcript,
                suggestedIntentStorageToken: suggestedIntentStorageToken,
                committedIntentStorageToken: committedIntentStorageToken,
                status: status,
                threadID: threadID
            )
        }

        return EmbeddedVoiceContinuityEntryPacket(
            transcript: parsed.transcript,
            suggestedIntentStorageToken: parsed.suggestedIntentStorageToken,
            committedIntentStorageToken: parsed.committedIntentStorageToken,
            status: parsed.status,
            threadID: parsed.threadID
        )
    }
}

public struct RustEmbeddedQueuedActionBridge: EmbeddedQueuedActionBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.packageQueuedAction != nil else { return nil }
        self.bindings = bindings
    }

    public func packageQueuedAction(
        kind: String,
        targetID: String?,
        text: String?,
        minutes: Int?
    ) -> EmbeddedQueuedActionPacket? {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.packageQueuedAction,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeQueuedActionPayload(
                kind: kind,
                targetID: targetID,
                text: text,
                minutes: minutes
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeQueuedAction(output),
        parsed.ready else {
            return NoopEmbeddedQueuedActionBridge().packageQueuedAction(
                kind: kind,
                targetID: targetID,
                text: text,
                minutes: minutes
            )
        }

        return EmbeddedQueuedActionPacket(
            queueKind: parsed.queueKind,
            targetID: parsed.targetID,
            text: parsed.text,
            minutes: parsed.minutes
        )
    }
}

public struct RustEmbeddedLinkingSettingsBridge: EmbeddedLinkingSettingsBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.normalizePairingToken != nil,
              bindings.collectRemoteRoutes != nil else { return nil }
        self.bindings = bindings
    }

    public func normalizePairingTokenInput(_ value: String) -> String {
        VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.normalizePairingToken,
            freeBuffer: bindings.freeBuffer,
            payload: value
        ) ?? NoopEmbeddedLinkingSettingsBridge().normalizePairingTokenInput(value)
    }

    public func collectRemoteRoutes(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> [EmbeddedRemoteRouteSummary] {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.collectRemoteRoutes,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeRemoteRoutesPayload(
                syncBaseURL: syncBaseURL,
                tailscaleBaseURL: tailscaleBaseURL,
                lanBaseURL: lanBaseURL,
                publicBaseURL: publicBaseURL
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeRemoteRoutes(output) else {
            return NoopEmbeddedLinkingSettingsBridge().collectRemoteRoutes(
                syncBaseURL: syncBaseURL,
                tailscaleBaseURL: tailscaleBaseURL,
                lanBaseURL: lanBaseURL,
                publicBaseURL: publicBaseURL
            )
        }

        return parsed.map { EmbeddedRemoteRouteSummary(label: $0.label, baseURL: $0.baseURL) }
    }
}

public struct RustEmbeddedAssistantEntryFallbackBridge: EmbeddedAssistantEntryFallbackBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareAssistantEntryFallback != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareAssistantEntryFallback(
        text: String,
        conversationID: String?
    ) -> EmbeddedAssistantEntryFallbackPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareAssistantEntryFallback,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeAssistantEntryFallbackPayload(
                text: text,
                conversationID: conversationID
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeAssistantEntryFallback(output),
        parsed.ready else {
            return NoopEmbeddedAssistantEntryFallbackBridge().prepareAssistantEntryFallback(
                text: text,
                conversationID: conversationID
            )
        }

        return EmbeddedAssistantEntryFallbackPacket(payload: parsed.payload)
    }
}

public struct RustEmbeddedLinkingRequestBridge: EmbeddedLinkingRequestBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareLinkingRequest != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareLinkingRequest(tokenCode: String?, targetBaseURL: String?) -> EmbeddedLinkingRequestPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareLinkingRequest,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeLinkingRequestPayload(
                tokenCode: tokenCode,
                targetBaseURL: targetBaseURL
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeLinkingRequest(output),
        parsed.ready else {
            return NoopEmbeddedLinkingRequestBridge().prepareLinkingRequest(
                tokenCode: tokenCode,
                targetBaseURL: targetBaseURL
            )
        }

        return EmbeddedLinkingRequestPacket(
            tokenCode: parsed.tokenCode,
            targetBaseURL: parsed.targetBaseURL
        )
    }

    public func preparePairingTokenIssueRequest(
        issuedByNodeID: String,
        targetNodeID: String?,
        targetNodeDisplayName: String?,
        targetBaseURL: String?
    ) -> EmbeddedPairingTokenIssueRequestPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.preparePairingTokenIssueRequest,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodePairingTokenIssueRequestPayload(
                issuedByNodeID: issuedByNodeID,
                targetNodeID: targetNodeID,
                targetNodeDisplayName: targetNodeDisplayName,
                targetBaseURL: targetBaseURL
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodePairingTokenIssueRequest(output),
        parsed.ready else {
            return NoopEmbeddedLinkingRequestBridge().preparePairingTokenIssueRequest(
                issuedByNodeID: issuedByNodeID,
                targetNodeID: targetNodeID,
                targetNodeDisplayName: targetNodeDisplayName,
                targetBaseURL: targetBaseURL
            )
        }

        return EmbeddedPairingTokenIssueRequestPacket(
            issuedByNodeID: parsed.issuedByNodeID,
            targetNodeID: parsed.targetNodeID,
            targetNodeDisplayName: parsed.targetNodeDisplayName,
            targetBaseURL: parsed.targetBaseURL
        )
    }

    public func preparePairingTokenRedeemRequest(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) -> EmbeddedPairingTokenRedeemRequestPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.preparePairingTokenRedeemRequest,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodePairingTokenRedeemRequestPayload(
                tokenCode: tokenCode,
                nodeID: nodeID,
                nodeDisplayName: nodeDisplayName,
                transportHint: transportHint,
                syncBaseURL: syncBaseURL,
                tailscaleBaseURL: tailscaleBaseURL,
                lanBaseURL: lanBaseURL,
                localhostBaseURL: localhostBaseURL,
                publicBaseURL: publicBaseURL
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodePairingTokenRedeemRequest(output),
        parsed.ready else {
            return NoopEmbeddedLinkingRequestBridge().preparePairingTokenRedeemRequest(
                tokenCode: tokenCode,
                nodeID: nodeID,
                nodeDisplayName: nodeDisplayName,
                transportHint: transportHint,
                syncBaseURL: syncBaseURL,
                tailscaleBaseURL: tailscaleBaseURL,
                lanBaseURL: lanBaseURL,
                localhostBaseURL: localhostBaseURL,
                publicBaseURL: publicBaseURL
            )
        }

        return EmbeddedPairingTokenRedeemRequestPacket(
            tokenCode: parsed.tokenCode,
            nodeID: parsed.nodeID,
            nodeDisplayName: parsed.nodeDisplayName,
            transportHint: parsed.transportHint,
            syncBaseURL: parsed.syncBaseURL,
            tailscaleBaseURL: parsed.tailscaleBaseURL,
            lanBaseURL: parsed.lanBaseURL,
            localhostBaseURL: parsed.localhostBaseURL,
            publicBaseURL: parsed.publicBaseURL
        )
    }
}

public struct RustEmbeddedCaptureMetadataBridge: EmbeddedCaptureMetadataBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareCaptureMetadata != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareQueuedCaptureText(text: String, type: String, source: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareCaptureMetadata,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeCaptureMetadataPayload(text: text, type: type, source: source)
        ),
        let parsed = VelEmbeddedRustBridge.decodeCaptureMetadata(output),
        parsed.ready else {
            return NoopEmbeddedCaptureMetadataBridge().prepareQueuedCaptureText(text: text, type: type, source: source)
        }

        return parsed.payload
    }
}

public struct RustEmbeddedVoiceContinuitySummaryBridge: EmbeddedVoiceContinuitySummaryBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareVoiceContinuitySummary != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareVoiceContinuitySummary(
        draftExists: Bool,
        threadedTranscript: String?,
        pendingRecoveryCount: Int,
        isReachable: Bool,
        mergedTranscript: String?
    ) -> EmbeddedVoiceContinuitySummaryPacket? {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareVoiceContinuitySummary,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceContinuitySummaryPayload(
                draftExists: draftExists,
                threadedTranscript: threadedTranscript,
                pendingRecoveryCount: pendingRecoveryCount,
                isReachable: isReachable,
                mergedTranscript: mergedTranscript
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeVoiceContinuitySummary(output),
        parsed.ready,
        let headline = parsed.headline else {
            return NoopEmbeddedVoiceContinuitySummaryBridge().prepareVoiceContinuitySummary(
                draftExists: draftExists,
                threadedTranscript: threadedTranscript,
                pendingRecoveryCount: pendingRecoveryCount,
                isReachable: isReachable,
                mergedTranscript: mergedTranscript
            )
        }

        return EmbeddedVoiceContinuitySummaryPacket(
            headline: headline,
            detail: parsed.detail
        )
    }
}

public struct RustEmbeddedVoiceOfflineResponseBridge: EmbeddedVoiceOfflineResponseBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareVoiceOfflineResponse != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareVoiceOfflineResponse(
        scenario: String,
        primaryText: String?,
        matchedText: String?,
        options: String?,
        minutes: Int?,
        isReachable: Bool
    ) -> EmbeddedVoiceOfflineResponsePacket? {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareVoiceOfflineResponse,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeVoiceOfflineResponsePayload(
                scenario: scenario,
                primaryText: primaryText,
                matchedText: matchedText,
                options: options,
                minutes: minutes,
                isReachable: isReachable
            )
        ),
        let parsed = VelEmbeddedRustBridge.decodeVoiceOfflineResponse(output),
        parsed.ready,
        let summary = parsed.summary else {
            return NoopEmbeddedVoiceOfflineResponseBridge().prepareVoiceOfflineResponse(
                scenario: scenario,
                primaryText: primaryText,
                matchedText: matchedText,
                options: options,
                minutes: minutes,
                isReachable: isReachable
            )
        }

        return EmbeddedVoiceOfflineResponsePacket(
            summary: summary,
            detail: parsed.detail,
            historyStatus: parsed.historyStatus,
            errorPrefix: parsed.errorPrefix
        )
    }
}

public struct RustEmbeddedAssistantEntryFallbackBridge: EmbeddedAssistantEntryFallbackBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareAssistantEntryFallback(
        text: String,
        conversationID: String?
    ) -> EmbeddedAssistantEntryFallbackPacket {
        NoopEmbeddedAssistantEntryFallbackBridge().prepareAssistantEntryFallback(
            text: text,
            conversationID: conversationID
        )
    }
}

public struct RustEmbeddedLinkingRequestBridge: EmbeddedLinkingRequestBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareLinkingRequest(tokenCode: String?, targetBaseURL: String?) -> EmbeddedLinkingRequestPacket {
        NoopEmbeddedLinkingRequestBridge().prepareLinkingRequest(
            tokenCode: tokenCode,
            targetBaseURL: targetBaseURL
        )
    }
    public func preparePairingTokenIssueRequest(
        issuedByNodeID: String,
        targetNodeID: String?,
        targetNodeDisplayName: String?,
        targetBaseURL: String?
    ) -> EmbeddedPairingTokenIssueRequestPacket {
        NoopEmbeddedLinkingRequestBridge().preparePairingTokenIssueRequest(
            issuedByNodeID: issuedByNodeID,
            targetNodeID: targetNodeID,
            targetNodeDisplayName: targetNodeDisplayName,
            targetBaseURL: targetBaseURL
        )
    }
    public func preparePairingTokenRedeemRequest(
        tokenCode: String,
        nodeID: String,
        nodeDisplayName: String,
        transportHint: String?,
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        localhostBaseURL: String?,
        publicBaseURL: String?
    ) -> EmbeddedPairingTokenRedeemRequestPacket {
        NoopEmbeddedLinkingRequestBridge().preparePairingTokenRedeemRequest(
            tokenCode: tokenCode,
            nodeID: nodeID,
            nodeDisplayName: nodeDisplayName,
            transportHint: transportHint,
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            localhostBaseURL: localhostBaseURL,
            publicBaseURL: publicBaseURL
        )
    }
}

public struct RustEmbeddedCaptureMetadataBridge: EmbeddedCaptureMetadataBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareQueuedCaptureText(text: String, type: String, source: String) -> String {
        NoopEmbeddedCaptureMetadataBridge().prepareQueuedCaptureText(text: text, type: type, source: source)
    }
}

public struct RustEmbeddedVoiceContinuitySummaryBridge: EmbeddedVoiceContinuitySummaryBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareVoiceContinuitySummary(
        draftExists: Bool,
        threadedTranscript: String?,
        pendingRecoveryCount: Int,
        isReachable: Bool,
        mergedTranscript: String?
    ) -> EmbeddedVoiceContinuitySummaryPacket? {
        NoopEmbeddedVoiceContinuitySummaryBridge().prepareVoiceContinuitySummary(
            draftExists: draftExists,
            threadedTranscript: threadedTranscript,
            pendingRecoveryCount: pendingRecoveryCount,
            isReachable: isReachable,
            mergedTranscript: mergedTranscript
        )
    }
}

public struct RustEmbeddedVoiceOfflineResponseBridge: EmbeddedVoiceOfflineResponseBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareVoiceOfflineResponse(
        scenario: String,
        primaryText: String?,
        matchedText: String?,
        options: String?,
        minutes: Int?,
        isReachable: Bool
    ) -> EmbeddedVoiceOfflineResponsePacket? {
        NoopEmbeddedVoiceOfflineResponseBridge().prepareVoiceOfflineResponse(
            scenario: scenario,
            primaryText: primaryText,
            matchedText: matchedText,
            options: options,
            minutes: minutes,
            isReachable: isReachable
        )
    }
}

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge
    public let voiceCaptureBridge: any EmbeddedVoiceCaptureBridge
    public let voiceQuickActionBridge: any EmbeddedVoiceQuickActionBridge
    public let voiceContinuityBridge: any EmbeddedVoiceContinuityBridge
    public let queuedActionBridge: any EmbeddedQueuedActionBridge
    public let linkingSettingsBridge: any EmbeddedLinkingSettingsBridge
    public let assistantEntryFallbackBridge: any EmbeddedAssistantEntryFallbackBridge
    public let linkingRequestBridge: any EmbeddedLinkingRequestBridge
    public let captureMetadataBridge: any EmbeddedCaptureMetadataBridge
    public let voiceContinuitySummaryBridge: any EmbeddedVoiceContinuitySummaryBridge
    public let voiceOfflineResponseBridge: any EmbeddedVoiceOfflineResponseBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        guard let bindings = VelEmbeddedRustBridge.bindings else {
            return nil
        }

        self.configuration = configuration
        self.runtimeStatus = VelEmbeddedRustBridge.runtimeStatus

        if let rustNow = RustEmbeddedNowBridge(bindings: bindings), configuration.permits(.cachedNowHydration) {
            self.nowBridge = rustNow
        } else {
            self.nowBridge = NoopEmbeddedNowBridge()
        }

        if let rustQuick = RustEmbeddedQuickActionBridge(bindings: bindings), configuration.permits(.localQuickActionPreparation) {
            self.quickActionBridge = rustQuick
        } else {
            self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        }

        if let rustOffline = RustEmbeddedOfflineRequestBridge(bindings: bindings), configuration.permits(.offlineRequestPackaging) {
            self.offlineRequestBridge = rustOffline
        } else {
            self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        }

        if let rustDomain = RustEmbeddedDomainHelpersBridge(bindings: bindings), configuration.permits(.deterministicDomainHelpers) {
            self.domainHelpersBridge = rustDomain
        } else {
            self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        }

        if let rustThreadDraft = RustEmbeddedThreadDraftBridge(bindings: bindings), configuration.permits(.localThreadDraftPackaging) {
            self.threadDraftBridge = rustThreadDraft
        } else {
            self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
        }

        if let rustVoiceCapture = RustEmbeddedVoiceCaptureBridge(bindings: bindings), configuration.permits(.localVoiceCapturePackaging) {
            self.voiceCaptureBridge = rustVoiceCapture
        } else {
            self.voiceCaptureBridge = NoopEmbeddedVoiceCaptureBridge()
        }

        if let rustVoiceQuickAction = RustEmbeddedVoiceQuickActionBridge(bindings: bindings), configuration.permits(.localVoiceQuickActionPackaging) {
            self.voiceQuickActionBridge = rustVoiceQuickAction
        } else {
            self.voiceQuickActionBridge = NoopEmbeddedVoiceQuickActionBridge()
        }

        if let rustVoiceContinuity = RustEmbeddedVoiceContinuityBridge(bindings: bindings), configuration.permits(.localVoiceContinuityPackaging) {
            self.voiceContinuityBridge = rustVoiceContinuity
        } else {
            self.voiceContinuityBridge = NoopEmbeddedVoiceContinuityBridge()
        }

        if let rustQueuedAction = RustEmbeddedQueuedActionBridge(bindings: bindings), configuration.permits(.localQueuedActionPackaging) {
            self.queuedActionBridge = rustQueuedAction
        } else {
            self.queuedActionBridge = NoopEmbeddedQueuedActionBridge()
        }

        if let rustLinkingSettings = RustEmbeddedLinkingSettingsBridge(bindings: bindings), configuration.permits(.localLinkingSettingsNormalization) {
            self.linkingSettingsBridge = rustLinkingSettings
        } else {
            self.linkingSettingsBridge = NoopEmbeddedLinkingSettingsBridge()
        }

        if let rustAssistantEntryFallback = RustEmbeddedAssistantEntryFallbackBridge(bindings: bindings), configuration.permits(.localAssistantEntryFallbackPackaging) {
            self.assistantEntryFallbackBridge = rustAssistantEntryFallback
        } else {
            self.assistantEntryFallbackBridge = NoopEmbeddedAssistantEntryFallbackBridge()
        }

        if let rustLinkingRequest = RustEmbeddedLinkingRequestBridge(bindings: bindings), configuration.permits(.localLinkingRequestPackaging) {
            self.linkingRequestBridge = rustLinkingRequest
        } else {
            self.linkingRequestBridge = NoopEmbeddedLinkingRequestBridge()
        }

        if let rustCaptureMetadata = RustEmbeddedCaptureMetadataBridge(bindings: bindings), configuration.permits(.localCaptureMetadataPackaging) {
            self.captureMetadataBridge = rustCaptureMetadata
        } else {
            self.captureMetadataBridge = NoopEmbeddedCaptureMetadataBridge()
        }

        if let rustVoiceContinuitySummary = RustEmbeddedVoiceContinuitySummaryBridge(bindings: bindings), configuration.permits(.localVoiceContinuitySummaryPackaging) {
            self.voiceContinuitySummaryBridge = rustVoiceContinuitySummary
        } else {
            self.voiceContinuitySummaryBridge = NoopEmbeddedVoiceContinuitySummaryBridge()
        }

        if let rustVoiceOfflineResponse = RustEmbeddedVoiceOfflineResponseBridge(bindings: bindings), configuration.permits(.localVoiceOfflineResponsePackaging) {
            self.voiceOfflineResponseBridge = rustVoiceOfflineResponse
        } else {
            self.voiceOfflineResponseBridge = NoopEmbeddedVoiceOfflineResponseBridge()
        }

        let isEmbedded = configuration.permits(.cachedNowHydration)
            || configuration.permits(.localQuickActionPreparation)
            || configuration.permits(.offlineRequestPackaging)
            || configuration.permits(.deterministicDomainHelpers)
            || configuration.permits(.localThreadDraftPackaging)
            || configuration.permits(.localVoiceCapturePackaging)
            || configuration.permits(.localVoiceQuickActionPackaging)
            || configuration.permits(.localVoiceContinuityPackaging)
            || configuration.permits(.localQueuedActionPackaging)
            || configuration.permits(.localLinkingSettingsNormalization)
            || configuration.permits(.localAssistantEntryFallbackPackaging)
            || configuration.permits(.localLinkingRequestPackaging)
            || configuration.permits(.localCaptureMetadataPackaging)
            || configuration.permits(.localVoiceContinuitySummaryPackaging)
            || configuration.permits(.localVoiceOfflineResponsePackaging)

        guard isEmbedded else { return nil }
    }
}
#else
public struct RustEmbeddedNowBridge: EmbeddedNowBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        []
    }
}

public struct RustEmbeddedQuickActionBridge: EmbeddedQuickActionBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareQuickCapture(_ text: String) -> String {
        text
    }
}

public struct RustEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func packageOfflineRequest(_ payload: String) -> String {
        guard let data = payload.data(using: .utf8),
              let envelope = try? JSONDecoder().decode(OfflineBridgeEnvelope.self, from: data),
              let envelopePayload = envelope.payload else {
            return payload
        }
        return envelopePayload
    }
}

public struct RustEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func normalizeDomainHint(_ input: String) -> String {
        input
    }
}

public struct RustEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        NoopEmbeddedThreadDraftBridge().prepareThreadDraft(text, conversationID: conversationID)
    }
}

public struct RustEmbeddedVoiceCaptureBridge: EmbeddedVoiceCaptureBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareVoiceCapturePayload(transcript: String, intentStorageToken: String) -> String {
        NoopEmbeddedVoiceCaptureBridge().prepareVoiceCapturePayload(
            transcript: transcript,
            intentStorageToken: intentStorageToken
        )
    }
}

public struct RustEmbeddedVoiceQuickActionBridge: EmbeddedVoiceQuickActionBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func packageVoiceQuickAction(
        intentStorageToken: String,
        primaryText: String,
        targetID: String?,
        minutes: Int?
    ) -> EmbeddedVoiceQuickActionPacket? {
        NoopEmbeddedVoiceQuickActionBridge().packageVoiceQuickAction(
            intentStorageToken: intentStorageToken,
            primaryText: primaryText,
            targetID: targetID,
            minutes: minutes
        )
    }
}

public struct RustEmbeddedVoiceContinuityBridge: EmbeddedVoiceContinuityBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareVoiceDraft(
        transcript: String,
        suggestedIntentStorageToken: String,
        suggestedText: String
    ) -> EmbeddedVoiceDraftPacket {
        NoopEmbeddedVoiceContinuityBridge().prepareVoiceDraft(
            transcript: transcript,
            suggestedIntentStorageToken: suggestedIntentStorageToken,
            suggestedText: suggestedText
        )
    }

    public func prepareVoiceContinuityEntry(
        transcript: String,
        suggestedIntentStorageToken: String,
        committedIntentStorageToken: String?,
        status: String,
        threadID: String?
    ) -> EmbeddedVoiceContinuityEntryPacket {
        NoopEmbeddedVoiceContinuityBridge().prepareVoiceContinuityEntry(
            transcript: transcript,
            suggestedIntentStorageToken: suggestedIntentStorageToken,
            committedIntentStorageToken: committedIntentStorageToken,
            status: status,
            threadID: threadID
        )
    }
}

public struct RustEmbeddedQueuedActionBridge: EmbeddedQueuedActionBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func packageQueuedAction(
        kind: String,
        targetID: String?,
        text: String?,
        minutes: Int?
    ) -> EmbeddedQueuedActionPacket? {
        NoopEmbeddedQueuedActionBridge().packageQueuedAction(
            kind: kind,
            targetID: targetID,
            text: text,
            minutes: minutes
        )
    }
}

public struct RustEmbeddedLinkingSettingsBridge: EmbeddedLinkingSettingsBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func normalizePairingTokenInput(_ value: String) -> String {
        NoopEmbeddedLinkingSettingsBridge().normalizePairingTokenInput(value)
    }
    public func collectRemoteRoutes(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> [EmbeddedRemoteRouteSummary] {
        NoopEmbeddedLinkingSettingsBridge().collectRemoteRoutes(
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            publicBaseURL: publicBaseURL
        )
    }
}

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge
    public let voiceCaptureBridge: any EmbeddedVoiceCaptureBridge
    public let voiceQuickActionBridge: any EmbeddedVoiceQuickActionBridge
    public let voiceContinuityBridge: any EmbeddedVoiceContinuityBridge
    public let queuedActionBridge: any EmbeddedQueuedActionBridge
    public let linkingSettingsBridge: any EmbeddedLinkingSettingsBridge
    public let assistantEntryFallbackBridge: any EmbeddedAssistantEntryFallbackBridge
    public let linkingRequestBridge: any EmbeddedLinkingRequestBridge
    public let captureMetadataBridge: any EmbeddedCaptureMetadataBridge
    public let voiceContinuitySummaryBridge: any EmbeddedVoiceContinuitySummaryBridge
    public let voiceOfflineResponseBridge: any EmbeddedVoiceOfflineResponseBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        self.configuration = configuration
        self.runtimeStatus = .unavailable
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
        self.voiceCaptureBridge = NoopEmbeddedVoiceCaptureBridge()
        self.voiceQuickActionBridge = NoopEmbeddedVoiceQuickActionBridge()
        self.voiceContinuityBridge = NoopEmbeddedVoiceContinuityBridge()
        self.queuedActionBridge = NoopEmbeddedQueuedActionBridge()
        self.linkingSettingsBridge = NoopEmbeddedLinkingSettingsBridge()
        self.assistantEntryFallbackBridge = NoopEmbeddedAssistantEntryFallbackBridge()
        self.linkingRequestBridge = NoopEmbeddedLinkingRequestBridge()
        self.captureMetadataBridge = NoopEmbeddedCaptureMetadataBridge()
        self.voiceContinuitySummaryBridge = NoopEmbeddedVoiceContinuitySummaryBridge()
        self.voiceOfflineResponseBridge = NoopEmbeddedVoiceOfflineResponseBridge()
        return nil
    }
}
#endif
