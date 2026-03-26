import XCTest
@testable import VelAPI

final class EndpointResolverTests: XCTestCase {
    func testCandidateBaseURLsPreferConfiguredThenDiscoveredThenLoopback() {
        let suiteName = "VelEndpointResolverTests.candidates.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        defaults.set("https://tail.example.ts.net:4130", forKey: "tailscale_base_url")
        defaults.set("http://192.168.1.44:4130", forKey: "lan_base_url")
        defaults.set([
            "http://192.168.1.55:4130",
            "http://192.168.1.44:4130",
            "http://192.168.1.56:4130",
        ], forKey: "vel_discovered_base_urls")

        let candidates = VelEndpointResolver.candidateBaseURLs(
            explicitBaseURL: URL(string: "https://explicit.example.com")!,
            userDefaults: defaults
        )

        XCTAssertEqual(
            candidates.map(\.absoluteString),
            [
                "https://explicit.example.com",
                "https://tail.example.ts.net:4130",
                "http://192.168.1.44:4130",
                "http://192.168.1.55:4130",
                "http://192.168.1.56:4130",
            ] + expectedLoopbackFallbacks()
        )
    }

    func testSaveDiscoveredBaseURLsPrependsNewValuesAndCapsHistory() {
        let suiteName = "VelEndpointResolverTests.persisted.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        let existing = (1...12).compactMap { URL(string: "http://192.168.1.\($0):4130") }
        VelEndpointResolver.saveDiscoveredBaseURLs(existing, userDefaults: defaults)

        VelEndpointResolver.saveDiscoveredBaseURLs([
            URL(string: "http://192.168.1.99:4130")!,
            URL(string: "http://192.168.1.3:4130")!,
        ], userDefaults: defaults)

        XCTAssertEqual(
            VelEndpointResolver.discoveredBaseURLs(userDefaults: defaults).map(\.absoluteString),
            [
                "http://192.168.1.99:4130",
                "http://192.168.1.3:4130",
                "http://192.168.1.1:4130",
                "http://192.168.1.2:4130",
                "http://192.168.1.4:4130",
                "http://192.168.1.5:4130",
                "http://192.168.1.6:4130",
                "http://192.168.1.7:4130",
                "http://192.168.1.8:4130",
                "http://192.168.1.9:4130",
                "http://192.168.1.10:4130",
                "http://192.168.1.11:4130",
            ]
        )
    }

    private func expectedLoopbackFallbacks() -> [String] {
        #if targetEnvironment(simulator)
        return [
            "http://127.0.0.1:4130",
            "http://localhost:4130",
        ]
        #else
        return []
        #endif
    }
}
