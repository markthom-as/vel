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

                if !voiceModel.transcript.isEmpty {
                    Text(voiceModel.transcript)
                        .font(.body)
                        .padding(.vertical, 4)
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

                if voiceModel.suggestedIntent.requiresNudgeTarget {
                    Text(voiceModel.targetHint(from: store.nudges))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                Button("Submit suggested action") {
                    Task { await voiceModel.submitSuggested(using: store) }
                }
                .buttonStyle(.borderedProminent)
                .disabled(voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

                HStack {
                    Button("Save as capture") {
                        Task { await voiceModel.submitAsCapture(using: store) }
                    }
                    .buttonStyle(.bordered)
                    .disabled(voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

                    Button("Create commitment") {
                        Task { await voiceModel.submitAsCommitment(using: store) }
                    }
                    .buttonStyle(.bordered)
                    .disabled(voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                }

                Text("Voice submissions always preserve transcript provenance as a voice capture in Vel.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
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
        case nudgeDone = "nudge_done"
        case nudgeSnooze = "nudge_snooze"
    }

    let kind: Kind
    let minutes: Int?

    static let capture = VoiceIntent(kind: .captureCreate, minutes: nil)
    static let commitment = VoiceIntent(kind: .commitmentCreate, minutes: nil)
    static let nudgeDone = VoiceIntent(kind: .nudgeDone, minutes: nil)
    static func nudgeSnooze(_ minutes: Int) -> VoiceIntent {
        VoiceIntent(kind: .nudgeSnooze, minutes: minutes)
    }

    var displayLabel: String {
        switch kind {
        case .captureCreate:
            return "Capture"
        case .commitmentCreate:
            return "Commitment"
        case .nudgeDone:
            return "Resolve top nudge"
        case .nudgeSnooze:
            return "Snooze top nudge (\(minutes ?? 10)m)"
        }
    }

    var storageToken: String {
        switch kind {
        case .captureCreate:
            return "capture_create"
        case .commitmentCreate:
            return "commitment_create"
        case .nudgeDone:
            return "nudge_done"
        case .nudgeSnooze:
            return "nudge_snooze_\(minutes ?? 10)m"
        }
    }

    var requiresNudgeTarget: Bool {
        kind == .nudgeDone || kind == .nudgeSnooze
    }
}

private struct VoiceIntentSuggestion {
    let intent: VoiceIntent
    let cleanedText: String
}

private enum VoiceIntentParser {
    static func suggest(for transcript: String) -> VoiceIntentSuggestion {
        let clean = cleanedTranscript(transcript)
        let normalized = clean.lowercased()

        if normalized.contains("snooze") && (normalized.contains("nudge") || normalized.contains("reminder")) {
            return VoiceIntentSuggestion(intent: .nudgeSnooze(extractMinutes(from: normalized) ?? 10), cleanedText: clean)
        }

        if normalized.contains("mark done") || normalized.contains("resolve nudge") || normalized.contains("done reminder") {
            return VoiceIntentSuggestion(intent: .nudgeDone, cleanedText: clean)
        }

        let commitmentPrefixes = [
            "todo",
            "to do",
            "task",
            "remind me to",
            "remember to",
            "i need to",
            "follow up",
            "follow-up",
            "next action"
        ]
        if commitmentPrefixes.contains(where: { normalized.contains($0) }) {
            let stripped = stripCommitmentPreamble(from: clean)
            return VoiceIntentSuggestion(intent: .commitment, cleanedText: stripped.isEmpty ? clean : stripped)
        }

        return VoiceIntentSuggestion(intent: .capture, cleanedText: clean)
    }

    private static func cleanedTranscript(_ transcript: String) -> String {
        transcript.trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private static func stripCommitmentPreamble(from transcript: String) -> String {
        let prefixes = [
            "todo",
            "to do",
            "task",
            "remind me to",
            "remember to",
            "i need to",
            "follow up to",
            "follow-up to",
            "next action"
        ]
        let lowercased = transcript.lowercased()
        for prefix in prefixes {
            if lowercased.hasPrefix(prefix) {
                let index = transcript.index(transcript.startIndex, offsetBy: prefix.count)
                return transcript[index...].trimmingCharacters(in: CharacterSet(charactersIn: ": -").union(.whitespacesAndNewlines))
            }
        }
        return transcript
    }

    private static func extractMinutes(from text: String) -> Int? {
        let parts = text.split(whereSeparator: { !$0.isNumber })
        for part in parts {
            if let value = Int(part), (1...180).contains(value) {
                return value
            }
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

    private let speechRecognizer = SFSpeechRecognizer(locale: Locale(identifier: "en_US"))
    private let audioEngine = AVAudioEngine()
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private let historyKey = "vel.voice.capture.history.v1"
    private var didSaveCurrentSession = false

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

    func targetHint(from nudges: [NudgeData]) -> String {
        guard suggestedIntent.requiresNudgeTarget else { return "" }
        guard let topNudge = nudges.first(where: { $0.state == "active" || $0.state == "snoozed" }) else {
            return "No active nudge available. Submission falls back to capture-only provenance."
        }
        return "Target nudge: \(topNudge.message)"
    }

    private func startRecording() async {
        errorMessage = nil
        transcript = ""
        suggestedIntent = .capture
        suggestedText = ""
        didSaveCurrentSession = false

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
                        self.transcript = text
                        let suggestion = VoiceIntentParser.suggest(for: text)
                        self.suggestedIntent = suggestion.intent
                        self.suggestedText = suggestion.cleanedText
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
        switch intent.kind {
        case .captureCreate:
            committedIntent = .capture
        case .commitmentCreate:
            await store.createCommitment(text: primaryText)
            committedIntent = .commitment
        case .nudgeDone:
            if let nudgeID = firstActionableNudgeID(from: store.nudges) {
                await store.markNudgeDone(id: nudgeID)
                committedIntent = .nudgeDone
            } else {
                errorMessage = "No active nudge found. Saved as capture only."
            }
        case .nudgeSnooze:
            let minutes = intent.minutes ?? 10
            if let nudgeID = firstActionableNudgeID(from: store.nudges) {
                await store.snoozeNudge(id: nudgeID, minutes: minutes)
                committedIntent = .nudgeSnooze(minutes)
            } else {
                errorMessage = "No active nudge found. Saved as capture only."
            }
        }

        appendHistoryEntry(
            transcript: text,
            suggestedIntent: suggestedIntent,
            committedIntent: committedIntent,
            status: store.isReachable ? "submitted" : "queued"
        )

        if !store.isReachable {
            errorMessage = "Voice action queued for sync while offline."
        }
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
