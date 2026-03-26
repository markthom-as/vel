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
                    .localThreadDraftPackaging
                ]
                : []
        )

        let embeddedBridge = VelEmbeddedRustBridgeSurface(configuration: embeddedConfiguration)
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
