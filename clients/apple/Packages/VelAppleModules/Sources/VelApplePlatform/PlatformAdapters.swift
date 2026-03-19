import Foundation
import VelFeatureFlags
#if canImport(UIKit)
import UIKit
#endif

public protocol WatchBridge: Sendable {
    func push(summary: String) async
}

public protocol NotificationRouter: Sendable {
    func route(actionIdentifier: String, payload: String?) async
}

public protocol AmbientStateProvider: Sendable {
    func currentStateDescription() async -> String
}

public enum AppleSurfaceRole: Sendable {
    case iphone
    case ipad
    case watch
    case mac
}

public enum FeatureCapabilityMapper {
    public static func capabilities(for surface: AppleSurfaceRole) -> FeatureCapabilities {
        switch surface {
        case .iphone:
            return .iPhone
        case .ipad:
            return .iPad
        case .watch:
            return .watch
        case .mac:
            return .mac
        }
    }

    public static func currentIOSDevice() -> FeatureCapabilities {
#if canImport(UIKit) && !os(watchOS)
        return UIDevice.current.userInterfaceIdiom == .pad ? .iPad : .iPhone
#else
        return .iPhone
#endif
    }
}

public struct NoopWatchBridge: WatchBridge {
    public init() {}
    public func push(summary: String) async {}
}

public struct NoopNotificationRouter: NotificationRouter {
    public init() {}
    public func route(actionIdentifier: String, payload: String?) async {}
}

public struct PlatformEnvironment: Sendable {
    public let capabilities: FeatureCapabilities

    public init(capabilities: FeatureCapabilities) {
        self.capabilities = capabilities
    }
}
