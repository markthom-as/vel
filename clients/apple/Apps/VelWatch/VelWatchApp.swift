import SwiftUI

@main
struct VelWatchApp: App {
    @StateObject private var store = VelWatchStore()
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
        }
    }
}

final class VelWatchStore: ObservableObject {
    let client: VelAPI.VelClient
    let offlineStore = VelAPI.VelOfflineStore()
    @Published var message: String = "Vel"
    @Published var nudgeCount: Int = 0
    @Published var transport: String?

    init() {
        let initial = VelAPI.VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelAPI.VelClient(baseURL: initial)
    }

    func refresh() async {
        let cached = offlineStore.cachedNudgesApplyingPendingActions()
        var hasCachedContent = false
        if !cached.isEmpty {
            hasCachedContent = true
            await MainActor.run {
                nudgeCount = cached.filter { $0.state == "active" || $0.state == "snoozed" }.count
                message = cached.first(where: { $0.state == "active" || $0.state == "snoozed" })?.message ?? "No nudges"
                transport = "cached"
            }
        }
        for candidate in VelAPI.VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            do {
                _ = await offlineStore.drainQueuedActions(using: client)
                let bootstrap = try await client.syncBootstrap()
                offlineStore.hydrate(from: bootstrap)
                let active = bootstrap.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
                await MainActor.run {
                    nudgeCount = active.count
                    message = active.first?.message ?? "No nudges"
                    transport = bootstrap.cluster.sync_transport
                }
                return
            } catch {
                continue
            }
        }

        await MainActor.run {
            if hasCachedContent {
                transport = "cached"
            } else {
                transport = nil
                message = "Offline"
            }
        }
    }
}
