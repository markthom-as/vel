import Foundation
import VelApplePlatform
import VelEmbeddedBridge
import VelDomain
import VelFeatureFlags
import VelInfrastructure

public struct SessionState: Sendable {
    public var isAuthenticated: Bool
    public var lastRefresh: Date?

    public init(isAuthenticated: Bool = false, lastRefresh: Date? = nil) {
        self.isAuthenticated = isAuthenticated
        self.lastRefresh = lastRefresh
    }
}

public protocol BriefingService: Sendable {
    func buildDailyBrief(context: VelContextSnapshot) async throws -> [String]
}

public protocol PlanningService: Sendable {
    func nextTasks(from tasks: [VelTask]) async -> [VelTask]
}

public protocol CaptureService: Sendable {
    func createQuickCapture(text: String) async throws
}

public struct PlaceholderBriefingService: BriefingService {
    public init() {}
    public func buildDailyBrief(context: VelContextSnapshot) async throws -> [String] {
        [
            "Mode: \(context.mode ?? "unknown")",
            "Next: \(context.nextEventTitle ?? "none")",
            "Nudges: \(context.nudgeCount)"
        ]
    }
}

public struct PlaceholderPlanningService: PlanningService {
    public init() {}
    public func nextTasks(from tasks: [VelTask]) async -> [VelTask] {
        tasks.filter { !$0.isDone }.prefix(3).map { $0 }
    }
}

public struct PlaceholderCaptureService: CaptureService {
    public init() {}
    public func createQuickCapture(text: String) async throws {}
}

public protocol WatchSurfaceActionClient: Sendable {
    func nudgeDone(id: String) async throws
    func nudgeSnooze(id: String, minutes: Int) async throws
    func submitAssistantEntry(text: String, conversationID: String) async throws
    func createCapture(text: String, type: String, source: String) async throws
}

public enum WatchSurfaceActionOutcome: Equatable, Sendable {
    case nudgeResolved(id: String)
    case nudgeSnoozed(id: String, minutes: Int)
    case threadAppended(conversationID: String)
    case captureCreated(type: String, source: String)
}

public struct WatchSurfaceActionService<Client: WatchSurfaceActionClient>: Sendable {
    private let client: Client

    public init(client: Client) {
        self.client = client
    }

    @discardableResult
    public func markNudgeDone(id: String) async throws -> WatchSurfaceActionOutcome {
        let cleanID = id.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !cleanID.isEmpty else { throw WatchSurfaceActionError.emptyNudgeID }

        try await client.nudgeDone(id: cleanID)
        return .nudgeResolved(id: cleanID)
    }

    @discardableResult
    public func snoozeNudge(id: String, minutes: Int = 10) async throws -> WatchSurfaceActionOutcome {
        let cleanID = id.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !cleanID.isEmpty else { throw WatchSurfaceActionError.emptyNudgeID }
        guard minutes > 0 else { throw WatchSurfaceActionError.invalidSnoozeMinutes }

        try await client.nudgeSnooze(id: cleanID, minutes: minutes)
        return .nudgeSnoozed(id: cleanID, minutes: minutes)
    }

    @discardableResult
    public func appendToThread(
        text: String,
        conversationID: String?,
        fallbackType: String = "watch_thread_capture",
        fallbackSource: String = "apple_watch"
    ) async throws -> WatchSurfaceActionOutcome {
        let cleanText = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !cleanText.isEmpty else { throw WatchSurfaceActionError.emptyText }

        if let conversationID = conversationID?.trimmingCharacters(in: .whitespacesAndNewlines),
           !conversationID.isEmpty {
            do {
                try await client.submitAssistantEntry(text: cleanText, conversationID: conversationID)
                return .threadAppended(conversationID: conversationID)
            } catch {
                try await client.createCapture(
                    text: queuedThreadAppendText(text: cleanText, conversationID: conversationID),
                    type: fallbackType,
                    source: fallbackSource
                )
                return .captureCreated(type: fallbackType, source: fallbackSource)
            }
        }

        try await client.createCapture(text: cleanText, type: fallbackType, source: fallbackSource)
        return .captureCreated(type: fallbackType, source: fallbackSource)
    }

