import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif
#if os(macOS)
import AppKit
#endif
#if canImport(HealthKit)
import HealthKit
#endif
#if canImport(EventKit)
import EventKit
#endif

public struct VelActivitySnapshot: Codable, Sendable {
    public var source: String
    public var events: [Event]

    public struct Event: Codable, Sendable {
        public var signal_type: String
        public var timestamp: Int
        public var source: String?
        public var host: String?
        public var details: [String: String]?
    }
}

public struct VelHealthSnapshot: Codable, Sendable {
    public var source: String
    public var samples: [Sample]

    public struct Sample: Codable, Sendable {
        public var metric_type: String
        public var timestamp: Int
        public var value: Double
        public var unit: String?
        public var source: String?
        public var source_app: String?
        public var device: String?
        public var source_ref: String?
        public var metadata: [String: String]?
    }
}

public struct VelMessagingSnapshot: Codable, Sendable {
    public var source: String
    public var account_id: String?
    public var threads: [Thread]

    public struct Thread: Codable, Sendable {
        public var thread_id: String
        public var platform: String
        public var title: String?
        public var participants: [Participant]
        public var latest_timestamp: Int
        public var waiting_state: String
        public var scheduling_related: Bool
        public var urgent: Bool
        public var summary: String?
        public var snippet: String?
    }

    public struct Participant: Codable, Sendable {
        public var id: String
        public var name: String?
        public var is_me: Bool
    }
}

public struct VelRemindersSnapshot: Codable, Sendable {
    public var source: String
    public var account_id: String?
    public var generated_at: Int
    public var reminders: [Reminder]

    public struct Reminder: Codable, Sendable {
        public var reminder_id: String
        public var title: String
        public var list_id: String?
        public var list_title: String?
        public var notes: String?
        public var due_at: Int?
        public var completed: Bool
        public var completed_at: Int?
        public var priority: Int?
        public var tags: [String]?
        public var metadata: [String: String]?
        public var updated_at: Int?
        public var source: String?
        public var source_ref: String?
    }
}

public struct VelLocalExportReport: Sendable {
    public var writtenSources: [VelLocalSourceKind]
    public var syncedSources: [VelLocalSourceKind]
    public var errors: [String]

    public init(
        writtenSources: [VelLocalSourceKind] = [],
        syncedSources: [VelLocalSourceKind] = [],
        errors: [String] = []
    ) {
        self.writtenSources = writtenSources
        self.syncedSources = syncedSources
        self.errors = errors
    }
}

public actor VelLocalSnapshotWriter {
    private let fileManager: FileManager

    public init(fileManager: FileManager = .default) {
        self.fileManager = fileManager
    }

    public func writeActivitySnapshot(_ snapshot: VelActivitySnapshot) throws -> URL {
        try write(snapshot, to: try snapshotURL(for: .activity))
    }

    public func writeHealthSnapshot(_ snapshot: VelHealthSnapshot) throws -> URL {
        try write(snapshot, to: try snapshotURL(for: .health))
    }

    public func writeMessagingSnapshot(_ snapshot: VelMessagingSnapshot) throws -> URL {
        try write(snapshot, to: try snapshotURL(for: .messaging))
    }

    public func writeRemindersSnapshot(_ snapshot: VelRemindersSnapshot) throws -> URL {
        try write(snapshot, to: try snapshotURL(for: .reminders))
    }

    public func snapshotURL(for source: VelLocalSourceKind) throws -> URL {
        let base = try applicationSupportRoot()
        switch source {
        case .activity:
            return base.appendingPathComponent("activity/snapshot.json", isDirectory: false)
        case .health:
            return base.appendingPathComponent("health/snapshot.json", isDirectory: false)
        case .git:
            return base.appendingPathComponent("git/snapshot.json", isDirectory: false)
        case .messaging:
            return base.appendingPathComponent("messages/snapshot.json", isDirectory: false)
        case .reminders:
            return base.appendingPathComponent("reminders/snapshot.json", isDirectory: false)
        case .notes:
            return base.appendingPathComponent("notes", isDirectory: true)
        case .transcripts:
            return base.appendingPathComponent("transcripts/snapshot.json", isDirectory: false)
        }
    }

    private func applicationSupportRoot() throws -> URL {
        #if os(macOS)
        let root = fileManager.homeDirectoryForCurrentUser
            .appendingPathComponent("Library/Application Support/Vel", isDirectory: true)
        #else
        guard let base = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask).first else {
            throw NSError(
                domain: "VelLocalSnapshotWriter",
                code: 1,
                userInfo: [NSLocalizedDescriptionKey: "Unable to resolve application support directory"]
            )
        }
        let root = base.appendingPathComponent("Vel", isDirectory: true)
        #endif
        try fileManager.createDirectory(
            at: root,
            withIntermediateDirectories: true,
            attributes: nil
        )
        return root
    }

    private func write<T: Encodable>(_ value: T, to url: URL) throws -> URL {
        try fileManager.createDirectory(
            at: url.deletingLastPathComponent(),
            withIntermediateDirectories: true,
            attributes: nil
        )
        let data = try JSONEncoder.velSnapshotEncoder.encode(value)
        let tmp = url.deletingLastPathComponent()
            .appendingPathComponent(".\(UUID().uuidString).tmp", isDirectory: false)
        try data.write(to: tmp, options: .atomic)
        _ = try? fileManager.removeItem(at: url)
        try fileManager.moveItem(at: tmp, to: url)
        return url
    }
}

