import AVFoundation
import Speech
import SwiftUI
import VelAPI

private enum VeliOSTab: Hashable {
    case today
    case nudges
    case activity
    case voice
    case settings
}

struct ContentView: View {
    @EnvironmentObject var store: VelClientStore
    @State private var selectedTab: VeliOSTab = .today
    @StateObject private var voiceModel = VoiceCaptureModel()

    var body: some View {
        NavigationStack {
            TabView(selection: $selectedTab) {
                TodayTab(store: store)
                    .tag(VeliOSTab.today)
                    .tabItem {
                        Label("Today", systemImage: "sun.max")
                    }

                NudgesTab(store: store)
                    .tag(VeliOSTab.nudges)
                    .tabItem {
                        Label("Nudges", systemImage: "bell.badge")
                    }

                ActivityTab(store: store)
                    .tag(VeliOSTab.activity)
                    .tabItem {
                        Label("Activity", systemImage: "chart.line.uptrend.xyaxis")
                    }

                VoiceTab(store: store, voiceModel: voiceModel)
                    .tag(VeliOSTab.voice)
                    .tabItem {
                        Label("Voice", systemImage: "waveform")
                    }

                SettingsTab(store: store)
                    .tag(VeliOSTab.settings)
                    .tabItem {
                        Label("Settings", systemImage: "gearshape")
                    }
            }
            .navigationTitle(title(for: selectedTab))
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    if store.isSyncing {
                        ProgressView()
                    } else {
                        Button {
                            Task { await store.refresh() }
                        } label: {
                            Image(systemName: "arrow.clockwise")
                        }
                    }
                }
            }
            .task {
                await store.refresh()
                await voiceModel.ensurePermissionsKnown()
            }
            .onChange(of: selectedTab) { tab in
                if tab == .activity {
                    Task { await store.refreshSignals() }
                }
            }
        }
    }

    private func title(for tab: VeliOSTab) -> String {
        switch tab {
        case .today:
            return "Vel Today"
        case .nudges:
            return "Nudges"
        case .activity:
            return "Activity"
        case .voice:
            return "Voice Capture"
        case .settings:
            return "Settings"
        }
    }
}

private struct TodayTab: View {
    @ObservedObject var store: VelClientStore
    @State private var commitmentText = ""
    @State private var captureText = ""

    var body: some View {
        List {
            Section("Connection") {
                ConnectionSummaryRow(store: store)
            }

            Section("Current context") {
                if let ctx = store.context?.context {
                    ContextValueRow(label: "Mode", value: ctx.mode)
                    ContextValueRow(label: "Morning state", value: ctx.morning_state)
                    ContextValueRow(label: "Meds", value: ctx.meds_status)
                    ContextValueRow(label: "Attention", value: ctx.attention_state)
                    ContextValueRow(label: "Drift", value: ctx.drift_type)

                    if let prep = ctx.prep_window_active {
                        BoolStatusRow(label: "Prep window", value: prep)
                    }
                    if let commute = ctx.commute_window_active {
                        BoolStatusRow(label: "Commute window", value: commute)
                    }
                    if let leaveBy = ctx.leave_by_ts {
                        ContextValueRow(label: "Leave by", value: formatUnix(leaveBy))
                    }
                    if let nextEvent = ctx.next_event_start_ts {
                        ContextValueRow(label: "Next event", value: formatUnix(nextEvent))
                    }
                    if let waitingCount = ctx.message_waiting_on_me_count {
                        ContextValueRow(label: "Waiting on me", value: "\(waitingCount)")
                    }
                    if let urgentThreads = ctx.message_urgent_thread_count {
                        ContextValueRow(label: "Urgent threads", value: "\(urgentThreads)")
                    }
                } else {
                    Text("No context yet. Run evaluate/sync on daemon or refresh once connected.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Open commitments") {
                let openCommitments = store.commitments
                    .filter { $0.status == "open" }
                    .prefix(8)
                if openCommitments.isEmpty {
                    Text("No open commitments.")
                        .foregroundStyle(.secondary)
                }
                ForEach(Array(openCommitments), id: \.id) { commitment in
                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text(commitment.text)
                            if let dueAt = commitment.due_at {
                                Text("Due \(formatUnix(dueAt))")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                        }
                        Spacer()
                        Button("Done") {
                            Task {
                                await store.markCommitmentDone(id: commitment.id)
                            }
                        }
                        .buttonStyle(.bordered)
                    }
                }
            }

            Section("Quick add") {
                VStack(alignment: .leading, spacing: 8) {
                    TextField("New commitment", text: $commitmentText)
                        .textInputAutocapitalization(.sentences)
                    Button("Create commitment") {
                        let text = commitmentText.trimmingCharacters(in: .whitespacesAndNewlines)
                        guard !text.isEmpty else { return }
                        Task {
                            await store.createCommitment(text: text)
                            commitmentText = ""
                        }
                    }
                    .buttonStyle(.borderedProminent)
                }

                VStack(alignment: .leading, spacing: 8) {
                    TextField("Quick capture", text: $captureText)
                        .textInputAutocapitalization(.sentences)
                    Button("Save capture") {
                        let text = captureText.trimmingCharacters(in: .whitespacesAndNewlines)
                        guard !text.isEmpty else { return }
                        Task {
                            await store.createCapture(text: text)
                            captureText = ""
                        }
                    }
                    .buttonStyle(.bordered)
                }
            }
        }
        .listStyle(.insetGrouped)
        .refreshable { await store.refresh() }
    }
}

private struct NudgesTab: View {
    @ObservedObject var store: VelClientStore

