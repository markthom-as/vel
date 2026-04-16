import XCTest
@testable import VelAPI

#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

final class DailyLoopTests: XCTestCase {
    func testDailyLoopSessionDecodesMorningAndStandupPayloads() throws {
        let decoder = JSONDecoder()

        let morning = try decoder.decode(
            APIEnvelope<DailyLoopSessionData>.self,
            from: Data(
                """
                {
                  "ok": true,
                  "data": {
                    "id": "dls_morning_1",
                    "session_date": "2026-03-19",
                    "phase": "morning_overview",
                    "status": "waiting_for_input",
                    "start": {
                      "source": "manual",
                      "surface": "apple_voice"
                    },
                    "turn_state": "waiting_for_input",
                    "continuity_summary": "Morning overview is waiting on question 1 of 3 with 1 captured signal(s).",
                    "allowed_actions": ["accept", "defer", "choose", "close"],
                    "current_prompt": {
                      "prompt_id": "morning_prompt_1",
                      "kind": "intent_question",
                      "text": "What most needs to happen before noon?",
                      "ordinal": 1,
                      "allow_skip": true
                    },
                    "state": {
                      "phase": "morning_overview",
                      "snapshot": "Two meetings before noon.",
                      "friction_callouts": [
                        { "label": "Packed morning", "detail": "Calendar is dense." }
                      ],
                      "signals": [
                        { "kind": "must_do_hint", "text": "Ship Phase 10." }
                      ]
                    },
                    "outcome": {
                      "phase": "morning_overview",
                      "signals": [
                        { "kind": "focus_intent", "text": "Protect a focus block." }
                      ]
                    }
                  },
                  "meta": { "request_id": "req_1" }
                }
                """.utf8
            )
        )

        XCTAssertEqual(morning.data?.phase, .morningOverview)
        XCTAssertEqual(morning.data?.start.surface, .appleVoice)
        XCTAssertEqual(morning.data?.continuity_summary, "Morning overview is waiting on question 1 of 3 with 1 captured signal(s).")
        XCTAssertEqual(morning.data?.allowed_actions, [.accept, .`defer`, .choose, .close])
        XCTAssertEqual(morning.data?.state.snapshot, "Two meetings before noon.")
        XCTAssertEqual(morning.data?.current_prompt?.kind, .intentQuestion)
        XCTAssertEqual(morning.data?.outcome?.signals.first?.kind, "focus_intent")

        let standup = try decoder.decode(
            APIEnvelope<DailyLoopSessionData>.self,
            from: Data(
                """
                {
                  "ok": true,
                  "data": {
                    "id": "dls_standup_1",
                    "session_date": "2026-03-19",
                    "phase": "standup",
                    "status": "completed",
                    "start": {
                      "source": "manual",
                      "surface": "web"
                    },
                    "turn_state": "completed",
                    "continuity_summary": "Standup continuity is available.",
                    "allowed_actions": ["accept", "choose", "close"],
                    "current_prompt": null,
                    "state": {
                      "phase": "standup",
                      "commitments": [
                        { "title": "Ship Phase 10", "bucket": "must", "source_ref": "todo_1" }
                      ],
                      "deferred_tasks": [
                        { "title": "Inbox cleanup", "source_ref": null, "reason": "Not today." }
                      ],
                      "confirmed_calendar": ["Design review at 10:00"],
                      "focus_blocks": [
                        {
                          "label": "Deep work",
                          "start_at": "2026-03-19T13:00:00Z",
                          "end_at": "2026-03-19T15:00:00Z",
                          "reason": "Protect implementation time."
                        }
                      ]
                    },
                    "outcome": {
                      "phase": "standup",
                      "signals": [],
                      "commitments": [
                        { "title": "Ship Phase 10", "bucket": "must", "source_ref": "todo_1" }
                      ],
                      "deferred_tasks": [],
                      "confirmed_calendar": ["Design review at 10:00"],
                      "focus_blocks": []
                    }
                  },
                  "meta": { "request_id": "req_2" }
                }
                """.utf8
            )
        )

        XCTAssertEqual(standup.data?.phase, .standup)
        XCTAssertEqual(standup.data?.status, .completed)
        XCTAssertEqual(standup.data?.continuity_summary, "Standup continuity is available.")
        XCTAssertEqual(standup.data?.allowed_actions, [.accept, .choose, .close])
        XCTAssertEqual(standup.data?.state.commitments.first?.bucket, .must)
        XCTAssertEqual(standup.data?.outcome?.commitments.first?.title, "Ship Phase 10")

        let encoded = try JSONEncoder().encode(standup.data)
        let encodedJSON = try JSONSerialization.jsonObject(with: encoded) as? [String: Any]
        XCTAssertEqual(encodedJSON?["phase"] as? String, "standup")
    }