public extension JSONEncoder {
    static var velSnapshotEncoder: JSONEncoder {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        return encoder
    }
}

#if os(macOS)
public actor VelMacLocalSourceExporter {
    private let writer: VelLocalSnapshotWriter
    private let processInfo: ProcessInfo

    public init(
        writer: VelLocalSnapshotWriter = VelLocalSnapshotWriter(),
        processInfo: ProcessInfo = .processInfo
    ) {
        self.writer = writer
        self.processInfo = processInfo
    }

    public func bootstrap(using client: VelClient?) async -> VelLocalExportReport {
        var report = VelLocalExportReport()

        do {
            _ = try await exportActivityHeartbeat()
            report.writtenSources.append(.activity)
            if let client {
                _ = try await client.syncLocalSource(.activity)
                report.syncedSources.append(.activity)
            }
        } catch {
            report.errors.append("activity: \(error.localizedDescription)")
        }

        do {
            if try await exportHealthSnapshot() != nil {
                report.writtenSources.append(.health)
                if let client {
                    _ = try await client.syncLocalSource(.health)
                    report.syncedSources.append(.health)
                }
            }
        } catch {
            report.errors.append("health: \(error.localizedDescription)")
        }

        do {
            if try await exportMessagesSnapshot(limit: 20) != nil {
                report.writtenSources.append(.messaging)
                if let client {
                    _ = try await client.syncLocalSource(.messaging)
                    report.syncedSources.append(.messaging)
                }
            }
        } catch {
            report.errors.append("messaging: \(error.localizedDescription)")
        }

        do {
            if try await exportRemindersSnapshot(limit: 100) != nil {
                report.writtenSources.append(.reminders)
                if let client {
                    _ = try await client.syncLocalSource(.reminders)
                    report.syncedSources.append(.reminders)
                }
            }
        } catch {
            report.errors.append("reminders: \(error.localizedDescription)")
        }

        return report
    }

    public func exportActivityHeartbeat() async throws -> URL {
        let host = processInfo.hostName
        let timestamp = Int(Date().timeIntervalSince1970)
        let appState = await MainActor.run { NSApp.isActive ? "active" : "background" }
        let snapshot = VelActivitySnapshot(
            source: "apple_local",
            events: [
                .init(
                    signal_type: "computer_activity",
                    timestamp: timestamp,
                    source: "apple_local",
                    host: host,
                    details: [
                        "app": "VelMac",
                        "platform": "macOS",
                        "state": appState
                    ]
                )
            ]
        )
        return try await writer.writeActivitySnapshot(snapshot)
    }

    public func exportMessagesSnapshot(limit: Int) async throws -> URL? {
        let databaseURL = FileManager.default.homeDirectoryForCurrentUser
            .appendingPathComponent("Library/Messages/chat.db", isDirectory: false)
        guard FileManager.default.fileExists(atPath: databaseURL.path) else {
            return nil
        }

        let rows = try queryRecentMessages(databaseURL: databaseURL, limit: limit)
        guard !rows.isEmpty else { return nil }
        let snapshot = VelMessagingSnapshot(
            source: "messages",
            account_id: "local-default",
            threads: buildThreads(from: rows)
        )
        return try await writer.writeMessagingSnapshot(snapshot)
    }

    public func exportHealthSnapshot() async throws -> URL? {
        #if canImport(HealthKit)
        guard HKHealthStore.isHealthDataAvailable() else { return nil }
        let store = HKHealthStore()
        var quantityMetrics: [HKQuantityTypeIdentifier] = [
            .stepCount,
            .activeEnergyBurned,
            .heartRate,
            .bloodPressureSystolic,
            .bloodPressureDiastolic
        ]
        if let standHoursType = standHoursQuantityTypeIdentifier {
            quantityMetrics.append(standHoursType)
        }
        let quantityTypes = Set(quantityMetrics.compactMap(HKObjectType.quantityType(forIdentifier:)))
        var readTypes = Set<HKObjectType>()
        readTypes.formUnion(quantityTypes)
        if let sleepType = HKObjectType.categoryType(forIdentifier: .sleepAnalysis) {
            readTypes.insert(sleepType)
        }
        try await requestAuthorization(store: store, types: readTypes)

        let startOfDay = Calendar.current.startOfDay(for: Date())
        let sleepWindowStart = Calendar.current.date(byAdding: .hour, value: -12, to: startOfDay) ?? startOfDay
        let stepCount = try await cumulativeSum(
            store: store,
            type: .stepCount,
            unit: HKUnit.count(),
            startDate: startOfDay
        )
        let activeEnergy = try await cumulativeSum(
            store: store,
            type: .activeEnergyBurned,
            unit: HKUnit.kilocalorie(),
            startDate: startOfDay
        )
        let standHours = try await standHoursCount(store: store, startDate: startOfDay)
        let heartRate = try await discreteAverage(
            store: store,
            type: .heartRate,
            unit: HKUnit.count().unitDivided(by: HKUnit.minute()),
            startDate: startOfDay
        )
        let bloodPressureSystolic = try await latestQuantityValue(
            store: store,
            type: .bloodPressureSystolic,
            unit: HKUnit.millimeterOfMercury()
        )
        let bloodPressureDiastolic = try await latestQuantityValue(
            store: store,
            type: .bloodPressureDiastolic,
            unit: HKUnit.millimeterOfMercury()
        )
        let sleepHours = try await sleepDurationHours(
            store: store,
            startDate: sleepWindowStart,
            endDate: Date()
        )

        var samples: [VelHealthSnapshot.Sample] = []
        let timestamp = Int(Date().timeIntervalSince1970)
        let dayOrdinal = Calendar.current.ordinality(of: .day, in: .era, for: Date()) ?? timestamp
        if let stepCount {
            samples.append(.init(
                metric_type: "step_count",
                timestamp: timestamp,
                value: stepCount,
                unit: "count",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:step_count:\(dayOrdinal)",
                metadata: ["window": "today"]
            ))
        }
        if let activeEnergy {
            samples.append(.init(
                metric_type: "active_energy_burned",
                timestamp: timestamp,
                value: activeEnergy,
                unit: "kcal",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:active_energy_burned:\(dayOrdinal)",
                metadata: ["window": "today"]
            ))
        }
        if let standHours {
            samples.append(.init(
                metric_type: "stand_hours",
                timestamp: timestamp,
                value: standHours,
                unit: "count",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:stand_hours:\(dayOrdinal)",
                metadata: ["window": "today"]
            ))
        }
        if let heartRate {
            samples.append(.init(
                metric_type: "heart_rate_bpm",
                timestamp: timestamp,
                value: heartRate,
                unit: "count/min",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:heart_rate_bpm:\(dayOrdinal)",
                metadata: ["window": "today", "aggregation": "average"]
            ))
        }
        if let bloodPressureSystolic {
            samples.append(.init(
                metric_type: "blood_pressure_systolic",
                timestamp: timestamp,
                value: bloodPressureSystolic,
                unit: "mmHg",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:blood_pressure_systolic:\(dayOrdinal)",
                metadata: ["window": "latest"]
            ))
        }
        if let bloodPressureDiastolic {
            samples.append(.init(
                metric_type: "blood_pressure_diastolic",
                timestamp: timestamp,
                value: bloodPressureDiastolic,
                unit: "mmHg",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:blood_pressure_diastolic:\(dayOrdinal)",
                metadata: ["window": "latest"]
            ))
        }
        if let sleepHours {
            samples.append(.init(
                metric_type: "sleep_hours",
                timestamp: timestamp,
                value: sleepHours,
                unit: "h",
                source: "healthkit",
                source_app: "Health",
                device: "Apple Health",
                source_ref: "healthkit:sleep_hours:\(dayOrdinal)",
                metadata: ["window": "overnight"]
            ))
        }
        guard !samples.isEmpty else { return nil }
        return try await writer.writeHealthSnapshot(.init(source: "healthkit", samples: samples))
        #else
        return nil
        #endif
    }

    public func exportRemindersSnapshot(limit: Int) async throws -> URL? {
        #if canImport(EventKit)
        let store = EKEventStore()
        try await requestRemindersAuthorization(store: store)
        let reminders = try await fetchReminders(store: store, limit: max(limit, 1))
        guard !reminders.isEmpty else { return nil }

        let generatedAt = Int(Date().timeIntervalSince1970)
        let payload = reminders.map { reminder -> VelRemindersSnapshot.Reminder in
            let dueAt = reminder.dueDateComponents
                .flatMap { Calendar.current.date(from: $0) }
                .map { Int($0.timeIntervalSince1970) }
            let completedAt = reminder.completionDate.map { Int($0.timeIntervalSince1970) }
            let updatedAt = reminder.lastModifiedDate.map { Int($0.timeIntervalSince1970) }
            let sourceRef = "eventkit:reminder:\(reminder.calendarItemIdentifier):\(updatedAt ?? generatedAt)"
            return VelRemindersSnapshot.Reminder(
                reminder_id: reminder.calendarItemIdentifier,
                title: reminder.title,
                list_id: reminder.calendar.calendarIdentifier,
                list_title: reminder.calendar.title,
                notes: reminder.notes,
                due_at: dueAt,
                completed: reminder.isCompleted,
                completed_at: completedAt,
                priority: reminder.priority,
                tags: nil,
                metadata: [
                    "calendar_source": reminder.calendar.source.title,
                    "calendar_type": "\(reminder.calendar.type.rawValue)"
                ],
                updated_at: updatedAt,
                source: "apple_reminders",
                source_ref: sourceRef
            )
        }
        let snapshot = VelRemindersSnapshot(
            source: "apple_reminders",
            account_id: "local-default",
            generated_at: generatedAt,
            reminders: payload
        )
        return try await writer.writeRemindersSnapshot(snapshot)
        #else
        return nil
        #endif
    }

    private struct MessageRow: Decodable {
        let thread_id: String
        let title: String?
        let participant_id: String?
        let participant_name: String?
        let is_from_me: Int
        let latest_timestamp: Int
        let snippet: String?
    }

    private func queryRecentMessages(databaseURL: URL, limit: Int) throws -> [MessageRow] {
        let sql = """
        SELECT
          COALESCE(chat.chat_identifier, handle.id, CAST(message.ROWID AS TEXT)) AS thread_id,
          COALESCE(chat.display_name, handle.id) AS title,
          handle.id AS participant_id,
          handle.uncanonicalized_id AS participant_name,
          message.is_from_me AS is_from_me,
          CAST((message.date / 1000000000) + 978307200 AS INTEGER) AS latest_timestamp,
          message.text AS snippet
        FROM message
        LEFT JOIN handle ON message.handle_id = handle.ROWID
        LEFT JOIN chat_message_join ON chat_message_join.message_id = message.ROWID
        LEFT JOIN chat ON chat.ROWID = chat_message_join.chat_id
        WHERE message.text IS NOT NULL AND LENGTH(TRIM(message.text)) > 0
        ORDER BY message.date DESC
        LIMIT \(max(limit, 1));
        """

        let process = Process()
        process.executableURL = URL(fileURLWithPath: "/usr/bin/sqlite3")
        process.arguments = ["-json", databaseURL.path, sql]
        let output = Pipe()
        let errors = Pipe()
        process.standardOutput = output
        process.standardError = errors
        try process.run()
        process.waitUntilExit()

        let stderr = String(data: errors.fileHandleForReading.readDataToEndOfFile(), encoding: .utf8) ?? ""
        guard process.terminationStatus == 0 else {
            throw NSError(
                domain: "VelMacLocalSourceExporter",
                code: Int(process.terminationStatus),
                userInfo: [NSLocalizedDescriptionKey: stderr.isEmpty ? "sqlite3 failed to read Messages database" : stderr]
            )
        }

        let data = output.fileHandleForReading.readDataToEndOfFile()
        guard !data.isEmpty else { return [] }
        return try JSONDecoder().decode([MessageRow].self, from: data)
    }

    private func buildThreads(from rows: [MessageRow]) -> [VelMessagingSnapshot.Thread] {
        let grouped = Dictionary(grouping: rows, by: \.thread_id)
        return grouped.values.compactMap { rows in
            guard let latest = rows.max(by: { $0.latest_timestamp < $1.latest_timestamp }) else {
                return nil
            }
            let participants = Dictionary(
                rows.compactMap { row -> (String, VelMessagingSnapshot.Participant)? in
                    guard let id = row.participant_id, !id.isEmpty else { return nil }
                    return (id, .init(id: id, name: row.participant_name, is_me: false))
                },
                uniquingKeysWith: { first, _ in first }
            )
            let waitingState = latest.is_from_me == 0 ? "me" : "other"
            let text = [latest.title, latest.snippet].compactMap { $0?.lowercased() }.joined(separator: " ")
            return VelMessagingSnapshot.Thread(
                thread_id: latest.thread_id,
                platform: "imessage",
                title: latest.title,
                participants: Array(participants.values),
                latest_timestamp: latest.latest_timestamp,
                waiting_state: waitingState,
                scheduling_related: text.contains("meet") || text.contains("schedule") || text.contains("tomorrow") || text.contains("when"),
                urgent: text.contains("asap") || text.contains("urgent") || text.contains("today"),
                summary: latest.snippet,
                snippet: latest.snippet
            )
        }
        .sorted(by: { $0.latest_timestamp > $1.latest_timestamp })
    }

    #if canImport(HealthKit)
    private var standHoursQuantityTypeIdentifier: HKQuantityTypeIdentifier? {
        HKQuantityTypeIdentifier(rawValue: "HKQuantityTypeIdentifierAppleStandHour")
    }

    private func standHoursCount(store: HKHealthStore, startDate: Date) async throws -> Double? {
        guard let standHoursType = standHoursQuantityTypeIdentifier else {
            return nil
        }
        return try await cumulativeSum(
            store: store,
            type: standHoursType,
            unit: HKUnit.count(),
            startDate: startDate
        )
    }

    private func requestAuthorization(store: HKHealthStore, types: Set<HKObjectType>) async throws {
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            store.requestAuthorization(toShare: [], read: types) { success, error in
                if let error {
                    continuation.resume(throwing: error)
                } else if success {
                    continuation.resume(returning: ())
                } else {
                    continuation.resume(throwing: NSError(
                        domain: "VelMacLocalSourceExporter",
                        code: 1,
                        userInfo: [NSLocalizedDescriptionKey: "HealthKit authorization was denied"]
                    ))
                }
            }
        }
    }

    private func cumulativeSum(
        store: HKHealthStore,
        type: HKQuantityTypeIdentifier,
        unit: HKUnit,
        startDate: Date
    ) async throws -> Double? {
        guard let quantityType = HKObjectType.quantityType(forIdentifier: type) else { return nil }
        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: Date())
        return try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Double?, Error>) in
            let query = HKStatisticsQuery(
                quantityType: quantityType,
                quantitySamplePredicate: predicate,
                options: .cumulativeSum
            ) { _, result, error in
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                let value = result?.sumQuantity()?.doubleValue(for: unit)
                continuation.resume(returning: value)
            }
            store.execute(query)
        }
    }

    private func discreteAverage(
        store: HKHealthStore,
        type: HKQuantityTypeIdentifier,
        unit: HKUnit,
        startDate: Date
    ) async throws -> Double? {
        guard let quantityType = HKObjectType.quantityType(forIdentifier: type) else { return nil }
        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: Date())
        return try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Double?, Error>) in
            let query = HKStatisticsQuery(
                quantityType: quantityType,
                quantitySamplePredicate: predicate,
                options: .discreteAverage
            ) { _, result, error in
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                let value = result?.averageQuantity()?.doubleValue(for: unit)
                continuation.resume(returning: value)
            }
            store.execute(query)
        }
    }

    private func latestQuantityValue(
        store: HKHealthStore,
        type: HKQuantityTypeIdentifier,
        unit: HKUnit
    ) async throws -> Double? {
        guard let quantityType = HKObjectType.quantityType(forIdentifier: type) else { return nil }
        let sort = NSSortDescriptor(key: HKSampleSortIdentifierEndDate, ascending: false)
        return try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Double?, Error>) in
            let query = HKSampleQuery(
                sampleType: quantityType,
                predicate: nil,
                limit: 1,
                sortDescriptors: [sort]
            ) { _, samples, error in
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                let value = (samples?.first as? HKQuantitySample)?
                    .quantity
                    .doubleValue(for: unit)
                continuation.resume(returning: value)
            }
            store.execute(query)
        }
    }

    private func sleepDurationHours(
        store: HKHealthStore,
        startDate: Date,
        endDate: Date
    ) async throws -> Double? {
        guard let sleepType = HKObjectType.categoryType(forIdentifier: .sleepAnalysis) else {
            return nil
        }
        let predicate = HKQuery.predicateForSamples(withStart: startDate, end: endDate)
        let samples: [HKCategorySample] = try await withCheckedThrowingContinuation { continuation in
            let sort = NSSortDescriptor(key: HKSampleSortIdentifierStartDate, ascending: true)
            let query = HKSampleQuery(
                sampleType: sleepType,
                predicate: predicate,
                limit: HKObjectQueryNoLimit,
                sortDescriptors: [sort]
            ) { _, results, error in
                if let error {
                    continuation.resume(throwing: error)
                    return
                }
                let mapped = (results as? [HKCategorySample]) ?? []
                continuation.resume(returning: mapped)
            }
            store.execute(query)
        }

        var asleepValues = Set<Int>([HKCategoryValueSleepAnalysis.asleep.rawValue])
        if #available(macOS 13.0, *) {
            asleepValues.insert(HKCategoryValueSleepAnalysis.asleepCore.rawValue)
            asleepValues.insert(HKCategoryValueSleepAnalysis.asleepDeep.rawValue)
            asleepValues.insert(HKCategoryValueSleepAnalysis.asleepREM.rawValue)
            asleepValues.insert(HKCategoryValueSleepAnalysis.asleepUnspecified.rawValue)
        }

        let seconds = samples
            .filter { asleepValues.contains($0.value) }
            .reduce(0.0) { partial, sample in
                partial + sample.endDate.timeIntervalSince(sample.startDate)
            }
        guard seconds > 0 else { return nil }
        return seconds / 3600.0
    }
    #endif

    #if canImport(EventKit)
    private func requestRemindersAuthorization(store: EKEventStore) async throws {
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            store.requestAccess(to: .reminder) { granted, error in
                if let error {
                    continuation.resume(throwing: error)
                } else if granted {
                    continuation.resume(returning: ())
                } else {
                    continuation.resume(throwing: NSError(
                        domain: "VelMacLocalSourceExporter",
                        code: 2,
                        userInfo: [NSLocalizedDescriptionKey: "Reminders authorization was denied"]
                    ))
                }
            }
        }
    }

    private func fetchReminders(store: EKEventStore, limit: Int) async throws -> [EKReminder] {
        let incomplete = try await fetchReminders(
            store: store,
            predicate: store.predicateForIncompleteReminders(
                withDueDateStarting: nil,
                ending: nil,
                calendars: nil
            )
        )
        let completeStart = Calendar.current.date(byAdding: .day, value: -14, to: Date())
        let completed = try await fetchReminders(
            store: store,
            predicate: store.predicateForCompletedReminders(
                withCompletionDateStarting: completeStart,
                ending: Date(),
                calendars: nil
            )
        )
        let all = (incomplete + completed)
            .sorted { left, right in
                let leftUpdated = left.lastModifiedDate ?? left.creationDate ?? .distantPast
                let rightUpdated = right.lastModifiedDate ?? right.creationDate ?? .distantPast
                return leftUpdated > rightUpdated
            }
        if all.count <= limit {
            return all
        }
        return Array(all.prefix(limit))
    }

    private func fetchReminders(
        store: EKEventStore,
        predicate: NSPredicate?
    ) async throws -> [EKReminder] {
        guard let predicate else {
            return []
        }
        return try await withCheckedThrowingContinuation { continuation in
            store.fetchReminders(matching: predicate) { reminders in
                continuation.resume(returning: reminders ?? [])
            }
        }
    }
    #endif
}
#endif
