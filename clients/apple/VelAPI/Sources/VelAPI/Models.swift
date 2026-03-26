import Foundation

// MARK: - Common envelope (veld returns { ok, data, ... })

public struct APIEnvelope<T: Decodable>: Decodable {
    public let ok: Bool
    public let data: T?
    public let error: ErrorPayload?
    public let meta: MetaPayload?

    public struct ErrorPayload: Decodable {
        public let code: String?
        public let message: String?
    }

    public struct MetaPayload: Decodable {
        public let request_id: String?
        public let degraded: Bool?
    }
}

// MARK: - Flexible JSON

public indirect enum JSONValue: Codable, Sendable, Equatable {
    case string(String)
    case number(Double)
    case bool(Bool)
    case object([String: JSONValue])
    case array([JSONValue])
    case null

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if container.decodeNil() {
            self = .null
        } else if let value = try? container.decode(Bool.self) {
            self = .bool(value)
        } else if let value = try? container.decode(Double.self) {
            self = .number(value)
        } else if let value = try? container.decode(String.self) {
            self = .string(value)
        } else if let value = try? container.decode([String: JSONValue].self) {
            self = .object(value)
        } else if let value = try? container.decode([JSONValue].self) {
            self = .array(value)
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Unsupported JSON value"
            )
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .string(let value):
            try container.encode(value)
        case .number(let value):
            try container.encode(value)
        case .bool(let value):
            try container.encode(value)
        case .object(let value):
            try container.encode(value)
        case .array(let value):
            try container.encode(value)
        case .null:
            try container.encodeNil()
        }
    }

    public var compactText: String {
        switch self {
        case .string(let value):
            return value
        case .number(let value):
            if value.rounded() == value {
                return String(Int(value))
            }
            return String(value)
        case .bool(let value):
            return value ? "true" : "false"
        case .object(let value):
            if value.isEmpty {
                return "{}"
            }
            if let summary = value["summary"]?.compactText, !summary.isEmpty {
                return summary
            }
            return value
                .prefix(3)
                .map { key, element in "\(key): \(element.compactText)" }
                .joined(separator: ", ")
        case .array(let values):
            if values.isEmpty {
                return "[]"
            }
            return values.prefix(3).map(\.compactText).joined(separator: ", ")
        case .null:
            return "null"
        }
    }
}

// MARK: - Health

public typealias HealthResponse = APIEnvelope<HealthData>
public struct HealthData: Codable, Sendable {
    public let status: String?
    public let version: String?
}

// MARK: - Assistant entry (web surface continuation)

public typealias AssistantEntryResponse = APIEnvelope<AssistantEntryResponseData>

public struct AssistantEntryRequestData: Encodable, Sendable {
    public let text: String
    public let conversation_id: String?

    public init(text: String, conversationID: String? = nil) {
        self.text = text
        self.conversation_id = conversationID
    }
}

public struct AssistantEntryResponseData: Codable, Sendable {
    public let conversation: AssistantEntryConversationData?
    public let route_target: String?
    public let assistant_error: String?
    public let assistant_error_retryable: Bool?

    public init(
        conversation: AssistantEntryConversationData? = nil,
        route_target: String? = nil,
        assistant_error: String? = nil,
        assistant_error_retryable: Bool? = nil,
    ) {
        self.conversation = conversation
        self.route_target = route_target
        self.assistant_error = assistant_error
        self.assistant_error_retryable = assistant_error_retryable
    }
}

public struct AssistantEntryConversationData: Codable, Sendable {
    public let id: String

    public init(id: String) {
        self.id = id
    }
}

// MARK: - Apple quick loops

public typealias AppleVoiceTurnResponse = APIEnvelope<AppleVoiceTurnResponseData>
public typealias AppleBehaviorSummaryResponse = APIEnvelope<AppleBehaviorSummaryData>
public typealias NowResponse = APIEnvelope<NowData>
public typealias PlanningProfileResponse = APIEnvelope<PlanningProfileResponseData>
public typealias DailyLoopSessionResponse = APIEnvelope<DailyLoopSessionData>
public typealias DailyLoopActiveSessionResponse = APIEnvelope<DailyLoopSessionData?>

public enum AppleClientSurfaceData: String, Codable, Sendable {
    case iosVoice = "ios_voice"
    case iosCapture = "ios_capture"
    case watchBriefing = "watch_briefing"
    case watchQuickAction = "watch_quick_action"
    case macContext = "mac_context"
}

public enum AppleRequestedOperationData: String, Codable, Sendable {
    case captureOnly = "capture_only"
    case queryOnly = "query_only"
    case captureAndQuery = "capture_and_query"
    case mutation
}

public enum AppleVoiceIntentData: String, Codable, Sendable {
    case capture
    case morningBriefing = "morning_briefing"
    case currentSchedule = "current_schedule"
    case nextCommitment = "next_commitment"
    case activeNudges = "active_nudges"
    case explainWhy = "explain_why"
    case behaviorSummary = "behavior_summary"
    case completeCommitment = "complete_commitment"
    case snoozeNudge = "snooze_nudge"
}

public struct AppleTurnProvenanceData: Codable, Sendable {
    public let source_device: String?
    public let locale: String?
    public let transcript_origin: String?
    public let recorded_at: String?
    public let offline_captured_at: String?
    public let queued_at: String?

    public init(
        source_device: String?,
        locale: String?,
        transcript_origin: String?,
        recorded_at: String?,
        offline_captured_at: String?,
        queued_at: String?
    ) {
        self.source_device = source_device
        self.locale = locale
        self.transcript_origin = transcript_origin
        self.recorded_at = recorded_at
        self.offline_captured_at = offline_captured_at
        self.queued_at = queued_at
    }
}

