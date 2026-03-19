import Foundation
import VelFeatureFlags

public struct SharedSectionModel: Sendable {
    public let title: String
    public let subtitle: String?

    public init(title: String, subtitle: String? = nil) {
        self.title = title
        self.subtitle = subtitle
    }
}

public enum AppDensity: String, Sendable {
    case compact
    case regular
}

public struct ThemeTokens: Sendable {
    public let density: AppDensity

    public init(density: AppDensity = .compact) {
        self.density = density
    }
}
