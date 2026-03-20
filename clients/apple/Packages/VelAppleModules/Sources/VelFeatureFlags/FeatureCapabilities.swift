import Foundation

public struct FeatureCapabilities: Sendable {
    public let supportsChat: Bool
    public let supportsVoicePushToTalk: Bool
    public let supportsDashboard: Bool
    public let supportsProjectInspector: Bool
    public let supportsWidgets: Bool
    public let supportsLiveActivities: Bool
    public let supportsComplications: Bool
    public let supportsAmbientHUD: Bool
    public let supportsScreenAwareness: Bool
    public let supportsQuickCapture: Bool
    public let supportsNotificationActions: Bool
    public let supportsSplitViewWorkspace: Bool
    public let supportsEmbeddedRustBridge: Bool
    public let roleLabel: String

    public init(
        supportsChat: Bool,
        supportsVoicePushToTalk: Bool,
        supportsDashboard: Bool,
        supportsProjectInspector: Bool,
        supportsWidgets: Bool,
        supportsLiveActivities: Bool,
        supportsComplications: Bool,
        supportsAmbientHUD: Bool,
        supportsScreenAwareness: Bool,
        supportsQuickCapture: Bool,
        supportsNotificationActions: Bool,
        supportsSplitViewWorkspace: Bool,
        supportsEmbeddedRustBridge: Bool,
        roleLabel: String
    ) {
        self.supportsChat = supportsChat
        self.supportsVoicePushToTalk = supportsVoicePushToTalk
        self.supportsDashboard = supportsDashboard
        self.supportsProjectInspector = supportsProjectInspector
        self.supportsWidgets = supportsWidgets
        self.supportsLiveActivities = supportsLiveActivities
        self.supportsComplications = supportsComplications
        self.supportsAmbientHUD = supportsAmbientHUD
        self.supportsScreenAwareness = supportsScreenAwareness
        self.supportsQuickCapture = supportsQuickCapture
        self.supportsNotificationActions = supportsNotificationActions
        self.supportsSplitViewWorkspace = supportsSplitViewWorkspace
        self.supportsEmbeddedRustBridge = supportsEmbeddedRustBridge
        self.roleLabel = roleLabel
    }
}

public enum EmbeddedRuntimeMode: String, Sendable {
    case daemonBacked = "daemon_backed"
    case embeddedCapable = "embedded_capable"
}

public enum EmbeddedRuntimeTarget: String, Sendable {
    case iphoneOnly = "iphone_only"
    case unavailable = "unavailable"
}

public extension FeatureCapabilities {
    static let iPhone = FeatureCapabilities(
        supportsChat: true,
        supportsVoicePushToTalk: true,
        supportsDashboard: true,
        supportsProjectInspector: false,
        supportsWidgets: true,
        supportsLiveActivities: true,
        supportsComplications: false,
        supportsAmbientHUD: false,
        supportsScreenAwareness: false,
        supportsQuickCapture: true,
        supportsNotificationActions: true,
        supportsSplitViewWorkspace: false,
        supportsEmbeddedRustBridge: true,
        roleLabel: "iPhone"
    )

    static let iPad = FeatureCapabilities(
        supportsChat: true,
        supportsVoicePushToTalk: true,
        supportsDashboard: true,
        supportsProjectInspector: true,
        supportsWidgets: true,
        supportsLiveActivities: true,
        supportsComplications: false,
        supportsAmbientHUD: false,
        supportsScreenAwareness: false,
        supportsQuickCapture: true,
        supportsNotificationActions: true,
        supportsSplitViewWorkspace: true,
        supportsEmbeddedRustBridge: false,
        roleLabel: "iPad"
    )

    static let watch = FeatureCapabilities(
        supportsChat: false,
        supportsVoicePushToTalk: true,
        supportsDashboard: false,
        supportsProjectInspector: false,
        supportsWidgets: true,
        supportsLiveActivities: false,
        supportsComplications: true,
        supportsAmbientHUD: false,
        supportsScreenAwareness: false,
        supportsQuickCapture: true,
        supportsNotificationActions: true,
        supportsSplitViewWorkspace: false,
        supportsEmbeddedRustBridge: false,
        roleLabel: "Watch"
    )

    static let mac = FeatureCapabilities(
        supportsChat: true,
        supportsVoicePushToTalk: false,
        supportsDashboard: true,
        supportsProjectInspector: true,
        supportsWidgets: true,
        supportsLiveActivities: false,
        supportsComplications: false,
        supportsAmbientHUD: true,
        supportsScreenAwareness: false,
        supportsQuickCapture: true,
        supportsNotificationActions: true,
        supportsSplitViewWorkspace: true,
        supportsEmbeddedRustBridge: false,
        roleLabel: "Mac"
    )
}