public struct AppleVoiceTurnRequestData: Codable, Sendable {
    public let transcript: String
    public let surface: AppleClientSurfaceData
    public let operation: AppleRequestedOperationData
    public let intents: [AppleVoiceIntentData]
    public let provenance: AppleTurnProvenanceData?

    public init(
        transcript: String,
        surface: AppleClientSurfaceData,
        operation: AppleRequestedOperationData,
        intents: [AppleVoiceIntentData],
        provenance: AppleTurnProvenanceData?
    ) {
        self.transcript = transcript
        self.surface = surface
        self.operation = operation
        self.intents = intents
        self.provenance = provenance
    }
}

public enum AppleResponseModeData: String, Codable, Sendable {
    case spokenSummary = "spoken_summary"
    case card
    case confirmation
    case clarificationRequired = "clarification_required"
}

public struct AppleResponseEvidenceData: Codable, Sendable {
    public let kind: String
    public let label: String
    public let detail: String
    public let source_id: String?
}

public struct AppleVoiceTurnQueuedMutationSummaryData: Codable, Sendable {
    public let mutation_kind: String
    public let queued: Bool
    public let summary: String
    public let action_reference_id: String?
}

public struct AppleScheduleEventData: Codable, Sendable {
    public let title: String
    public let start_ts: Int
    public let end_ts: Int?
    public let location: String?
    public let leave_by_ts: Int?
}

public struct AppleScheduleSnapshotData: Codable, Sendable {
    public let generated_at: Int
    public let timezone: String
    public let focus_summary: String?
    public let next_event: AppleScheduleEventData?
    public let upcoming_events: [AppleScheduleEventData]
    public let reasons: [String]
}

public enum AppleBehaviorSummaryScopeData: String, Codable, Sendable {
    case daily
}

public struct AppleBehaviorMetricData: Codable, Sendable {
    public let metric_key: String
    public let display_label: String
    public let value: Double
    public let unit: String
    public let recorded_at: Int
    public let reasons: [String]
}

public struct AppleBehaviorSummaryData: Codable, Sendable {
    public let generated_at: Int
    public let timezone: String
    public let scope: AppleBehaviorSummaryScopeData
    public let headline: String
    public let metrics: [AppleBehaviorMetricData]
    public let reasons: [String]
    public let freshness_seconds: Int?
}

public struct AppleVoiceTurnResponseData: Codable, Sendable {
    public let operation: AppleRequestedOperationData
    public let mode: AppleResponseModeData
    public let summary: String
    public let capture_id: String?
    public let thread_id: String?
    public let reasons: [String]
    public let evidence: [AppleResponseEvidenceData]
    public let queued_mutation: AppleVoiceTurnQueuedMutationSummaryData?
    public let schedule: AppleScheduleSnapshotData?
    public let behavior_summary: AppleBehaviorSummaryData?
}

// MARK: - Daily loop

public enum DailyLoopPhaseData: String, Codable, Sendable {
    case morningOverview = "morning_overview"
    case standup
}

public enum DailyLoopStatusData: String, Codable, Sendable {
    case active
    case waitingForInput = "waiting_for_input"
    case completed
    case cancelled
}

public enum DailyLoopStartSourceData: String, Codable, Sendable {
    case manual
    case automatic
}

public enum DailyLoopSurfaceData: String, Codable, Sendable {
    case cli
    case web
    case appleVoice = "apple_voice"
    case appleText = "apple_text"
}

public enum DailyLoopTurnActionData: String, Codable, Sendable {
    case submit
    case skip
    case resume
}

public enum DailyLoopTurnStateData: String, Codable, Sendable {
    case inProgress = "in_progress"
    case waitingForInput = "waiting_for_input"
    case completed
}

public enum DailyLoopCommitmentActionData: String, Codable, Sendable {
    case accept
    case `defer`
    case choose
    case close
}

public enum DailyLoopPromptKindData: String, Codable, Sendable {
    case intentQuestion = "intent_question"
    case commitmentReduction = "commitment_reduction"
    case constraintCheck = "constraint_check"
}

public enum DailyStandupBucketData: String, Codable, Sendable {
    case must
    case should
    case stretch
}

public struct DailyLoopStartMetadataData: Codable, Sendable {
    public let source: DailyLoopStartSourceData
    public let surface: DailyLoopSurfaceData

    public init(source: DailyLoopStartSourceData, surface: DailyLoopSurfaceData) {
        self.source = source
        self.surface = surface
    }
}

public struct DailyLoopStartRequestData: Codable, Sendable {
    public let phase: DailyLoopPhaseData
    public let session_date: String
    public let start: DailyLoopStartMetadataData

    public init(phase: DailyLoopPhaseData, session_date: String, start: DailyLoopStartMetadataData) {
        self.phase = phase
        self.session_date = session_date
        self.start = start
    }
}

public struct DailyLoopTurnRequestData: Codable, Sendable {
    public let session_id: String
    public let action: DailyLoopTurnActionData
    public let response_text: String?

    public init(session_id: String, action: DailyLoopTurnActionData, response_text: String?) {
        self.session_id = session_id
        self.action = action
        self.response_text = response_text
    }
}

public enum DailyLoopCheckInSkipSourceData: String, Codable, Sendable {
    case user
    case inferred
}

public struct DailyLoopCheckInSkipRequestData: Codable, Sendable {
    public let source: DailyLoopCheckInSkipSourceData?
    public let answered_at: Int?
    public let reason_code: String?
    public let reason_text: String?

    public init(
        source: DailyLoopCheckInSkipSourceData? = nil,
        answered_at: Int? = nil,
        reason_code: String? = nil,
        reason_text: String? = nil
    ) {
        self.source = source
        self.answered_at = answered_at
        self.reason_code = reason_code
        self.reason_text = reason_text
    }
}

