import SwiftUI

@main
struct VelApp: App {
    @StateObject private var client = VelClientStore()
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(client)
        }
    }
}

/// Shared store for API client and base URL. Configure baseURL (e.g. from Settings) before use.
final class VelClientStore: ObservableObject {
    let client: VelAPI.VelClient
    @Published var isReachable = false
    @Published var errorMessage: String?

    init() {
        // Default: localhost. Override with UserDefaults or Settings bundle for device/simulator.
        var url = URL(string: "http://localhost:4242")!
        #if !targetEnvironment(simulator)
        // On device, use your machine's IP or a configured host
        if let host = UserDefaults.standard.string(forKey: "vel_base_url"), let u = URL(string: host) {
            url = u
        }
        #endif
        client = VelAPI.VelClient(baseURL: url)
    }

    func checkReachability() async {
        do {
            _ = try await client.health()
            await MainActor.run { isReachable = true; errorMessage = nil }
        } catch {
            await MainActor.run { isReachable = false; errorMessage = error.localizedDescription }
        }
    }
}
