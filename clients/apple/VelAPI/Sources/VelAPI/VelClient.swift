import Foundation

/// HTTP client for the Vel daemon (veld) API. All clients talk to the same core.
/// Configure baseURL (default http://localhost:4130) before use.
public final class VelClient: Sendable {
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

    // MARK: - Context

    public func currentContext() async throws -> CurrentContextData {
        try await get("/v1/context/current")
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

    // MARK: - Captures

    public func createCapture(text: String, type: String = "note", source: String = "apple") async throws -> CaptureData {
        try await post("/v1/captures", body: ["text": text, "capture_type": type, "source": source])
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

    private func request(path: String, method: String, body: Data?) async throws -> Data {
        let url = baseURL.appendingPathComponent(path.dropFirst())
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        if method == "POST" {
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            request.httpBody = body
        }
        let (data, response) = try await session.data(for: request)
        guard let http = response as? HTTPURLResponse else { return data }
        guard (200..<300).contains(http.statusCode) else {
            let message = (try? JSONDecoder().decode(APIErrorEnvelope.self, from: data)).map { $0.error.message } ?? String(data: data, encoding: .utf8) ?? "Unknown error"
            throw VelClientError.http(statusCode: http.statusCode, message: message)
        }
        return data
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