public struct DailyLoopCheckInSkipResponseData: Codable, Sendable {
    public let check_in_event_id: String
    public let session_id: String
    public let status: String
    public let supersedes_event_id: String?

    public init(
        check_in_event_id: String,
        session_id: String,
        status: String,
        supersedes_event_id: String?
    ) {
        self.check_in_event_id = check_in_event_id
        self.session_id = session_id
        self.status = status
        self.supersedes_event_id = supersedes_event_id
    }
}

public struct DailyLoopPromptData: Codable, Sendable {
    public let prompt_id: String
    public let kind: DailyLoopPromptKindData
    public let text: String
    public let ordinal: Int
    public let allow_skip: Bool
}

public struct MorningFrictionCalloutData: Codable, Sendable {
    public let label: String
    public let detail: String
}

public struct MorningIntentSignalData: Codable, Sendable {
    public let kind: String
    public let text: String
}

public enum DailyLoopCheckInResolutionKindData: String, Codable, Sendable {
    case submitted
    case bypassed
}

public struct DailyLoopCheckInResolutionData: Codable, Sendable {
    public let prompt_id: String
    public let ordinal: Int
    public let kind: DailyLoopCheckInResolutionKindData
    public let response_text: String?
    public let note_text: String?
}

public struct MorningOverviewStateData: Codable, Sendable {
    public let snapshot: String
    public let friction_callouts: [MorningFrictionCalloutData]
    public let signals: [MorningIntentSignalData]
    public let check_in_history: [DailyLoopCheckInResolutionData]
}

public struct DailyCommitmentDraftData: Codable, Sendable {
    public let title: String
    public let bucket: DailyStandupBucketData
    public let source_ref: String?
}

public struct DailyDeferredTaskData: Codable, Sendable {
    public let title: String
    public let source_ref: String?
    public let reason: String
}

public struct DailyFocusBlockProposalData: Codable, Sendable {
    public let label: String
    public let start_at: String
    public let end_at: String
    public let reason: String
}

public struct DailyStandupOutcomeData: Codable, Sendable {
    public let commitments: [DailyCommitmentDraftData]
    public let deferred_tasks: [DailyDeferredTaskData]
    public let confirmed_calendar: [String]
    public let focus_blocks: [DailyFocusBlockProposalData]
    public let check_in_history: [DailyLoopCheckInResolutionData]
}

public struct DailyLoopSessionStateData: Codable, Sendable {
    public let phase: DailyLoopPhaseData
    public let snapshot: String?
    public let friction_callouts: [MorningFrictionCalloutData]
    public let signals: [MorningIntentSignalData]
    public let commitments: [DailyCommitmentDraftData]
    public let deferred_tasks: [DailyDeferredTaskData]
    public let confirmed_calendar: [String]
    public let focus_blocks: [DailyFocusBlockProposalData]
    public let check_in_history: [DailyLoopCheckInResolutionData]
}

public struct DailyLoopSessionOutcomeData: Codable, Sendable {
    public let phase: DailyLoopPhaseData
    public let signals: [MorningIntentSignalData]
    public let commitments: [DailyCommitmentDraftData]
    public let deferred_tasks: [DailyDeferredTaskData]
    public let confirmed_calendar: [String]
    public let focus_blocks: [DailyFocusBlockProposalData]
    public let check_in_history: [DailyLoopCheckInResolutionData]
}

public struct DailyLoopSessionData: Codable, Sendable, Identifiable {
    public let id: String
    public let session_date: String
    public let phase: DailyLoopPhaseData
    public let status: DailyLoopStatusData
    public let start: DailyLoopStartMetadataData
    public let turn_state: DailyLoopTurnStateData
    public let current_prompt: DailyLoopPromptData?
    public let continuity_summary: String
    public let allowed_actions: [DailyLoopCommitmentActionData]
    public let state: DailyLoopSessionStateData
    public let outcome: DailyLoopSessionOutcomeData?
}

// MARK: - Planning profile

public enum RoutineBlockSourceKindData: String, Codable, Sendable {
    case inferred
    case operatorDeclared = "operator_declared"
}

public enum ScheduleTimeWindowData: String, Codable, Sendable {
    case prenoon
    case afternoon
    case evening
    case night
    case day
}

public enum PlanningConstraintKindData: String, Codable, Sendable {
    case maxScheduledItems = "max_scheduled_items"
    case reserveBufferBeforeCalendar = "reserve_buffer_before_calendar"
    case reserveBufferAfterCalendar = "reserve_buffer_after_calendar"
    case defaultTimeWindow = "default_time_window"
    case requireJudgmentForOverflow = "require_judgment_for_overflow"
}

public struct DurableRoutineBlockData: Codable, Sendable, Identifiable {
    public let id: String
    public let label: String
    public let source: RoutineBlockSourceKindData
    public let local_timezone: String
    public let start_local_time: String
    public let end_local_time: String
    public let days_of_week: [UInt8]
    public let protected: Bool
    public let active: Bool
}

public struct PlanningConstraintData: Codable, Sendable, Identifiable {
    public let id: String
    public let label: String
    public let kind: PlanningConstraintKindData
    public let detail: String?
    public let time_window: ScheduleTimeWindowData?
    public let minutes: UInt32?
    public let max_items: UInt32?
    public let active: Bool
}

public struct RoutinePlanningProfileData: Codable, Sendable {
    public let routine_blocks: [DurableRoutineBlockData]
    public let planning_constraints: [PlanningConstraintData]
}

public struct PlanningProfileResponseData: Codable, Sendable {
    public let profile: RoutinePlanningProfileData
    public let proposal_summary: PlanningProfileProposalSummaryData?
}