    func testVelClientUsesDailyLoopRoutes() async throws {
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [MockURLProtocol.self]
        let session = URLSession(configuration: config)
        let client = VelClient(
            baseURL: URL(string: "http://localhost:4130")!,
            session: session,
            configuration: VelClientConfiguration(operatorToken: "operator-secret")
        )

        var requests: [URLRequest] = []
        MockURLProtocol.handler = { request in
            requests.append(request)
            let path = request.url?.path ?? ""
            let query = request.url?.query ?? ""
            let response = HTTPURLResponse(
                url: request.url!,
                statusCode: 200,
                httpVersion: nil,
                headerFields: ["Content-Type": "application/json"]
            )!

            if path == "/v1/daily-loop/sessions" {
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["phase"] as? String, "morning_overview")
                return (response, Data(mockSessionEnvelopeJSON(id: "dls_1", phase: "morning_overview", status: "waiting_for_input").utf8))
            }

            if path == "/v1/daily-loop/sessions/active" {
                XCTAssertEqual(request.httpMethod, "GET")
                XCTAssertTrue(query.contains("session_date=2026-03-19"))
                XCTAssertTrue(query.contains("phase=standup"))
                return (response, Data(mockOptionalSessionEnvelopeJSON(id: "dls_2", phase: "standup", status: "waiting_for_input").utf8))
            }

            if path == "/v1/daily-loop/sessions/dls_2/turn" {
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["action"] as? String, "skip")
                return (response, Data(mockSessionEnvelopeJSON(id: "dls_2", phase: "standup", status: "completed").utf8))
            }

            if path == "/v1/daily-loop/check-ins/dci_2/skip" {
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["reason_code"] as? String, "not_applicable")
                XCTAssertEqual(payload["reason_text"] as? String, "in a meeting")
                XCTAssertEqual(payload["source"] as? String, "user")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "check_in_event_id": "dci_2",
                            "session_id": "dls_2",
                            "status": "applied",
                            "supersedes_event_id": "dci_1"
                          },
                          "meta": { "request_id": "req_skip" }
                        }
                        """
                        .utf8
                    )
                )
            }

            XCTFail("Unexpected request path: \(path)")
            return (response, Data())
        }

        _ = try await client.startDailyLoopSession(
            DailyLoopStartRequestData(
                phase: .morningOverview,
                session_date: "2026-03-19",
                start: DailyLoopStartMetadataData(source: .manual, surface: .appleVoice)
            )
        )
        _ = try await client.activeDailyLoopSession(sessionDate: "2026-03-19", phase: .standup)
        _ = try await client.submitDailyLoopTurn(sessionID: "dls_2", action: .skip)
        _ = try await client.skipDailyLoopCheckIn(
            checkInEventID: "dci_2",
            request: DailyLoopCheckInSkipRequestData(
                source: .user,
                answered_at: nil,
                reason_code: "not_applicable",
                reason_text: "in a meeting"
            )
        )

        XCTAssertEqual(requests.count, 4)
        XCTAssertEqual(requests.first?.value(forHTTPHeaderField: "x-vel-operator-token"), "operator-secret")
    }

    func testVelClientUsesDailyLoopOverdueRoutes() async throws {
        let config = URLSessionConfiguration.ephemeral
        config.protocolClasses = [MockURLProtocol.self]
        let session = URLSession(configuration: config)
        let client = VelClient(
            baseURL: URL(string: "http://localhost:4130")!,
            session: session,
            configuration: VelClientConfiguration(operatorToken: "operator-secret")
        )

        var requests: [URLRequest] = []
        MockURLProtocol.handler = { request in
            requests.append(request)
            let path = request.url?.path ?? ""
            let response = HTTPURLResponse(
                url: request.url!,
                statusCode: 200,
                httpVersion: nil,
                headerFields: ["Content-Type": "application/json"]
            )!

            switch path {
            case "/v1/daily-loop/sessions/dls_standup_1/overdue/menu":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["today"] as? String, "2026-04-16")
                XCTAssertEqual(payload["include_vel_guess"] as? Bool, true)
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "session_id": "dls_standup_1",
                            "items": [
                              {
                                "commitment_id": "com_overdue_1",
                                "title": "Reply to overdue task",
                                "due_at": "2026-04-15T16:00:00Z",
                                "actions": ["close", "reschedule", "back_to_inbox", "tombstone"],
                                "vel_due_guess": {
                                  "suggested_due_at": "2026-04-17T16:00:00Z",
                                  "confidence": "medium",
                                  "reason": "next free block + similar task duration"
                                }
                              }
                            ]
                          },
                          "meta": { "request_id": "req_overdue_menu" }
                        }
                        """
                        .utf8
                    )
                )
            case "/v1/daily-loop/sessions/dls_standup_1/overdue/confirm":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["commitment_id"] as? String, "com_overdue_1")
                XCTAssertEqual(payload["action"] as? String, "reschedule")
                let actionPayload = payload["payload"] as? [String: Any]
                XCTAssertEqual(actionPayload?["source"] as? String, "vel_guess")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "proposal_id": "mp_1",
                            "confirmation_token": "confirm:mp_1",
                            "requires_confirmation": true,
                            "write_scope": ["commitment:com_overdue_1:due_at"],
                            "idempotency_hint": "ovd:dls_standup_1:com_overdue_1:reschedule"
                          },
                          "meta": { "request_id": "req_overdue_confirm" }
                        }
                        """
                        .utf8
                    )
                )
            case "/v1/daily-loop/sessions/dls_standup_1/overdue/apply":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["proposal_id"] as? String, "mp_1")
                XCTAssertEqual(payload["confirmation_token"] as? String, "confirm:mp_1")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "applied": true,
                            "action_event_id": "evt_1",
                            "run_id": "run_1",
                            "before": { "due_at": "2026-04-15T16:00:00Z", "status": "open" },
                            "after": { "due_at": "2026-04-17T16:00:00Z", "status": "open" },
                            "undo_supported": true
                          },
                          "meta": { "request_id": "req_overdue_apply" }
                        }
                        """
                        .utf8
                    )
                )
            case "/v1/daily-loop/sessions/dls_standup_1/overdue/undo":
                XCTAssertEqual(request.httpMethod, "POST")
                let payload = try XCTUnwrap(jsonObject(from: request))
                XCTAssertEqual(payload["action_event_id"] as? String, "evt_1")
                return (
                    response,
                    Data(
                        """
                        {
                          "ok": true,
                          "data": {
                            "undone": true,
                            "run_id": "run_undo_1",
                            "before": { "due_at": "2026-04-17T16:00:00Z", "status": "open" },
                            "after": { "due_at": "2026-04-15T16:00:00Z", "status": "open" }
                          },
                          "meta": { "request_id": "req_overdue_undo" }
                        }
                        """
                        .utf8
                    )
                )
            default:
                XCTFail("Unexpected request path: \(path)")
                return (response, Data())
            }
        }

        let menu = try await client.dailyLoopOverdueMenu(
            sessionID: "dls_standup_1",
            request: DailyLoopOverdueMenuRequestData(today: "2026-04-16")
        )
        let confirm = try await client.dailyLoopOverdueConfirm(
            sessionID: "dls_standup_1",
            request: DailyLoopOverdueConfirmRequestData(
                commitment_id: "com_overdue_1",
                action: .reschedule,
                payload: DailyLoopOverdueReschedulePayloadData(
                    due_at: "2026-04-17T16:00:00Z",
                    source: "vel_guess"
                ),
                operator_reason: "watch quick reaction"
            )
        )
        let apply = try await client.dailyLoopOverdueApply(
            sessionID: "dls_standup_1",
            request: DailyLoopOverdueApplyRequestData(
                proposal_id: confirm.proposal_id,
                idempotency_key: confirm.idempotency_hint,
                confirmation_token: confirm.confirmation_token
            )
        )
        let undo = try await client.dailyLoopOverdueUndo(
            sessionID: "dls_standup_1",
            request: DailyLoopOverdueUndoRequestData(
                action_event_id: apply.action_event_id,
                idempotency_key: "ovd-undo:evt_1"
            )
        )

        XCTAssertEqual(menu.items.first?.actions, [.close, .reschedule, .backToInbox, .tombstone])
        XCTAssertEqual(confirm.proposal_id, "mp_1")
        XCTAssertTrue(apply.applied)
        XCTAssertTrue(undo.undone)
        XCTAssertEqual(requests.count, 4)
        XCTAssertTrue(requests.allSatisfy { $0.value(forHTTPHeaderField: "x-vel-operator-token") == "operator-secret" })
    }
}

