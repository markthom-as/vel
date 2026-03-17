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
    @Published var activeNudgeID: String?
    @Published var pendingActionCount: Int = 0

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
            let active = cached.filter { $0.state == "active" || $0.state == "snoozed" }
            await MainActor.run {
                nudgeCount = active.count
                message = active.first?.message ?? "No nudges"
                transport = "cached"
                activeNudgeID = active.first?.nudge_id
                pendingActionCount = offlineStore.pendingActionCount()
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
                    activeNudgeID = active.first?.nudge_id
                    pendingActionCount = offlineStore.pendingActionCount()
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
            pendingActionCount = offlineStore.pendingActionCount()
        }
    }

    func markTopNudgeDone() async {
        guard let nudgeID = activeNudgeID else { return }
        do {
            _ = try await client.nudgeDone(id: nudgeID)
        } catch {
            offlineStore.enqueueNudgeDone(id: nudgeID)
        }
        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
    }

    func snoozeTopNudge(minutes: Int = 10) async {
        guard let nudgeID = activeNudgeID else { return }
        do {
            _ = try await client.nudgeSnooze(id: nudgeID, minutes: minutes)
        } catch {
            offlineStore.enqueueNudgeSnooze(id: nudgeID, minutes: minutes)
        }
        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
    }
}
