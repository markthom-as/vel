import AVFoundation
#if canImport(Speech)
import Speech
#endif
#if canImport(WatchKit)
import WatchKit
#endif
import SwiftUI
import VelApplePlatform
import VelApplication

struct ContentView: View {
    let appEnvironment: VelAppEnvironment
    @EnvironmentObject var store: VelWatchStore
    @StateObject private var voiceModel = WatchVoiceCaptureModel()
    @State private var threadText = ""
    @State private var captureText = ""
    @State private var signalNote = ""

    var body: some View {
        List {
            sectionSnapshot
            sectionNudges
            sectionSignals
            sectionCapture
            sectionThreadAppend
            sectionVoice
        }
        .navigationTitle("Vel")
        .listStyle(.plain)
        .task {
            await store.refresh()
            await voiceModel.ensurePermissionsKnown()
        }
        .refreshable {
            await store.refresh()
        }
    }

    @ViewBuilder
    private var sectionSnapshot: some View {
        Section("Right now") {
            VStack(alignment: .leading, spacing: 6) {
                Text(store.compactStatusLine)
                    .font(.headline)

                if let scheduleSummary = store.scheduleSummary {
                    Label(scheduleSummary, systemImage: "calendar")
                        .font(.caption)
                }

                if let scheduleDetail = store.scheduleDetail {
                    Text(scheduleDetail)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                if let mode = store.mode {
                    Text("Mode: \(mode)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                if let driftSummary = store.driftSummary {
                    Label(driftSummary, systemImage: "waveform.path.ecg")
                        .font(.caption)
                        .foregroundStyle(.orange)
                }

                if let scheduleProposalStatus = store.scheduleProposalStatus {
                    Text(scheduleProposalStatus)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                if store.pendingActionCount > 0 {
                    Label("\(store.pendingActionCount) queued", systemImage: "arrow.triangle.2.circlepath")
                        .font(.caption2)
                        .foregroundStyle(.yellow)
                }

                Text(store.transportSummary)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
    }

    @ViewBuilder
    private var sectionNudges: some View {
        Section("Nudges") {
            HStack {
                Text("Active")
                Spacer()
                Text("\(store.nudgeCount)")
                    .foregroundStyle(.secondary)
            }

            if store.nudgeCount == 0 {
                Text("No active nudges")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } else {
                VStack(alignment: .leading, spacing: 8) {
                    Text(store.message)
                        .font(.caption)
                    HStack {
                        Button("Done") {
                            playHaptic(.success)
                            Task { await store.markTopNudgeDone() }
                        }
                        Button("Snooze 10m") {
                            playHaptic(.directionDown)
                            Task { await store.snoozeTopNudge(minutes: 10) }
                        }
                    }
                    .buttonStyle(.bordered)
                }
            }
        }
    }

    @ViewBuilder
    private var sectionSignals: some View {
        Section("Signals") {
            Text("Send a quick event upstream without opening a longer flow.")
                .font(.caption2)
                .foregroundStyle(.secondary)

            ForEach(WatchSignalKind.allCases) { kind in
                Button(kind.title) {
                    playHaptic(.click)
                    Task { await store.emitSignal(kind, note: signalNote) }
                }
                .buttonStyle(.bordered)
            }

            TextField("Optional note", text: $signalNote)
                .textInputAutocapitalization(.sentences)

            statusFootnote(store.lastActionStatus)
        }
    }

    @ViewBuilder
    private var sectionCapture: some View {
        Section("Quick capture") {
            TextField("Note, task, feeling", text: $captureText)
                .textInputAutocapitalization(.sentences)

            Button("Save note") {
                submitCapture(asTask: false)
            }
            .buttonStyle(.borderedProminent)
            .disabled(captureText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

            Button("Add task") {
                submitCapture(asTask: true)
            }
            .buttonStyle(.bordered)
            .disabled(captureText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

            Text("For anything longer, hand off to iPhone or Mac.")
                .font(.caption2)
                .foregroundStyle(.secondary)

            statusFootnote(store.lastActionStatus)
        }
    }

    @ViewBuilder
    private var sectionThreadAppend: some View {
        Section("Follow through") {
            if let threadID = store.activeThreadID {
                Text("Current thread: \(threadID)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)
            } else {
                Text("No active thread found. Input is saved as a note.")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }

            TextField("Type note", text: $threadText)
                .textInputAutocapitalization(.sentences)
                .submitLabel(.send)
                .onSubmit {
                    submitThreadText()
                }

            Button("Append") {
                submitThreadText()
            }
            .buttonStyle(.borderedProminent)
            .disabled(threadText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

            Button("Escalate to phone") {
                submitEscalation()
            }
            .buttonStyle(.bordered)
            .disabled(threadText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

            Text(store.handoffSummary)
                .font(.caption2)
                .foregroundStyle(.secondary)

            statusFootnote(store.lastActionStatus)
        }
    }

    @ViewBuilder
    private var sectionVoice: some View {
        Section("Voice") {
            PermissionRow(label: "Microphone", state: voiceModel.microphonePermission)
            PermissionRow(label: "Speech recognition", state: voiceModel.speechPermission)

            HStack {
                Button(voiceModel.isRecording ? "Stop" : "Start capture") {
                    playHaptic(voiceModel.isRecording ? .stop : .start)
                    Task { await voiceModel.toggleRecording() }
                }
                .buttonStyle(.borderedProminent)

                if voiceModel.isRecording {
                    ProgressView()
                        .controlSize(.small)
                }
            }

            if voiceModel.speechPermission != .granted || voiceModel.microphonePermission != .granted {
                Button("Request permissions") {
                    Task {
                        await voiceModel.requestPermissions()
                    }
                }
                .buttonStyle(.bordered)
            }

            if voiceModel.isRecording {
                Text("Listening…")
                    .font(.caption)
                    .foregroundStyle(.orange)
            }

            if !voiceModel.transcript.isEmpty {
                Text(voiceModel.transcript)
                    .font(.caption)
                    .foregroundStyle(.secondary)

                Button("Append transcript") {
                    playHaptic(.click)
                    submitThreadText(from: voiceModel.transcript)
                    voiceModel.clearTranscript()
                }
                .buttonStyle(.bordered)

                Button("Save as note") {
                    playHaptic(.click)
                    submitVoiceCapture()
                }
                .buttonStyle(.bordered)
            }

            if let error = voiceModel.errorMessage {
                statusFootnote(error)
            }

            statusFootnote(store.lastActionStatus)
        }
    }

    @ViewBuilder
    private func statusFootnote(_ value: String?) -> some View {
        if let value, !value.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            Text(value)
                .font(.caption2)
                .foregroundStyle(.secondary)
        }
    }

    private func submitThreadText() {
        submitThreadText(from: threadText)
        Task {
            await MainActor.run { threadText = "" }
        }
    }

    private func submitThreadText(from value: String) {
        let trimmed = value.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }

        Task {
            await store.submitThreadText(trimmed)
        }
    }

    private func submitCapture(asTask: Bool) {
        let trimmed = captureText.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }
        playHaptic(asTask ? .success : .click)
        Task {
            if asTask {
                await store.createCommitment(text: trimmed)
            } else {
                await store.createCapture(text: trimmed)
            }
            await MainActor.run { captureText = "" }
        }
    }

    private func submitVoiceCapture() {
        let trimmed = voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }
        Task {
            await store.createCapture(text: trimmed, type: "watch_voice_note", source: "apple_watch_voice")
            await MainActor.run {
                voiceModel.clearTranscript()
            }
        }
    }

    private func submitEscalation() {
        let trimmed = threadText.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }
        playHaptic(.notification)
        Task {
            await store.queueEscalationRequest(trimmed)
            await MainActor.run { threadText = "" }
        }
    }

    private func playHaptic(_ type: WKHapticType) {
        #if canImport(WatchKit)
        WKInterfaceDevice.current().play(type)
        #endif
    }
}

private enum WatchPermissionState: String, Codable {
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
            return "xmark.circle"
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

private struct PermissionRow: View {
    let label: String
    let state: WatchPermissionState

    var body: some View {
        HStack {
            Text(label)
            Spacer()
            Label(state.displayLabel, systemImage: state.icon)
                .foregroundStyle(state.color)
                .font(.caption2)
        }
    }
}

@MainActor
private final class WatchVoiceCaptureModel: NSObject, ObservableObject {
    @Published var speechPermission: WatchPermissionState = .unknown
    @Published var microphonePermission: WatchPermissionState = .unknown
    @Published var isRecording = false
    @Published var transcript = ""
    @Published var errorMessage: String?

#if canImport(Speech)
    private let speechRecognizer = SFSpeechRecognizer(locale: Locale(identifier: "en_US"))
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
#endif
    private let audioEngine = AVAudioEngine()

    func ensurePermissionsKnown() async {
#if canImport(Speech)
        speechPermission = Self.mapSpeechPermission(SFSpeechRecognizer.authorizationStatus())
#else
        speechPermission = .denied
#endif
        microphonePermission = Self.mapMicrophonePermission(AVAudioSession.sharedInstance().recordPermission)
    }

    func requestPermissions() async {
#if canImport(Speech)
        let speechStatus = await withCheckedContinuation { continuation in
            SFSpeechRecognizer.requestAuthorization { status in
                continuation.resume(returning: status)
            }
        }
#else
        let speechStatus = WatchPermissionState.denied
#endif
        let micGranted = await withCheckedContinuation { continuation in
            AVAudioSession.sharedInstance().requestRecordPermission { granted in
                continuation.resume(returning: granted)
            }
        }

#if canImport(Speech)
        speechPermission = Self.mapSpeechPermission(speechStatus)
#else
        speechPermission = speechStatus
#endif
        microphonePermission = micGranted ? .granted : .denied
    }

    func toggleRecording() async {
        if isRecording {
            stopRecording(save: true)
        } else {
            await startRecording()
        }
    }

    func clearTranscript() {
        transcript = ""
        errorMessage = nil
    }

    private func startRecording() async {
        errorMessage = nil
        transcript = ""

#if !canImport(Speech)
        speechPermission = .denied
        errorMessage = "Speech recognition is unavailable for this watch build."
        return
#else
        if speechPermission == .unknown || microphonePermission == .unknown {
            await requestPermissions()
        }

        guard speechPermission == .granted else {
            errorMessage = "Speech recognition permission required."
            return
        }
        guard microphonePermission == .granted else {
            errorMessage = "Microphone permission required."
            return
        }
        guard let speechRecognizer, speechRecognizer.isAvailable else {
            errorMessage = "Speech recognizer is unavailable."
            return
        }

        stopRecording(save: false)

        do {
            let session = AVAudioSession.sharedInstance()
            try session.setCategory(.record, mode: .default, options: [.duckOthers])
            try session.setActive(true, options: .notifyOthersOnDeactivation)

            let request = SFSpeechAudioBufferRecognitionRequest()
            request.shouldReportPartialResults = true
            recognitionRequest = request

            let inputNode = audioEngine.inputNode
            inputNode.removeTap(onBus: 0)
            let format = inputNode.outputFormat(forBus: 0)
            inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) { [weak self] buffer, _ in
                self?.recognitionRequest?.append(buffer)
            }

            audioEngine.prepare()
            try audioEngine.start()
            isRecording = true

            recognitionTask = speechRecognizer.recognitionTask(with: request) { [weak self] result, error in
                guard let self else { return }
                Task { @MainActor in
                    if let result {
                        self.transcript = result.bestTranscription.formattedString
                        if result.isFinal {
                            self.stopRecording(save: true)
                        }
                    }

                    if let error {
                        self.errorMessage = error.localizedDescription
                        self.stopRecording(save: !self.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                    }
                }
            }
        } catch {
            errorMessage = "Failed to start voice capture. \(error.localizedDescription)"
            stopRecording(save: false)
        }
#endif
    }

    private func stopRecording(save: Bool) {
        if audioEngine.isRunning {
            audioEngine.stop()
            audioEngine.inputNode.removeTap(onBus: 0)
        }

#if canImport(Speech)
        recognitionRequest?.endAudio()
        recognitionRequest = nil
        recognitionTask?.cancel()
        recognitionTask = nil
#endif
        isRecording = false

        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)

        if !save {
            transcript = ""
        }
    }

    #if canImport(Speech)
    private static func mapSpeechPermission(_ value: SFSpeechRecognizerAuthorizationStatus) -> WatchPermissionState {
        switch value {
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
    #endif

    private static func mapMicrophonePermission(_ value: AVAudioSession.RecordPermission) -> WatchPermissionState {
        switch value {
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
