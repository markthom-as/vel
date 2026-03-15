import SwiftUI

@main
struct VelMacApp: App {
    @StateObject private var client = VelClientStore()
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(client)
        }
        .windowStyle(.automatic)
        .defaultSize(width: 400, height: 500)
    }
}

/// Reuse same store pattern as iOS; configure base URL (e.g. http://localhost:4242).
final class VelClientStore: ObservableObject {
    let client: VelAPI.VelClient
    @Published var isReachable = false
    @Published var errorMessage: String?

    init() {
        client = VelAPI.VelClient(baseURL: URL(string: "http://localhost:4242")!)
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
