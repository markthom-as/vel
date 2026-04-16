import XCTest
@testable import VelAPI

#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

final class WatchSurfaceActionTests: XCTestCase {
    override func tearDown() {
        WatchSurfaceMockURLProtocol.handler = nil
        super.tearDown()
    }

    func testVelClientUsesSharedNudgeAssistantAndCaptureRoutesForWatchActions() async throws {
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [WatchSurfaceMockURLProtocol.self]
        let session = URLSession(configuration: config)
        let client = VelClient(
            baseURL: URL(string: "http://localhost:4130")!,
            session: session,
            configuration: VelClientConfiguration(operatorToken: "operator-secret")
        )

        var requests: [URLRequest] = []
        WatchSurfaceMockURLProtocol.handler = { request in
            requests.append(request)
            let response = HTTPURLResponse(
                url: request.url!,
                statusCode: 200,
                httpVersion: nil,
                headerFields: ["Content-Type": "application/json"]
            )!
            let path = request.url?.path ?? ""

            switch path {
            case "/v1/nudges/nudge_1/done":
                XCTAssertEqual(request.httpMethod, "POST")
                return (response, nudgeEnvelope(id: "nudge_1", state: "resolved"))
            case "/v1/nudges/nudge_2/snooze":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["minutes"] as? Int, 10)
                return (response, nudgeEnvelope(id: "nudge_2", state: "snoozed"))
            case "/api/assistant/entry":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["text"] as? String, "Follow up from watch")
                XCTAssertEqual(payload["conversation_id"] as? String, "conv_watch_1")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "conversation": { "id": "conv_watch_1" },
                            "route_target": "thread"
                          },
                          "meta": { "request_id": "req_assistant" }
                        }
                        """.utf8
                    )
                )
            case "/v1/captures":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["content_text"] as? String, "watch fallback")
                XCTAssertEqual(payload["capture_type"] as? String, "watch_thread_capture")
                XCTAssertEqual(payload["source_device"] as? String, "apple_watch")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": { "capture_id": "cap_watch_fallback", "accepted_at": "2026-04-16T21:00:00Z" },
                          "meta": { "request_id": "req_capture" }
                        }
                        """.utf8
                    )
                )
            default:
                XCTFail("Unexpected request path: \(path)")
                return (response, Data())
            }
        }

        let done = try await client.nudgeDone(id: "nudge_1")
        let snoozed = try await client.nudgeSnooze(id: "nudge_2", minutes: 10)
        let assistant = try await client.submitAssistantEntry(text: "Follow up from watch", conversationID: "conv_watch_1")
        let capture = try await client.createCapture(
            text: "watch fallback",
            type: "watch_thread_capture",
            source: "apple_watch"
        )

        XCTAssertEqual(done.state, "resolved")
        XCTAssertEqual(snoozed.state, "snoozed")
        XCTAssertEqual(assistant.conversation?.id, "conv_watch_1")
        XCTAssertEqual(capture.capture_id, "cap_watch_fallback")
        XCTAssertEqual(requests.count, 4)
        XCTAssertTrue(requests.allSatisfy { $0.value(forHTTPHeaderField: "x-vel-operator-token") == "operator-secret" })
    }

    func testOfflineStoreAppliesWatchNudgeActionsAndPreservesWatchCaptureQueueSemantics() {
        let suiteName = "WatchSurfaceActionTests.offline.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        let store = VelOfflineStore(userDefaults: defaults)

        let createdAt = 1_710_000_000
        let nudges = [
            NudgeData(
                nudge_id: "nudge_done",
                nudge_type: "standup",
                level: "warning",
                state: "active",
                message: "Resolve me",
                created_at: createdAt,
                snoozed_until: nil,
                resolved_at: nil,
                related_commitment_id: nil
            ),
            NudgeData(
                nudge_id: "nudge_snooze",
                nudge_type: "standup",
                level: "warning",
                state: "active",
                message: "Snooze me",
                created_at: createdAt,
                snoozed_until: nil,
                resolved_at: nil,
                related_commitment_id: nil
            )
        ]

        store.enqueueNudgeDone(id: "nudge_done")
        store.enqueueNudgeSnooze(id: "nudge_snooze", minutes: 10)
        store.enqueueCaptureCreate(text: "queued_capture_metadata:\nrequested_capture_type: watch_thread_capture\nrequested_source_device: apple_watch\n\nwatch follow up")

        let now = Date(timeIntervalSince1970: TimeInterval(createdAt))
        let projected = store.applyPendingActions(to: nudges, now: now)
        let done = projected.first { $0.nudge_id == "nudge_done" }
        let snoozed = projected.first { $0.nudge_id == "nudge_snooze" }
        let queued = store.queuedActionRequests()

        XCTAssertEqual(done?.state, "resolved")
        XCTAssertEqual(done?.resolved_at, createdAt)
        XCTAssertEqual(snoozed?.state, "snoozed")
        XCTAssertEqual(snoozed?.snoozed_until, createdAt + 600)
        XCTAssertEqual(queued.map(\.action_type), ["nudge_done", "nudge_snooze", "capture_create"])
        XCTAssertEqual(queued[0].target_id, "nudge_done")
        XCTAssertEqual(queued[1].target_id, "nudge_snooze")
        XCTAssertEqual(queued[1].minutes, 10)
        XCTAssertTrue(queued[2].text?.contains("requested_capture_type: watch_thread_capture") == true)
        XCTAssertTrue(queued[2].text?.contains("requested_source_device: apple_watch") == true)
    }
}

