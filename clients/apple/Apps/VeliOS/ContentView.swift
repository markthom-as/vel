import AVFoundation
import PhotosUI
import Speech
import SwiftUI
import VelAPI
#if canImport(UIKit)
import UIKit
#endif

private enum VeliOSTab: Hashable {
    case today
    case nudges
    case activity
    case capture
    case voice
    case settings
}

private struct CaptureDraftSeed: Equatable {
    let id: UUID
    let transcript: String
    let note: String

    init(transcript: String, note: String = "") {
        id = UUID()
        self.transcript = transcript
        self.note = note
    }
}

struct ContentView: View {
    @EnvironmentObject var store: VelClientStore
    @State private var selectedTab: VeliOSTab = .today
    @StateObject private var voiceModel = VoiceCaptureModel()
    @State private var captureSeed: CaptureDraftSeed?

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

                CaptureTab(
                    store: store,
                    voiceModel: voiceModel,
                    incomingSeed: $captureSeed
                )
                    .tag(VeliOSTab.capture)
                    .tabItem {
                        Label("Capture", systemImage: "camera")
                    }

                VoiceTab(store: store, voiceModel: voiceModel) { transcript in
                    let trimmed = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !trimmed.isEmpty else { return }
                    captureSeed = CaptureDraftSeed(transcript: trimmed)
                    selectedTab = .capture
                }
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
        case .capture:
            return "Capture"
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

    private var cachedProjects: [ProjectRecordData] {
        Array(store.offlineStore.cachedProjects().prefix(5))
    }

    private var actionItems: [ActionItemData] {
        store.offlineStore.cachedActionItems()
            .filter { $0.surface == .now }
            .sorted { $0.rank < $1.rank }
    }

