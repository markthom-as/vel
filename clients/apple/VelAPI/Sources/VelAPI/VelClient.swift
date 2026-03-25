import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

/// HTTP client for the Vel daemon (veld) API. Apple shells stay thin over the same backend-owned core as web and CLI.
/// Configure baseURL (default http://localhost:4130) before use.
public final class VelClient {
    public var baseURL: URL
    public var configuration: VelClientConfiguration
    private let session: URLSession

    public init(
        baseURL: URL = URL(string: "http://localhost:4130")!,
        session: URLSession = .shared,
        configuration: VelClientConfiguration = .shared()
    ) {
        self.baseURL = baseURL
        self.session = session
        self.configuration = configuration
    }

    // MARK: - Health

    public func health() async throws -> HealthData {
        try await get("/v1/health")
    }

    public func clusterBootstrap() async throws -> ClusterBootstrapData {
        try await get("/v1/cluster/bootstrap")
    }

    public func clusterWorkers() async throws -> ClusterWorkersData {
        try await get("/v1/cluster/workers")
    }

    public func syncBootstrap() async throws -> SyncBootstrapData {
        try await get("/v1/sync/bootstrap")
    }

    // MARK: - Connect runtime

    public func listConnectInstances() async throws -> [ConnectInstanceData] {
        try await get("/v1/connect/instances")
    }

    public func getConnectInstance(id: String) async throws -> ConnectInstanceData {
        try await get("/v1/connect/instances/\(id)")
    }

    public func attachConnectInstance(id: String) async throws -> ConnectAttachData {
        try await get("/v1/connect/instances/\(id)/attach")
    }

    public func launchConnectInstance(_ request: ConnectLaunchRequestData) async throws -> ConnectInstanceData {
        try await post("/v1/connect/instances", body: request)
    }

    public func heartbeatConnectInstance(
        id: String,
        status: String = "healthy"
    ) async throws -> ConnectHeartbeatResponseData {
        try await post(
            "/v1/connect/instances/\(id)/heartbeat",
            body: ConnectHeartbeatRequestData(status: status)
        )
    }

    public func terminateConnectInstance(
        id: String,
        reason: String = "operator_requested"
    ) async throws -> ConnectInstanceData {
        try await post(
            "/v1/connect/instances/\(id)/terminate",
            body: ConnectTerminateRequestData(reason: reason)
        )
    }

    public func writeConnectInstanceStdin(
        id: String,
        input: String
    ) async throws -> ConnectStdinWriteAckData {
        try await post(
            "/v1/connect/instances/\(id)/stdin",
            body: ConnectStdinRequestData(input: input)
        )
    }

    public func listConnectInstanceEvents(
        id: String,
        afterID: Int? = nil,
        limit: Int? = nil
    ) async throws -> [ConnectRunEventData] {
        var query: [String] = []
        if let afterID {
            query.append("after_id=\(afterID)")
        }
        if let limit {
            query.append("limit=\(limit)")
        }
        let suffix = query.isEmpty ? "" : "?\(query.joined(separator: "&"))"
        return try await get("/v1/connect/instances/\(id)/events\(suffix)")
    }

    public func streamConnectInstanceEvents(
        id: String,
        afterID: Int? = nil,
        limit: Int? = nil,
        pollMS: Int? = nil,
        maxEvents: Int? = nil
    ) -> AsyncThrowingStream<ConnectRunEventData, Error> {
        var query: [String] = []
        if let afterID {
            query.append("after_id=\(afterID)")
        }
        if let limit {
            query.append("limit=\(limit)")
        }
        if let pollMS {
            query.append("poll_ms=\(pollMS)")
        }
        if let maxEvents {
            query.append("max_events=\(maxEvents)")
        }
        let suffix = query.isEmpty ? "" : "?\(query.joined(separator: "&"))"
        let path = "/v1/connect/instances/\(id)/events/stream\(suffix)"

        return AsyncThrowingStream { continuation in
            let task = Task {
                do {
                    #if canImport(FoundationNetworking)
                    var cursor = afterID
                    var emitted = 0
                    let clampedPollMS = max(50, pollMS ?? 500)
                    let sleepNanos = UInt64(clampedPollMS) * 1_000_000
                    while !Task.isCancelled {
                        let events = try await listConnectInstanceEvents(
                            id: id,
                            afterID: cursor,
                            limit: max(1, limit ?? 200)
                        )
                        if events.isEmpty {
                            try await Task.sleep(nanoseconds: sleepNanos)
                            continue
                        }
                        for event in events {
                            continuation.yield(event)
                            cursor = event.id
                            emitted += 1
                            if let maxEvents, emitted >= maxEvents {
                                continuation.finish()
                                return
                            }
                        }
                    }
                    continuation.finish()
                    #else
                    let request = try makeRequest(
                        path: path,
                        method: "GET",
                        body: nil,
                        accept: "text/event-stream"
                    )
                    let (bytes, response) = try await session.bytes(for: request)
                    guard let http = response as? HTTPURLResponse else {
                        continuation.finish()
                        return
                    }
                    guard (200..<300).contains(http.statusCode) else {
                        throw VelClientError.http(statusCode: http.statusCode, message: "connect stream failed")
                    }

                    var parser = ConnectSSEParser()
                    for try await line in bytes.lines {
                        if Task.isCancelled {
                            break
                        }
                        if let frame = parser.consume(line: line) {
                            if frame.eventName == "connect_error" {
                                throw VelClientError.apiError(frame.data)
                            }
                            if frame.eventName == "connect_event" || frame.eventName == nil {
                                let data = Data(frame.data.utf8)
                                let event = try JSONDecoder().decode(ConnectRunEventData.self, from: data)
                                continuation.yield(event)
                            }
                        }
                    }
                    continuation.finish()
                    #endif
                } catch {
                    continuation.finish(throwing: error)
                }
            }
            continuation.onTermination = { _ in
                task.cancel()
            }
        }
    }