public struct PlanningProfileProposalSummaryItemData: Codable, Sendable {
    public let thread_id: String
    public let state: String
    public let title: String
    public let summary: String
    public let outcome_summary: String?
    public let updated_at: Int
}

public struct PlanningProfileProposalSummaryData: Codable, Sendable {
    public let pending_count: Int
    public let latest_pending: PlanningProfileProposalSummaryItemData?
    public let latest_applied: PlanningProfileProposalSummaryItemData?
    public let latest_failed: PlanningProfileProposalSummaryItemData?
}

public struct CommitmentSchedulingProposalSummaryItemData: Codable, Sendable {
    public let thread_id: String
    public let state: String
    public let title: String
    public let summary: String
    public let outcome_summary: String?
    public let updated_at: Int
}

public struct CommitmentSchedulingProposalSummaryData: Codable, Sendable {
    public let pending_count: Int
    public let latest_pending: CommitmentSchedulingProposalSummaryItemData?
    public let latest_applied: CommitmentSchedulingProposalSummaryItemData?
    public let latest_failed: CommitmentSchedulingProposalSummaryItemData?
}

// MARK: - Now

public struct NowLabelData: Codable, Sendable {
    public let key: String
    public let label: String
}

public enum NowHeaderBucketKindData: String, Codable, Sendable {
    case threads_by_type
    case needs_input
    case new_nudges
    case search_filter
    case snoozed
    case review_apply
    case reflow
    case follow_up
}

public enum NowCountDisplayModeData: String, Codable, Sendable {
    case always_show
    case show_nonzero
    case hidden_until_active
}

public struct NowThreadFilterTargetData: Codable, Sendable {
    public let bucket: NowHeaderBucketKindData
    public let thread_id: String?
}

public struct NowHeaderBucketData: Codable, Sendable, Identifiable {
    public var id: String { kind.rawValue }
    public let kind: NowHeaderBucketKindData
    public let count: Int
    public let count_display: NowCountDisplayModeData
    public let urgent: Bool
    public let route_target: NowThreadFilterTargetData
}

public struct NowHeaderData: Codable, Sendable {
    public let title: String
    public let buckets: [NowHeaderBucketData]
}

public enum NowMeshSyncStateData: String, Codable, Sendable {
    case synced
    case stale
    case local_only
    case offline
}

public enum NowRepairRouteTargetData: String, Codable, Sendable {
    case settings_sync
    case settings_linking
    case settings_recovery
}

public struct NowRepairRouteData: Codable, Sendable {
    public let target: NowRepairRouteTargetData
    public let summary: String
}

public struct NowMeshSummaryData: Codable, Sendable {
    public let authority_node_id: String
    public let authority_label: String
    public let sync_state: NowMeshSyncStateData
    public let linked_node_count: Int
    public let queued_write_count: Int
    public let last_sync_at: Int?
    public let urgent: Bool
    public let repair_route: NowRepairRouteData?
}

public struct NowStatusRowData: Codable, Sendable {
    public let date_label: String
    public let time_label: String
    public let context_label: String
    public let elapsed_label: String
}

public struct NowContextLineData: Codable, Sendable {
    public let text: String
    public let thread_id: String?
    public let fallback_used: Bool
}

public enum NowNudgeBarKindData: String, Codable, Sendable {
    case nudge
    case needs_input
    case review_request
    case reflow_proposal
    case thread_continuation
    case trust_warning
    case freshness_warning
}

public struct NowNudgeActionData: Codable, Sendable, Identifiable {
    public var id: String { kind + ":" + label }
    public let kind: String
    public let label: String
}

public struct NowNudgeBarData: Codable, Sendable, Identifiable {
    public let id: String
    public let kind: NowNudgeBarKindData
    public let title: String
    public let summary: String
    public let urgent: Bool
    public let primary_thread_id: String?
    public let actions: [NowNudgeActionData]
}

public enum NowTaskKindData: String, Codable, Sendable {
    case task
    case commitment
    case event
}

public struct NowTaskLaneItemData: Codable, Sendable, Identifiable {
    public let id: String
    public let task_kind: NowTaskKindData
    public let text: String
    public let state: String
    public let project: String?
    public let primary_thread_id: String?
}

public struct NowTaskLaneData: Codable, Sendable {
    public let active: NowTaskLaneItemData?
    public let pending: [NowTaskLaneItemData]
    public let recent_completed: [NowTaskLaneItemData]
    public let overflow_count: Int
}

public enum NowDockedInputIntentData: String, Codable, Sendable {
    case task
    case question
    case note
    case command
    case continuation
    case reflection
    case scheduling
}

public struct NowDockedInputData: Codable, Sendable {
    public let supported_intents: [NowDockedInputIntentData]
    public let day_thread_id: String?
    public let raw_capture_thread_id: String?
}

public struct NowRiskSummaryData: Codable, Sendable {
    public let level: String
    public let score: Double?
    public let label: String
}

public struct NowSummaryData: Codable, Sendable {
    public let mode: NowLabelData
    public let phase: NowLabelData
    public let meds: NowLabelData
    public let risk: NowRiskSummaryData
}

public struct NowEventData: Codable, Sendable {
    public let title: String
    public let start_ts: Int
    public let end_ts: Int?
    public let location: String?
    public let prep_minutes: Int?
    public let travel_minutes: Int?
    public let leave_by_ts: Int?
}

public struct NowTaskData: Codable, Sendable, Identifiable {
    public let id: String
    public let text: String
    public let source_type: String
    public let due_at: String?
    public let project: String?
    public let commitment_kind: String?
}

public struct NowScheduleData: Codable, Sendable {
    public let empty_message: String?
    public let next_event: NowEventData?
    public let upcoming_events: [NowEventData]
}

public struct NowTasksData: Codable, Sendable {
    public let todoist: [NowTaskData]
    public let other_open: [NowTaskData]
    public let next_commitment: NowTaskData?
}

