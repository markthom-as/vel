import SwiftUI
import VelApplePlatform
import VelApplication
import VelAPI

@main
struct VelWatchApp: App {
    @StateObject private var store = VelWatchStore()
    private let appEnvironment = VelAppEnvironment.bootstrap(
        capabilities: FeatureCapabilityMapper.capabilities(for: .watch)
    )

    var body: some Scene {
        WindowGroup {
            ContentView(appEnvironment: appEnvironment)
                .environmentObject(store)
                .preferredColorScheme(.dark)
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
    @Published var scheduleProposalStatus: String?
    @Published var topActionTitle: String?
    @Published var activeThreadID: String?
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
                scheduleProposalStatus = nil
                activeThreadID = nil
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

    func submitThreadText(_ text: String) async {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        guard let threadID = resolveActiveThreadID(from: offlineStore.cachedNow()) else {
            await createCapture(
                text: trimmed,
                type: "watch_thread_capture",
                source: "apple_watch",
            )
            lastActionStatus = "No active thread found. Saved as watch note."
            return
        }

        do {
            _ = try await client.submitAssistantEntry(text: trimmed, conversationID: threadID)
            lastActionStatus = "Saved to thread \(threadID)."
        } catch {
            offlineStore.enqueueCaptureCreate(
                text: queuedCaptureText(
                    text: "watch_thread_append:\nthread_id: \(threadID)\n\n\(trimmed)",
                    type: "watch_thread_capture",
                    source: "apple_watch",
                ),
            )
            lastActionStatus = "Thread save failed; queued for review."
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

        let scheduleProposalStatusValue: String?
        if let proposalSummary = now?.commitment_scheduling_summary {
            if let latestPending = proposalSummary.latest_pending {
                scheduleProposalStatusValue = "Pending schedule edit: \(latestPending.title)"
            } else if let latestApplied = proposalSummary.latest_applied {
                scheduleProposalStatusValue = "Last applied schedule edit: \(latestApplied.title)"
            } else if let latestFailed = proposalSummary.latest_failed {
                scheduleProposalStatusValue = "Last failed schedule edit: \(latestFailed.title)"
            } else {
                scheduleProposalStatusValue = "Schedule continuity: \(proposalSummary.pending_count) pending"
            }
        } else {
            scheduleProposalStatusValue = nil
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
        scheduleProposalStatus = scheduleProposalStatusValue
        activeThreadID = resolveActiveThreadID(from: now)
        topActionTitle = now?.action_items.first?.title
        behaviorHeadline = behavior?.headline
        behaviorReason = behavior?.reasons.first
    }

    private func resolveActiveThreadID(from now: NowData?) -> String? {
        guard let now else { return nil }
        if let nudgeThreadID = now.nudge_bars?.first(where: { normalizeThreadID($0.primary_thread_id) != nil })?.primary_thread_id {
            return normalizeThreadID(nudgeThreadID)
        }
        if let taskThreadID = now.task_lane?.active?.primary_thread_id {
            return normalizeThreadID(taskThreadID)
        }
        if let dockedThreadID = now.docked_input?.raw_capture_thread_id {
            return normalizeThreadID(dockedThreadID)
        }
        if let contextThreadID = now.context_line?.thread_id {
            return normalizeThreadID(contextThreadID)
        }
        return nil
    }

    private func normalizeThreadID(_ threadID: String?) -> String? {
        let trimmed = threadID?.trimmingCharacters(in: .whitespacesAndNewlines)
        return trimmed == nil || trimmed!.isEmpty ? nil : trimmed
    }
}

private func formatUnix(_ unix: Int) -> String {
    let formatter = DateFormatter()
    formatter.dateStyle = .none
    formatter.timeStyle = .short
    return formatter.string(from: Date(timeIntervalSince1970: TimeInterval(unix)))
}