private func mockSessionObjectJSON(id: String, phase: String, status: String) -> String {
    """
    {
      "id": "\(id)",
      "session_date": "2026-03-19",
      "phase": "\(phase)",
      "status": "\(status)",
      "start": {
        "source": "manual",
        "surface": "apple_voice"
      },
      "turn_state": "\(status == "completed" ? "completed" : "waiting_for_input")",
      "continuity_summary": "\(phase == "standup" ? "Standup continuity is available." : "Morning overview continuity is available.")",
      "allowed_actions": ["accept", "choose", "close"],
      "current_prompt": null,
      "state": {
        "phase": "\(phase)",
        "snapshot": "Snapshot",
        "friction_callouts": [],
        "signals": [],
        "commitments": [],
        "deferred_tasks": [],
        "confirmed_calendar": [],
        "focus_blocks": []
      },
      "outcome": null
    }
    """
}

private func mockSessionEnvelopeJSON(id: String, phase: String, status: String) -> String {
    """
    {
      "ok": true,
      "data": \(mockSessionObjectJSON(id: id, phase: phase, status: status)),
      "meta": { "request_id": "req_mock" }
    }
    """
}

private func mockOptionalSessionEnvelopeJSON(id: String, phase: String, status: String) -> String {
    """
    {
      "ok": true,
      "data": \(mockSessionObjectJSON(id: id, phase: phase, status: status)),
      "meta": { "request_id": "req_mock_optional" }
    }
    """
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

private final class MockURLProtocol: URLProtocol {
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