public struct NowAttentionData: Codable, Sendable {
    public let state: NowLabelData
    public let drift: NowLabelData
    public let severity: NowLabelData
    public let confidence: Double?
    public let reasons: [String]
}

public struct NowSourceActivityData: Codable, Sendable {
    public let label: String
    public let timestamp: Int
    public let summary: JSONValue
}

public struct NowSourcesData: Codable, Sendable {
    public let git_activity: NowSourceActivityData?
    public let health: NowSourceActivityData?
    public let mood: NowSourceActivityData?
    public let pain: NowSourceActivityData?
    public let note_document: NowSourceActivityData?
    public let assistant_message: NowSourceActivityData?
}

public struct NowFreshnessEntryData: Codable, Sendable, Identifiable {
    public var id: String { key }
    public let key: String
    public let label: String
    public let status: String
    public let last_sync_at: Int?
    public let age_seconds: Int?
    public let guidance: String?
}

public struct NowFreshnessData: Codable, Sendable {
    public let overall_status: String
    public let sources: [NowFreshnessEntryData]
}

public struct NowDebugData: Codable, Sendable {
    public let raw_context: JSONValue
    public let signals_used: [String]
    public let commitments_used: [String]
    public let risk_used: [String]
}

public struct NowData: Codable, Sendable {
    public let computed_at: Int
    public let timezone: String
    public let header: NowHeaderData?
    public let mesh_summary: NowMeshSummaryData?
    public let status_row: NowStatusRowData?
    public let context_line: NowContextLineData?
    public let nudge_bars: [NowNudgeBarData]?
    public let task_lane: NowTaskLaneData?
    public let docked_input: NowDockedInputData?
    public let summary: NowSummaryData
    public let schedule: NowScheduleData
    public let tasks: NowTasksData
    public let attention: NowAttentionData
    public let sources: NowSourcesData
    public let freshness: NowFreshnessData
    public let planning_profile_summary: PlanningProfileProposalSummaryData?
    public let commitment_scheduling_summary: CommitmentSchedulingProposalSummaryData?
    public let action_items: [ActionItemData]
    public let reasons: [String]
    public let debug: NowDebugData
}

// MARK: - Cluster bootstrap

public typealias ClusterBootstrapResponse = APIEnvelope<ClusterBootstrapData>
public struct ClusterBootstrapData: Codable, Sendable {
    public let node_id: String
    public let node_display_name: String
    public let active_authority_node_id: String
    public let active_authority_epoch: Int
    public let configured_base_url: String
    public let sync_base_url: String
    public let sync_transport: String
    public let tailscale_base_url: String?
    public let lan_base_url: String?
    public let localhost_base_url: String?
    public let capabilities: [String]?
    public let branch_sync: BranchSyncCapabilityData?
    public let validation_profiles: [ValidationProfileData]?
    public let linked_nodes: [LinkedNodeData]
    public let projects: [ProjectRecordData]
    public let action_items: [ActionItemData]

    private enum CodingKeys: String, CodingKey {
        case node_id
        case node_display_name
        case active_authority_node_id
        case active_authority_epoch
        case configured_base_url
        case sync_base_url
        case sync_transport
        case tailscale_base_url
        case lan_base_url
        case localhost_base_url
        case capabilities
        case branch_sync
        case validation_profiles
        case linked_nodes
        case projects
        case action_items
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        node_id = try container.decode(String.self, forKey: .node_id)
        node_display_name = try container.decode(String.self, forKey: .node_display_name)
        active_authority_node_id = try container.decode(String.self, forKey: .active_authority_node_id)
        active_authority_epoch = try container.decode(Int.self, forKey: .active_authority_epoch)
        configured_base_url = try container.decode(String.self, forKey: .configured_base_url)
        sync_base_url = try container.decode(String.self, forKey: .sync_base_url)
        sync_transport = try container.decode(String.self, forKey: .sync_transport)
        tailscale_base_url = try container.decodeIfPresent(String.self, forKey: .tailscale_base_url)
        lan_base_url = try container.decodeIfPresent(String.self, forKey: .lan_base_url)
        localhost_base_url = try container.decodeIfPresent(String.self, forKey: .localhost_base_url)
        capabilities = try container.decodeIfPresent([String].self, forKey: .capabilities)
        branch_sync = try container.decodeIfPresent(BranchSyncCapabilityData.self, forKey: .branch_sync)
        validation_profiles = try container.decodeIfPresent([ValidationProfileData].self, forKey: .validation_profiles)
        linked_nodes = try container.decodeIfPresent([LinkedNodeData].self, forKey: .linked_nodes) ?? []
        projects = try container.decodeIfPresent([ProjectRecordData].self, forKey: .projects) ?? []
        action_items = try container.decodeIfPresent([ActionItemData].self, forKey: .action_items) ?? []
    }
}

public enum ProjectFamilyData: String, Codable, Sendable {
    case personal
    case creative
    case work
}

public enum ProjectStatusData: String, Codable, Sendable {
    case active
    case paused
    case archived
}

public struct ProjectRootRefData: Codable, Sendable {
    public let path: String
    public let label: String
    public let kind: String
}

public struct ProjectProvisionRequestData: Codable, Sendable {
    public let create_repo: Bool
    public let create_notes_root: Bool
}

public struct ProjectRecordData: Codable, Sendable, Identifiable {
    public let id: String
    public let slug: String
    public let name: String
    public let family: ProjectFamilyData
    public let status: ProjectStatusData
    public let primary_repo: ProjectRootRefData
    public let primary_notes_root: ProjectRootRefData
    public let secondary_repos: [ProjectRootRefData]
    public let secondary_notes_roots: [ProjectRootRefData]
    public let upstream_ids: [String: String]
    public let pending_provision: ProjectProvisionRequestData
    public let created_at: String
    public let updated_at: String
    public let archived_at: String?

