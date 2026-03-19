import Foundation

public struct VelTask: Identifiable, Codable, Sendable {
    public let id: String
    public var title: String
    public var isDone: Bool

    public init(id: String, title: String, isDone: Bool = false) {
        self.id = id
        self.title = title
        self.isDone = isDone
    }
}

public struct VelContextSnapshot: Codable, Sendable {
    public var mode: String?
    public var nextEventTitle: String?
    public var nudgeCount: Int

    public init(mode: String? = nil, nextEventTitle: String? = nil, nudgeCount: Int = 0) {
        self.mode = mode
        self.nextEventTitle = nextEventTitle
        self.nudgeCount = nudgeCount
    }
}

public enum VelQuickCommand: String, Codable, Sendable, CaseIterable {
    case confirmTask
    case deferTask
    case snoozeNudge
    case quickCapture
}
