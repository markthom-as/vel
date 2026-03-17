import Foundation

public struct VelDocumentationReference: Identifiable, Sendable {
    public let id: String
    public let category: String
    public let title: String
    public let path: String
    public let summary: String

    public init(
        id: String,
        category: String,
        title: String,
        path: String,
        summary: String
    ) {
        self.id = id
        self.category = category
        self.title = title
        self.path = path
        self.summary = summary
    }
}

public enum VelDocumentationCatalog {
    public static let core: [VelDocumentationReference] = [
        .init(
            id: "core-docs-guide",
            category: "core",
            title: "Docs Guide",
            path: "docs/README.md",
            summary: "Top-level documentation authority and navigation guide."
        ),
        .init(
            id: "core-status",
            category: "core",
            title: "Status",
            path: "docs/status.md",
            summary: "Canonical implementation truth for shipped behavior."
        ),
        .init(
            id: "core-architecture",
            category: "core",
            title: "Architecture",
            path: "docs/architecture.md",
            summary: "System structure and core boundaries."
        ),
    ]

    public static let user: [VelDocumentationReference] = [
        .init(
            id: "user-docs-entry",
            category: "user",
            title: "User Docs",
            path: "docs/user/README.md",
            summary: "Operator-facing entrypoint for running Vel."
        ),
        .init(
            id: "user-quickstart",
            category: "user",
            title: "Quickstart",
            path: "docs/user/quickstart.md",
            summary: "Shortest path to first working local setup."
        ),
        .init(
            id: "user-setup",
            category: "user",
            title: "Setup",
            path: "docs/user/setup.md",
            summary: "Config, integrations, and macOS local-source setup."
        ),
        .init(
            id: "user-daily-use",
            category: "user",
            title: "Daily Use",
            path: "docs/user/daily-use.md",
            summary: "Repeated workflow for day-to-day operation."
        ),
    ]
}