    private enum CodingKeys: String, CodingKey {
        case id
        case slug
        case name
        case family
        case status
        case primary_repo
        case primary_notes_root
        case secondary_repos
        case secondary_notes_roots
        case upstream_ids
        case pending_provision
        case created_at
        case updated_at
        case archived_at
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(String.self, forKey: .id)
        slug = try container.decode(String.self, forKey: .slug)
        name = try container.decode(String.self, forKey: .name)
        family = try container.decode(ProjectFamilyData.self, forKey: .family)
        status = try container.decode(ProjectStatusData.self, forKey: .status)
        primary_repo = try container.decode(ProjectRootRefData.self, forKey: .primary_repo)
        primary_notes_root = try container.decode(ProjectRootRefData.self, forKey: .primary_notes_root)
        secondary_repos = try container.decodeIfPresent([ProjectRootRefData].self, forKey: .secondary_repos) ?? []
        secondary_notes_roots = try container.decodeIfPresent([ProjectRootRefData].self, forKey: .secondary_notes_roots) ?? []
        upstream_ids = try container.decodeIfPresent([String: String].self, forKey: .upstream_ids) ?? [:]
        pending_provision = try container.decodeIfPresent(ProjectProvisionRequestData.self, forKey: .pending_provision)
            ?? ProjectProvisionRequestData(create_repo: false, create_notes_root: false)
        created_at = try container.decode(String.self, forKey: .created_at)
        updated_at = try container.decode(String.self, forKey: .updated_at)
        archived_at = try container.decodeIfPresent(String.self, forKey: .archived_at)
    }
}

public enum ActionSurfaceData: String, Codable, Sendable {
    case now
    case inbox
}

public enum ActionKindData: String, Codable, Sendable {
    case next_step
    case intervention
    case review
    case freshness
    case blocked
    case conflict
    case linking
}

public enum ActionStateData: String, Codable, Sendable {
    case active
    case acknowledged
    case resolved
    case dismissed
    case snoozed
}

public struct ActionEvidenceRefData: Codable, Sendable {
    public let source_kind: String
    public let source_id: String
    public let label: String
    public let detail: String?
}

public struct ActionItemData: Codable, Sendable, Identifiable {
    public let id: String
    public let surface: ActionSurfaceData
    public let kind: ActionKindData
    public let title: String
    public let summary: String
    public let project_id: String?
    public let state: ActionStateData
    public let rank: Int
    public let surfaced_at: String
    public let snoozed_until: String?
    public let evidence: [ActionEvidenceRefData]

    private enum CodingKeys: String, CodingKey {
        case id
        case surface
        case kind
        case title
        case summary
        case project_id
        case state
        case rank
        case surfaced_at
        case snoozed_until
        case evidence
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(String.self, forKey: .id)
        surface = try container.decode(ActionSurfaceData.self, forKey: .surface)
        kind = try container.decode(ActionKindData.self, forKey: .kind)
        title = try container.decode(String.self, forKey: .title)
        summary = try container.decode(String.self, forKey: .summary)
        project_id = try container.decodeIfPresent(String.self, forKey: .project_id)
        state = try container.decode(ActionStateData.self, forKey: .state)
        rank = try container.decode(Int.self, forKey: .rank)
        surfaced_at = try container.decode(String.self, forKey: .surfaced_at)
        snoozed_until = try container.decodeIfPresent(String.self, forKey: .snoozed_until)
        evidence = try container.decodeIfPresent([ActionEvidenceRefData].self, forKey: .evidence) ?? []
    }
}

public struct LinkScopeData: Codable, Sendable {
    public let read_context: Bool
    public let write_safe_actions: Bool
    public let execute_repo_tasks: Bool

    public init(
        read_context: Bool,
        write_safe_actions: Bool,
        execute_repo_tasks: Bool
    ) {
        self.read_context = read_context
        self.write_safe_actions = write_safe_actions
        self.execute_repo_tasks = execute_repo_tasks
    }
}

public enum LinkStatusData: String, Codable, Sendable {
    case pending
    case linked
    case revoked
    case expired
}

public struct LinkedNodeData: Codable, Sendable, Identifiable {
    public var id: String { node_id }
    public let node_id: String
    public let node_display_name: String
    public let status: LinkStatusData
    public let scopes: LinkScopeData
    public let linked_at: String
    public let last_seen_at: String?
    public let transport_hint: String?
    public let sync_base_url: String?
    public let tailscale_base_url: String?
    public let lan_base_url: String?
    public let localhost_base_url: String?
    public let public_base_url: String?
}

public struct LinkTargetSuggestionData: Codable, Sendable, Identifiable {
    public var id: String { "\(label):\(base_url)" }
    public let label: String
    public let base_url: String
    public let transport_hint: String
    public let recommended: Bool
    public let redeem_command_hint: String
}

public struct PairingTokenData: Codable, Sendable {
    public let token_id: String
    public let token_code: String
    public let issued_at: String
    public let expires_at: String
    public let issued_by_node_id: String
    public let scopes: LinkScopeData
    public let suggested_targets: [LinkTargetSuggestionData]
}

public struct LinkingPromptData: Codable, Sendable {
    public let target_node_id: String
    public let target_node_display_name: String?
    public let issued_by_node_id: String
    public let issued_by_node_display_name: String?
    public let issued_at: String
    public let expires_at: String
    public let scopes: LinkScopeData
    public let issuer_sync_base_url: String
    public let issuer_sync_transport: String
    public let issuer_tailscale_base_url: String?
    public let issuer_lan_base_url: String?
    public let issuer_localhost_base_url: String?
    public let issuer_public_base_url: String?
}

