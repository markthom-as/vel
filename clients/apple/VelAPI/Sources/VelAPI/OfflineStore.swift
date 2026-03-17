import Foundation

public struct QueuedAction: Codable, Sendable, Identifiable {
    public enum Kind: String, Codable, Sendable {
        case nudgeDone = "nudge.done"
        case nudgeSnooze = "nudge.snooze"
        case commitmentDone = "commitment.done"
        case commitmentCreate = "commitment.create"
        case captureCreate = "capture.create"
    }

    public let id: UUID
    public let kind: Kind
    public let targetID: String?
    public let text: String?
    public let minutes: Int?
    public let queuedAt: Date

    public init(
        id: UUID = UUID(),
        kind: Kind,
        targetID: String? = nil,
        text: String? = nil,
        minutes: Int? = nil,
        queuedAt: Date = .now
    ) {
        self.id = id
        self.kind = kind
        self.targetID = targetID
        self.text = text
        self.minutes = minutes
        self.queuedAt = queuedAt
    }
}

public final class VelOfflineStore {
    private enum Keys {
        static let currentContext = "vel.cached.current_context"
        static let nudges = "vel.cached.nudges"
        static let commitments = "vel.cached.commitments"
        static let queuedActions = "vel.queued.actions"
    }

    private let userDefaults: UserDefaults
    private let encoder = JSONEncoder()
    private let decoder = JSONDecoder()

    public init(userDefaults: UserDefaults = .standard) {
        self.userDefaults = userDefaults
        encoder.dateEncodingStrategy = .iso8601
        decoder.dateDecodingStrategy = .iso8601
    }

    public func cachedContext() -> CurrentContextData? {
        decode(CurrentContextData.self, forKey: Keys.currentContext)
    }

    public func saveCachedContext(_ context: CurrentContextData) {
        encode(context, forKey: Keys.currentContext)
    }

    public func cachedNudges() -> [NudgeData] {
        decode([NudgeData].self, forKey: Keys.nudges) ?? []
    }

    public func saveCachedNudges(_ nudges: [NudgeData]) {
        encode(nudges, forKey: Keys.nudges)
    }

    public func cachedCommitments() -> [CommitmentData] {
        decode([CommitmentData].self, forKey: Keys.commitments) ?? []
    }

    public func saveCachedCommitments(_ commitments: [CommitmentData]) {
        encode(commitments, forKey: Keys.commitments)
    }

    public func queuedActions() -> [QueuedAction] {
        decode([QueuedAction].self, forKey: Keys.queuedActions) ?? []
    }

    public func pendingActionCount() -> Int {
        queuedActions().count
    }

    public func enqueueNudgeDone(id: String) {
        var actions = queuedActions()
        actions.append(QueuedAction(kind: .nudgeDone, targetID: id))
        saveQueuedActions(actions)
    }

    public func enqueueNudgeSnooze(id: String, minutes: Int) {
        var actions = queuedActions()
        actions.append(QueuedAction(kind: .nudgeSnooze, targetID: id, minutes: minutes))
        saveQueuedActions(actions)
    }

    public func enqueueCommitmentDone(id: String) {
        var actions = queuedActions()
        actions.append(QueuedAction(kind: .commitmentDone, targetID: id))
        saveQueuedActions(actions)
    }

    public func enqueueCommitmentCreate(text: String) {
        var actions = queuedActions()
        actions.append(QueuedAction(kind: .commitmentCreate, text: text))
        saveQueuedActions(actions)
    }

    public func enqueueCaptureCreate(text: String) {
        var actions = queuedActions()
        actions.append(QueuedAction(kind: .captureCreate, text: text))
        saveQueuedActions(actions)
    }

    public func hydrate(from bootstrap: SyncBootstrapData) {
        if let currentContext = bootstrap.current_context {
            saveCachedContext(currentContext)
        }
        saveCachedNudges(bootstrap.nudges)
        saveCachedCommitments(bootstrap.commitments)
    }

    public func queuedActionRequests() -> [SyncActionRequestData] {
        queuedActions().map { action in
            SyncActionRequestData(
                action_id: action.id.uuidString,
                action_type: action.kind.rawValue.replacingOccurrences(of: ".", with: "_"),
                target_id: action.targetID,
                text: action.text,
                minutes: action.minutes
            )
        }
    }

