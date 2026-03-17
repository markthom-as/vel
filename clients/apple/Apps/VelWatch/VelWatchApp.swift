import SwiftUI
import VelAPI

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
    let client: VelClient
    let offlineStore = VelOfflineStore()
    @Published var message: String = "Vel"
    @Published var nudgeCount: Int = 0
    @Published var transport: String?
    @Published var activeNudgeID: String?
    @Published var pendingActionCount: Int = 0
    @Published var mode: String?
    @Published var nextCommitmentText: String?

    init() {
        let initial = VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelClient(baseURL: initial)
    }

    func refresh() async {
        let cached = offlineStore.cachedNudgesApplyingPendingActions()
        let cachedContext = offlineStore.cachedContext()
        let cachedCommitments = offlineStore.cachedCommitmentsApplyingPendingActions()
        let hasCachedContent = !cached.isEmpty
        if hasCachedContent {
            let active = cached.filter { $0.state == "active" || $0.state == "snoozed" }
            await MainActor.run {
                nudgeCount = active.count
                message = active.first?.message ?? "No nudges"
                transport = "cached"
                activeNudgeID = active.first?.nudge_id
                pendingActionCount = offlineStore.pendingActionCount()
                mode = cachedContext?.context?.mode
                nextCommitmentText = resolveNextCommitment(
                    preferredID: cachedContext?.context?.next_commitment_id,
                    commitments: cachedCommitments
                )?.text
            }
        }
        for candidate in VelEndpointResolver.candidateBaseURLs() {
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
                    mode = bootstrap.current_context?.context?.mode
                    nextCommitmentText = resolveNextCommitment(
                        preferredID: bootstrap.current_context?.context?.next_commitment_id,
                        commitments: bootstrap.commitments
                    )?.text
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
                mode = nil
                nextCommitmentText = nil
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

    private func resolveNextCommitment(
        preferredID: String?,
        commitments: [CommitmentData]
    ) -> CommitmentData? {
        let open = commitments.filter { $0.status == "open" }
        if let preferredID, let matched = open.first(where: { $0.id == preferredID }) {
            return matched
        }
        return open.sorted { lhs, rhs in
            switch (lhs.due_at, rhs.due_at) {
            case let (l?, r?):
                return l < r
            case (.some, .none):
                return true
            case (.none, .some):
                return false
            case (.none, .none):
                return lhs.text < rhs.text
            }
        }.first
    }
}
