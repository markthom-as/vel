import SwiftUI
import VelAPI

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

@MainActor
final class VelClientStore: ObservableObject {
    let client: VelClient
    let offlineStore = VelOfflineStore()

    @Published var isReachable = false
    @Published var isSyncing = false
    @Published var errorMessage: String?
    @Published var activeBaseURL: String?
    @Published var activeTransport: String?
    @Published var authorityLabel: String?
    @Published var pendingActionCount = 0
    @Published var lastSyncAt: Date?

    @Published var context: CurrentContextData?
    @Published var nudges: [NudgeData] = []
    @Published var commitments: [CommitmentData] = []
    @Published var signals: [SignalData] = []

    init() {
        let initial = VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelClient(baseURL: initial)
        applyCachedState()
    }

    func refresh() async {
        isSyncing = true
        defer { isSyncing = false }

        applyCachedState()
        var lastError: Error?

        for candidate in VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            do {
                _ = try await client.health()
                _ = await offlineStore.drainQueuedActions(using: client)

                let bootstrap = try await client.syncBootstrap()
                offlineStore.hydrate(from: bootstrap)

                let recentSignals = try await client.signals(limit: 80)
                offlineStore.saveCachedSignals(recentSignals)

                isReachable = true
                errorMessage = nil
                activeBaseURL = candidate.absoluteString
                activeTransport = bootstrap.cluster.sync_transport
                authorityLabel = bootstrap.cluster.node_display_name
                lastSyncAt = Date()

                context = bootstrap.current_context ?? offlineStore.cachedContext()
                nudges = offlineStore.cachedNudgesApplyingPendingActions()
                commitments = offlineStore.cachedCommitmentsApplyingPendingActions()
                signals = recentSignals
                pendingActionCount = offlineStore.pendingActionCount()
                return
            } catch {
                lastError = error
                continue
            }
        }

        isReachable = false
        activeBaseURL = nil
        activeTransport = nil
        authorityLabel = nil
        applyCachedState()

        if let lastError {
            errorMessage = "Offline cache in use. \(lastError.localizedDescription)"
        } else {
            errorMessage = "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url."
        }
    }

    func refreshSignals() async {
        guard isReachable else {
            signals = offlineStore.cachedSignals()
            return
        }

        do {
            let recentSignals = try await client.signals(limit: 80)
            offlineStore.saveCachedSignals(recentSignals)
            signals = recentSignals
            lastSyncAt = Date()
            errorMessage = nil
        } catch {
            signals = offlineStore.cachedSignals()
            errorMessage = "Could not refresh activity feed. \(error.localizedDescription)"
        }
    }

    func markNudgeDone(id: String) async {
        await performAction(
            queuedMessage: "Queued nudge completion for sync.",
            remote: {
                _ = try await client.nudgeDone(id: id)
            },
            queueFallback: {
                offlineStore.enqueueNudgeDone(id: id)
            }
        )
    }

    func snoozeNudge(id: String, minutes: Int = 10) async {
        await performAction(
            queuedMessage: "Queued nudge snooze for sync.",
            remote: {
                _ = try await client.nudgeSnooze(id: id, minutes: minutes)
            },
            queueFallback: {
                offlineStore.enqueueNudgeSnooze(id: id, minutes: minutes)
            }
        )
    }

    func markCommitmentDone(id: String) async {
        await performAction(
            queuedMessage: "Queued commitment completion for sync.",
            remote: {
                _ = try await client.markCommitmentDone(id: id)
            },
            queueFallback: {
                offlineStore.enqueueCommitmentDone(id: id)
            }
        )
    }

    func createCommitment(text: String) async {
        await performAction(
            queuedMessage: "Queued commitment for sync.",
            remote: {
                _ = try await client.createCommitment(text: text)
            },
            queueFallback: {
                offlineStore.enqueueCommitmentCreate(text: text)
            }
        )
    }

    func createCapture(
        text: String,
        type: String = "note",
        source: String = "apple"
    ) async {
        await performAction(
            queuedMessage: "Queued capture for sync.",
            remote: {
                _ = try await client.createCapture(text: text, type: type, source: source)
            },
            queueFallback: {
                offlineStore.enqueueCaptureCreate(
                    text: queuedCaptureText(text: text, type: type, source: source)
                )
            }
        )
    }

    func setBaseURLOverride(_ value: String?) {
        let trimmed = value?.trimmingCharacters(in: .whitespacesAndNewlines)
        if let trimmed, !trimmed.isEmpty {
            UserDefaults.standard.set(trimmed, forKey: "vel_base_url")
        } else {
            UserDefaults.standard.removeObject(forKey: "vel_base_url")
        }
    }

    func clearError() {
        errorMessage = nil
    }

    private func performAction(
        queuedMessage: String,
        remote: () async throws -> Void,
        queueFallback: () -> Void
    ) async {
        do {
            try await remote()
            pendingActionCount = offlineStore.pendingActionCount()
            await refresh()
        } catch {
            queueFallback()
            applyCachedState()
            errorMessage = queuedMessage
        }
    }

    private func applyCachedState() {
        context = offlineStore.cachedContext()
        nudges = offlineStore.cachedNudgesApplyingPendingActions()
        commitments = offlineStore.cachedCommitmentsApplyingPendingActions()
        signals = offlineStore.cachedSignals()
        pendingActionCount = offlineStore.pendingActionCount()
    }

    private func queuedCaptureText(text: String, type: String, source: String) -> String {
        let cleanType = type.trimmingCharacters(in: .whitespacesAndNewlines)
        let cleanSource = source.trimmingCharacters(in: .whitespacesAndNewlines)
        guard cleanType != "note" || cleanSource != "apple" else {
            return text
        }

        return [
            "queued_capture_metadata:",
            "requested_capture_type: \(cleanType)",
            "requested_source_device: \(cleanSource)",
            "",
            text
        ].joined(separator: "\n")
    }
}
