import XCTest
@testable import VelApplication

final class WatchSurfaceActionServiceTests: XCTestCase {
    func testMapsWatchNudgeActionsToSharedClientRoutes() async throws {
        let client = RecordingWatchSurfaceActionClient()
        let service = WatchSurfaceActionService(client: client)

        let done = try await service.markNudgeDone(id: " nudge_done ")
        let snoozed = try await service.snoozeNudge(id: "nudge_snooze", minutes: 15)

        XCTAssertEqual(done, .nudgeResolved(id: "nudge_done"))
        XCTAssertEqual(snoozed, .nudgeSnoozed(id: "nudge_snooze", minutes: 15))
        let operations = await client.operationSnapshot()
        XCTAssertEqual(operations, [
            .nudgeDone(id: "nudge_done"),
            .nudgeSnooze(id: "nudge_snooze", minutes: 15),
        ])
    }

    func testMapsWatchThreadAppendToAssistantEntryWhenThreadExists() async throws {
        let client = RecordingWatchSurfaceActionClient()
        let service = WatchSurfaceActionService(client: client)

        let outcome = try await service.appendToThread(
            text: " Follow up from watch ",
            conversationID: " conv_watch_1 ",
        )

        XCTAssertEqual(outcome, .threadAppended(conversationID: "conv_watch_1"))
        let operations = await client.operationSnapshot()
        XCTAssertEqual(operations, [
            .assistantEntry(text: "Follow up from watch", conversationID: "conv_watch_1"),
        ])
    }

    func testFallsBackToWatchCaptureWhenThreadAppendCannotReachAssistantEntry() async throws {
        let client = RecordingWatchSurfaceActionClient(failAssistantEntry: true)
        let service = WatchSurfaceActionService(client: client)

        let outcome = try await service.appendToThread(
            text: "Follow up later",
            conversationID: "conv_watch_1",
        )

        XCTAssertEqual(outcome, .captureCreated(type: "watch_thread_capture", source: "apple_watch"))
        let operations = await client.operationSnapshot()
        XCTAssertEqual(operations, [
            .assistantEntry(text: "Follow up later", conversationID: "conv_watch_1"),
            .capture(
                text: "watch_thread_append:\nthread_id: conv_watch_1\n\nFollow up later",
                type: "watch_thread_capture",
                source: "apple_watch",
            ),
        ])
    }

    func testFallsBackToWatchCaptureWhenNoThreadExists() async throws {
        let client = RecordingWatchSurfaceActionClient()
        let service = WatchSurfaceActionService(client: client)

        let outcome = try await service.appendToThread(text: "Loose watch note", conversationID: nil)

        XCTAssertEqual(outcome, .captureCreated(type: "watch_thread_capture", source: "apple_watch"))
        let operations = await client.operationSnapshot()
        XCTAssertEqual(operations, [
            .capture(text: "Loose watch note", type: "watch_thread_capture", source: "apple_watch"),
        ])
    }
}

private actor RecordingWatchSurfaceActionClient: WatchSurfaceActionClient {
    enum Operation: Equatable {
        case nudgeDone(id: String)
        case nudgeSnooze(id: String, minutes: Int)
        case assistantEntry(text: String, conversationID: String)
        case capture(text: String, type: String, source: String)
    }

    private(set) var operations: [Operation] = []
    private let failAssistantEntry: Bool

    init(failAssistantEntry: Bool = false) {
        self.failAssistantEntry = failAssistantEntry
    }

    func nudgeDone(id: String) async throws {
        operations.append(.nudgeDone(id: id))
    }

    func nudgeSnooze(id: String, minutes: Int) async throws {
        operations.append(.nudgeSnooze(id: id, minutes: minutes))
    }

    func submitAssistantEntry(text: String, conversationID: String) async throws {
        operations.append(.assistantEntry(text: text, conversationID: conversationID))
        if failAssistantEntry {
            throw URLError(.cannotConnectToHost)
        }
    }

    func createCapture(text: String, type: String, source: String) async throws {
        operations.append(.capture(text: text, type: type, source: source))
    }

    func operationSnapshot() -> [Operation] {
        operations
    }
}
