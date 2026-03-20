import Foundation
import VelDomain
import VelFeatureFlags

public enum EmbeddedAppleFlow: String, Sendable, CaseIterable {
    case cachedNowHydration = "cached_now_hydration"
    case localQuickActionPreparation = "local_quick_action_preparation"
    case offlineRequestPackaging = "offline_request_packaging"
    case deterministicDomainHelpers = "deterministic_domain_helpers"
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

public protocol EmbeddedBridgeSurface: Sendable {
    var configuration: EmbeddedBridgeConfiguration { get }
    var nowBridge: any EmbeddedNowBridge { get }
    var quickActionBridge: any EmbeddedQuickActionBridge { get }
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

public struct NoopEmbeddedBridgeSurface: EmbeddedBridgeSurface {
    public let configuration: EmbeddedBridgeConfiguration
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge

    public init(configuration: EmbeddedBridgeConfiguration = .daemonBackedDefault()) {
        self.configuration = configuration
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
    }
}