    private func queuedThreadAppendText(text: String, conversationID: String) -> String {
        [
            "watch_thread_append:",
            "thread_id: \(conversationID)",
            "",
            text
        ].joined(separator: "\n")
    }
}

public enum WatchSurfaceActionError: Error, Equatable, Sendable {
    case emptyNudgeID
    case invalidSnoozeMinutes
    case emptyText
}

public struct NotificationRoute: Sendable {
    public let action: String
    public let payload: String

    public init(action: String, payload: String) {
        self.action = action
        self.payload = payload
    }
}

public struct VelAppEnvironment {
    public var sessionStore: SessionState
    public var syncController: any SyncController
    public var apiClient: any APIClient
    public var captureService: any CaptureService
    public var briefingService: any BriefingService
    public var planningService: any PlanningService
    public var notificationRouter: any NotificationRouter
    public var watchBridge: any WatchBridge
    public var embeddedBridge: any EmbeddedBridgeSurface
    public var featureCapabilities: FeatureCapabilities
    public var logger: any AppLogger
    public var auditStore: any AuditStore

    public init(
        sessionStore: SessionState,
        syncController: any SyncController,
        apiClient: any APIClient,
        captureService: any CaptureService,
        briefingService: any BriefingService,
        planningService: any PlanningService,
        notificationRouter: any NotificationRouter,
        watchBridge: any WatchBridge,
        embeddedBridge: any EmbeddedBridgeSurface,
        featureCapabilities: FeatureCapabilities,
        logger: any AppLogger,
        auditStore: any AuditStore
    ) {
        self.sessionStore = sessionStore
        self.syncController = syncController
        self.apiClient = apiClient
        self.captureService = captureService
        self.briefingService = briefingService
        self.planningService = planningService
        self.notificationRouter = notificationRouter
        self.watchBridge = watchBridge
        self.embeddedBridge = embeddedBridge
        self.featureCapabilities = featureCapabilities
        self.logger = logger
        self.auditStore = auditStore
    }

    public static func bootstrap(capabilities: FeatureCapabilities) -> VelAppEnvironment {
        let embeddedConfiguration = EmbeddedBridgeConfiguration(
            isBridgeAvailableInBuild: capabilities.supportsEmbeddedRustBridge,
            mode: capabilities.supportsEmbeddedRustBridge ? .embeddedCapable : .daemonBacked,
            target: capabilities.supportsEmbeddedRustBridge ? .iphoneOnly : .unavailable,
            approvedFlows: capabilities.supportsEmbeddedRustBridge
                ? [
                    .cachedNowHydration,
                    .localQuickActionPreparation,
                    .offlineRequestPackaging,
                    .deterministicDomainHelpers,
                    .localThreadDraftPackaging,
                    .localVoiceCapturePackaging,
                    .localVoiceQuickActionPackaging,
                    .localVoiceContinuityPackaging,
                    .localQueuedActionPackaging,
                    .localLinkingSettingsNormalization,
                    .localAssistantEntryFallbackPackaging,
                    .localLinkingRequestPackaging,
                    .localCaptureMetadataPackaging,
                    .localVoiceContinuitySummaryPackaging,
                    .localVoiceOfflineResponsePackaging,
                    .localVoiceCachedQueryPackaging,
                    .localLinkingFeedbackPackaging,
                    .localAppShellFeedbackPackaging
                ]
                : []
        )

        let embeddedBridge: any EmbeddedBridgeSurface = VelEmbeddedRustBridgeSurface(configuration: embeddedConfiguration)
            ?? NoopEmbeddedBridgeSurface(configuration: embeddedConfiguration)

        return VelAppEnvironment(
            sessionStore: SessionState(),
            syncController: NoopSyncController(),
            apiClient: NoopAPIClient(),
            captureService: PlaceholderCaptureService(),
            briefingService: PlaceholderBriefingService(),
            planningService: PlaceholderPlanningService(),
            notificationRouter: NoopNotificationRouter(),
            watchBridge: NoopWatchBridge(),
            embeddedBridge: embeddedBridge,
            featureCapabilities: capabilities,
            logger: NoopAppLogger(),
            auditStore: NoopAuditStore()
        )
    }
}
