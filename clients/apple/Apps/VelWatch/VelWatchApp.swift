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

@MainActor
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
    @Published var lastActionStatus: String?

    init() {
        let initial = VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelClient(baseURL: initial)
    }

    func refresh() async {
        let cached = offlineStore.cachedNudgesApplyingPendingActions()
        let cachedContext = offlineStore.cachedContext()
        let cachedCommitments = offlineStore.cachedCommitmentsApplyingPendingActions()
        let hasCachedContent = !cached.isEmpty || cachedContext != nil || !cachedCommitments.isEmpty
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
            lastActionStatus = "Top nudge resolved."
        } catch {
            offlineStore.enqueueNudgeDone(id: nudgeID)
            lastActionStatus = "Queued nudge resolution."
        }
        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
    }

    func snoozeTopNudge(minutes: Int = 10) async {
        guard let nudgeID = activeNudgeID else { return }
        do {
            _ = try await client.nudgeSnooze(id: nudgeID, minutes: minutes)
            lastActionStatus = "Top nudge snoozed \(minutes)m."
        } catch {
            offlineStore.enqueueNudgeSnooze(id: nudgeID, minutes: minutes)
            lastActionStatus = "Queued nudge snooze."
        }
        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
    }

    func createCapture(
        text: String,
        type: String = "watch_note",
        source: String = "apple_watch"
    ) async {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        do {
            _ = try await client.createCapture(text: trimmed, type: type, source: source)
            lastActionStatus = "Capture saved."
        } catch {
            offlineStore.enqueueCaptureCreate(text: queuedCaptureText(text: trimmed, type: type, source: source))
            lastActionStatus = "Capture queued for sync."
        }

        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
    }

    func createCommitment(text: String) async {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        do {
            _ = try await client.createCommitment(text: trimmed)
            lastActionStatus = "Task added."
        } catch {
            offlineStore.enqueueCommitmentCreate(text: trimmed)
            lastActionStatus = "Task queued for sync."
        }

        pendingActionCount = offlineStore.pendingActionCount()
        await refresh()
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
