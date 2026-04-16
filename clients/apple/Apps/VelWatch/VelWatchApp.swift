import SwiftUI
import VelApplePlatform
import VelApplication
import VelAPI

enum WatchSignalKind: String, CaseIterable, Identifiable {
    case drifting
    case onTrack
    case needFocus
    case wakingUp

    var id: String { rawValue }

    var title: String {
        switch self {
        case .drifting:
            return "Drifting"
        case .onTrack:
            return "On track"
        case .needFocus:
            return "Need focus"
        case .wakingUp:
            return "Waking up"
        }
    }

    var captureType: String {
        switch self {
        case .drifting:
            return "watch_signal_drift"
        case .onTrack:
            return "watch_signal_on_track"
        case .needFocus:
            return "watch_signal_need_focus"
        case .wakingUp:
            return "watch_signal_wake"
        }
    }

    var backendSignalType: String {
        switch self {
        case .drifting:
            return "drifting"
        case .onTrack:
            return "on_track"
        case .needFocus:
            return "need_focus"
        case .wakingUp:
            return "wake"
        }
    }

    var defaultBody: String {
        switch self {
        case .drifting:
            return "signal_type: drifting"
        case .onTrack:
            return "signal_type: on_track"
        case .needFocus:
            return "signal_type: need_focus"
        case .wakingUp:
            return "signal_type: wake"
        }
    }
}

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
    @Published var activeStandupSessionID: String?
    @Published var overdueItems: [DailyLoopOverdueMenuItemData] = []
    @Published var overdueStatus: String?
    @Published var lastActionStatus: String?

    var compactStatusLine: String {
        if let topActionTitle, !topActionTitle.isEmpty {
            return topActionTitle
        }
        if let nextCommitmentText, !nextCommitmentText.isEmpty {
            return nextCommitmentText
        }
        return message
    }

    var driftSummary: String? {
        let headline = behaviorHeadline?.trimmingCharacters(in: .whitespacesAndNewlines)
        if let headline, !headline.isEmpty {
            return headline
        }
        let reason = behaviorReason?.trimmingCharacters(in: .whitespacesAndNewlines)
        return reason?.isEmpty == false ? reason : nil
    }

    var handoffSummary: String {
        if activeThreadID != nil {
            return "Use iPhone or Mac for deeper follow-through."
        }
        return "No active thread. Longer follow-through should hand off to iPhone or Mac."
    }

    var transportSummary: String {
        switch transport?.trimmingCharacters(in: .whitespacesAndNewlines).lowercased() {
        case "cached":
            return "Using cached state through the bridge."
        case let value? where !value.isEmpty:
            return "Bridge transport: \(value)"
        default:
            return "Waiting for bridge state."
        }
    }

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
                await refreshOverdueMenu(for: Date())
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
                activeStandupSessionID = nil
                overdueItems = []
                overdueStatus = nil
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

    func queueEscalationRequest(_ text: String) async {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        let body = [
            "watch_escalation_request:",
            "handoff: phone_or_mac",
            "",
            trimmed,
        ].joined(separator: "\n")
        await createCapture(text: body, type: "watch_escalation", source: "apple_watch")
        lastActionStatus = "Escalation queued for phone or Mac."
    }

    func emitSignal(_ kind: WatchSignalKind, note: String? = nil) async {
        let trimmedNote = note?.trimmingCharacters(in: .whitespacesAndNewlines)
        do {
            _ = try await client.createWatchSignal(
                WatchSignalCreateRequestData(
                    signal_type: kind.backendSignalType,
                    note: trimmedNote?.isEmpty == false ? trimmedNote : nil,
                    context: .object(["surface": .string("watch")]),
                    source_device: "apple_watch_signal"
                )
            )
            lastActionStatus = "\(kind.title) signal sent."
        } catch {
            var lines = [
                kind.defaultBody,
                "source_surface: watch",
            ]
            if let trimmedNote, !trimmedNote.isEmpty {
                lines.append("")
                lines.append(trimmedNote)
            }
            await createCapture(
                text: lines.joined(separator: "\n"),
                type: kind.captureType,
                source: "apple_watch_signal"
            )
            lastActionStatus = "\(kind.title) signal queued as capture."
        }
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

    func refreshOverdueMenu(for date: Date = Date()) async {
        let sessionDate = dailyLoopDateString(from: date)

        do {
            guard let session = try await client.activeDailyLoopSession(
                sessionDate: sessionDate,
                phase: .standup
            ) else {
                activeStandupSessionID = nil
                overdueItems = []
                overdueStatus = "Start standup to resolve overdue tasks."
                return
            }

            let menu = try await client.dailyLoopOverdueMenu(
                sessionID: session.id,
                request: DailyLoopOverdueMenuRequestData(today: sessionDate, include_vel_guess: true, limit: 5)
            )
            activeStandupSessionID = menu.session_id
            overdueItems = menu.items
            overdueStatus = menu.items.isEmpty ? "No overdue tasks in standup." : nil
        } catch {
            activeStandupSessionID = nil
            overdueItems = []
            overdueStatus = "Reconnect to Vel for overdue actions."
        }
    }

    func applyOverdueShortcut(
        item: DailyLoopOverdueMenuItemData,
        action: DailyLoopOverdueActionData,
        reason: String = "apple_watch_quick_reaction"
    ) async {
        guard let sessionID = activeStandupSessionID else {
            overdueStatus = "Start standup before using overdue shortcuts."
            return
        }
        guard item.actions.contains(action) else {
            overdueStatus = "That overdue action is not available for this task."
            return
        }

        let payload: DailyLoopOverdueReschedulePayloadData?
        if action == .reschedule {
            guard let guess = item.vel_due_guess else {
                overdueStatus = "Reschedule needs typed confirmation on iPhone or Mac."
                return
            }
            payload = DailyLoopOverdueReschedulePayloadData(
                due_at: guess.suggested_due_at,
                source: "vel_guess"
            )
        } else {
            payload = nil
        }

        do {
            let confirm = try await client.dailyLoopOverdueConfirm(
                sessionID: sessionID,
                request: DailyLoopOverdueConfirmRequestData(
                    commitment_id: item.commitment_id,
                    action: action,
                    payload: payload,
                    operator_reason: reason
                )
            )
            let applied = try await client.dailyLoopOverdueApply(
                sessionID: sessionID,
                request: DailyLoopOverdueApplyRequestData(
                    proposal_id: confirm.proposal_id,
                    idempotency_key: confirm.idempotency_hint,
                    confirmation_token: confirm.confirmation_token
                )
            )
            overdueStatus = "\(overdueActionLabel(action)) applied. Run \(applied.run_id)."
            await refreshOverdueMenu()
        } catch {
            overdueStatus = "Overdue action needs typed confirmation on iPhone or Mac."
        }
    }

    func applyVoiceOverdueShortcut(_ transcript: String) async {
        let trimmed = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }
        guard overdueItems.count == 1, let item = overdueItems.first else {
            overdueStatus = "Voice overdue shortcut needs exactly one visible overdue task."
            return
        }
        guard let action = parseOverdueVoiceAction(trimmed) else {
            overdueStatus = "Voice shortcut uncertain. Use typed buttons to confirm."
            return
        }
        await applyOverdueShortcut(item: item, action: action, reason: "apple_watch_voice_quick_reaction")
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

    private func parseOverdueVoiceAction(_ transcript: String) -> DailyLoopOverdueActionData? {
        let normalized = transcript.lowercased()
        let candidates: [DailyLoopOverdueActionData] = [
            normalized.contains("back to inbox") || normalized.contains("inbox") ? .backToInbox : nil,
            normalized.contains("reschedule") || normalized.contains("tomorrow") || normalized.contains("move it") ? .reschedule : nil,
            normalized.contains("delete") || normalized.contains("tombstone") ? .tombstone : nil,
            normalized.contains("close") || normalized.contains("done") || normalized.contains("complete") ? .close : nil,
        ].compactMap { $0 }
        return candidates.count == 1 ? candidates[0] : nil
    }
}

private func formatUnix(_ unix: Int) -> String {
    let formatter = DateFormatter()
    formatter.dateStyle = .none
    formatter.timeStyle = .short
    return formatter.string(from: Date(timeIntervalSince1970: TimeInterval(unix)))
}

private func dailyLoopDateString(from date: Date) -> String {
    let formatter = DateFormatter()
    formatter.calendar = Calendar(identifier: .gregorian)
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.timeZone = TimeZone.current
    formatter.dateFormat = "yyyy-MM-dd"
    return formatter.string(from: date)
}

private func overdueActionLabel(_ action: DailyLoopOverdueActionData) -> String {
    switch action {
    case .close:
        return "Close"
    case .reschedule:
        return "Reschedule"
    case .backToInbox:
        return "Back to inbox"
    case .tombstone:
        return "Delete"
    }
}