    var body: some View {
        List {
            Section("Active nudges") {
                let active = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
                if active.isEmpty {
                    Text("No active nudges.")
                        .foregroundStyle(.secondary)
                }
                ForEach(active) { nudge in
                    VStack(alignment: .leading, spacing: 8) {
                        Text(nudge.message)
                            .font(.body)
                        Text("\(nudge.nudge_type) · \(nudge.level) · \(nudge.state)")
                            .font(.caption)
                            .foregroundStyle(.secondary)

                        HStack {
                            Button("Done") {
                                Task {
                                    await store.markNudgeDone(id: nudge.nudge_id)
                                }
                            }
                            .buttonStyle(.borderedProminent)

                            Button("Snooze 10m") {
                                Task {
                                    await store.snoozeNudge(id: nudge.nudge_id, minutes: 10)
                                }
                            }
                            .buttonStyle(.bordered)
                        }
                    }
                    .padding(.vertical, 4)
                }
            }
        }
        .listStyle(.insetGrouped)
        .refreshable { await store.refresh() }
    }
}

private struct ActivityTab: View {
    @ObservedObject var store: VelClientStore

    var body: some View {
        List {
            Section("Recent signals") {
                if store.signals.isEmpty {
                    Text("No signals available yet.")
                        .foregroundStyle(.secondary)
                }
                ForEach(store.signals.prefix(80), id: \.signal_id) { signal in
                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Text(signal.signal_type)
                                .font(.subheadline)
                                .fontWeight(.semibold)
                            Spacer()
                            Text(formatUnix(signal.timestamp))
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                        Text("source: \(signal.source)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        if signal.payload != .null {
                            Text(signal.payload.compactText)
                                .font(.caption)
                                .foregroundStyle(.tertiary)
                                .lineLimit(3)
                        }
                    }
                    .padding(.vertical, 4)
                }
            }
        }
        .listStyle(.insetGrouped)
        .refreshable { await store.refreshSignals() }
    }
}