private func nudgeEnvelope(id: String, state: String) -> Data {
    Data(
        """
        {
          "ok": true,
          "data": {
            "nudge_id": "\(id)",
            "nudge_type": "standup",
            "level": "warning",
            "state": "\(state)",
            "message": "Watch nudge",
            "created_at": 1710000000,
            "snoozed_until": null,
            "resolved_at": null,
            "related_commitment_id": null
          },
          "meta": { "request_id": "req_\(id)" }
        }
        """.utf8
    )
}

private func jsonObject(from requestBody: Data?) -> [String: Any]? {
    guard let requestBody, !requestBody.isEmpty else { return nil }
    return (try? JSONSerialization.jsonObject(with: requestBody)) as? [String: Any]
}

private func jsonObject(from request: URLRequest) -> [String: Any]? {
    if let payload = jsonObject(from: request.httpBody) {
        return payload
    }
    guard let stream = request.httpBodyStream else { return nil }
    let data = Data(reading: stream)
    return jsonObject(from: data)
}

private extension Data {
    init(reading stream: InputStream) {
        self.init()
        stream.open()
        defer { stream.close() }

        let bufferSize = 1024
        let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: bufferSize)
        defer { buffer.deallocate() }

        while stream.hasBytesAvailable {
            let readCount = stream.read(buffer, maxLength: bufferSize)
            guard readCount > 0 else { break }
            append(buffer, count: readCount)
        }
    }
}

private final class WatchSurfaceMockURLProtocol: URLProtocol {
    static var handler: ((URLRequest) throws -> (HTTPURLResponse, Data))?

    override class func canInit(with request: URLRequest) -> Bool {
        true
    }

    override class func canonicalRequest(for request: URLRequest) -> URLRequest {
        request
    }

    override func startLoading() {
        guard let handler = Self.handler else {
            client?.urlProtocol(self, didFailWithError: URLError(.badServerResponse))
            return
        }

        do {
            let (response, data) = try handler(request)
            client?.urlProtocol(self, didReceive: response, cacheStoragePolicy: .notAllowed)
            client?.urlProtocol(self, didLoad: data)
            client?.urlProtocolDidFinishLoading(self)
        } catch {
            client?.urlProtocol(self, didFailWithError: error)
        }
    }

    override func stopLoading() {}
}
