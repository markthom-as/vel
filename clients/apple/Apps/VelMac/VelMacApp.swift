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

/// Reuse same store pattern as iOS; configure base URL (default http://localhost:4130).
final class VelClientStore: ObservableObject {
    let client: VelAPI.VelClient
    @Published var isReachable = false
    @Published var errorMessage: String?
    @Published var activeBaseURL: String?
    @Published var activeTransport: String?
    @Published var authorityLabel: String?

    init() {
        let initial = VelAPI.VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelAPI.VelClient(baseURL: initial)
    }

    func checkReachability() async {
        for candidate in VelAPI.VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            do {
                _ = try await client.health()
                let bootstrap = try await client.clusterBootstrap()
                await MainActor.run {
                    isReachable = true
                    errorMessage = nil
                    activeBaseURL = candidate.absoluteString
                    activeTransport = bootstrap.sync_transport
                    authorityLabel = bootstrap.node_display_name
                }
                return
            } catch {
                continue
            }
        }

        await MainActor.run {
            isReachable = false
            activeBaseURL = nil
            activeTransport = nil
            authorityLabel = nil
            errorMessage = "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url."
        }
    }
}
