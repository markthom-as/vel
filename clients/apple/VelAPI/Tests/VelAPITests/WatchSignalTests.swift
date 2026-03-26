import XCTest
@testable import VelAPI

#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

final class WatchSignalTests: XCTestCase {
    func testVelClientUsesWatchSignalRoute() async throws {
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [WatchSignalMockURLProtocol.self]
        let session = URLSession(configuration: config)
        let client = VelClient(
            baseURL: URL(string: "http://localhost:4130")!,
            session: session,
            configuration: VelClientConfiguration(operatorToken: "operator-secret")
        )

        var requests: [URLRequest] = []
        WatchSignalMockURLProtocol.handler = { request in
            requests.append(request)
            let path = request.url?.path ?? ""
            let response = HTTPURLResponse(
                url: request.url!,
                statusCode: 200,
                httpVersion: nil,
                headerFields: ["Content-Type": "application/json"]
            )!

            if path == "/v1/journal/watch-signal" {
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["signal_type"] as? String, "need_focus")
                XCTAssertEqual(payload["note"] as? String, "context switching")
                let context = payload["context"] as? [String: Any]
                XCTAssertEqual(context?["surface"] as? String, "watch")
                XCTAssertEqual(payload["source_device"] as? String, "apple_watch_signal")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "capture_id": "cap_watch_1",
                            "content_text": "watch signal: need_focus - context switching",
                            "capture_type": "watch_signal_need_focus",
                            "source_device": "apple_watch_signal",
                            "occurred_at": "2026-03-26T09:00:00Z",
                            "created_at": "2026-03-26T09:00:00Z"
                          },
                          "meta": { "request_id": "req_watch_signal" }
                        }
                        """
                        .utf8
                    )
                )
            }

            XCTFail("Unexpected request path: \(path)")
            return (response, Data())
        }

        let data = try await client.createWatchSignal(
            WatchSignalCreateRequestData(
                signal_type: "need_focus",
                note: "context switching",
                context: .object(["surface": .string("watch")]),
                source_device: "apple_watch_signal"
            )
        )

        XCTAssertEqual(data.capture_id, "cap_watch_1")
        XCTAssertEqual(requests.count, 1)
        XCTAssertEqual(requests.first?.value(forHTTPHeaderField: "x-vel-operator-token"), "operator-secret")
    }
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

private final class WatchSignalMockURLProtocol: URLProtocol {
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
