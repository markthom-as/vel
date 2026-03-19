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
    @Published var scheduleSummary: String?
    @Published var scheduleDetail: String?
    @Published var topActionTitle: String?
    @Published var behaviorHeadline: String?
    @Published var behaviorReason: String?
    @Published var lastActionStatus: String?

    init() {
        let initial = VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelClient(baseURL: initial)
    }

    func refresh() async {
        let cached = offlineStore.cachedNudgesApplyingPendingActions()
        let cachedNow = offlineStore.cachedNow()
        let cachedBehavior = offlineStore.cachedAppleBehaviorSummary()
        let hasCachedContent = !cached.isEmpty || cachedNow != nil || cachedBehavior != nil
        if hasCachedContent {
            applySnapshot(nudges: cached, now: cachedNow, behavior: cachedBehavior, transportLabel: "cached")
        }
        for candidate in VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            client.configuration = .shared()
            do {
                _ = await offlineStore.drainQueuedActions(using: client)
                let bootstrap = try await client.syncBootstrap()
                offlineStore.hydrate(from: bootstrap)
                let now = try await client.now()
                offlineStore.saveCachedNow(now)
                let behavior = try? await client.appleBehaviorSummary()
                if let behavior {
                    offlineStore.saveCachedAppleBehaviorSummary(behavior)
                }
                let active = bootstrap.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
                applySnapshot(
                    nudges: active,
                    now: now,
                    behavior: behavior,
                    transportLabel: bootstrap.cluster.sync_transport
                )
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
                scheduleSummary = nil
                scheduleDetail = nil
                topActionTitle = nil
                behaviorHeadline = nil
                behaviorReason = nil
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

    private func applySnapshot(
        nudges: [NudgeData],
        now: NowData?,
        behavior: AppleBehaviorSummaryData?,
        transportLabel: String
    ) {
        let active = nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        let nextEvent = now?.schedule.next_event
        let scheduleSummaryValue: String?
        if let nextEvent {
            scheduleSummaryValue = nextEvent.title
        } else {
            scheduleSummaryValue = now?.schedule.empty_message
        }

        let scheduleDetailValue: String?
        if let leaveBy = nextEvent?.leave_by_ts {
            scheduleDetailValue = "Leave by \(formatUnix(leaveBy))"
        } else if let eventStart = nextEvent?.start_ts {
            scheduleDetailValue = "Starts \(formatUnix(eventStart))"
        } else {
            scheduleDetailValue = nil
        }

        nudgeCount = active.count
        message = scheduleSummaryValue ?? active.first?.message ?? "No quick-loop state yet"
        transport = transportLabel
        activeNudgeID = active.first?.nudge_id
        pendingActionCount = offlineStore.pendingActionCount()
        mode = now?.summary.mode.label
        nextCommitmentText = now?.tasks.next_commitment?.text
        scheduleSummary = scheduleSummaryValue
        scheduleDetail = scheduleDetailValue
        topActionTitle = now?.action_items.first?.title
        behaviorHeadline = behavior?.headline
        behaviorReason = behavior?.reasons.first
    }
}

private func formatUnix(_ unix: Int) -> String {
    let formatter = DateFormatter()
    formatter.dateStyle = .none
    formatter.timeStyle = .short
    return formatter.string(from: Date(timeIntervalSince1970: TimeInterval(unix)))
}