    public func launchExecutionHandoff(
        handoffID: String,
        request: LaunchExecutionHandoffRequestData
    ) async throws -> ConnectInstanceData {
        try await post("/v1/execution/handoffs/\(handoffID)/launch", body: request)
    }

    // MARK: - Context

    public func currentContext() async throws -> CurrentContextData {
        try await get("/v1/context/current")
    }

    public func now() async throws -> NowData {
        try await get("/v1/now")
    }

    public func planningProfile() async throws -> PlanningProfileResponseData {
        try await get("/v1/planning-profile")
    }

    // MARK: - Signals / activity

    public func signals(
        signalType: String? = nil,
        sinceTs: Int? = nil,
        limit: Int = 50
    ) async throws -> [SignalData] {
        var items = ["limit=\(max(limit, 1))"]
        if let signalType, !signalType.isEmpty {
            items.append("signal_type=\(signalType.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? signalType)")
        }
        if let sinceTs {
            items.append("since_ts=\(sinceTs)")
        }
        return try await get("/v1/signals?\(items.joined(separator: "&"))")
    }

    // MARK: - Nudges

    public func nudges() async throws -> [NudgeData] {
        try await get("/v1/nudges")
    }

    public func nudgeDone(id: String) async throws -> NudgeData {
        try await post("/v1/nudges/\(id)/done", body: nil as String?)
    }

    public func nudgeSnooze(id: String, minutes: Int = 10) async throws -> NudgeData {
        try await post("/v1/nudges/\(id)/snooze", body: ["minutes": minutes])
    }

    // MARK: - Commitments

    public func commitments(status: String? = "open", limit: Int = 50) async throws -> [CommitmentData] {
        var path = "/v1/commitments?limit=\(limit)"
        if let s = status { path += "&status=\(s)" }
        return try await get(path)
    }

    public func createCommitment(
        text: String,
        sourceType: String = "apple",
        project: String? = nil,
        commitmentKind: String? = nil
    ) async throws -> CommitmentData {
        let body = CommitmentCreateBody(
            text: text,
            source_type: sourceType,
            project: project,
            commitment_kind: commitmentKind
        )
        return try await post("/v1/commitments", body: body)
    }

    public func markCommitmentDone(id: String) async throws -> CommitmentData {
        let body = CommitmentPatchBody(status: "done")
        return try await patch("/v1/commitments/\(id)", body: body)
    }

    // MARK: - Captures

    public func createCapture(text: String, type: String = "note", source: String = "apple") async throws -> CaptureData {
        let body = CaptureCreateBody(
            content_text: text,
            capture_type: type,
            source_device: source
        )
        return try await post("/v1/captures", body: body)
    }

    // MARK: - Assistant

    public func submitAssistantEntry(
        text: String,
        conversationID: String? = nil
    ) async throws -> AssistantEntryResponseData {
        let body = AssistantEntryRequestData(text: text, conversationID: conversationID)
        return try await post("/api/assistant/entry", body: body)
    }

    // MARK: - Local source sync

    public func syncLocalSource(_ source: VelLocalSourceKind) async throws -> SyncResultData {
        try await post("/v1/sync/\(source.rawValue)", body: Optional<String>.none)
    }

