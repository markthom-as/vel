import SwiftUI
import VelApplePlatform
import VelApplication
import VelAPI

@main
struct VelMacApp: App {
    @StateObject private var client = VelClientStore()
    private let appEnvironment = VelAppEnvironment.bootstrap(
        capabilities: FeatureCapabilityMapper.capabilities(for: .mac)
    )

    var body: some Scene {
        WindowGroup {
            ContentView(appEnvironment: appEnvironment)
                .environmentObject(client)
                .tint(.orange)
        }
        .windowStyle(.automatic)
        .defaultSize(width: 400, height: 500)
    }
}

/// Reuse same store pattern as iOS; configure base URL (default http://localhost:4130).
final class VelClientStore: ObservableObject {
    let client: VelAPI.VelClient
    let offlineStore = VelAPI.VelOfflineStore()
    let localExporter = VelAPI.VelMacLocalSourceExporter()
    @Published var isReachable = false
    @Published var errorMessage: String?
    @Published var activeBaseURL: String?
    @Published var activeTransport: String?
    @Published var authorityLabel: String?
    @Published var pendingActionCount: Int = 0
    @Published var planningProfile: VelAPI.PlanningProfileResponseData?

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
                let exportReport = await localExporter.bootstrap(using: client)
                _ = await offlineStore.drainQueuedActions(using: client)
                let bootstrap = try await client.syncBootstrap()
                let planningProfile = try? await client.planningProfile()
                offlineStore.hydrate(from: bootstrap)
                await MainActor.run {
                    isReachable = true
                    errorMessage = exportReport.errors.isEmpty
                        ? nil
                        : exportReport.errors.joined(separator: " | ")
                    activeBaseURL = candidate.absoluteString
                    activeTransport = bootstrap.cluster.sync_transport
                    authorityLabel = bootstrap.cluster.node_display_name
                    pendingActionCount = offlineStore.pendingActionCount()
                    self.planningProfile = planningProfile
                }
                return
            } catch {
                continue
            }
        }

        let exportReport = await localExporter.bootstrap(using: nil)
        await MainActor.run {
            isReachable = false
            activeBaseURL = nil
            activeTransport = nil
            authorityLabel = nil
            pendingActionCount = offlineStore.pendingActionCount()
            planningProfile = nil
            let exportMessage = exportReport.errors.isEmpty ? nil : exportReport.errors.joined(separator: " | ")
            errorMessage = [
                "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url.",
                exportMessage
            ]
            .compactMap { $0 }
            .joined(separator: " ")
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