    @discardableResult
    public func drainQueuedActions(using client: VelClient) async -> Int {
        let actions = queuedActions()
        guard !actions.isEmpty else { return 0 }
        do {
            let result = try await client.syncActions(SyncActionsRequestData(actions: queuedActionRequests()))
            let remainingIDs = Set(
                result.results
                    .filter { $0.status != "applied" }
                    .compactMap(\.action_id)
            )
            if remainingIDs.isEmpty {
                saveQueuedActions([])
            } else {
                saveQueuedActions(actions.filter { remainingIDs.contains($0.id.uuidString) })
            }
            return result.applied
        } catch {
            return 0
        }
    }

    public func cachedNudgesApplyingPendingActions(now: Date = .now) -> [NudgeData] {
        applyPendingActions(to: cachedNudges(), now: now)
    }

    public func cachedCommitmentsApplyingPendingActions(now: Date = .now) -> [CommitmentData] {
        applyPendingActions(to: cachedCommitments(), now: now)
    }

    public func applyPendingActions(to nudges: [NudgeData], now: Date = .now) -> [NudgeData] {
        queuedActions().reduce(nudges) { partial, action in
            apply(action, to: partial, now: now)
        }
    }

    public func applyPendingActions(to commitments: [CommitmentData], now: Date = .now) -> [CommitmentData] {
        queuedActions().reduce(commitments) { partial, action in
            apply(action, to: partial, now: now)
        }
    }

    private func saveQueuedActions(_ actions: [QueuedAction]) {
        encode(actions, forKey: Keys.queuedActions)
    }

    private func apply(_ action: QueuedAction, to nudges: [NudgeData], now: Date) -> [NudgeData] {
        nudges.map { nudge in
            guard nudge.nudge_id == action.targetID else { return nudge }
            switch action.kind {
            case .nudgeDone:
                return NudgeData(
                    nudge_id: nudge.nudge_id,
                    nudge_type: nudge.nudge_type,
                    level: nudge.level,
                    state: "resolved",
                    message: nudge.message,
                    created_at: nudge.created_at,
                    snoozed_until: nudge.snoozed_until,
                    resolved_at: Int(now.timeIntervalSince1970),
                    related_commitment_id: nudge.related_commitment_id
                )
            case .nudgeSnooze:
                let minutes = action.minutes ?? 10
                let snoozedUntil = Int(now.addingTimeInterval(TimeInterval(minutes * 60)).timeIntervalSince1970)
                return NudgeData(
                    nudge_id: nudge.nudge_id,
                    nudge_type: nudge.nudge_type,
                    level: nudge.level,
                    state: "snoozed",
                    message: nudge.message,
                    created_at: nudge.created_at,
                    snoozed_until: snoozedUntil,
                    resolved_at: nudge.resolved_at,
                    related_commitment_id: nudge.related_commitment_id
                )
            case .commitmentDone, .commitmentCreate, .captureCreate:
                return nudge
            }
        }
    }

    private func apply(_ action: QueuedAction, to commitments: [CommitmentData], now _: Date) -> [CommitmentData] {
        switch action.kind {
        case .commitmentDone:
            return commitments.map { commitment in
                guard commitment.id == action.targetID else { return commitment }
                return CommitmentData(
                    id: commitment.id,
                    text: commitment.text,
                    status: "done",
                    due_at: commitment.due_at,
                    project: commitment.project,
                    commitment_kind: commitment.commitment_kind
                )
            }
        case .commitmentCreate:
            guard let text = action.text, !text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
                return commitments
            }
            if commitments.contains(where: { $0.text == text && $0.status == "open" }) {
                return commitments
            }
            let placeholder = CommitmentData(
                id: "queued-\(action.id.uuidString)",
                text: text,
                status: "open",
                due_at: nil,
                project: nil,
                commitment_kind: nil
            )
            return [placeholder] + commitments
        case .nudgeDone, .nudgeSnooze, .captureCreate:
            return commitments
        }
    }

    private func encode<T: Encodable>(_ value: T, forKey key: String) {
        guard let data = try? encoder.encode(value) else { return }
        userDefaults.set(data, forKey: key)
    }

    private func decode<T: Decodable>(_ type: T.Type, forKey key: String) -> T? {
        guard let data = userDefaults.data(forKey: key) else { return nil }
        return try? decoder.decode(type, from: data)
    }
}