    public func syncActions(_ request: SyncActionsRequestData) async throws -> SyncActionsResultData {
        try await post("/v1/sync/actions", body: request)
    }

    // MARK: - Linking

    public func issuePairingToken(_ request: PairingTokenIssueRequestData) async throws -> PairingTokenData {
        try await post("/v1/linking/tokens", body: request)
    }

    public func redeemPairingToken(_ request: PairingTokenRedeemRequestData) async throws -> LinkedNodeData {
        try await post("/v1/linking/redeem", body: request)
    }

    public func revokeLinkedNode(nodeID: String) async throws -> LinkedNodeData {
        try await post("/v1/linking/revoke/\(nodeID)", body: Optional<String>.none)
    }

    public func linkingStatus() async throws -> [LinkedNodeData] {
        try await get("/v1/linking/status")
    }

    // MARK: - Apple quick loops

    /// Apple voice remains transcript-first and backend-owned.
    /// The route may return a shared thread hint when the backend persisted the turn
    /// into the same continuity substrate used by other assistant surfaces.
    public func appleVoiceTurn(_ request: AppleVoiceTurnRequestData) async throws -> AppleVoiceTurnResponseData {
        try await post("/v1/apple/voice/turn", body: request)
    }

    public func appleBehaviorSummary() async throws -> AppleBehaviorSummaryData {
        try await get("/v1/apple/behavior-summary")
    }

    public func startDailyLoopSession(_ request: DailyLoopStartRequestData) async throws -> DailyLoopSessionData {
        try await post("/v1/daily-loop/sessions", body: request)
    }

    public func activeDailyLoopSession(
        sessionDate: String,
        phase: DailyLoopPhaseData
    ) async throws -> DailyLoopSessionData? {
        let encodedDate = sessionDate.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? sessionDate
        return try await get("/v1/daily-loop/sessions/active?session_date=\(encodedDate)&phase=\(phase.rawValue)")
    }

    public func submitDailyLoopTurn(
        sessionID: String,
        action: DailyLoopTurnActionData,
        responseText: String? = nil
    ) async throws -> DailyLoopSessionData {
        try await post(
            "/v1/daily-loop/sessions/\(sessionID)/turn",
            body: DailyLoopTurnRequestData(
                session_id: sessionID,
                action: action,
                response_text: responseText
            )
        )
    }

    // MARK: - Private

    private func get<T: Decodable>(_ path: String) async throws -> T {
        let data = try await request(path: path, method: "GET", body: nil as Data?)
        let envelope = try JSONDecoder().decode(APIEnvelope<T>.self, from: data)
        guard envelope.ok, let value = envelope.data else {
            throw VelClientError.apiError(envelope.error?.message ?? "No data")
        }
        return value
    }

    private func post<T: Decodable, B: Encodable>(_ path: String, body: B?) async throws -> T {
        let bodyData = body.flatMap { try? JSONEncoder().encode($0) }
        let data = try await request(path: path, method: "POST", body: bodyData)
        let envelope = try JSONDecoder().decode(APIEnvelope<T>.self, from: data)
        guard envelope.ok, let value = envelope.data else {
            throw VelClientError.apiError(envelope.error?.message ?? "No data")
        }
        return value
    }

    private func patch<T: Decodable, B: Encodable>(_ path: String, body: B?) async throws -> T {
        let bodyData = body.flatMap { try? JSONEncoder().encode($0) }
        let data = try await request(path: path, method: "PATCH", body: bodyData)
        let envelope = try JSONDecoder().decode(APIEnvelope<T>.self, from: data)
        guard envelope.ok, let value = envelope.data else {
            throw VelClientError.apiError(envelope.error?.message ?? "No data")
        }
        return value
    }

    private func request(path: String, method: String, body: Data?) async throws -> Data {
        let request = try makeRequest(path: path, method: method, body: body)
        let (data, response) = try await send(request)
        guard let http = response as? HTTPURLResponse else { return data }
        guard (200..<300).contains(http.statusCode) else {
            let message = (try? JSONDecoder().decode(APIErrorEnvelope.self, from: data)).map { $0.error.message } ?? String(data: data, encoding: .utf8) ?? "Unknown error"
            throw VelClientError.http(statusCode: http.statusCode, message: message)
        }
        return data
    }

