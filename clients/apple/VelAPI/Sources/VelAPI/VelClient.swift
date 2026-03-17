import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

/// HTTP client for the Vel daemon (veld) API. All clients talk to the same core.
/// Configure baseURL (default http://localhost:4130) before use.
public final class VelClient {
    public var baseURL: URL
    private let session: URLSession

    public init(baseURL: URL = URL(string: "http://localhost:4130")!, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
    }

    // MARK: - Health

    public func health() async throws -> HealthData {
        try await get("/v1/health")
    }

    public func clusterBootstrap() async throws -> ClusterBootstrapData {
        try await get("/v1/cluster/bootstrap")
    }

    public func syncBootstrap() async throws -> SyncBootstrapData {
        try await get("/v1/sync/bootstrap")
    }

    // MARK: - Context

    public func currentContext() async throws -> CurrentContextData {
        try await get("/v1/context/current")
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

    // MARK: - Local source sync

    public func syncLocalSource(_ source: VelLocalSourceKind) async throws -> SyncResultData {
        try await post("/v1/sync/\(source.rawValue)", body: Optional<String>.none)
    }

    public func syncActions(_ request: SyncActionsRequestData) async throws -> SyncActionsResultData {
        try await post("/v1/sync/actions", body: request)
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
        guard let url = URL(string: path, relativeTo: baseURL) else {
            throw VelClientError.apiError("Invalid URL path: \(path)")
        }
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        if method == "POST" || method == "PATCH" {
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            request.httpBody = body
        }
        let (data, response) = try await send(request)
        guard let http = response as? HTTPURLResponse else { return data }
        guard (200..<300).contains(http.statusCode) else {
            let message = (try? JSONDecoder().decode(APIErrorEnvelope.self, from: data)).map { $0.error.message } ?? String(data: data, encoding: .utf8) ?? "Unknown error"
            throw VelClientError.http(statusCode: http.statusCode, message: message)
        }
        return data
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

        append(userDefaults.string(forKey: "vel_base_url"))
        append(userDefaults.string(forKey: "vel_tailscale_url"))
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