public struct WorkerCapacityData: Codable, Sendable {
    public let max_concurrency: Int
    public let current_load: Int
    public let available_concurrency: Int
}

public struct WorkerPresenceData: Codable, Sendable, Identifiable {
    public var id: String { worker_id }
    public let worker_id: String
    public let node_id: String
    public let node_display_name: String
    public let client_kind: String?
    public let client_version: String?
    public let protocol_version: String?
    public let build_id: String?
    public let worker_classes: [String]
    public let capabilities: [String]
    public let status: String
    public let queue_depth: Int
    public let reachability: String
    public let latency_class: String
    public let compute_class: String
    public let power_class: String
    public let recent_failure_rate: Double
    public let tailscale_preferred: Bool
    public let last_heartbeat_at: Int
    public let started_at: Int?
    public let sync_base_url: String
    public let sync_transport: String
    public let tailscale_base_url: String?
    public let preferred_tailnet_endpoint: String?
    public let tailscale_reachable: Bool
    public let lan_base_url: String?
    public let localhost_base_url: String?
    public let ping_ms: Int?
    public let sync_status: String?
    public let last_upstream_sync_at: Int?
    public let last_downstream_sync_at: Int?
    public let last_sync_error: String?
    public let incoming_linking_prompt: LinkingPromptData?
    public let capacity: WorkerCapacityData
}

public struct ClusterWorkersData: Codable, Sendable {
    public let active_authority_node_id: String
    public let active_authority_epoch: Int
    public let generated_at: Int
    public let workers: [WorkerPresenceData]
}

public struct BranchSyncCapabilityData: Codable, Sendable {
    public let repo_root: String
    public let default_remote: String
    public let supports_fetch: Bool
    public let supports_pull: Bool
    public let supports_push: Bool
}

public struct ValidationProfileData: Codable, Sendable, Identifiable {
    public var id: String { profile_id }
    public let profile_id: String
    public let label: String
    public let command_hint: String
    public let environment: String
}

// MARK: - Sync bootstrap / action batch

public typealias SyncBootstrapResponse = APIEnvelope<SyncBootstrapData>
public struct SyncBootstrapData: Codable, Sendable {
    public let cluster: ClusterBootstrapData
    public let current_context: CurrentContextData?
    public let nudges: [NudgeData]
    public let commitments: [CommitmentData]
    public let linked_nodes: [LinkedNodeData]
    public let projects: [ProjectRecordData]
    public let action_items: [ActionItemData]

    private enum CodingKeys: String, CodingKey {
        case cluster
        case current_context
        case nudges
        case commitments
        case linked_nodes
        case projects
        case action_items
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        cluster = try container.decode(ClusterBootstrapData.self, forKey: .cluster)
        current_context = try container.decodeIfPresent(CurrentContextData.self, forKey: .current_context)
        nudges = try container.decodeIfPresent([NudgeData].self, forKey: .nudges) ?? []
        commitments = try container.decodeIfPresent([CommitmentData].self, forKey: .commitments) ?? []
        linked_nodes = try container.decodeIfPresent([LinkedNodeData].self, forKey: .linked_nodes)
            ?? cluster.linked_nodes
        projects = try container.decodeIfPresent([ProjectRecordData].self, forKey: .projects)
            ?? cluster.projects
        action_items = try container.decodeIfPresent([ActionItemData].self, forKey: .action_items)
            ?? cluster.action_items
    }
}

public typealias SyncActionsResponse = APIEnvelope<SyncActionsResultData>
public struct SyncActionRequestData: Codable, Sendable {
    public let action_id: String?
    public let action_type: String
    public let target_id: String?
    public let text: String?
    public let minutes: Int?
}

public struct SyncActionsRequestData: Codable, Sendable {
    public let actions: [SyncActionRequestData]
}

public struct PairingTokenIssueRequestData: Codable, Sendable {
    public let issued_by_node_id: String
    public let ttl_seconds: Int?
    public let scopes: LinkScopeData
    public let target_node_id: String?
    public let target_node_display_name: String?
    public let target_base_url: String?

    public init(
        issued_by_node_id: String,
        ttl_seconds: Int?,
        scopes: LinkScopeData,
        target_node_id: String?,
        target_node_display_name: String?,
        target_base_url: String?
    ) {
        self.issued_by_node_id = issued_by_node_id
        self.ttl_seconds = ttl_seconds
        self.scopes = scopes
        self.target_node_id = target_node_id
        self.target_node_display_name = target_node_display_name
        self.target_base_url = target_base_url
    }
}

public struct PairingTokenRedeemRequestData: Codable, Sendable {
    public let token_code: String
    public let node_id: String
    public let node_display_name: String
    public let transport_hint: String?
    public let requested_scopes: LinkScopeData?
    public let sync_base_url: String?
    public let tailscale_base_url: String?
    public let lan_base_url: String?
    public let localhost_base_url: String?
    public let public_base_url: String?

    public init(
        token_code: String,
        node_id: String,
        node_display_name: String,
        transport_hint: String?,
        requested_scopes: LinkScopeData?,
        sync_base_url: String?,
        tailscale_base_url: String?,
        lan_base_url: String?,
        localhost_base_url: String?,
        public_base_url: String?
    ) {
        self.token_code = token_code
        self.node_id = node_id
        self.node_display_name = node_display_name
        self.transport_hint = transport_hint
        self.requested_scopes = requested_scopes
        self.sync_base_url = sync_base_url
        self.tailscale_base_url = tailscale_base_url
        self.lan_base_url = lan_base_url
        self.localhost_base_url = localhost_base_url
        self.public_base_url = public_base_url
    }
}

public struct SyncActionResultData: Codable, Sendable {
    public let action_id: String?
    public let action_type: String
    public let target_id: String?
    public let status: String
    public let message: String
}

