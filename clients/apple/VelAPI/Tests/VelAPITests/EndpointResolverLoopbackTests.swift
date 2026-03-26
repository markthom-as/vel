import XCTest
@testable import VelAPI

final class EndpointResolverLoopbackTests: XCTestCase {
    func testSaveDiscoveredBaseURLsDropsLoopbackEntries() {
        let suiteName = "VelEndpointResolverLoopbackTests.loopback.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)

        VelEndpointResolver.saveDiscoveredBaseURLs([
            URL(string: "http://127.0.0.1:4130")!,
            URL(string: "http://localhost:4130")!,
            URL(string: "http://192.168.1.23:4130")!,
        ], userDefaults: defaults)

        XCTAssertEqual(
            VelEndpointResolver.discoveredBaseURLs(userDefaults: defaults).map(\.absoluteString),
            ["http://192.168.1.23:4130"]
        )
    }

    func testCandidateBaseURLsDeduplicatesConfiguredAndDiscoveredValues() {
        let suiteName = "VelEndpointResolverLoopbackTests.dedupe.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        defaults.set("http://192.168.1.44:4130", forKey: "vel_lan_base_url")
        defaults.set([
            "http://192.168.1.44:4130",
            "http://192.168.1.45:4130",
        ], forKey: "vel_discovered_base_urls")

        let candidates = VelEndpointResolver.candidateBaseURLs(userDefaults: defaults)

        XCTAssertEqual(
            candidates.map(\.absoluteString),
            [
                "http://192.168.1.44:4130",
                "http://192.168.1.45:4130",
            ] + expectedLoopbackFallbacks()
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