    private func makeRequest(path: String, method: String, body: Data?, accept: String = "application/json") throws -> URLRequest {
        guard let url = URL(string: path, relativeTo: baseURL) else {
            throw VelClientError.apiError("Invalid URL path: \(path)")
        }
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue(accept, forHTTPHeaderField: "Accept")
        applyAuthHeaders(to: &request)
        if method == "POST" || method == "PATCH" {
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            request.httpBody = body
        }
        return request
    }

    private func send(_ request: URLRequest) async throws -> (Data, URLResponse) {
        try await withCheckedThrowingContinuation { continuation in
            let task = session.dataTask(with: request) { data, response, error in
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                guard let data, let response else {
                    continuation.resume(throwing: VelClientError.apiError("No response"))
                    return
                }
                continuation.resume(returning: (data, response))
            }
            task.resume()
        }
    }

    private func applyAuthHeaders(to request: inout URLRequest) {
        if let bearerToken = configuration.bearerToken?.trimmingCharacters(in: .whitespacesAndNewlines),
           !bearerToken.isEmpty {
            request.setValue("Bearer \(bearerToken)", forHTTPHeaderField: "Authorization")
        }

        if let operatorToken = configuration.operatorToken?.trimmingCharacters(in: .whitespacesAndNewlines),
           !operatorToken.isEmpty {
            request.setValue(operatorToken, forHTTPHeaderField: "x-vel-operator-token")
        }
    }
}

public struct VelClientConfiguration: Sendable {
    public let operatorToken: String?
    public let bearerToken: String?

    public init(operatorToken: String? = nil, bearerToken: String? = nil) {
        self.operatorToken = operatorToken
        self.bearerToken = bearerToken
    }

    public static func shared(userDefaults: UserDefaults = .standard) -> VelClientConfiguration {
        func value(for keys: [String]) -> String? {
            for key in keys {
                if let raw = userDefaults.string(forKey: key)?
                    .trimmingCharacters(in: .whitespacesAndNewlines),
                   !raw.isEmpty {
                    return raw
                }
            }
            return nil
        }

        return VelClientConfiguration(
            operatorToken: value(for: ["vel_operator_token", "vel_operator_api_token"]),
            bearerToken: value(for: ["vel_bearer_token", "vel_api_bearer_token"])
        )
    }
}

public enum VelEndpointResolver {
    public static func candidateBaseURLs(
        explicitBaseURL: URL? = nil,
        userDefaults: UserDefaults = .standard
    ) -> [URL] {
        var candidates: [URL] = []

        func append(_ value: String?) {
            guard let value, let url = URL(string: value) else { return }
            if !candidates.contains(url) {
                candidates.append(url)
            }
        }

        if let explicitBaseURL {
            candidates.append(explicitBaseURL)
        }

        append(userDefaults.string(forKey: "vel_tailscale_url"))
        append(userDefaults.string(forKey: "vel_base_url"))
        append(userDefaults.string(forKey: "vel_lan_base_url"))
        append("http://127.0.0.1:4130")
        append("http://localhost:4130")

        return candidates
    }
}

public enum VelClientError: Error, Sendable {
    case http(statusCode: Int, message: String)
    case apiError(String)
    case decoding(Error)
}

private struct APIErrorEnvelope: Decodable {
    let error: APIError
    struct APIError: Decodable { let message: String }
}

private struct CaptureCreateBody: Encodable {
    let content_text: String
    let capture_type: String
    let source_device: String
}

private struct CommitmentCreateBody: Encodable {
    let text: String
    let source_type: String
    let project: String?
    let commitment_kind: String?
}

private struct CommitmentPatchBody: Encodable {
    let status: String
}

struct ConnectSSEFrame: Equatable {
    let eventName: String?
    let data: String
}

struct ConnectSSEParser {
    private var eventName: String?
    private var dataLines: [String] = []

    mutating func consume(line: String) -> ConnectSSEFrame? {
        if line.isEmpty {
            return flush()
        }
        if line.hasPrefix(":") {
            return nil
        }
        if line.hasPrefix("event:") {
            let value = String(line.dropFirst("event:".count))
            eventName = value.trimmingCharacters(in: .whitespaces)
            return nil
        }
        if line.hasPrefix("data:") {
            let value = String(line.dropFirst("data:".count))
            dataLines.append(value.trimmingCharacters(in: .whitespaces))
            return nil
        }
        return nil
    }

    private mutating func flush() -> ConnectSSEFrame? {
        guard !dataLines.isEmpty else {
            eventName = nil
            return nil
        }
        let frame = ConnectSSEFrame(eventName: eventName, data: dataLines.joined(separator: "\n"))
        eventName = nil
        dataLines.removeAll(keepingCapacity: true)
        return frame
    }
}
