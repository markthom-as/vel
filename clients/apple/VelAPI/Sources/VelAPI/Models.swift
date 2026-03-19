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

// MARK: - Apple quick loops

public typealias AppleVoiceTurnResponse = APIEnvelope<AppleVoiceTurnResponseData>
public typealias AppleBehaviorSummaryResponse = APIEnvelope<AppleBehaviorSummaryData>
public typealias NowResponse = APIEnvelope<NowData>

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
}

public struct AppleVoiceTurnRequestData: Codable, Sendable {
    public let transcript: String
    public let surface: AppleClientSurfaceData
    public let operation: AppleRequestedOperationData
    public let intents: [AppleVoiceIntentData]
    public let provenance: AppleTurnProvenanceData?
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
    public let reasons: [String]
    public let evidence: [AppleResponseEvidenceData]
    public let queued_mutation: AppleVoiceTurnQueuedMutationSummaryData?
    public let schedule: AppleScheduleSnapshotData?
    public let behavior_summary: AppleBehaviorSummaryData?
}

// MARK: - Now

public struct NowLabelData: Codable, Sendable {
    public let key: String
    public let label: String
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
    public let summary: NowSummaryData
    public let schedule: NowScheduleData
    public let tasks: NowTasksData
    public let attention: NowAttentionData
    public let sources: NowSourcesData
    public let freshness: NowFreshnessData
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