    var body: some View {
        List {
            Section("Connection") {
                ConnectionSummaryRow(store: store)
            }

            Section("Top action") {
                if let action = actionItems.first {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(action.title)
                        if let projectLabel = projectLabel(for: action.project_id) {
                            Text(projectLabel)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                        Text(action.summary)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                } else {
                    Text("No backend-ranked action is cached yet.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
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

            Section("Projects") {
                if cachedProjects.isEmpty {
                    Text("No cached projects.")
                        .foregroundStyle(.secondary)
                } else {
                    ForEach(projectGroups(from: cachedProjects)) { group in
                        VStack(alignment: .leading, spacing: 6) {
                            Text(group.title)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                            ForEach(group.projects) { project in
                                VStack(alignment: .leading, spacing: 2) {
                                    Text(project.name)
                                    Text(project.primary_repo.path)
                                        .font(.caption2)
                                        .foregroundStyle(.tertiary)
                                }
                                .padding(.vertical, 2)
                            }
                        }
                    }
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

    private func projectLabel(for projectID: String?) -> String? {
        guard let projectID else { return nil }
        return store.offlineStore.cachedProjects().first(where: { $0.id == projectID })?.name
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

private struct CaptureTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel
    @Binding var incomingSeed: CaptureDraftSeed?

    @State private var noteText = ""
    @State private var selectedPhotoItem: PhotosPickerItem?
    @State private var selectedPhotoData: Data?
    @State private var selectedPhotoSummary: String?
    @State private var seededVoiceTranscript: String?
    @State private var includeVoiceContext = true
    @State private var includeContextSnapshot = true
    @State private var includeEmbeddedPhotoData = true
    @State private var embeddedPhotoData: Data?
    @State private var embeddedPhotoSummary: String?
    @State private var embeddedPhotoWarning: String?
    @State private var statusMessage: String?
    @State private var photoLoadError: String?

    private var trimmedNote: String {
        noteText.trimmingCharacters(in: .whitespacesAndNewlines)
    }

    private var availableVoiceTranscript: String? {
        let seeded = seededVoiceTranscript?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
        if !seeded.isEmpty {
            return seeded
        }
        let current = voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        if !current.isEmpty {
            return current
        }
        if let latest = voiceModel.history.first?.transcript.trimmingCharacters(in: .whitespacesAndNewlines), !latest.isEmpty {
            return latest
        }
        return nil
    }

    private var hasContextSnapshotContent: Bool {
        if store.context?.context != nil {
            return true
        }
        return !store.nudges.isEmpty || !store.commitments.isEmpty
    }

    private var hasDraftContent: Bool {
        !trimmedNote.isEmpty
            || selectedPhotoData != nil
            || (includeVoiceContext && availableVoiceTranscript != nil)
            || (includeContextSnapshot && hasContextSnapshotContent)
    }

    private var payloadPreview: String {
        buildMultimodalPayload(
            note: trimmedNote,
            voiceTranscript: includeVoiceContext ? availableVoiceTranscript : nil,
            photoData: selectedPhotoData,
            photoSummary: selectedPhotoSummary,
            embeddedPhotoData: includeEmbeddedPhotoData ? embeddedPhotoData : nil,
            includeContextSnapshot: includeContextSnapshot,
            includeBinaryData: false
        )
    }

    private var estimatedPayloadBytes: Int {
        var total = payloadPreview.lengthOfBytes(using: .utf8)
        if includeEmbeddedPhotoData, let embeddedPhotoData {
            total += estimatedBase64Length(for: embeddedPhotoData.count)
        }
        return total
    }

    var body: some View {
        List {
            Section("Draft inputs") {
                PhotosPicker(
                    selection: $selectedPhotoItem,
                    matching: .images,
                    photoLibrary: .shared()
                ) {
                    Label(
                        selectedPhotoData == nil ? "Select photo" : "Replace photo",
                        systemImage: "photo.on.rectangle.angled"
                    )
                }
                .buttonStyle(.bordered)

                if let photoLoadError, !photoLoadError.isEmpty {
                    Text(photoLoadError)
                        .font(.caption2)
                        .foregroundStyle(.red)
                }

                TextField("Add note or context", text: $noteText, axis: .vertical)
                    .textInputAutocapitalization(.sentences)

                if let transcript = availableVoiceTranscript {
                    Toggle("Include voice transcript context", isOn: $includeVoiceContext)
                    Text(transcript)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
                } else {
                    Text("No voice transcript available yet. Use Voice tab to record one.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                Toggle("Include current context snapshot", isOn: $includeContextSnapshot)
                    .disabled(!hasContextSnapshotContent)
                if includeContextSnapshot {
                    Text(contextSnapshotPreview())
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(4)
                }
            }

            Section("Photo preview") {
                if let data = selectedPhotoData {
#if canImport(UIKit)
                    if let image = UIImage(data: data) {
                        Image(uiImage: image)
                            .resizable()
                            .scaledToFit()
                            .frame(maxHeight: 220)
                            .clipShape(RoundedRectangle(cornerRadius: 10))
                    } else {
                        Text("Image preview unavailable.")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
#else
                    Text("Image preview is unavailable on this platform.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
#endif
                    if let summary = selectedPhotoSummary {
                        Text(summary)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }

                    Toggle("Embed compressed photo bytes", isOn: $includeEmbeddedPhotoData)
                        .disabled(embeddedPhotoData == nil)

                    if let embeddedPhotoSummary, !embeddedPhotoSummary.isEmpty {
                        Text(embeddedPhotoSummary)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    if let embeddedPhotoWarning, !embeddedPhotoWarning.isEmpty {
                        Text(embeddedPhotoWarning)
                            .font(.caption2)
                            .foregroundStyle(.orange)
                    }
                } else {
                    Text("No photo selected.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Payload preview") {
                Text(payloadPreview)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
                    .textSelection(.enabled)

                Text("Estimated payload size: \(ByteCountFormatter.string(fromByteCount: Int64(estimatedPayloadBytes), countStyle: .file))")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            }

            Section("Save") {
                Button("Save multimodal capture") {
                    saveCapture()
                }
                .buttonStyle(.borderedProminent)
                .disabled(!hasDraftContent)

                Button("Clear draft", role: .destructive) {
                    clearDraft()
                }
                .buttonStyle(.bordered)
                .disabled(!hasDraftContent && selectedPhotoData == nil)

                if let statusMessage, !statusMessage.isEmpty {
                    Text(statusMessage)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
        .listStyle(.insetGrouped)
        .onChange(of: selectedPhotoItem) { item in
            Task { await loadPhoto(from: item) }
        }
        .onAppear {
            applyIncomingSeedIfNeeded()
        }
        .onChange(of: incomingSeed?.id) { _ in
            applyIncomingSeedIfNeeded()
        }
    }

    @MainActor
    private func loadPhoto(from item: PhotosPickerItem?) async {
        guard let item else {
            selectedPhotoData = nil
            selectedPhotoSummary = nil
            embeddedPhotoData = nil
            embeddedPhotoSummary = nil
            embeddedPhotoWarning = nil
            includeEmbeddedPhotoData = true
            photoLoadError = nil
            return
        }

        do {
            guard let data = try await item.loadTransferable(type: Data.self) else {
                selectedPhotoData = nil
                selectedPhotoSummary = nil
                photoLoadError = "Could not load selected photo."
                return
            }
            selectedPhotoData = data
            selectedPhotoSummary = summarizePhoto(data: data)
            prepareEmbeddedPhoto(data: data)
            photoLoadError = nil
        } catch {
            selectedPhotoData = nil
            selectedPhotoSummary = nil
            embeddedPhotoData = nil
            embeddedPhotoSummary = nil
            embeddedPhotoWarning = nil
            photoLoadError = "Photo import failed: \(error.localizedDescription)"
        }
    }

    private func prepareEmbeddedPhoto(data: Data) {
#if canImport(UIKit)
        guard let image = UIImage(data: data) else {
            embeddedPhotoData = nil
            embeddedPhotoSummary = nil
            embeddedPhotoWarning = "Could not decode image for inline payload encoding."
            includeEmbeddedPhotoData = false
            return
        }

        let maxBytes = 900_000
        let qualities: [CGFloat] = [0.82, 0.70, 0.58, 0.46, 0.34]
        for quality in qualities {
            guard let jpeg = image.jpegData(compressionQuality: quality) else { continue }
            guard jpeg.count <= maxBytes else { continue }
            embeddedPhotoData = jpeg
            includeEmbeddedPhotoData = true
            embeddedPhotoSummary = "Inline JPEG payload: \(ByteCountFormatter.string(fromByteCount: Int64(jpeg.count), countStyle: .file)) (quality \(Int(quality * 100))%)."
            embeddedPhotoWarning = nil
            return
        }

        embeddedPhotoData = nil
        embeddedPhotoSummary = nil
        includeEmbeddedPhotoData = false
        embeddedPhotoWarning = "Photo is too large for inline payload. Capture will include metadata only."
#else
        embeddedPhotoData = nil
        embeddedPhotoSummary = nil
        embeddedPhotoWarning = nil
        includeEmbeddedPhotoData = false
#endif
    }

    private func summarizePhoto(data: Data) -> String {
        let bytes = ByteCountFormatter.string(fromByteCount: Int64(data.count), countStyle: .file)
#if canImport(UIKit)
        if let image = UIImage(data: data) {
            let width = Int(image.size.width * image.scale)
            let height = Int(image.size.height * image.scale)
            return "\(width)x\(height) px · \(bytes)"
        }
#endif
        return bytes
    }

    private func saveCapture() {
        let transcript = includeVoiceContext ? availableVoiceTranscript : nil
        let payload = buildMultimodalPayload(
            note: trimmedNote,
            voiceTranscript: transcript,
            photoData: selectedPhotoData,
            photoSummary: selectedPhotoSummary,
            embeddedPhotoData: includeEmbeddedPhotoData ? embeddedPhotoData : nil,
            includeContextSnapshot: includeContextSnapshot,
            includeBinaryData: true
        )

        Task {
            await store.createCapture(
                text: payload,
                type: "multimodal_note",
                source: "apple_ios_multimodal"
            )

            if store.isReachable {
                statusMessage = "Multimodal capture submitted."
            } else {
                statusMessage = "Multimodal capture queued for sync."
            }
            clearDraft(keepStatus: true)
        }
    }

    private func clearDraft(keepStatus: Bool = false) {
        noteText = ""
        selectedPhotoItem = nil
        selectedPhotoData = nil
        selectedPhotoSummary = nil
        seededVoiceTranscript = nil
        embeddedPhotoData = nil
        embeddedPhotoSummary = nil
        embeddedPhotoWarning = nil
        includeEmbeddedPhotoData = true
        photoLoadError = nil
        includeVoiceContext = true
        includeContextSnapshot = true
        if !keepStatus {
            statusMessage = nil
        }
    }

    private func buildMultimodalPayload(
        note: String,
        voiceTranscript: String?,
        photoData: Data?,
        photoSummary: String?,
        embeddedPhotoData: Data?,
        includeContextSnapshot: Bool,
        includeBinaryData: Bool
    ) -> String {
        var lines: [String] = [
            "multimodal_capture:",
            "captured_at: \(iso8601Now())",
            "client_surface: ios_capture_tab"
        ]

        if let photoData {
            lines.append("image_size_bytes: \(photoData.count)")
        }
        if let photoSummary, !photoSummary.isEmpty {
            lines.append("image_summary: \(photoSummary)")
        }
        if let embeddedPhotoData {
            lines.append("image_payload_format: jpeg_base64")
            lines.append("image_payload_bytes: \(embeddedPhotoData.count)")
            if includeBinaryData {
                lines.append("image_payload_base64: \(embeddedPhotoData.base64EncodedString())")
            } else {
                lines.append("image_payload_base64: <omitted in preview>")
            }
        }

        if includeContextSnapshot {
            appendContextSnapshotLines(to: &lines)
        }

        if let voiceTranscript, !voiceTranscript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            lines.append("")
            lines.append("voice_transcript:")
            lines.append(voiceTranscript)
        }

        if !note.isEmpty {
            lines.append("")
            lines.append("note:")
            lines.append(note)
        }

        return lines.joined(separator: "\n")
    }

    private func appendContextSnapshotLines(to lines: inout [String]) {
        lines.append("")
        lines.append("context_snapshot:")

        guard let context = store.context?.context else {
            lines.append("context_unavailable: true")
            return
        }

        if let mode = context.mode, !mode.isEmpty {
            lines.append("mode: \(mode)")
        }
        if let morning = context.morning_state, !morning.isEmpty {
            lines.append("morning_state: \(morning)")
        }
        if let meds = context.meds_status, !meds.isEmpty {
            lines.append("meds_status: \(meds)")
        }
        if let attention = context.attention_state, !attention.isEmpty {
            lines.append("attention_state: \(attention)")
        }
        if let drift = context.drift_type, !drift.isEmpty {
            lines.append("drift_type: \(drift)")
        }
        if let prep = context.prep_window_active {
            lines.append("prep_window_active: \(prep)")
        }
        if let commute = context.commute_window_active {
            lines.append("commute_window_active: \(commute)")
        }
        if let leaveBy = context.leave_by_ts {
            lines.append("leave_by: \(formatUnix(leaveBy))")
        }
        if let nextEvent = context.next_event_start_ts {
            lines.append("next_event_start: \(formatUnix(nextEvent))")
        }

        let activeNudges = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        lines.append("active_nudges_count: \(activeNudges.count)")
        if let topNudge = activeNudges.first {
            lines.append("top_nudge_message: \(topNudge.message)")
        }

        if let nextCommitment = resolveNextCommitment(preferredID: context.next_commitment_id) {
            lines.append("next_commitment_text: \(nextCommitment.text)")
            if let dueAt = nextCommitment.due_at {
                lines.append("next_commitment_due: \(formatUnix(dueAt))")
            }
        }
    }

    private func resolveNextCommitment(preferredID: String?) -> CommitmentData? {
        let open = store.commitments.filter { $0.status == "open" }
        if let preferredID, let matched = open.first(where: { $0.id == preferredID }) {
            return matched
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

    private func contextSnapshotPreview() -> String {
        guard let context = store.context?.context else {
            return "Context is not cached yet. Refresh when Vel is reachable."
        }
        var parts: [String] = []
        if let mode = context.mode, !mode.isEmpty {
            parts.append("Mode \(mode)")
        }
        if let morning = context.morning_state, !morning.isEmpty {
            parts.append("Morning \(morning)")
        }
        if let meds = context.meds_status, !meds.isEmpty {
            parts.append("Meds \(meds)")
        }
        if context.prep_window_active == true {
            parts.append("Prep active")
        }
        if context.commute_window_active == true {
            parts.append("Commute active")
        }
        if let topNudge = store.nudges.first(where: { $0.state == "active" || $0.state == "snoozed" }) {
            parts.append("Top nudge: \(topNudge.message)")
        }
        if let nextCommitment = resolveNextCommitment(preferredID: context.next_commitment_id) {
            parts.append("Next: \(nextCommitment.text)")
        }
        if parts.isEmpty {
            return "Context snapshot will include whatever fields are currently available."
        }
        return parts.joined(separator: " • ")
    }

    private func estimatedBase64Length(for byteCount: Int) -> Int {
        ((byteCount + 2) / 3) * 4
    }

    private func applyIncomingSeedIfNeeded() {
        guard let incomingSeed else { return }
        let transcript = incomingSeed.transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        let note = incomingSeed.note.trimmingCharacters(in: .whitespacesAndNewlines)

        if !transcript.isEmpty {
            seededVoiceTranscript = transcript
            includeVoiceContext = true
        }

        if !note.isEmpty {
            if trimmedNote.isEmpty {
                noteText = note
            } else {
                noteText = "\(trimmedNote)\n\(note)"
            }
        }

        statusMessage = "Draft seeded from voice transcript."
        self.incomingSeed = nil
    }

    private func iso8601Now() -> String {
        ISO8601DateFormatter().string(from: Date())
    }
}

private struct VoiceTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel
    let onOpenCaptureComposer: (String) -> Void

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

                Text("Voice submissions preserve transcript provenance, then defer schedule and behavior replies to the backend Apple route.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)

                Button("Open multimodal composer") {
                    onOpenCaptureComposer(voiceModel.transcript)
                }
                .buttonStyle(.bordered)
                .disabled(!voiceModel.hasTranscript)
            }

            Section("Quick commands") {
                ForEach(VoiceCommandExample.defaults) { example in
                    Button(example.label) {
                        voiceModel.applyCommandExample(example.command)
                    }
                    .buttonStyle(.bordered)

                    Text(example.command)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
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
                    Text("Run a voice query like “what matters right now?” or “give me a behavior summary” for a backend-owned reply.")
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

                        Button("Use In Capture Tab") {
                            onOpenCaptureComposer(entry.transcript)
                        }
                        .buttonStyle(.bordered)
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
    @State private var operatorToken = UserDefaults.standard.string(forKey: "vel_operator_token") ?? ""
    @State private var pairingReadContext = true
    @State private var pairingWriteSafeActions = false
    @State private var pairingExecuteRepoTasks = false
    @State private var selectedDiscoveredNodeID: String?
    @State private var pairingToken: PairingTokenData?
    @State private var pairingCodeInput = ""
    @State private var pairingFeedback: String?
    @State private var isIssuingPairingToken = false
    @State private var isRedeemingPairingToken = false
    @State private var unpairingNodeID: String?
    @State private var confirmUnpairNodeID: String?
    @State private var linkedPermissionDrafts: [String: LinkScopeData] = [:]

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

                Text("Resolution order: vel_tailscale_url, vel_base_url, vel_lan_base_url, localhost.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            }

            Section("Linking") {
                scopeToggle(label: "Read context", value: $pairingReadContext)
                scopeToggle(label: "Write safe actions", value: $pairingWriteSafeActions)
                scopeToggle(label: "Execute repo tasks", value: $pairingExecuteRepoTasks)

                if store.discoveredWorkers.isEmpty {
                    Text("No unlinked discovered nodes are active right now.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                } else {
                    Picker("Discovered node", selection: Binding(
                        get: {
                            selectedDiscoveredNodeID ?? store.discoveredWorkers.first?.node_id ?? ""
                        },
                        set: { selectedDiscoveredNodeID = $0 }
                    )) {
                        ForEach(store.discoveredWorkers) { worker in
                            Text(worker.node_display_name).tag(worker.node_id)
                        }
                    }

                    ForEach(store.discoveredWorkers) { worker in
                        if worker.node_id == (selectedDiscoveredNodeID ?? store.discoveredWorkers.first?.node_id) {
                            RemoteNodeSummaryCard(
                                title: worker.node_display_name,
                                subtitle: worker.sync_status ?? worker.status,
                                routes: collectRemoteRoutes(
                                    syncBaseURL: worker.sync_base_url,
                                    tailscaleBaseURL: worker.tailscale_base_url,
                                    lanBaseURL: worker.lan_base_url,
                                    publicBaseURL: nil
                                ),
                                prompt: worker.incoming_linking_prompt
                            )
                        }
                    }
                }

                Button(isIssuingPairingToken ? "Pairing…" : "Pair nodes") {
                    Task { await issuePairingToken() }
                }
                .buttonStyle(.borderedProminent)
                .disabled(isIssuingPairingToken)

                if let prompt = store.localIncomingLinkingPrompt {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Incoming prompt from \(prompt.issued_by_node_display_name ?? prompt.issued_by_node_id)")
                            .font(.subheadline.weight(.semibold))
                        Text(scopeSummary(prompt.scopes))
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        TextField(
                            "ABC-123",
                            text: Binding(
                                get: { pairingCodeInput },
                                set: { pairingCodeInput = formatPairingTokenInput($0) }
                            )
                        )
                        .textInputAutocapitalization(.characters)
                        .autocorrectionDisabled()
                        .keyboardType(.asciiCapable)

                        Button(isRedeemingPairingToken ? "Entering…" : "Enter token") {
                            Task { await redeemPairingToken(using: prompt) }
                        }
                        .buttonStyle(.bordered)
                        .disabled(isRedeemingPairingToken)
                    }
                }

                if let pairingToken {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Current token")
                            .font(.subheadline.weight(.semibold))
                        Text(pairingToken.token_code)
                            .font(.system(.body, design: .monospaced))
                        Text("Expires \(pairingToken.expires_at)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text(scopeSummary(pairingToken.scopes))
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }

                if let pairingFeedback, !pairingFeedback.isEmpty {
                    Text(pairingFeedback)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Linked devices") {
                if store.linkedNodes.isEmpty {
                    Text("No linked devices yet.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                } else {
                    ForEach(store.linkedNodes) { node in
                        VStack(alignment: .leading, spacing: 10) {
                            Text(node.node_display_name)
                                .font(.headline)
                            Text(scopeSummary(linkedPermissionDrafts[node.node_id] ?? node.scopes))
                                .font(.caption)
                                .foregroundStyle(.secondary)

                            ForEach(collectRemoteRoutes(
                                syncBaseURL: node.sync_base_url,
                                tailscaleBaseURL: node.tailscale_base_url,
                                lanBaseURL: node.lan_base_url,
                                publicBaseURL: node.public_base_url
                            ), id: \.baseURL) { route in
                                Text("\(route.label): \(route.baseURL)")
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }

                            scopeToggle(
                                label: "Read context",
                                value: Binding(
                                    get: { (linkedPermissionDrafts[node.node_id] ?? node.scopes).read_context },
                                    set: { setLinkedScope(nodeID: node.node_id, keyPath: \.read_context, value: $0, fallback: node.scopes) }
                                )
                            )
                            scopeToggle(
                                label: "Write safe actions",
                                value: Binding(
                                    get: { (linkedPermissionDrafts[node.node_id] ?? node.scopes).write_safe_actions },
                                    set: { setLinkedScope(nodeID: node.node_id, keyPath: \.write_safe_actions, value: $0, fallback: node.scopes) }
                                )
                            )
                            scopeToggle(
                                label: "Execute repo tasks",
                                value: Binding(
                                    get: { (linkedPermissionDrafts[node.node_id] ?? node.scopes).execute_repo_tasks },
                                    set: { setLinkedScope(nodeID: node.node_id, keyPath: \.execute_repo_tasks, value: $0, fallback: node.scopes) }
                                )
                            )

                            Button("Request updated access") {
                                Task { await renegotiateLinkedNode(node) }
                            }
                            .buttonStyle(.bordered)

                            if confirmUnpairNodeID == node.node_id {
                                HStack {
                                    Button(unpairingNodeID == node.node_id ? "Unpairing…" : "Confirm unpair") {
                                        Task { await unpair(node) }
                                    }
                                    .buttonStyle(.borderedProminent)
                                    .disabled(unpairingNodeID == node.node_id)

                                    Button("Cancel") {
                                        confirmUnpairNodeID = nil
                                    }
                                    .buttonStyle(.bordered)
                                }
                            } else {
                                Button("Unpair") {
                                    confirmUnpairNodeID = node.node_id
                                }
                                .buttonStyle(.bordered)
                            }
                        }
                        .padding(.vertical, 6)
                    }
                }
            }

            Section("Operator auth") {
                SecureField("x-vel-operator-token", text: $operatorToken)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()

                Button("Save auth and reconnect") {
                    let trimmed = operatorToken.trimmingCharacters(in: .whitespacesAndNewlines)
                    if trimmed.isEmpty {
                        UserDefaults.standard.removeObject(forKey: "vel_operator_token")
                    } else {
                        UserDefaults.standard.set(trimmed, forKey: "vel_operator_token")
                    }
                    store.client.configuration = .shared()
                    Task { await store.refresh() }
                }
                .buttonStyle(.bordered)

                Text("Operator-authenticated /v1 routes send x-vel-operator-token when configured.")
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
        .onAppear {
            if selectedDiscoveredNodeID == nil {
                selectedDiscoveredNodeID = store.discoveredWorkers.first?.node_id
            }
        }
        .onChange(of: store.discoveredWorkers.map(\.node_id)) { nodeIDs in
            if !nodeIDs.contains(selectedDiscoveredNodeID ?? "") {
                selectedDiscoveredNodeID = nodeIDs.first
            }
        }
    }

    private var pairingScopes: LinkScopeData {
        LinkScopeData(
            read_context: pairingReadContext,
            write_safe_actions: pairingWriteSafeActions,
            execute_repo_tasks: pairingExecuteRepoTasks
        )
    }

    private func issuePairingToken() async {
        isIssuingPairingToken = true
        defer { isIssuingPairingToken = false }

        do {
            let target = store.discoveredWorkers.first(where: { $0.node_id == selectedDiscoveredNodeID })
            pairingToken = try await store.issuePairingToken(scopes: pairingScopes, targetWorker: target)
            pairingFeedback = target == nil
                ? "Pair nodes code created."
                : "Pair nodes code created. \(target?.node_display_name ?? "Remote client") has been prompted to enter it on that client."
        } catch {
            pairingFeedback = error.localizedDescription
        }
    }

    private func redeemPairingToken(using prompt: LinkingPromptData) async {
        let normalized = formatPairingTokenInput(pairingCodeInput)
        guard !normalized.isEmpty else {
            pairingFeedback = "Enter the pairing token shown on the issuing node."
            return
        }

        isRedeemingPairingToken = true
        defer { isRedeemingPairingToken = false }

        do {
            let linked = try await store.redeemPairingToken(
                tokenCode: normalized,
                requestedScopes: prompt.scopes
            )
            pairingCodeInput = ""
            pairingToken = nil
            pairingFeedback = "Linked as \(linked.node_display_name). The link has been saved locally and the issuing client has been notified."
        } catch {
            pairingFeedback = error.localizedDescription
        }
    }

    private func renegotiateLinkedNode(_ node: LinkedNodeData) async {
        isIssuingPairingToken = true
        defer { isIssuingPairingToken = false }

        do {
            let token = try await store.issuePairingToken(
                scopes: linkedPermissionDrafts[node.node_id] ?? node.scopes,
                targetWorker: store.clusterWorkers?.workers.first(where: { $0.node_id == node.node_id })
            )
            pairingToken = token
            pairingFeedback = "Pair nodes code created for \(node.node_display_name). That client has been prompted to approve the new access."
        } catch {
            pairingFeedback = error.localizedDescription
        }
    }

    private func unpair(_ node: LinkedNodeData) async {
        unpairingNodeID = node.node_id
        defer {
            unpairingNodeID = nil
            confirmUnpairNodeID = nil
        }

        do {
            try await store.revokeLinkedNode(nodeID: node.node_id)
            pairingFeedback = "Unpaired \(node.node_display_name)."
        } catch {
            pairingFeedback = error.localizedDescription
        }
    }

    private func setLinkedScope(
        nodeID: String,
        keyPath: WritableKeyPath<LinkScopeData, Bool>,
        value: Bool,
        fallback: LinkScopeData
    ) {
        var draft = linkedPermissionDrafts[nodeID] ?? fallback
        draft[keyPath: keyPath] = value
        linkedPermissionDrafts[nodeID] = draft
    }

    private func scopeToggle(label: String, value: Binding<Bool>) -> some View {
        Toggle(isOn: value) {
            Text(label)
        }
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

private struct RouteSummary: Identifiable, Equatable {
    let label: String
    let baseURL: String

    var id: String { "\(label):\(baseURL)" }
}

private struct RemoteNodeSummaryCard: View {
    let title: String
    let subtitle: String
    let routes: [RouteSummary]
    let prompt: LinkingPromptData?

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(title)
                .font(.subheadline.weight(.semibold))
            Text(subtitle)
                .font(.caption)
                .foregroundStyle(.secondary)
            ForEach(routes) { route in
                Text("\(route.label): \(route.baseURL)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            if let prompt {
                Text("Incoming prompt from \(prompt.issued_by_node_display_name ?? prompt.issued_by_node_id)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }
}

private func collectRemoteRoutes(
    syncBaseURL: String?,
    tailscaleBaseURL: String?,
    lanBaseURL: String?,
    publicBaseURL: String?
) -> [RouteSummary] {
    let entries: [(String, String?)] = [
        ("primary", syncBaseURL),
        ("tailscale", tailscaleBaseURL),
        ("lan", lanBaseURL),
        ("public", publicBaseURL),
    ]
    var seen = Set<String>()
    var routes: [RouteSummary] = []
    for (label, value) in entries {
        let trimmed = value?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
        if trimmed.isEmpty || trimmed.contains("127.0.0.1") || trimmed.contains("localhost") || seen.contains(trimmed) {
            continue
        }
        seen.insert(trimmed)
        routes.append(RouteSummary(label: label, baseURL: trimmed))
    }
    return routes
}

private func scopeSummary(_ scopes: LinkScopeData) -> String {
    var labels: [String] = []
    if scopes.read_context { labels.append("read_context") }
    if scopes.write_safe_actions { labels.append("write_safe_actions") }
    if scopes.execute_repo_tasks { labels.append("execute_repo_tasks") }
    return labels.isEmpty ? "No scopes selected" : labels.joined(separator: ", ")
}

private func formatPairingTokenInput(_ value: String) -> String {
    let normalized = value.uppercased().filter { character in
        character.isASCII && (character.isLetter || character.isNumber)
    }.prefix(6)
    let text = String(normalized)
    if text.count <= 3 { return text }
    let splitIndex = text.index(text.startIndex, offsetBy: 3)
    return "\(text[..<splitIndex])-\(text[splitIndex...])"
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
        case morningBriefing = "morning_briefing"
        case currentSchedule = "current_schedule"
        case queryNextCommitment = "query_next_commitment"
        case queryNudges = "query_nudges"
        case explainWhy = "explain_why"
        case behaviorSummary = "behavior_summary"
    }

    let kind: Kind
    let minutes: Int?

    static let capture = VoiceIntent(kind: .captureCreate, minutes: nil)
    static let commitment = VoiceIntent(kind: .commitmentCreate, minutes: nil)
    static let commitmentDone = VoiceIntent(kind: .commitmentDone, minutes: nil)
    static let nudgeDone = VoiceIntent(kind: .nudgeDone, minutes: nil)
    static let morningBriefing = VoiceIntent(kind: .morningBriefing, minutes: nil)
    static let currentSchedule = VoiceIntent(kind: .currentSchedule, minutes: nil)
    static let queryNextCommitment = VoiceIntent(kind: .queryNextCommitment, minutes: nil)
    static let queryNudges = VoiceIntent(kind: .queryNudges, minutes: nil)
    static let explainWhy = VoiceIntent(kind: .explainWhy, minutes: nil)
    static let behaviorSummary = VoiceIntent(kind: .behaviorSummary, minutes: nil)
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
        case .morningBriefing:
            return "Morning briefing"
        case .currentSchedule:
            return "Current schedule"
        case .queryNextCommitment:
            return "Query next commitment"
        case .queryNudges:
            return "Active nudges"
        case .explainWhy:
            return "Explain why now"
        case .behaviorSummary:
            return "Behavior summary"
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
        case .morningBriefing:
            return "morning_briefing"
        case .currentSchedule:
            return "current_schedule"
        case .queryNextCommitment:
            return "query_next_commitment"
        case .queryNudges:
            return "query_nudges"
        case .explainWhy:
            return "explain_why"
        case .behaviorSummary:
            return "behavior_summary"
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
        case .morningBriefing, .currentSchedule, .queryNextCommitment, .queryNudges, .explainWhy, .behaviorSummary:
            return true
        case .captureCreate, .commitmentCreate, .commitmentDone, .nudgeDone, .nudgeSnooze:
            return false
        }
    }

    var appleIntent: AppleVoiceIntentData? {
        switch kind {
        case .captureCreate:
            return .capture
        case .commitmentCreate:
            return nil
        case .commitmentDone:
            return .completeCommitment
        case .nudgeDone:
            return .activeNudges
        case .nudgeSnooze:
            return .snoozeNudge
        case .morningBriefing:
            return .morningBriefing
        case .currentSchedule:
            return .currentSchedule
        case .queryNextCommitment:
            return .nextCommitment
        case .queryNudges:
            return .activeNudges
        case .explainWhy:
            return .explainWhy
        case .behaviorSummary:
            return .behaviorSummary
        }
    }

    var appleOperation: AppleRequestedOperationData? {
        switch kind {
        case .captureCreate:
            return .captureOnly
        case .commitmentCreate:
            return nil
        case .commitmentDone, .nudgeDone, .nudgeSnooze:
            return .mutation
        case .morningBriefing, .currentSchedule, .queryNextCommitment, .queryNudges, .explainWhy, .behaviorSummary:
            return .queryOnly
        }
    }

    var usesBackendVoiceTurn: Bool {
        appleIntent != nil && appleOperation != nil
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

        if isMorningBriefingQuery(normalized) {
            return VoiceIntentSuggestion(intent: .morningBriefing, cleanedText: clean)
        }

        if isContextQuery(normalized) {
            return VoiceIntentSuggestion(intent: .currentSchedule, cleanedText: clean)
        }

        if isNextCommitmentQuery(normalized) {
            return VoiceIntentSuggestion(intent: .queryNextCommitment, cleanedText: clean)
        }

        if isNudgesQuery(normalized) {
            return VoiceIntentSuggestion(intent: .queryNudges, cleanedText: clean)
        }

        if isExplainQuery(normalized) {
            return VoiceIntentSuggestion(intent: .explainWhy, cleanedText: clean)
        }

        if isBehaviorSummaryQuery(normalized) {
            return VoiceIntentSuggestion(intent: .behaviorSummary, cleanedText: clean)
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

    private static func isMorningBriefingQuery(_ text: String) -> Bool {
        [
            "good morning",
            "morning briefing",
            "start my day",
            "morning plan",
            "morning check"
        ].contains(where: { text.contains($0) })
    }

    private static func isContextQuery(_ text: String) -> Bool {
        [
            "what matters",
            "what do i need",
            "what should i do right now",
            "current context",
            "status right now"
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

    private static func isBehaviorSummaryQuery(_ text: String) -> Bool {
        [
            "behavior summary",
            "behaviour summary",
            "activity summary",
            "health summary",
            "how am i moving"
        ].contains(where: { text.contains($0) })
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

private struct VoiceCommandExample: Identifiable {
    let id: String
    let label: String
    let command: String

    static let defaults: [VoiceCommandExample] = [
        VoiceCommandExample(id: "morning", label: "Morning brief", command: "Good morning"),
        VoiceCommandExample(id: "context", label: "What matters", command: "What matters right now?"),
        VoiceCommandExample(id: "next", label: "What is next", command: "What's my next commitment?"),
        VoiceCommandExample(id: "nudges", label: "Active nudges", command: "What are my active nudges?"),
        VoiceCommandExample(id: "behavior", label: "Behavior summary", command: "Give me a behavior summary"),
        VoiceCommandExample(id: "done", label: "Mark meds done", command: "Mark meds done"),
        VoiceCommandExample(id: "snooze", label: "Snooze 10", command: "Snooze that 10 minutes")
    ]
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

    func applyCommandExample(_ value: String) {
        updateTranscript(value)
        latestResponse = nil
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

        if intent.usesBackendVoiceTurn {
            let result = await submitBackendVoiceTurn(
                using: store,
                transcript: text,
                primaryText: primaryText,
                intent: intent
            )
            appendHistoryEntry(
                transcript: text,
                suggestedIntent: suggestedIntent,
                committedIntent: result.committedIntent,
                status: result.historyStatus
            )
            errorMessage = result.errorMessage
            return
        }

        let result = await submitViaQueuedShell(
            using: store,
            transcript: text,
            primaryText: primaryText,
            intent: intent
        )
        appendHistoryEntry(
            transcript: text,
            suggestedIntent: suggestedIntent,
            committedIntent: result.committedIntent,
            status: result.historyStatus
        )
        errorMessage = result.errorMessage
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

    private struct VoiceSubmitResult {
        let committedIntent: VoiceIntent?
        let historyStatus: String
        let errorMessage: String?
    }

    private func submitBackendVoiceTurn(
        using store: VelClientStore,
        transcript: String,
        primaryText: String,
        intent: VoiceIntent
    ) async -> VoiceSubmitResult {
        guard let appleIntent = intent.appleIntent, let operation = intent.appleOperation else {
            return VoiceSubmitResult(committedIntent: nil, historyStatus: "unsupported", errorMessage: "This voice action is not supported by the Apple backend route.")
        }

        if store.isReachable {
            do {
                let response = try await store.client.appleVoiceTurn(
                    AppleVoiceTurnRequestData(
                        transcript: transcript,
                        surface: .iosVoice,
                        operation: operation,
                        intents: [appleIntent],
                        provenance: appleProvenance(isOfflineFallback: false)
                    )
                )
                await refreshBackendCaches(using: store)
                await store.refresh()
                setResponse(from: response, offlineStore: store.offlineStore)
                return VoiceSubmitResult(
                    committedIntent: intent,
                    historyStatus: response.queued_mutation?.queued == true ? "queued" : historyStatus(for: intent, isReachable: true),
                    errorMessage: nil
                )
            } catch {
                let fallback = await submitOfflineVoiceFallback(
                    using: store,
                    transcript: transcript,
                    primaryText: primaryText,
                    intent: intent,
                    underlyingError: error
                )
                return fallback
            }
        }

        return await submitOfflineVoiceFallback(
            using: store,
            transcript: transcript,
            primaryText: primaryText,
            intent: intent,
            underlyingError: nil
        )
    }

    private func submitViaQueuedShell(
        using store: VelClientStore,
        transcript: String,
        primaryText: String,
        intent: VoiceIntent
    ) async -> VoiceSubmitResult {
        let capturePayload = voiceCapturePayload(transcript: transcript, intent: intent)
        await store.createCapture(
            text: capturePayload,
            type: "voice_note",
            source: "apple_ios_voice"
        )

        switch intent.kind {
        case .captureCreate:
            setResponse(
                summary: store.isReachable ? "Saved voice capture." : "Voice capture queued for sync.",
                detail: primaryText
            )
            return VoiceSubmitResult(
                committedIntent: .capture,
                historyStatus: historyStatus(for: intent, isReachable: store.isReachable),
                errorMessage: store.isReachable ? nil : "Voice transcript queued for sync."
            )
        case .commitmentCreate:
            await store.createCommitment(text: primaryText)
            setResponse(
                summary: store.isReachable ? "Created commitment." : "Commitment queued for sync.",
                detail: primaryText
            )
            return VoiceSubmitResult(
                committedIntent: .commitment,
                historyStatus: historyStatus(for: intent, isReachable: store.isReachable),
                errorMessage: store.isReachable ? nil : "Commitment request queued for sync."
            )
        case .commitmentDone, .nudgeDone, .nudgeSnooze, .morningBriefing, .currentSchedule, .queryNextCommitment, .queryNudges, .explainWhy, .behaviorSummary:
            setResponse(
                summary: "This voice action now requires the backend Apple route.",
                detail: "Reconnect to Vel so the server can interpret and answer it."
            )
            return VoiceSubmitResult(
                committedIntent: nil,
                historyStatus: "backend_required",
                errorMessage: "Transcript capture was preserved, but the action needs the backend-owned Apple route."
            )
        }
    }

    private func submitOfflineVoiceFallback(
        using store: VelClientStore,
        transcript: String,
        primaryText: String,
        intent: VoiceIntent,
        underlyingError: Error?
    ) async -> VoiceSubmitResult {
        let capturePayload = voiceCapturePayload(transcript: transcript, intent: intent)
        await store.createCapture(
            text: capturePayload,
            type: "voice_note",
            source: "apple_ios_voice"
        )

        switch intent.kind {
        case .captureCreate:
            setResponse(summary: "Voice capture queued for sync.", detail: primaryText)
            return VoiceSubmitResult(
                committedIntent: .capture,
                historyStatus: "queued",
                errorMessage: fallbackErrorMessage(prefix: "Transcript capture queued for sync.", underlyingError: underlyingError)
            )
        case .commitmentDone:
            let target = primaryText.trimmingCharacters(in: .whitespacesAndNewlines)
            guard !target.isEmpty else {
                setResponse(summary: "Commitment target is missing.", detail: "Try phrasing like “mark meds done.”")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "needs_clarification",
                    errorMessage: fallbackErrorMessage(prefix: "Commitment target missing.", underlyingError: underlyingError)
                )
            }
            let matches = rankedCommitmentMatches(
                for: target,
                in: store.commitments.filter { $0.status == "open" }
            )
            guard let best = matches.first?.commitment else {
                setResponse(summary: "No open commitment matched.", detail: "Transcript capture was queued for sync.")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "capture_only",
                    errorMessage: fallbackErrorMessage(prefix: "No local commitment match for offline queueing.", underlyingError: underlyingError)
                )
            }
            if isAmbiguousTopMatch(matches) {
                let options = matches.prefix(3).map { $0.commitment.text }.joined(separator: " | ")
                setResponse(summary: "Ambiguous commitment target.", detail: "Could match: \(options)")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "needs_clarification",
                    errorMessage: fallbackErrorMessage(prefix: "Commitment target was ambiguous.", underlyingError: underlyingError)
                )
            }
            store.offlineStore.enqueueCommitmentDone(id: best.id)
            await store.refresh()
            setResponse(summary: "Commitment completion queued.", detail: best.text)
            return VoiceSubmitResult(
                committedIntent: .commitmentDone,
                historyStatus: "queued",
                errorMessage: fallbackErrorMessage(prefix: "Commitment completion queued for backend replay.", underlyingError: underlyingError)
            )
        case .nudgeDone:
            guard let nudgeID = firstActionableNudgeID(from: store.nudges) else {
                setResponse(summary: "No active nudge found.", detail: "Transcript capture was queued for sync.")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "capture_only",
                    errorMessage: fallbackErrorMessage(prefix: "No active nudge available for offline queueing.", underlyingError: underlyingError)
                )
            }
            store.offlineStore.enqueueNudgeDone(id: nudgeID)
            await store.refresh()
            setResponse(summary: "Top nudge resolution queued.", detail: nil)
            return VoiceSubmitResult(
                committedIntent: .nudgeDone,
                historyStatus: "queued",
                errorMessage: fallbackErrorMessage(prefix: "Top nudge resolution queued for backend replay.", underlyingError: underlyingError)
            )
        case .nudgeSnooze:
            guard let nudgeID = firstActionableNudgeID(from: store.nudges) else {
                setResponse(summary: "No active nudge found.", detail: "Transcript capture was queued for sync.")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "capture_only",
                    errorMessage: fallbackErrorMessage(prefix: "No active nudge available for offline queueing.", underlyingError: underlyingError)
                )
            }
            let minutes = intent.minutes ?? 10
            store.offlineStore.enqueueNudgeSnooze(id: nudgeID, minutes: minutes)
            await store.refresh()
            setResponse(summary: "Top nudge snooze queued.", detail: "\(minutes) minutes")
            return VoiceSubmitResult(
                committedIntent: .nudgeSnooze(minutes),
                historyStatus: "queued",
                errorMessage: fallbackErrorMessage(prefix: "Top nudge snooze queued for backend replay.", underlyingError: underlyingError)
            )
        case .morningBriefing, .currentSchedule, .queryNextCommitment:
            let cached = offlineCachedScheduleResponse(for: intent, offlineStore: store.offlineStore)
            setResponse(cached)
            return VoiceSubmitResult(
                committedIntent: intent,
                historyStatus: cached.summary.contains("Unavailable") ? "backend_required" : "answered_cached",
                errorMessage: fallbackErrorMessage(prefix: "Showing cached backend schedule state only.", underlyingError: underlyingError)
            )
        case .behaviorSummary:
            let cached = offlineCachedBehaviorResponse(offlineStore: store.offlineStore)
            setResponse(cached)
            return VoiceSubmitResult(
                committedIntent: intent,
                historyStatus: cached.summary.contains("Unavailable") ? "backend_required" : "answered_cached",
                errorMessage: fallbackErrorMessage(prefix: "Showing cached backend behavior summary only.", underlyingError: underlyingError)
            )
        case .queryNudges, .explainWhy:
            setResponse(
                summary: "Unavailable offline.",
                detail: "This reply is backend-owned and is not synthesized from local Swift cache."
            )
            return VoiceSubmitResult(
                committedIntent: nil,
                historyStatus: "backend_required",
                errorMessage: fallbackErrorMessage(prefix: "Transcript capture queued, but this voice reply requires the backend route.", underlyingError: underlyingError)
            )
        case .commitmentCreate:
            return await submitViaQueuedShell(
                using: store,
                transcript: transcript,
                primaryText: primaryText,
                intent: intent
            )
        }
    }

    private func refreshBackendCaches(using store: VelClientStore) async {
        if let now = try? await store.client.now() {
            store.offlineStore.saveCachedNow(now)
        }
        if let behavior = try? await store.client.appleBehaviorSummary() {
            store.offlineStore.saveCachedAppleBehaviorSummary(behavior)
        }
    }

    private func setResponse(from response: AppleVoiceTurnResponseData, offlineStore: VelOfflineStore) {
        if let behaviorSummary = response.behavior_summary {
            offlineStore.saveCachedAppleBehaviorSummary(behaviorSummary)
        }

        let detailParts = response.queued_mutation.map(\.summary).flatMap { [$0] } ?? []
        let reasonParts = Array(response.reasons.prefix(2))
        let evidenceParts = response.evidence.prefix(2).map { "\($0.label): \($0.detail)" }
        let detail = (detailParts + reasonParts + evidenceParts)
            .filter { !$0.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty }
            .joined(separator: " ")
        setResponse(summary: response.summary, detail: detail.isEmpty ? nil : detail)
    }

    private func offlineCachedScheduleResponse(
        for intent: VoiceIntent,
        offlineStore: VelOfflineStore
    ) -> VoiceResponse {
        guard let now = offlineStore.cachedNow() else {
            return VoiceResponse(
                summary: "Unavailable offline.",
                detail: "No cached backend /v1/now payload is available yet."
            )
        }

        switch intent.kind {
        case .morningBriefing, .currentSchedule:
            if let next = now.schedule.next_event {
                let detail = next.leave_by_ts.map { "Leave by \(formatUnix($0))." }
                return VoiceResponse(
                    summary: "Next event: \(next.title).",
                    detail: detail ?? now.schedule.empty_message
                )
            }
            return VoiceResponse(
                summary: now.schedule.empty_message ?? "No upcoming schedule is cached.",
                detail: now.reasons.first
            )
        case .queryNextCommitment:
            if let next = now.tasks.next_commitment {
                return VoiceResponse(
                    summary: "Next commitment: \(next.text).",
                    detail: next.due_at
                )
            }
            return VoiceResponse(
                summary: "No next commitment is cached.",
                detail: now.schedule.empty_message
            )
        default:
            return VoiceResponse(
                summary: "Unavailable offline.",
                detail: "Reconnect to fetch a backend-owned reply."
            )
        }
    }

    private func offlineCachedBehaviorResponse(offlineStore: VelOfflineStore) -> VoiceResponse {
        guard let summary = offlineStore.cachedAppleBehaviorSummary() else {
            return VoiceResponse(
                summary: "Unavailable offline.",
                detail: "No cached backend behavior summary is available yet."
            )
        }
        return VoiceResponse(
            summary: summary.headline,
            detail: summary.reasons.first
        )
    }

    private func fallbackErrorMessage(prefix: String, underlyingError: Error?) -> String {
        if let underlyingError {
            return "\(prefix) \(underlyingError.localizedDescription)"
        }
        return prefix
    }

    private func appleProvenance(isOfflineFallback: Bool) -> AppleTurnProvenanceData {
        let timestamp = ISO8601DateFormatter().string(from: Date())
        return AppleTurnProvenanceData(
            source_device: "apple_ios",
            locale: Locale.current.identifier,
            transcript_origin: "speech_recognition",
            recorded_at: timestamp,
            offline_captured_at: isOfflineFallback ? timestamp : nil,
            queued_at: isOfflineFallback ? timestamp : nil
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

private struct ProjectGroupSection: Identifiable {
    let id: String
    let title: String
    let projects: [ProjectRecordData]
}

private func projectGroups(from projects: [ProjectRecordData]) -> [ProjectGroupSection] {
    let groups: [(ProjectFamilyData, String)] = [
        (.personal, "Personal"),
        (.creative, "Creative"),
        (.work, "Work")
    ]

    return groups.compactMap { family, title in
        let matching = projects.filter { $0.family == family }
        guard !matching.isEmpty else { return nil }
        return ProjectGroupSection(id: title, title: title, projects: matching)
    }
}

#Preview {
    ContentView()
        .environmentObject(VelClientStore())
}