public struct SyncActionsResultData: Codable, Sendable {
    public let applied: Int
    public let results: [SyncActionResultData]
}

// MARK: - Context

public typealias CurrentContextResponse = APIEnvelope<CurrentContextData>
public struct CurrentContextData: Codable, Sendable {
    public let computed_at: Int?
    public let context: ContextPayload?

    public struct ContextPayload: Codable, Sendable {
        public let mode: String?
        public let morning_state: String?
        public let meds_status: String?
        public let prep_window_active: Bool?
        public let commute_window_active: Bool?
        public let next_commitment_id: String?
        public let leave_by_ts: Int?
        public let next_event_start_ts: Int?
        public let top_risk_commitment_ids: [String]?
        public let attention_state: String?
        public let drift_type: String?
        public let message_waiting_on_me_count: Int?
        public let message_urgent_thread_count: Int?
    }
}

// MARK: - Signals

public typealias SignalsResponse = APIEnvelope<[SignalData]>
public struct SignalData: Codable, Sendable, Identifiable {
    public var id: String { signal_id }
    public let signal_id: String
    public let signal_type: String
    public let source: String
    public let source_ref: String?
    public let timestamp: Int
    public let payload: JSONValue
    public let created_at: Int
}

// MARK: - Nudges

public typealias NudgesResponse = APIEnvelope<[NudgeData]>
public typealias NudgeResponse = APIEnvelope<NudgeData>
public struct NudgeData: Codable, Sendable, Identifiable {
    public var id: String { nudge_id }
    public let nudge_id: String
    public let nudge_type: String
    public let level: String
    public let state: String
    public let message: String
    public let created_at: Int?
    public let snoozed_until: Int?
    public let resolved_at: Int?
    public let related_commitment_id: String?
}

// MARK: - Commitments

public typealias CommitmentsResponse = APIEnvelope<[CommitmentData]>
public struct CommitmentData: Codable, Sendable, Identifiable {
    public let id: String
    public let text: String
    public let status: String
    public let due_at: Int?
    public let project: String?
    public let commitment_kind: String?
}

// MARK: - Captures

public typealias CaptureResponse = APIEnvelope<CaptureData>
public struct CaptureData: Codable, Sendable {
    public let capture_id: String?
    public let accepted_at: String?
}

// MARK: - Sync

public typealias SyncResponse = APIEnvelope<SyncResultData>
public struct SyncResultData: Codable, Sendable {
    public let source: String
    public let signals_ingested: Int
}

public enum VelLocalSourceKind: String, Codable, Sendable, CaseIterable {
    case activity
    case health
    case git
    case messaging
    case reminders
    case notes
    case transcripts
}

// MARK: - Connect runtime

public struct ConnectRuntimeCapabilityData: Codable, Sendable {
    public let runtime_id: String
    public let display_name: String
    public let supports_launch: Bool
    public let supports_interactive_followup: Bool
    public let supports_native_open: Bool
    public let supports_host_agent_control: Bool
}

public struct ConnectInstanceCapabilityManifestData: Codable, Sendable {
    public let worker_classes: [String]
    public let capabilities: [String]
    public let launchable_runtimes: [ConnectRuntimeCapabilityData]
    public let supports_agent_launch: Bool
    public let supports_interactive_followup: Bool
    public let supports_native_open: Bool
    public let supports_host_agent_control: Bool
}

public struct ConnectInstanceData: Codable, Sendable {
    public let id: String
    public let node_id: String
    public let display_name: String
    public let connection_id: String?
    public let status: String
    public let reachability: String
    public let sync_base_url: String?
    public let sync_transport: String?
    public let tailscale_base_url: String?
    public let lan_base_url: String?
    public let localhost_base_url: String?
    public let worker_ids: [String]
    public let worker_classes: [String]
    public let last_seen_at: String?
    public let manifest: ConnectInstanceCapabilityManifestData
    public let metadata: JSONValue
}

public struct ConnectCapabilityDescriptorData: Codable, Sendable {
    public let scope: String
    public let resource: String?
    public let action: String
}

public struct ConnectLaunchRequestData: Codable, Sendable {
    public let runtime_kind: String
    public let actor_id: String
    public let display_name: String?
    public let command: [String]
    public let working_dir: String?
    public let writable_roots: [String]
    public let capability_allowlist: [ConnectCapabilityDescriptorData]
    public let lease_seconds: Int?
}

public struct ConnectHeartbeatRequestData: Codable, Sendable {
    public let status: String
}

public struct ConnectHeartbeatResponseData: Codable, Sendable {
    public let id: String
    public let status: String
    public let lease_expires_at: Int
    public let trace_id: String
}

public struct ConnectTerminateRequestData: Codable, Sendable {
    public let reason: String
}

public struct ConnectStdinRequestData: Codable, Sendable {
    public let input: String
}

public struct ConnectStdinWriteAckData: Codable, Sendable {
    public let run_id: String
    public let accepted_bytes: Int
    public let event_id: Int
    public let trace_id: String?
}

public struct ConnectRunEventData: Codable, Sendable {
    public let id: Int
    public let run_id: String
    public let stream: String
    public let chunk: String
    public let created_at: Int
}

public struct ConnectAttachData: Codable, Sendable {
    public let instance: ConnectInstanceData
    public let latest_event_id: Int?
    public let stream_path: String
}

public struct LaunchExecutionHandoffRequestData: Codable, Sendable {
    public let runtime_kind: String
    public let actor_id: String?
    public let display_name: String?
    public let command: [String]
    public let working_dir: String?
    public let writable_roots: [String]
    public let capability_allowlist: [ConnectCapabilityDescriptorData]
    public let lease_seconds: Int?
}
