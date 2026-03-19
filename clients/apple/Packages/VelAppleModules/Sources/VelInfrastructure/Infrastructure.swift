import Foundation
import VelDomain

public protocol APIClient: Sendable {
    func fetchContext() async throws -> VelContextSnapshot
    func submitQuickCommand(_ command: VelQuickCommand) async throws
}

public protocol AuditStore: Sendable {
    func append(event: String)
}

public protocol AppLogger: Sendable {
    func log(_ message: String)
}

public protocol SyncController: Sendable {
    func refresh() async throws
}

public struct NoopAPIClient: APIClient {
    public init() {}

    public func fetchContext() async throws -> VelContextSnapshot {
        VelContextSnapshot()
    }

    public func submitQuickCommand(_ command: VelQuickCommand) async throws {}
}

public struct NoopAuditStore: AuditStore {
    public init() {}
    public func append(event: String) {}
}

public struct NoopSyncController: SyncController {
    public init() {}
    public func refresh() async throws {}
}

public struct NoopAppLogger: AppLogger {
    public init() {}
    public func log(_ message: String) {}
}