private struct VoiceTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel

    var body: some View {
        List {
            Section("Permissions") {
                PermissionRow(label: "Speech recognition", state: voiceModel.speechPermission)
                PermissionRow(label: "Microphone", state: voiceModel.microphonePermission)

                Button("Request permissions") {
                    Task { await voiceModel.requestPermissions() }
                }
                .buttonStyle(.bordered)
            }

            Section("Record") {
                Button {
                    Task { await voiceModel.toggleRecording() }
                } label: {
                    Label(
                        voiceModel.isRecording ? "Stop recording" : "Start recording",
                        systemImage: voiceModel.isRecording ? "stop.circle.fill" : "mic.circle.fill"
                    )
                    .font(.headline)
                    .foregroundStyle(voiceModel.isRecording ? .red : .accentColor)
                }
                .buttonStyle(.plain)

                if voiceModel.hasTranscript {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Transcript")
                            .font(.caption2)
                            .foregroundStyle(.secondary)

                        TextEditor(
                            text: Binding(
                                get: { voiceModel.transcript },
                                set: { voiceModel.updateTranscript($0) }
                            )
                        )
                        .frame(minHeight: 110)

                        HStack {
                            Button("Clear transcript") {
                                voiceModel.clearTranscript()
                            }
                            .buttonStyle(.bordered)

                            Spacer()

                            Text("\(voiceModel.transcript.count) chars")
                                .font(.caption2)
                                .foregroundStyle(.tertiary)
                        }
                    }
                } else {
                    Text("Tap to record a quick thought or command fragment.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                if let message = voiceModel.errorMessage, !message.isEmpty {
                    Text(message)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Suggested action") {
                HStack {
                    Text("Recommendation")
                    Spacer()
                    Text(voiceModel.suggestedIntent.displayLabel)
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 3)
                        .background(Color.secondary.opacity(0.15))
                        .clipShape(Capsule())
                }

                if voiceModel.suggestedIntent.requiresNudgeTarget || voiceModel.suggestedIntent.requiresCommitmentTarget {
                    Text(voiceModel.targetHint(from: store.nudges, commitments: store.commitments))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                Button(voiceModel.suggestedIntent.submitButtonLabel) {
                    Task { await voiceModel.submitSuggested(using: store) }
                }
                .buttonStyle(.borderedProminent)
                .disabled(!voiceModel.hasTranscript)

                HStack {
                    Button("Save as capture") {
                        Task { await voiceModel.submitAsCapture(using: store) }
                    }
                    .buttonStyle(.bordered)
                    .disabled(!voiceModel.hasTranscript)

                    Button("Create commitment") {
                        Task { await voiceModel.submitAsCommitment(using: store) }
                    }
                    .buttonStyle(.bordered)
                    .disabled(!voiceModel.hasTranscript)
                }

                Text("Voice submissions always preserve transcript provenance as a voice capture in Vel.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            }

            Section("Response") {
                if let response = voiceModel.latestResponse {
                    Text(response.summary)
                        .font(.body)
                    if let detail = response.detail, !detail.isEmpty {
                        Text(detail)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    Button("Speak response") {
                        voiceModel.speakLatestResponse()
                    }
                    .buttonStyle(.bordered)
                } else {
                    Text("Run a voice query like “what matters now?” or “why now?” to get an immediate response.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Recent voice entries") {
                if voiceModel.history.isEmpty {
                    Text("No voice entries yet.")
                        .foregroundStyle(.secondary)
                }

                ForEach(voiceModel.history.prefix(12)) { entry in
                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Text(formatDate(entry.createdAt))
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                            Spacer()
                            Text(entry.status)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                        Text(entry.transcript)
                            .font(.subheadline)
                            .lineLimit(3)
                        Text("Suggested: \(entry.suggestedIntent.displayLabel)")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                    }
                    .padding(.vertical, 4)
                }
            }
        }
        .listStyle(.insetGrouped)
    }
}

private struct SettingsTab: View {
    @ObservedObject var store: VelClientStore
    @State private var baseURLOverride = UserDefaults.standard.string(forKey: "vel_base_url") ?? ""

    var body: some View {
        List {
            Section("Runtime") {
                ConnectionSummaryRow(store: store)
                if let lastSyncAt = store.lastSyncAt {
                    Text("Last sync: \(formatDate(lastSyncAt))")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                if store.pendingActionCount > 0 {
                    Text("Pending queued actions: \(store.pendingActionCount)")
                        .font(.caption)
                        .foregroundStyle(.orange)
                }
            }

            Section("Endpoint override") {
                TextField("http://host:4130", text: $baseURLOverride)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()

                Button("Save and reconnect") {
                    store.setBaseURLOverride(baseURLOverride)
                    Task { await store.refresh() }
                }
                .buttonStyle(.borderedProminent)

                Button("Clear override") {
                    baseURLOverride = ""
                    store.setBaseURLOverride(nil)
                    Task { await store.refresh() }
                }
                .buttonStyle(.bordered)

                Text("Resolution order: vel_base_url, vel_tailscale_url, vel_lan_base_url, localhost.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            }

            Section("Docs") {
                ForEach(VelDocumentationCatalog.core) { doc in
                    VStack(alignment: .leading, spacing: 2) {
                        Text(doc.title)
                        Text(doc.path)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }
            }
        }
        .listStyle(.insetGrouped)
    }
}

private struct ConnectionSummaryRow: View {
    @ObservedObject var store: VelClientStore

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Label(
                store.isReachable ? "Connected" : "Offline cache",
                systemImage: store.isReachable ? "checkmark.circle" : "wifi.slash"
            )
            .font(.subheadline)

            if let authority = store.authorityLabel {
                Text("Authority: \(authority)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            if let transport = store.activeTransport {
                Text("Transport: \(transport)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            if let baseURL = store.activeBaseURL {
                Text(baseURL)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            if let message = store.errorMessage, !message.isEmpty {
                Text(message)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

private struct ContextValueRow: View {
    let label: String
    let value: String?

    var body: some View {
        if let value, !value.isEmpty {
            HStack {
                Text(label)
                Spacer()
                Text(value)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

private struct BoolStatusRow: View {
    let label: String
    let value: Bool

    var body: some View {
        HStack {
            Text(label)
            Spacer()
            Text(value ? "Active" : "Inactive")
                .foregroundStyle(value ? .orange : .secondary)
        }
    }
}

private struct PermissionRow: View {
    let label: String
    let state: VoicePermissionState

    var body: some View {
        HStack {
            Text(label)
            Spacer()
            Label(state.displayLabel, systemImage: state.icon)
                .font(.caption)
                .foregroundStyle(state.color)
        }
    }
}

private enum VoicePermissionState: String, Codable {
    case unknown
    case granted
    case denied

    var displayLabel: String {
        switch self {
        case .unknown:
            return "Unknown"
        case .granted:
            return "Granted"
        case .denied:
            return "Denied"
        }
    }

    var icon: String {
        switch self {
        case .unknown:
            return "questionmark.circle"
        case .granted:
            return "checkmark.circle"
        case .denied:
            return "xmark.octagon"
        }
    }

    var color: Color {
        switch self {
        case .unknown:
            return .secondary
        case .granted:
            return .green
        case .denied:
            return .red
        }
    }
}

private struct VoiceIntent: Codable, Equatable {
    enum Kind: String, Codable {
        case captureCreate = "capture_create"
        case commitmentCreate = "commitment_create"
        case commitmentDone = "commitment_done"
        case nudgeDone = "nudge_done"
        case nudgeSnooze = "nudge_snooze"
        case queryContext = "query_context"
        case queryNextCommitment = "query_next_commitment"
        case queryNudges = "query_nudges"
        case explainContext = "explain_context"
    }

    let kind: Kind
    let minutes: Int?

    static let capture = VoiceIntent(kind: .captureCreate, minutes: nil)
    static let commitment = VoiceIntent(kind: .commitmentCreate, minutes: nil)
    static let commitmentDone = VoiceIntent(kind: .commitmentDone, minutes: nil)
    static let nudgeDone = VoiceIntent(kind: .nudgeDone, minutes: nil)
    static let queryContext = VoiceIntent(kind: .queryContext, minutes: nil)
    static let queryNextCommitment = VoiceIntent(kind: .queryNextCommitment, minutes: nil)
    static let queryNudges = VoiceIntent(kind: .queryNudges, minutes: nil)
    static let explainContext = VoiceIntent(kind: .explainContext, minutes: nil)
    static func nudgeSnooze(_ minutes: Int) -> VoiceIntent {
        VoiceIntent(kind: .nudgeSnooze, minutes: minutes)
    }

    var displayLabel: String {
        switch kind {
        case .captureCreate:
            return "Capture"
        case .commitmentCreate:
            return "Commitment"
        case .commitmentDone:
            return "Resolve commitment"
        case .nudgeDone:
            return "Resolve top nudge"
        case .nudgeSnooze:
            return "Snooze top nudge (\(minutes ?? 10)m)"
        case .queryContext:
            return "Query current context"
        case .queryNextCommitment:
            return "Query next commitment"
        case .queryNudges:
            return "Query active nudges"
        case .explainContext:
            return "Explain why now"
        }
    }

    var storageToken: String {
        switch kind {
        case .captureCreate:
            return "capture_create"
        case .commitmentCreate:
            return "commitment_create"
        case .commitmentDone:
            return "commitment_done"
        case .nudgeDone:
            return "nudge_done"
        case .nudgeSnooze:
            return "nudge_snooze_\(minutes ?? 10)m"
        case .queryContext:
            return "query_context"
        case .queryNextCommitment:
            return "query_next_commitment"
        case .queryNudges:
            return "query_nudges"
        case .explainContext:
            return "explain_context"
        }
    }

    var requiresNudgeTarget: Bool {
        kind == .nudgeDone || kind == .nudgeSnooze
    }

    var requiresCommitmentTarget: Bool {
        kind == .commitmentDone
    }

    var isQuery: Bool {
        switch kind {
        case .queryContext, .queryNextCommitment, .queryNudges, .explainContext:
            return true
        case .captureCreate, .commitmentCreate, .commitmentDone, .nudgeDone, .nudgeSnooze:
            return false
        }
    }

    var submitButtonLabel: String {
        isQuery ? "Run query" : "Submit suggested action"
    }
}

private struct VoiceIntentSuggestion {
    let intent: VoiceIntent
    let cleanedText: String
}

private enum VoiceIntentParser {
    private static let commitmentPrefixes = [
        "todo",
        "to do",
        "task",
        "remind me to",
        "remember to",
        "i need to",
        "follow up",
        "follow-up",
        "next action",
        "add commitment"
    ]

    private static let capturePrefixes = [
        "capture",
        "note",
        "idea",
        "memo",
        "log this"
    ]

    private static let minuteWords: [String: Int] = [
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
        "ten": 10,
        "fifteen": 15,
        "twenty": 20,
        "thirty": 30,
        "forty": 40,
        "fortyfive": 45,
        "forty-five": 45,
        "fifty": 50,
        "sixty": 60
    ]

    static func suggest(for transcript: String) -> VoiceIntentSuggestion {
        let clean = cleanedTranscript(transcript)
        guard !clean.isEmpty else {
            return VoiceIntentSuggestion(intent: .capture, cleanedText: clean)
        }

        let normalized = clean.lowercased()

        if isContextQuery(normalized) {
            return VoiceIntentSuggestion(intent: .queryContext, cleanedText: clean)
        }

        if isNextCommitmentQuery(normalized) {
            return VoiceIntentSuggestion(intent: .queryNextCommitment, cleanedText: clean)
        }

        if isNudgesQuery(normalized) {
            return VoiceIntentSuggestion(intent: .queryNudges, cleanedText: clean)
        }

        if isExplainQuery(normalized) {
            return VoiceIntentSuggestion(intent: .explainContext, cleanedText: clean)
        }

        if normalized.contains("snooze") {
            return VoiceIntentSuggestion(intent: .nudgeSnooze(extractMinutes(from: normalized) ?? 10), cleanedText: clean)
        }

        if isNudgeDoneCommand(normalized) {
            return VoiceIntentSuggestion(intent: .nudgeDone, cleanedText: clean)
        }

        if isCommitmentDoneCommand(normalized) {
            let stripped = stripCommitmentDonePreamble(from: clean)
            return VoiceIntentSuggestion(intent: .commitmentDone, cleanedText: stripped.isEmpty ? clean : stripped)
        }

        if commitmentPrefixes.contains(where: { normalized.contains($0) }) {
            let stripped = stripCommitmentPreamble(from: clean)
            return VoiceIntentSuggestion(intent: .commitment, cleanedText: stripped.isEmpty ? clean : stripped)
        }

        let strippedCapture = stripCapturePreamble(from: clean)
        return VoiceIntentSuggestion(intent: .capture, cleanedText: strippedCapture.isEmpty ? clean : strippedCapture)
    }

    private static func cleanedTranscript(_ transcript: String) -> String {
        transcript.trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private static func stripCommitmentPreamble(from transcript: String) -> String {
        let lowercased = transcript.lowercased()
        for prefix in commitmentPrefixes {
            if lowercased.hasPrefix(prefix) {
                let index = transcript.index(transcript.startIndex, offsetBy: prefix.count)
                return transcript[index...].trimmingCharacters(in: CharacterSet(charactersIn: ": -").union(.whitespacesAndNewlines))
            }
        }
        return transcript
    }

    private static func stripCapturePreamble(from transcript: String) -> String {
        let lowercased = transcript.lowercased()
        for prefix in capturePrefixes {
            if lowercased.hasPrefix(prefix) {
                let index = transcript.index(transcript.startIndex, offsetBy: prefix.count)
                return transcript[index...].trimmingCharacters(in: CharacterSet(charactersIn: ": -").union(.whitespacesAndNewlines))
            }
        }
        return transcript
    }

    private static func stripCommitmentDonePreamble(from transcript: String) -> String {
        let prefixes = [
            "mark ",
            "set ",
            "complete ",
            "completed ",
            "finish ",
            "finished ",
            "done ",
            "i finished ",
            "i completed "
        ]

        var value = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        let lowercased = value.lowercased()
        for prefix in prefixes {
            if lowercased.hasPrefix(prefix) {
                let index = value.index(value.startIndex, offsetBy: prefix.count)
                value = String(value[index...]).trimmingCharacters(in: .whitespacesAndNewlines)
                break
            }
        }

        let suffixes = [
            " as done",
            " done",
            " completed",
            " complete",
            " finished",
            " finish"
        ]
        for suffix in suffixes {
            if value.lowercased().hasSuffix(suffix) {
                value = String(value.dropLast(suffix.count))
                    .trimmingCharacters(in: CharacterSet(charactersIn: ": -").union(.whitespacesAndNewlines))
                break
            }
        }

        return value.trimmingCharacters(in: CharacterSet(charactersIn: ": -").union(.whitespacesAndNewlines))
    }

    private static func isContextQuery(_ text: String) -> Bool {
        [
            "what matters",
            "what do i need",
            "what should i do right now",
            "current context",
            "status right now",
            "good morning"
        ].contains(where: { text.contains($0) })
    }

    private static func isNextCommitmentQuery(_ text: String) -> Bool {
        [
            "what's next",
            "what is next",
            "next commitment",
            "next task",
            "what do i have next"
        ].contains(where: { text.contains($0) })
    }

    private static func isNudgesQuery(_ text: String) -> Bool {
        [
            "active nudges",
            "active reminders",
            "my nudges",
            "my reminders",
            "what are my nudges"
        ].contains(where: { text.contains($0) })
    }

    private static func isExplainQuery(_ text: String) -> Bool {
        if text.contains("what changed") {
            return true
        }
        if text.contains("why") && (text.contains("risk") || text.contains("warning") || text.contains("nudge") || text.contains("now")) {
            return true
        }
        return false
    }

    private static func isNudgeDoneCommand(_ text: String) -> Bool {
        if text == "done" {
            return true
        }
        if text.contains("mark done") || text.contains("resolve nudge") || text.contains("done reminder") {
            return true
        }
        if text.contains("done") && (text.contains("nudge") || text.contains("reminder") || text.contains("that")) {
            return true
        }
        return false
    }

    private static func isCommitmentDoneCommand(_ text: String) -> Bool {
        let hasDoneKeyword = text.contains("done")
            || text.contains("complete")
            || text.contains("completed")
            || text.contains("finish")
            || text.contains("finished")
        guard hasDoneKeyword else { return false }

        if text.contains("nudge") || text.contains("reminder") {
            return false
        }

        if text.hasPrefix("mark ")
            || text.hasPrefix("set ")
            || text.hasPrefix("complete ")
            || text.hasPrefix("completed ")
            || text.hasPrefix("finish ")
            || text.hasPrefix("finished ")
            || text.hasPrefix("done ")
            || text.hasPrefix("i finished ")
            || text.hasPrefix("i completed ")
            || text.hasSuffix(" done")
            || text.hasSuffix(" completed")
            || text.hasSuffix(" finished")
        {
            return true
        }

        return false
    }

    private static func extractMinutes(from text: String) -> Int? {
        let parts = text.split(whereSeparator: { !$0.isNumber })
        for part in parts {
            if let value = Int(part), (1...180).contains(value) {
                return value
            }
        }

        let tokens = text
            .split(whereSeparator: { !$0.isLetter && $0 != "-" })
            .map { String($0) }
        for token in tokens {
            if let value = minuteWords[token], (1...180).contains(value) {
                return value
            }
        }

        if text.contains("half hour") || text.contains("half an hour") {
            return 30
        }
        if text.contains("quarter hour") {
            return 15
        }
        return nil
    }
}

private struct VoiceCaptureEntry: Codable, Identifiable {
    let id: UUID
    let createdAt: Date
    let transcript: String
    let suggestedIntent: VoiceIntent
    let committedIntent: VoiceIntent?
    let status: String
}

private struct VoiceResponse {
    let summary: String
    let detail: String?
}

@MainActor
private final class VoiceCaptureModel: NSObject, ObservableObject {
    @Published var speechPermission: VoicePermissionState = .unknown
    @Published var microphonePermission: VoicePermissionState = .unknown
    @Published var isRecording = false
    @Published var transcript = ""
    @Published var errorMessage: String?
    @Published var suggestedIntent: VoiceIntent = .capture
    @Published var suggestedText = ""
    @Published var history: [VoiceCaptureEntry] = []
    @Published var latestResponse: VoiceResponse?

    private let speechRecognizer = SFSpeechRecognizer(locale: Locale(identifier: "en_US"))
    private let speechSynthesizer = AVSpeechSynthesizer()
    private let audioEngine = AVAudioEngine()
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private let historyKey = "vel.voice.capture.history.v1"
    private var didSaveCurrentSession = false

    var hasTranscript: Bool {
        !transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    override init() {
        super.init()
        loadHistory()
    }

    func ensurePermissionsKnown() async {
        speechPermission = Self.mapSpeechPermission(SFSpeechRecognizer.authorizationStatus())
        microphonePermission = Self.mapMicrophonePermission(AVAudioSession.sharedInstance().recordPermission)
    }

    func requestPermissions() async {
        let speechStatus = await withCheckedContinuation { continuation in
            SFSpeechRecognizer.requestAuthorization { status in
                continuation.resume(returning: status)
            }
        }

        let micGranted = await withCheckedContinuation { continuation in
            AVAudioSession.sharedInstance().requestRecordPermission { granted in
                continuation.resume(returning: granted)
            }
        }

        speechPermission = Self.mapSpeechPermission(speechStatus)
        microphonePermission = micGranted ? .granted : .denied
    }

    func updateTranscript(_ value: String) {
        transcript = value
        let suggestion = VoiceIntentParser.suggest(for: value)
        suggestedIntent = suggestion.intent
        suggestedText = suggestion.cleanedText
    }

    func clearTranscript() {
        transcript = ""
        suggestedText = ""
        suggestedIntent = .capture
        errorMessage = nil
    }

    func speakLatestResponse() {
        guard let latestResponse else { return }
        let parts = [latestResponse.summary, latestResponse.detail].compactMap { $0?.trimmingCharacters(in: .whitespacesAndNewlines) }
        let text = parts.joined(separator: " ")
        guard !text.isEmpty else { return }
        speechSynthesizer.stopSpeaking(at: .immediate)
        let utterance = AVSpeechUtterance(string: text)
        utterance.voice = AVSpeechSynthesisVoice(language: "en-US")
        utterance.rate = AVSpeechUtteranceDefaultSpeechRate * 0.95
        speechSynthesizer.speak(utterance)
    }

    func toggleRecording() async {
        if isRecording {
            stopRecording(saveEntry: true)
        } else {
            await startRecording()
        }
    }

    func submitSuggested(using store: VelClientStore) async {
        let intent = suggestedIntent
        await submit(using: store, intent: intent)
    }

    func submitAsCapture(using store: VelClientStore) async {
        await submit(using: store, intent: .capture)
    }

    func submitAsCommitment(using store: VelClientStore) async {
        await submit(using: store, intent: .commitment)
    }

    func targetHint(from nudges: [NudgeData], commitments: [CommitmentData]) -> String {
        if suggestedIntent.requiresNudgeTarget {
            guard let topNudge = nudges.first(where: { $0.state == "active" || $0.state == "snoozed" }) else {
                return "No active nudge available. Submission falls back to capture-only provenance."
            }
            return "Target nudge: \(topNudge.message)"
        }

        if suggestedIntent.requiresCommitmentTarget {
            let target = suggestedText.trimmingCharacters(in: .whitespacesAndNewlines)
            let openCommitments = commitments.filter { $0.status == "open" }
            guard !openCommitments.isEmpty else {
                return "No open commitments available. Submission falls back to capture-only provenance."
            }
            guard !target.isEmpty else {
                return "Include commitment text, for example: “mark meds done.”"
            }

            let matches = rankedCommitmentMatches(for: target, in: openCommitments)
            if matches.isEmpty {
                return "No open commitment match for: \(target)"
            }
            if matches.count == 1 || !isAmbiguousTopMatch(matches) {
                return "Target commitment: \(matches[0].commitment.text)"
            }

            let options = matches.prefix(3).map { $0.commitment.text }.joined(separator: " | ")
            return "Ambiguous target. Could match: \(options)"
        }

        return ""
    }

    private func startRecording() async {
        errorMessage = nil
        transcript = ""
        suggestedIntent = .capture
        suggestedText = ""
        latestResponse = nil
        didSaveCurrentSession = false
        speechSynthesizer.stopSpeaking(at: .immediate)

        if speechPermission == .unknown || microphonePermission == .unknown {
            await requestPermissions()
        }

        guard speechPermission == .granted else {
            errorMessage = "Speech recognition permission is required for voice capture."
            return
        }
        guard microphonePermission == .granted else {
            errorMessage = "Microphone permission is required for voice capture."
            return
        }
        guard let speechRecognizer, speechRecognizer.isAvailable else {
            errorMessage = "Speech recognizer is currently unavailable."
            return
        }

        stopRecording(saveEntry: false)

        do {
            let audioSession = AVAudioSession.sharedInstance()
            try audioSession.setCategory(.record, mode: .measurement, options: [.duckOthers])
            try audioSession.setActive(true, options: .notifyOthersOnDeactivation)

            let request = SFSpeechAudioBufferRecognitionRequest()
            request.shouldReportPartialResults = true
            recognitionRequest = request

            let inputNode = audioEngine.inputNode
            inputNode.removeTap(onBus: 0)
            let recordingFormat = inputNode.outputFormat(forBus: 0)
            inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { [weak self] buffer, _ in
                self?.recognitionRequest?.append(buffer)
            }

            audioEngine.prepare()
            try audioEngine.start()
            isRecording = true

            recognitionTask = speechRecognizer.recognitionTask(with: request) { [weak self] result, error in
                guard let self else { return }
                Task { @MainActor in
                    if let result {
                        let text = result.bestTranscription.formattedString
                        self.updateTranscript(text)
                        if result.isFinal {
                            self.stopRecording(saveEntry: true)
                        }
                    }

                    if let error {
                        self.errorMessage = error.localizedDescription
                        self.stopRecording(saveEntry: !self.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                    }
                }
            }
        } catch {
            errorMessage = "Could not start recording. \(error.localizedDescription)"
            stopRecording(saveEntry: false)
        }
    }

    private func stopRecording(saveEntry: Bool) {
        if audioEngine.isRunning {
            audioEngine.stop()
            audioEngine.inputNode.removeTap(onBus: 0)
        }

        recognitionRequest?.endAudio()
        recognitionRequest = nil
        recognitionTask?.cancel()
        recognitionTask = nil
        isRecording = false

        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)

        if saveEntry {
            persistCurrentTranscriptIfNeeded()
        }
    }

    private func submit(using store: VelClientStore, intent: VoiceIntent) async {
        let text = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !text.isEmpty else {
            errorMessage = "No transcript available. Record first or retry recognition."
            return
        }

        let primaryText = suggestedText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
            ? text
            : suggestedText.trimmingCharacters(in: .whitespacesAndNewlines)

        let capturePayload = voiceCapturePayload(transcript: text, intent: intent)
        await store.createCapture(
            text: capturePayload,
            type: "voice_note",
            source: "apple_ios_voice"
        )

        var committedIntent: VoiceIntent?
        var historyStatus = historyStatus(for: intent, isReachable: store.isReachable)
        switch intent.kind {
        case .captureCreate:
            committedIntent = .capture
            setResponse(
                summary: store.isReachable ? "Saved voice capture." : "Voice capture queued for sync.",
                detail: primaryText
            )
        case .commitmentCreate:
            await store.createCommitment(text: primaryText)
            committedIntent = .commitment
            setResponse(
                summary: store.isReachable ? "Created commitment." : "Commitment queued for sync.",
                detail: primaryText
            )
        case .commitmentDone:
            let target = primaryText.trimmingCharacters(in: .whitespacesAndNewlines)
            guard !target.isEmpty else {
                historyStatus = "needs_clarification"
                errorMessage = "Commitment target missing. Clarify and retry."
                setResponse(
                    summary: "Commitment target is missing.",
                    detail: "Try phrasing like “mark meds done.”"
                )
                break
            }
            let matches = rankedCommitmentMatches(
                for: target,
                in: store.commitments.filter { $0.status == "open" }
            )
            if let best = matches.first?.commitment {
                if isAmbiguousTopMatch(matches) {
                    historyStatus = "needs_clarification"
                    let options = matches.prefix(3).map { $0.commitment.text }.joined(separator: " | ")
                    errorMessage = "Commitment target was ambiguous. Clarify and retry."
                    setResponse(
                        summary: "Ambiguous commitment target.",
                        detail: "Could match: \(options)"
                    )
                } else {
                    await store.markCommitmentDone(id: best.id)
                    committedIntent = .commitmentDone
                    setResponse(
                        summary: store.isReachable ? "Resolved commitment." : "Commitment completion queued.",
                        detail: best.text
                    )
                }
            } else {
                historyStatus = "capture_only"
                errorMessage = "No open commitment matched. Saved as capture only."
                let detail = target.isEmpty
                    ? "Saved transcript as capture provenance only."
                    : "Target: \(target). Saved transcript as capture provenance only."
                setResponse(
                    summary: "No open commitment matched.",
                    detail: detail
                )
            }
        case .nudgeDone:
            if let nudgeID = firstActionableNudgeID(from: store.nudges) {
                await store.markNudgeDone(id: nudgeID)
                committedIntent = .nudgeDone
                setResponse(
                    summary: store.isReachable ? "Resolved top nudge." : "Top nudge resolution queued.",
                    detail: nil
                )
            } else {
                errorMessage = "No active nudge found. Saved as capture only."
                historyStatus = "capture_only"
                setResponse(
                    summary: "No active nudge found.",
                    detail: "Saved transcript as capture provenance only."
                )
            }
        case .nudgeSnooze:
            let minutes = intent.minutes ?? 10
            if let nudgeID = firstActionableNudgeID(from: store.nudges) {
                await store.snoozeNudge(id: nudgeID, minutes: minutes)
                committedIntent = .nudgeSnooze(minutes)
                setResponse(
                    summary: store.isReachable ? "Snoozed top nudge \(minutes) minutes." : "Top nudge snooze queued for \(minutes) minutes.",
                    detail: nil
                )
            } else {
                errorMessage = "No active nudge found. Saved as capture only."
                historyStatus = "capture_only"
                setResponse(
                    summary: "No active nudge found.",
                    detail: "Saved transcript as capture provenance only."
                )
            }
        case .queryContext:
            committedIntent = .queryContext
            setResponse(buildContextResponse(from: store))
        case .queryNextCommitment:
            committedIntent = .queryNextCommitment
            setResponse(buildNextCommitmentResponse(from: store))
        case .queryNudges:
            committedIntent = .queryNudges
            setResponse(buildNudgesResponse(from: store))
        case .explainContext:
            committedIntent = .explainContext
            setResponse(buildExplanationResponse(from: store))
        }

        appendHistoryEntry(
            transcript: text,
            suggestedIntent: suggestedIntent,
            committedIntent: committedIntent,
            status: historyStatus
        )

        if !store.isReachable {
            if errorMessage == nil {
                if intent.isQuery {
                    errorMessage = "Answered from offline cache. Transcript capture queued for sync."
                } else {
                    errorMessage = "Voice action queued for sync while offline."
                }
            }
        } else if committedIntent != nil {
            errorMessage = nil
        }
    }

    private func historyStatus(for intent: VoiceIntent, isReachable: Bool) -> String {
        if intent.isQuery {
            return isReachable ? "answered" : "answered_cached"
        }
        return isReachable ? "submitted" : "queued"
    }

    private func setResponse(_ response: VoiceResponse) {
        setResponse(summary: response.summary, detail: response.detail)
    }

    private func setResponse(summary: String, detail: String?) {
        let cleanSummary = summary.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !cleanSummary.isEmpty else {
            latestResponse = nil
            return
        }
        let cleanDetail = detail?.trimmingCharacters(in: .whitespacesAndNewlines)
        latestResponse = VoiceResponse(
            summary: cleanSummary,
            detail: (cleanDetail?.isEmpty ?? true) ? nil : cleanDetail
        )
    }

    private struct CommitmentMatch {
        let commitment: CommitmentData
        let score: Int
    }

    private func rankedCommitmentMatches(for query: String, in commitments: [CommitmentData]) -> [CommitmentMatch] {
        let normalizedQuery = normalizeForMatching(query)
        guard !normalizedQuery.isEmpty else { return [] }
        let queryTokens = Set(normalizedQuery.split(separator: " ").map(String.init))

        let matches = commitments.compactMap { commitment -> CommitmentMatch? in
            let normalizedText = normalizeForMatching(commitment.text)
            if normalizedText.isEmpty {
                return nil
            }

            var score = 0
            if !normalizedQuery.isEmpty {
                if normalizedText == normalizedQuery {
                    score += 120
                }
                if normalizedText.hasPrefix(normalizedQuery) {
                    score += 30
                }
                if normalizedText.contains(normalizedQuery) {
                    score += 80
                }
                if normalizedQuery.contains(normalizedText) {
                    score += 30
                }
            }

            let textTokens = Set(normalizedText.split(separator: " ").map(String.init))
            let overlap = queryTokens.intersection(textTokens).count
            score += overlap * 15

            guard score > 0 else { return nil }
            return CommitmentMatch(commitment: commitment, score: score)
        }

        return matches.sorted { lhs, rhs in
            if lhs.score != rhs.score {
                return lhs.score > rhs.score
            }
            switch (lhs.commitment.due_at, rhs.commitment.due_at) {
            case let (l?, r?):
                return l < r
            case (.some, .none):
                return true
            case (.none, .some):
                return false
            case (.none, .none):
                return lhs.commitment.text < rhs.commitment.text
            }
        }
    }

    private func isAmbiguousTopMatch(_ matches: [CommitmentMatch]) -> Bool {
        guard matches.count > 1 else { return false }
        let first = matches[0]
        let second = matches[1]
        return second.score >= max(first.score - 8, 35)
    }

    private func normalizeForMatching(_ text: String) -> String {
        text
            .lowercased()
            .components(separatedBy: CharacterSet.alphanumerics.inverted)
            .filter { !$0.isEmpty }
            .joined(separator: " ")
    }

    private func nextOpenCommitment(from store: VelClientStore, preferredID: String?) -> CommitmentData? {
        let open = store.commitments.filter { $0.status == "open" }
        if let preferredID, let matching = open.first(where: { $0.id == preferredID }) {
            return matching
        }
        return open.sorted { lhs, rhs in
            switch (lhs.due_at, rhs.due_at) {
            case let (l?, r?):
                return l < r
            case (.some, .none):
                return true
            case (.none, .some):
                return false
            case (.none, .none):
                return lhs.text < rhs.text
            }
        }.first
    }

    private func buildContextResponse(from store: VelClientStore) -> VoiceResponse {
        guard let context = store.context?.context else {
            return VoiceResponse(
                summary: "No current context is cached yet.",
                detail: "Refresh when Vel is reachable to load context."
            )
        }

        var summaryParts: [String] = []
        if let mode = context.mode, !mode.isEmpty {
            summaryParts.append("Mode: \(mode).")
        }

        if let next = nextOpenCommitment(from: store, preferredID: context.next_commitment_id) {
            if let due = next.due_at {
                summaryParts.append("Next commitment: \(next.text) due \(formatUnix(due)).")
            } else {
                summaryParts.append("Next commitment: \(next.text).")
            }
        }

        if context.prep_window_active == true {
            summaryParts.append("Prep window is active.")
        }
        if context.commute_window_active == true {
            summaryParts.append("Commute window is active.")
        }

        let activeNudges = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        var detailParts: [String] = []
        if let meds = context.meds_status, !meds.isEmpty {
            detailParts.append("Meds: \(meds).")
        }
        if let attention = context.attention_state, !attention.isEmpty {
            detailParts.append("Attention: \(attention).")
        }
        if let drift = context.drift_type, !drift.isEmpty {
            detailParts.append("Drift: \(drift).")
        }
        if !activeNudges.isEmpty {
            detailParts.append("Active nudges: \(activeNudges.count). Top: \(activeNudges[0].message)")
        }

        let summary = summaryParts.isEmpty ? "Context is available." : summaryParts.joined(separator: " ")
        let detail = detailParts.isEmpty ? nil : detailParts.joined(separator: " ")
        return VoiceResponse(summary: summary, detail: detail)
    }

    private func buildNextCommitmentResponse(from store: VelClientStore) -> VoiceResponse {
        let preferred = store.context?.context?.next_commitment_id
        guard let next = nextOpenCommitment(from: store, preferredID: preferred) else {
            return VoiceResponse(
                summary: "No open commitments are cached.",
                detail: "Sync or create a commitment to set a next action."
            )
        }
        if let due = next.due_at {
            return VoiceResponse(
                summary: "Next commitment: \(next.text).",
                detail: "Due \(formatUnix(due))."
            )
        }
        return VoiceResponse(
            summary: "Next commitment: \(next.text).",
            detail: nil
        )
    }

    private func buildNudgesResponse(from store: VelClientStore) -> VoiceResponse {
        let active = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        guard !active.isEmpty else {
            return VoiceResponse(
                summary: "No active nudges right now.",
                detail: "You're clear on current nudge backlog."
            )
        }

        let top = active[0]
        let summary = "You have \(active.count) active nudges. Top nudge: \(top.message)"
        var detail = "Top nudge type: \(top.nudge_type), level: \(top.level), state: \(top.state)."
        if let snoozedUntil = top.snoozed_until, top.state == "snoozed" {
            detail += " Snoozed until \(formatUnix(snoozedUntil))."
        }
        return VoiceResponse(summary: summary, detail: detail)
    }

    private func buildExplanationResponse(from store: VelClientStore) -> VoiceResponse {
        let active = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        let context = store.context?.context

        var reasons: [String] = []
        if context?.prep_window_active == true {
            reasons.append("Prep window is currently active.")
        }
        if context?.commute_window_active == true {
            reasons.append("Commute window is active.")
        }
        if let meds = context?.meds_status, !meds.isEmpty, meds.lowercased() != "taken" {
            reasons.append("Meds status is \(meds).")
        }
        if let attention = context?.attention_state, !attention.isEmpty {
            reasons.append("Attention state is \(attention).")
        }
        if let drift = context?.drift_type, !drift.isEmpty {
            reasons.append("Drift signal is \(drift).")
        }
        if let top = active.first {
            reasons.append("Top nudge is \(top.message) (\(top.level)).")
        }

        guard !reasons.isEmpty else {
            return VoiceResponse(
                summary: "No strong risk signals are visible in the cached context.",
                detail: "Try refreshing for newer context and nudges."
            )
        }

        let summary = reasons[0]
        let detail = reasons.dropFirst().joined(separator: " ")
        return VoiceResponse(
            summary: summary,
            detail: detail.isEmpty ? nil : detail
        )
    }

    private func voiceCapturePayload(transcript: String, intent: VoiceIntent) -> String {
        [
            "voice_transcript:",
            transcript,
            "",
            "intent_candidate: \(intent.storageToken)",
            "client_surface: ios_voice"
        ]
        .joined(separator: "\n")
    }

    private func persistCurrentTranscriptIfNeeded() {
        guard !didSaveCurrentSession else { return }
        let clean = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !clean.isEmpty else { return }
        didSaveCurrentSession = true
        appendHistoryEntry(
            transcript: clean,
            suggestedIntent: suggestedIntent,
            committedIntent: nil,
            status: "pending_review"
        )
    }

    private func appendHistoryEntry(
        transcript: String,
        suggestedIntent: VoiceIntent,
        committedIntent: VoiceIntent?,
        status: String
    ) {
        let entry = VoiceCaptureEntry(
            id: UUID(),
            createdAt: Date(),
            transcript: transcript,
            suggestedIntent: suggestedIntent,
            committedIntent: committedIntent,
            status: status
        )
        history.insert(entry, at: 0)
        history = Array(history.prefix(40))
        saveHistory()
    }

    private func firstActionableNudgeID(from nudges: [NudgeData]) -> String? {
        nudges.first(where: { $0.state == "active" || $0.state == "snoozed" })?.nudge_id
    }

    private func loadHistory() {
        guard let data = UserDefaults.standard.data(forKey: historyKey) else { return }
        if let decoded = try? JSONDecoder().decode([VoiceCaptureEntry].self, from: data) {
            history = decoded
        }
    }

    private func saveHistory() {
        if let data = try? JSONEncoder().encode(history) {
            UserDefaults.standard.set(data, forKey: historyKey)
        }
    }

    private static func mapSpeechPermission(_ status: SFSpeechRecognizerAuthorizationStatus) -> VoicePermissionState {
        switch status {
        case .authorized:
            return .granted
        case .denied, .restricted:
            return .denied
        case .notDetermined:
            return .unknown
        @unknown default:
            return .unknown
        }
    }

    private static func mapMicrophonePermission(_ status: AVAudioSession.RecordPermission) -> VoicePermissionState {
        switch status {
        case .granted:
            return .granted
        case .denied:
            return .denied
        case .undetermined:
            return .unknown
        @unknown default:
            return .unknown
        }
    }
}

private func formatUnix(_ timestamp: Int) -> String {
    let date = Date(timeIntervalSince1970: TimeInterval(timestamp))
    return formatDate(date)
}

private func formatDate(_ date: Date) -> String {
    let formatter = DateFormatter()
    formatter.dateStyle = .medium
    formatter.timeStyle = .short
    return formatter.string(from: date)
}

#Preview {
    ContentView()
        .environmentObject(VelClientStore())
}
