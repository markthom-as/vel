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

// MARK: - Health

public typealias HealthResponse = APIEnvelope<HealthData>
public struct HealthData: Codable, Sendable {
    public let status: String?
    public let version: String?
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
        public let next_commitment_id: String?
        public let top_risk_commitment_ids: [String]?
        public let attention_state: String?
        public let drift_type: String?
    }
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
