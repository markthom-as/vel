import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#elseif canImport(Darwin)
import Darwin
#if canImport(OSLog)
import OSLog
#endif
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

    public func discoveryBootstrap() async throws -> ClusterBootstrapData {
        try await get("/v1/discovery/bootstrap")
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

    public func conversations(
        archived: Bool = false,
        limit: Int = 100
    ) async throws -> [ConversationData] {
        try await get("/api/conversations?archived=\(archived ? "true" : "false")&limit=\(limit)")
    }

    public func conversationMessages(
        conversationID: String,
        limit: Int = 200
    ) async throws -> [MessageData] {
        let encodedID = conversationID.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? conversationID
        return try await get("/api/conversations/\(encodedID)/messages?limit=\(limit)")
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

    public func dailyLoopOverdueMenu(
        sessionID: String,
        request: DailyLoopOverdueMenuRequestData
    ) async throws -> DailyLoopOverdueMenuResponseData {
        try await post("/v1/daily-loop/sessions/\(sessionID)/overdue/menu", body: request)
    }

    public func dailyLoopOverdueConfirm(
        sessionID: String,
        request: DailyLoopOverdueConfirmRequestData
    ) async throws -> DailyLoopOverdueConfirmResponseData {
        try await post("/v1/daily-loop/sessions/\(sessionID)/overdue/confirm", body: request)
    }

    public func dailyLoopOverdueApply(
        sessionID: String,
        request: DailyLoopOverdueApplyRequestData
    ) async throws -> DailyLoopOverdueApplyResponseData {
        try await post("/v1/daily-loop/sessions/\(sessionID)/overdue/apply", body: request)
    }

    public func dailyLoopOverdueUndo(
        sessionID: String,
        request: DailyLoopOverdueUndoRequestData
    ) async throws -> DailyLoopOverdueUndoResponseData {
        try await post("/v1/daily-loop/sessions/\(sessionID)/overdue/undo", body: request)
    }

    public func skipDailyLoopCheckIn(
        checkInEventID: String,
        request: DailyLoopCheckInSkipRequestData
    ) async throws -> DailyLoopCheckInSkipResponseData {
        try await post(
            "/v1/daily-loop/check-ins/\(checkInEventID)/skip",
            body: request
        )
    }

    public func createWatchSignal(_ request: WatchSignalCreateRequestData) async throws -> CaptureData {
        try await post("/v1/journal/watch-signal", body: request)
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
    private static let discoveredBaseURLsKey = "vel_discovered_base_urls"
    private static let discoveryProtocolVersion = "vel_lan_discovery_v1"
    private static let discoveryBroadcastPort = 4131
    private static let discoveryResponseTimeoutSeconds = 0.25
    #if canImport(OSLog)
    private static let logger = Logger(subsystem: "vel.apple", category: "endpoint-resolver")
    #endif

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
        append(userDefaults.string(forKey: "tailscale_base_url"))
        append(userDefaults.string(forKey: "vel_base_url"))
        append(userDefaults.string(forKey: "base_url"))
        append(userDefaults.string(forKey: "vel_lan_base_url"))
        append(userDefaults.string(forKey: "lan_base_url"))
        for url in discoveredBaseURLs(userDefaults: userDefaults) {
            if !candidates.contains(url) {
                candidates.append(url)
            }
        }
        if shouldIncludeLoopbackFallbacks() {
            append("http://127.0.0.1:4130")
            append("http://localhost:4130")
        }

        log("candidate URLs: \(candidates.map(\.absoluteString).joined(separator: ", "))")

        return candidates
    }

    public static func discoveredBaseURLs(
        userDefaults: UserDefaults = .standard
    ) -> [URL] {
        let values = userDefaults.stringArray(forKey: discoveredBaseURLsKey) ?? []
        var urls: [URL] = []
        for value in values {
            guard let url = URL(string: value), !urls.contains(url) else { continue }
            urls.append(url)
        }
        return urls
    }

    public static func saveDiscoveredBaseURLs(
        _ urls: [URL],
        userDefaults: UserDefaults = .standard
    ) {
        let existing = discoveredBaseURLs(userDefaults: userDefaults)
        let merged = (urls + existing).reduce(into: [URL]()) { partial, url in
            guard !isLoopbackURL(url), !partial.contains(url) else { return }
            partial.append(url)
        }
        userDefaults.set(
            merged.prefix(12).map(\.absoluteString),
            forKey: discoveredBaseURLsKey
        )
    }

    public static func discoverLANBaseURLs(
        session: URLSession = .shared,
        userDefaults: UserDefaults = .standard
    ) async -> [URL] {
        var discovered: [URL] = []
        log("starting LAN discovery")
        let udpDiscovered = await discoverLANBaseURLsViaUDP()
        for url in udpDiscovered where !discovered.contains(url) {
            discovered.append(url)
        }
        if udpDiscovered.isEmpty {
            log("UDP discovery returned no routes")
        } else {
            log("UDP discovery routes: \(udpDiscovered.map(\.absoluteString).joined(separator: ", "))")
        }

        if discovered.isEmpty {
            let candidates = localDiscoveryProbeBaseURLs(userDefaults: userDefaults)
            log("HTTP discovery probe candidates: \(candidates.prefix(16).map(\.absoluteString).joined(separator: ", "))\(candidates.count > 16 ? " ..." : "")")
            await withTaskGroup(of: [URL].self) { group in
                for candidate in candidates {
                    group.addTask {
                        await probeDiscoveryBootstrap(at: candidate, session: session)
                    }
                }

                for await result in group {
                    for url in result where !discovered.contains(url) {
                        discovered.append(url)
                    }
                }
            }
        }

        saveDiscoveredBaseURLs(discovered, userDefaults: userDefaults)
        log("final discovered routes: \(discovered.map(\.absoluteString).joined(separator: ", "))")
        return discovered
    }

    private static func discoverLANBaseURLsViaUDP() async -> [URL] {
        #if canImport(Darwin)
        return await Task.detached(priority: .utility) {
            performUDPBroadcastDiscovery()
        }.value
        #else
        return []
        #endif
    }

    private static func probeDiscoveryBootstrap(
        at candidate: URL,
        session: URLSession
    ) async -> [URL] {
        let endpoint = candidate.appending(path: "/v1/discovery/bootstrap")
        var request = URLRequest(url: endpoint)
        request.timeoutInterval = 0.2

        do {
            let (data, response) = try await session.data(for: request)
            guard let http = response as? HTTPURLResponse, (200..<300).contains(http.statusCode) else {
                return []
            }
            let payload = try JSONDecoder().decode(DiscoveryBootstrapEnvelope.self, from: data)
            guard let cluster = payload.data else { return [] }
            let routes = bootstrapReachableBaseURLs(cluster: cluster)
            if !routes.isEmpty {
                log("HTTP discovery hit \(endpoint.absoluteString) -> \(routes.map(\.absoluteString).joined(separator: ", "))")
            }
            return routes
        } catch {
            return []
        }
    }

    private static func bootstrapReachableBaseURLs(cluster: ClusterBootstrapData) -> [URL] {
        let values = [
            cluster.sync_base_url,
            cluster.tailscale_base_url,
            cluster.lan_base_url,
            cluster.configured_base_url,
        ]

        var urls: [URL] = []
        for value in values {
            guard
                let value,
                let url = URL(string: value),
                !isLoopbackURL(url),
                !urls.contains(url)
            else { continue }
            urls.append(url)
        }
        return urls
    }

    private static func isLoopbackURL(_ url: URL) -> Bool {
        guard let host = url.host?.lowercased() else { return false }
        return host == "localhost" || host == "127.0.0.1" || host == "::1"
    }

    private static func localDiscoveryProbeBaseURLs(
        userDefaults: UserDefaults
    ) -> [URL] {
        let ports = discoveryProbePorts(userDefaults: userDefaults)
        let hostURLs = localIPv4ProbeHosts().flatMap { host in
            ports.compactMap { port in
                URL(string: "http://\(host):\(port)")
            }
        }

        var unique: [URL] = []
        for url in hostURLs where !unique.contains(url) {
            unique.append(url)
        }
        return unique
    }

    private static func discoveryProbePorts(
        userDefaults: UserDefaults
    ) -> [Int] {
        var ports: [Int] = []

        func append(port: Int?) {
            guard let port, (1...65535).contains(port), !ports.contains(port) else { return }
            ports.append(port)
        }

        let urlKeys = [
            "vel_tailscale_url",
            "tailscale_base_url",
            "vel_base_url",
            "base_url",
            "vel_lan_base_url",
            "lan_base_url",
        ]
        for key in urlKeys {
            guard let raw = userDefaults.string(forKey: key),
                  let url = URL(string: raw)
            else { continue }
            append(port: url.port)
            if url.port == nil {
                append(port: url.scheme?.lowercased() == "https" ? 443 : 80)
            }
        }

        append(port: 4130)
        append(port: 8443)
        log("HTTP discovery ports: \(ports.map(String.init).joined(separator: ", "))")
        return ports
    }

    #if canImport(Darwin)
    private static func performUDPBroadcastDiscovery() -> [URL] {
        let socketFD = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP)
        guard socketFD >= 0 else { return [] }
        defer { close(socketFD) }

        var enableBroadcast: Int32 = 1
        guard setsockopt(
            socketFD,
            SOL_SOCKET,
            SO_BROADCAST,
            &enableBroadcast,
            socklen_t(MemoryLayout<Int32>.size)
        ) == 0 else {
            return []
        }

        var timeout = timeval(tv_sec: 0, tv_usec: 50_000)
        _ = withUnsafePointer(to: &timeout) { pointer in
            setsockopt(
                socketFD,
                SOL_SOCKET,
                SO_RCVTIMEO,
                pointer,
                socklen_t(MemoryLayout<timeval>.size)
            )
        }

        var bindAddress = sockaddr_in()
        bindAddress.sin_len = UInt8(MemoryLayout<sockaddr_in>.size)
        bindAddress.sin_family = sa_family_t(AF_INET)
        bindAddress.sin_port = in_port_t(0).bigEndian
        bindAddress.sin_addr = in_addr(s_addr: in_addr_t(0))

        let bindResult = withUnsafePointer(to: &bindAddress) { pointer in
            pointer.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPointer in
                bind(socketFD, sockaddrPointer, socklen_t(MemoryLayout<sockaddr_in>.size))
            }
        }
        guard bindResult == 0 else { return [] }

        let requestID = "lan-disc-\(UUID().uuidString.lowercased())"
        let query = LANDiscoveryQuery(
            protocol: discoveryProtocolVersion,
            request_id: requestID,
            sender_node_id: "apple-client"
        )
        guard let payload = try? JSONEncoder().encode(query) else { return [] }

        var sent = false
        for host in localDiscoveryBroadcastHosts() {
            guard var destination = sockaddrIn(host: host, port: discoveryBroadcastPort) else {
                continue
            }
            let result = payload.withUnsafeBytes { bytes in
                withUnsafePointer(to: &destination) { pointer in
                    pointer.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPointer in
                        sendto(
                            socketFD,
                            bytes.baseAddress,
                            bytes.count,
                            0,
                            sockaddrPointer,
                            socklen_t(MemoryLayout<sockaddr_in>.size)
                        )
                    }
                }
            }
            if result >= 0 {
                sent = true
            }
        }
        guard sent else {
            log("UDP discovery broadcast send failed")
            return []
        }

        let deadline = Date().addingTimeInterval(discoveryResponseTimeoutSeconds)
        var discovered: [URL] = []
        var seenNodeIDs = Set<String>()

        while Date() < deadline {
            var buffer = [UInt8](repeating: 0, count: 16 * 1024)
            var sourceAddress = sockaddr_storage()
            var sourceLength = socklen_t(MemoryLayout<sockaddr_storage>.size)
            let received = withUnsafeMutablePointer(to: &sourceAddress) { sourcePointer in
                sourcePointer.withMemoryRebound(to: sockaddr.self, capacity: 1) { sockaddrPointer in
                    recvfrom(
                        socketFD,
                        &buffer,
                        buffer.count,
                        0,
                        sockaddrPointer,
                        &sourceLength
                    )
                }
            }
            guard received > 0 else { continue }

            let data = Data(buffer.prefix(received))
            guard
                let response = try? JSONDecoder().decode(LANDiscoveryResponse.self, from: data),
                response.protocol == discoveryProtocolVersion,
                response.request_id == requestID,
                seenNodeIDs.insert(response.cluster.node_id).inserted
            else {
                continue
            }

            for url in bootstrapReachableBaseURLs(cluster: response.cluster) where !discovered.contains(url) {
                discovered.append(url)
            }
        }

        if discovered.isEmpty {
            log("UDP discovery received no responses")
        }
        return discovered
    }

    private static func localDiscoveryBroadcastHosts() -> [String] {
        var hosts: [String] = ["255.255.255.255"]
        var addresses: UnsafeMutablePointer<ifaddrs>?
        guard getifaddrs(&addresses) == 0, let first = addresses else { return hosts }
        defer { freeifaddrs(addresses) }

        var pointer: UnsafeMutablePointer<ifaddrs>? = first
        while let current = pointer {
            let interface = current.pointee
            pointer = interface.ifa_next

            guard
                let address = interface.ifa_addr,
                let netmask = interface.ifa_netmask
            else { continue }

            let flags = Int32(interface.ifa_flags)
            let isUp = (flags & IFF_UP) != 0
            let isLoopback = (flags & IFF_LOOPBACK) != 0
            guard isUp, !isLoopback, address.pointee.sa_family == UInt8(AF_INET) else {
                continue
            }

            let ip = address.withMemoryRebound(to: sockaddr_in.self, capacity: 1) { pointer in
                UInt32(bigEndian: pointer.pointee.sin_addr.s_addr)
            }
            let mask = netmask.withMemoryRebound(to: sockaddr_in.self, capacity: 1) { pointer in
                UInt32(bigEndian: pointer.pointee.sin_addr.s_addr)
            }

            let octets = (
                Int((ip >> 24) & 0xff),
                Int((ip >> 16) & 0xff),
                Int((ip >> 8) & 0xff),
                Int(ip & 0xff)
            )
            guard shouldProbeLANSubnet(octets) else { continue }

            let broadcast = ip | ~mask
            let host = [
                String((broadcast >> 24) & 0xff),
                String((broadcast >> 16) & 0xff),
                String((broadcast >> 8) & 0xff),
                String(broadcast & 0xff),
            ].joined(separator: ".")
            if !hosts.contains(host) {
                hosts.append(host)
            }
        }

        log("UDP discovery broadcast hosts: \(hosts.joined(separator: ", "))")
        return hosts
    }

    private static func sockaddrIn(host: String, port: Int) -> sockaddr_in? {
        guard (1...65535).contains(port) else { return nil }
        var address = sockaddr_in()
        address.sin_len = UInt8(MemoryLayout<sockaddr_in>.size)
        address.sin_family = sa_family_t(AF_INET)
        address.sin_port = in_port_t(port).bigEndian

        let status = host.withCString { hostPointer in
            inet_pton(AF_INET, hostPointer, &address.sin_addr)
        }
        return status == 1 ? address : nil
    }
    #endif

    private static func localIPv4ProbeHosts() -> [String] {
        var hosts: [String] = []
        var addresses: UnsafeMutablePointer<ifaddrs>?
        guard getifaddrs(&addresses) == 0, let first = addresses else { return [] }
        defer { freeifaddrs(addresses) }

        var pointer: UnsafeMutablePointer<ifaddrs>? = first
        while let current = pointer {
            let interface = current.pointee
            pointer = interface.ifa_next

            guard let address = interface.ifa_addr else { continue }

            let flags = Int32(interface.ifa_flags)
            let isUp = (flags & IFF_UP) != 0
            let isLoopback = (flags & IFF_LOOPBACK) != 0
            guard isUp, !isLoopback, address.pointee.sa_family == UInt8(AF_INET) else {
                continue
            }

            let ipv4 = address.withMemoryRebound(to: sockaddr_in.self, capacity: 1) { pointer in
                UInt32(bigEndian: pointer.pointee.sin_addr.s_addr)
            }
            let octets = (
                Int((ipv4 >> 24) & 0xff),
                Int((ipv4 >> 16) & 0xff),
                Int((ipv4 >> 8) & 0xff),
                Int(ipv4 & 0xff)
            )
            guard shouldProbeLANSubnet(octets) else { continue }

            for lastOctet in 1...254 {
                guard lastOctet != octets.3 else { continue }
                let candidate = "\(octets.0).\(octets.1).\(octets.2).\(lastOctet)"
                if !hosts.contains(candidate) {
                    hosts.append(candidate)
                }
            }
        }

        log("HTTP discovery probe hosts count: \(hosts.count)")
        return hosts
    }

    private static func shouldProbeLANSubnet(_ octets: (Int, Int, Int, Int)) -> Bool {
        switch octets {
        case (10, _, _, _):
            return true
        case (172, 16...31, _, _):
            return true
        case (192, 168, _, _):
            return true
        case (100, 64...127, _, _):
            // Include carrier-grade / Tailnet-style addresses, but avoid scanning arbitrary public subnets.
            return true
        default:
            return false
        }
    }

    private static func shouldIncludeLoopbackFallbacks() -> Bool {
        #if targetEnvironment(simulator)
        return true
        #else
        return false
        #endif
    }

    private static func log(_ message: String) {
        #if canImport(OSLog)
        logger.info("\(message)")
        #endif
    }

}

private struct DiscoveryBootstrapEnvelope: Decodable {
    let data: ClusterBootstrapData?
}

private struct LANDiscoveryQuery: Encodable {
    let `protocol`: String
    let request_id: String
    let sender_node_id: String
}

private struct LANDiscoveryResponse: Decodable {
    let `protocol`: String
    let request_id: String
    let cluster: ClusterBootstrapData
}

public enum VelClientError: Error, Sendable {
    case http(statusCode: Int, message: String)
    case apiError(String)
    case decoding(Error)
}

extension VelClientError: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case let .http(statusCode, message):
            let trimmed = message.trimmingCharacters(in: .whitespacesAndNewlines)
            return trimmed.isEmpty ? "Request failed with status \(statusCode)." : "Request failed (\(statusCode)): \(trimmed)"
        case let .apiError(message):
            let trimmed = message.trimmingCharacters(in: .whitespacesAndNewlines)
            return trimmed.isEmpty ? "The Vel API returned an empty error." : trimmed
        case let .decoding(error):
            return "Could not decode Vel API response. \(error.localizedDescription)"
        }
    }
}

extension VelClientError: CustomStringConvertible {
    public var description: String {
        errorDescription ?? String(describing: self)
    }
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
