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
    let offlineStore = VelAPI.VelOfflineStore()
    @Published var isReachable = false
    @Published var errorMessage: String?
    @Published var activeBaseURL: String?
    @Published var activeTransport: String?
    @Published var authorityLabel: String?
    @Published var pendingActionCount: Int = 0

    init() {
        let initial = VelAPI.VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelAPI.VelClient(baseURL: initial)
        pendingActionCount = offlineStore.pendingActionCount()
    }

    func checkReachability() async {
        for candidate in VelAPI.VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            do {
                _ = try await client.health()
                let bootstrap = try await client.clusterBootstrap()
                _ = await offlineStore.drainQueuedActions(using: client)
                await MainActor.run {
                    isReachable = true
                    errorMessage = nil
                    activeBaseURL = candidate.absoluteString
                    activeTransport = bootstrap.sync_transport
                    authorityLabel = bootstrap.node_display_name
                    pendingActionCount = offlineStore.pendingActionCount()
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
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url."
        }
    }

    func markNudgeDone(id: String) async {
        do {
            _ = try await client.nudgeDone(id: id)
            pendingActionCount = offlineStore.pendingActionCount()
        } catch {
            offlineStore.enqueueNudgeDone(id: id)
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Queued nudge action for sync."
        }
    }

    func snoozeNudge(id: String, minutes: Int = 10) async {
        do {
            _ = try await client.nudgeSnooze(id: id, minutes: minutes)
            pendingActionCount = offlineStore.pendingActionCount()
        } catch {
            offlineStore.enqueueNudgeSnooze(id: id, minutes: minutes)
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Queued nudge action for sync."
        }
    }

    func markCommitmentDone(id: String) async {
        do {
            _ = try await client.markCommitmentDone(id: id)
            pendingActionCount = offlineStore.pendingActionCount()
        } catch {
            offlineStore.enqueueCommitmentDone(id: id)
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Queued commitment action for sync."
        }
    }

    func createCommitment(text: String) async {
        do {
            _ = try await client.createCommitment(text: text)
            pendingActionCount = offlineStore.pendingActionCount()
        } catch {
            offlineStore.enqueueCommitmentCreate(text: text)
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Queued commitment for sync."
        }
    }

    func createCapture(text: String) async {
        do {
            _ = try await client.createCapture(text: text)
            pendingActionCount = offlineStore.pendingActionCount()
        } catch {
            offlineStore.enqueueCaptureCreate(text: text)
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Queued capture for sync."
        }
    }
}
