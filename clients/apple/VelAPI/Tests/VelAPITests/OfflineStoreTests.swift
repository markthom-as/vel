import XCTest
@testable import VelAPI

final class OfflineStoreTests: XCTestCase {
    func testVoiceDraftRoundTripsThroughOfflineStore() {
        let suiteName = "VelOfflineStoreTests.voiceDraft.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        let store = VelOfflineStore(userDefaults: defaults)

        let draft = AppleVoiceDraftData(
            transcript: "Mark meds done",
            suggested_intent: "commitment_done",
            suggested_text: "meds"
        )

        store.saveVoiceDraft(draft)

        XCTAssertEqual(store.cachedVoiceDraft()?.transcript, "Mark meds done")
        XCTAssertEqual(store.cachedVoiceDraft()?.suggested_intent, "commitment_done")

        store.clearVoiceDraft()
        XCTAssertNil(store.cachedVoiceDraft())
    }

    func testVoiceContinuityHistoryPersistsAndCapsAtFortyEntries() {
        let suiteName = "VelOfflineStoreTests.voiceHistory.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        let store = VelOfflineStore(userDefaults: defaults)

        let history = (0..<45).map { index in
            AppleVoiceContinuityEntryData(
                transcript: "Transcript \(index)",
                suggested_intent: "capture",
                committed_intent: index.isMultiple(of: 2) ? "capture" : nil,
                status: index.isMultiple(of: 2) ? "queued" : "pending_review"
            )
        }

        store.saveVoiceContinuityHistory(history)

        let persisted = store.cachedVoiceContinuityHistory()
        XCTAssertEqual(persisted.count, 40)
        XCTAssertEqual(persisted.first?.transcript, "Transcript 0")
        XCTAssertEqual(persisted.last?.transcript, "Transcript 39")
    }

    func testVoiceContinuityHistoryPersistsThreadHintAndMergedTimestamp() {
        let suiteName = "VelOfflineStoreTests.voiceHistoryMerged.\(UUID().uuidString)"
        let defaults = UserDefaults(suiteName: suiteName)!
        defaults.removePersistentDomain(forName: suiteName)
        let store = VelOfflineStore(userDefaults: defaults)
        let mergedAt = Date()

        store.saveVoiceContinuityHistory([
            AppleVoiceContinuityEntryData(
                transcript: "What matters right now?",
                suggested_intent: "current_schedule",
                committed_intent: "current_schedule",
                status: "answered",
                thread_id: "thread_123",
                merged_at: mergedAt
            )
        ])

        let persisted = store.cachedVoiceContinuityHistory()
        XCTAssertEqual(persisted.first?.thread_id, "thread_123")
        XCTAssertEqual(persisted.first?.merged_at?.timeIntervalSince1970, mergedAt.timeIntervalSince1970, accuracy: 1)
    }
}
