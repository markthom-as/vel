import AVFoundation
import PhotosUI
import Speech
import SwiftUI
import VelApplePlatform
import VelApplication
import VelAPI
import VelDomain
import VelFeatureFlags
#if canImport(UIKit)
import UIKit
#endif

private enum VeliOSTab: String, Hashable, CaseIterable, Identifiable {
    case now
    case inbox
    case threads
    case chat
    case projects
    case settings

    var id: String { rawValue }

    var title: String {
        switch self {
        case .now:
            return "Now"
        case .inbox:
            return "Inbox"
        case .threads:
            return "Threads"
        case .chat:
            return "Chat"
        case .projects:
            return "Projects"
        case .settings:
            return "Settings"
        }
    }

    var systemImage: String {
        switch self {
        case .now:
            return "sun.max.fill"
        case .inbox:
            return "tray.full.fill"
        case .threads:
            return "bubble.left.and.bubble.right.fill"
        case .chat:
            return "message"
        case .projects:
            return "square.grid.2x2.fill"
        case .settings:
            return "gearshape.fill"
        }
    }
}

private enum VeliPadSection: String, CaseIterable, Hashable, Identifiable {
    case now
    case inbox
    case threads
    case chat
    case projects
    case quickEntry
    case settings

    var id: String { rawValue }

    var title: String {
        switch self {
        case .now:
            return "Now"
        case .inbox:
            return "Inbox"
        case .threads:
            return "Threads"
        case .chat:
            return "Chat"
        case .projects:
            return "Projects"
        case .quickEntry:
            return "Quick entry"
        case .settings:
            return "Settings"
        }
    }

    var systemImage: String {
        switch self {
        case .now:
            return "sun.max"
        case .inbox:
            return "calendar"
        case .threads:
            return "bubble.left.and.bubble.right"
        case .chat:
            return "message"
        case .projects:
            return "square.grid.2x2"
        case .quickEntry:
            return "plus.circle"
        case .settings:
            return "gearshape"
        }
    }
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

private enum QuickEntrySurface: String, Identifiable {
    case capture
    case voice

    var id: String { rawValue }
}

private enum SettingsLaunchSection: String, Hashable, Identifiable {
    case overview
    case linking

    var id: String { rawValue }

    var sectionAnchor: String {
        "settings-\(rawValue)"
    }
}

struct ContentView: View {
    let appEnvironment: VelAppEnvironment
    @EnvironmentObject var store: VelClientStore
    @State private var selectedTab: VeliOSTab = .now
    @State private var selectedPadSection: VeliPadSection = .now
    @StateObject private var voiceModel: VoiceCaptureModel
    @State private var captureSeed: CaptureDraftSeed?
    @State private var quickEntrySurface: QuickEntrySurface?
    @State private var settingsLaunchSection: SettingsLaunchSection = .overview

    init(appEnvironment: VelAppEnvironment, voiceOfflineStore: VelOfflineStore = VelOfflineStore()) {
        self.appEnvironment = appEnvironment
        _voiceModel = StateObject(wrappedValue: VoiceCaptureModel(offlineStore: voiceOfflineStore))
    }

    private var capabilities: FeatureCapabilities {
        appEnvironment.featureCapabilities
    }

    @ViewBuilder
    var body: some View {
        if capabilities.supportsSplitViewWorkspace {
            iPadShell
        } else {
            iPhoneShell
        }
    }

    private var iPhoneShell: some View {
        attachQuickEntrySheet {
            VStack(spacing: 0) {
                iPhoneMVPHeader
                Divider()
                iPhoneShellContent
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            }
            .task {
                await store.refresh()
                voiceModel.reconcileRecoveryState(using: store)
                await voiceModel.ensurePermissionsKnown()
            }
            .onChange(of: store.isReachable) { _ in
                voiceModel.reconcileRecoveryState(using: store)
            }
            .onChange(of: store.pendingActionCount) { _ in
                voiceModel.reconcileRecoveryState(using: store)
            }
            .onChange(of: selectedTab) { tab in
                if tab == .threads {
                    Task { await store.refreshSignals() }
                }
            }
            .safeAreaInset(edge: .bottom) {
                iPhoneBottomRail
            }
        }
    }

    @ViewBuilder
    private var iPhoneShellContent: some View {
        switch selectedTab {
        case .now:
            iPhoneTab {
                TodayTab(
                    store: store,
                    voiceModel: voiceModel,
                    onOpenCapture: { quickEntrySurface = .capture },
                    onOpenVoice: { quickEntrySurface = .voice },
                    onOpenChat: { selectedTab = .chat }
                )
            }
        case .inbox:
            iPhoneTab {
                NudgesTab(store: store)
            }
        case .threads:
            iPhoneTab {
                ActivityTab(store: store, voiceModel: voiceModel)
            }
        case .chat:
            iPhoneTab {
                ChatTab(
                    store: store,
                    voiceModel: voiceModel,
                    onOpenCapture: { quickEntrySurface = .capture },
                    onOpenVoice: { quickEntrySurface = .voice },
                    onOpenThreads: { selectedTab = .threads },
                    onOpenSettings: {
                        settingsLaunchSection = .overview
                        selectedTab = .settings
                    },
                    onOpenLinking: {
                        settingsLaunchSection = .linking
                        selectedTab = .settings
                    }
                )
            }
        case .projects:
            iPhoneTab {
                ProjectsTab(store: store)
            }
        case .settings:
            iPhoneTab {
                SettingsTab(store: store, appEnvironment: appEnvironment, initialSection: settingsLaunchSection)
            }
        }
    }

    private var iPhoneBottomRail: some View {
        HStack(spacing: 6) {
            ForEach(VeliOSTab.allCases) { tab in
                Button {
                    if tab == .settings {
                        settingsLaunchSection = .overview
                    }
                    selectedTab = tab
                } label: {
                    VStack(spacing: 3) {
                        Image(systemName: tab.systemImage)
                            .font(.caption2)
                        Text(tab.title)
                            .font(.caption2)
                            .lineLimit(1)
                    }
                    .frame(maxWidth: .infinity)
                    .padding(.vertical, 8)
                    .foregroundStyle(selectedTab == tab ? Color.orange : .secondary)
                    .background(selectedTab == tab ? Color.orange.opacity(0.2) : .clear)
                    .clipShape(.capsule)
                }
                .buttonStyle(.plain)
            }
        }
        .padding(.horizontal, 8)
        .padding(.vertical, 8)
        .background(.thinMaterial)
        .overlay(alignment: .top) {
            Divider()
        }
    }

    private var iPhoneMVPHeader: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Vel")
                        .font(.system(.title2, design: .rounded, weight: .bold))
                    Text("MVP · \(capabilities.roleLabel)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                Spacer()
                if store.isSyncing {
                    ProgressView()
                        .progressViewStyle(.circular)
                        .tint(.orange)
                } else {
                    Button {
                        Task { await store.refresh() }
                    } label: {
                        Image(systemName: "arrow.clockwise")
                    }
                }
            }
            Text(Date.now.formatted(.dateTime.weekday(.wide).month(.abbreviated).day().hour().minute()))
                .font(.caption)
                .foregroundStyle(.secondary)
        }
        .padding(.horizontal, 14)
        .padding(.top, 10)
        .padding(.bottom, 10)
        .background(Color(uiColor: .systemGray6))
    }

    @ViewBuilder
    private func iPhoneTab<Content: View>(@ViewBuilder content: () -> Content) -> some View {
        content()
    }

    @ToolbarContentBuilder
    private var refreshToolbar: some ToolbarContent {
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

    private var iPadShell: some View {
        attachQuickEntrySheet {
            NavigationSplitView {
                List {
            ForEach(VeliPadSection.allCases) { section in
                Button {
                    if section == .settings {
                        settingsLaunchSection = .overview
                    }
                    selectedPadSection = section
                } label: {
                            HStack {
                                Label(section.title, systemImage: section.systemImage)
                                Spacer()
                                if selectedPadSection == section {
                                    Image(systemName: "checkmark")
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                        .buttonStyle(.plain)
                    }
                }
                .navigationTitle("Vel \(capabilities.roleLabel)")
                .velCompactListStyle()
            } detail: {
                NavigationStack {
                    iPadDetail(for: selectedPadSection)
                        .navigationTitle(selectedPadSection.title)
                        .navigationBarTitleDisplayMode(.inline)
                        .toolbar { refreshToolbar }
                }
            }
            .navigationSplitViewStyle(.balanced)
            .task {
                await store.refresh()
                voiceModel.reconcileRecoveryState(using: store)
                await voiceModel.ensurePermissionsKnown()
            }
            .onChange(of: store.isReachable) { _ in
                voiceModel.reconcileRecoveryState(using: store)
            }
            .onChange(of: store.pendingActionCount) { _ in
                voiceModel.reconcileRecoveryState(using: store)
            }
            .onChange(of: selectedPadSection) { section in
                if section == .threads {
                    Task { await store.refreshSignals() }
                }
            }
        }
    }

    @ViewBuilder
    private func attachQuickEntrySheet<Content: View>(
        @ViewBuilder content: () -> Content
    ) -> some View {
        content().sheet(item: $quickEntrySurface) { surface in
            quickEntrySheet(for: surface)
        }
    }

    @ViewBuilder
    private func quickEntrySheet(for surface: QuickEntrySurface) -> some View {
        NavigationStack {
            switch surface {
            case .capture:
                CaptureTab(
                    store: store,
                    voiceModel: voiceModel,
                    incomingSeed: $captureSeed
                )
                .navigationTitle("Capture")
                .navigationBarTitleDisplayMode(.inline)
            case .voice:
                VoiceTab(store: store, voiceModel: voiceModel) { transcript in
                    let trimmed = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !trimmed.isEmpty else { return }
                    captureSeed = CaptureDraftSeed(transcript: trimmed)
                    quickEntrySurface = .capture
                }
                .navigationTitle("Voice")
                .navigationBarTitleDisplayMode(.inline)
            }
        }
    }

    @ViewBuilder
    private func iPadDetail(for section: VeliPadSection) -> some View {
        switch section {
        case .now:
            TodayTab(
                store: store,
                voiceModel: voiceModel,
                onOpenCapture: { quickEntrySurface = .capture },
                onOpenVoice: { quickEntrySurface = .voice },
                onOpenChat: { selectedPadSection = .chat }
            )
        case .inbox:
            NudgesTab(store: store)
        case .threads:
            ActivityTab(store: store, voiceModel: voiceModel)
        case .chat:
                ChatTab(
                    store: store,
                    voiceModel: voiceModel,
                    onOpenCapture: { quickEntrySurface = .capture },
                    onOpenVoice: { quickEntrySurface = .voice },
                    onOpenThreads: { selectedPadSection = .threads },
                    onOpenSettings: {
                        settingsLaunchSection = .overview
                        selectedPadSection = .settings
                    },
                    onOpenLinking: {
                        settingsLaunchSection = .linking
                        selectedPadSection = .settings
                    }
                )
        case .projects:
            ProjectsTab(store: store)
        case .quickEntry:
            CaptureTab(
                store: store,
                voiceModel: voiceModel,
                incomingSeed: $captureSeed
            )
        case .settings:
            SettingsTab(store: store, appEnvironment: appEnvironment, initialSection: settingsLaunchSection)
        }
    }

}

private struct ProjectsTab: View {
    @ObservedObject var store: VelClientStore

    var body: some View {
        List {
            Section("Project context") {
                Text("Projects stay secondary on Apple. Use them for durable roots and project-specific context after `Now`, `Inbox`, or `Threads` point you here.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                let projects = store.offlineStore.cachedProjects()
                if projects.isEmpty {
                    Text("No cached projects yet.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                } else {
                    ForEach(Array(projects.prefix(25)), id: \.id) { project in
                        VStack(alignment: .leading, spacing: 4) {
                            Text(project.name)
                                .font(.body)
                            Text(project.primary_repo.path)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                        .padding(.vertical, 2)
                    }
                }
            }

            Section("Project-owned detail") {
                Text("Project drill-down is available here without turning Apple into a second daily-use dashboard.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .velCompactListStyle()
        .refreshable { await store.refresh() }
    }
}

private struct TodayTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel
    let onOpenCapture: () -> Void
    let onOpenVoice: () -> Void
    let onOpenChat: () -> Void
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

    private var cachedNow: NowData? {
        store.offlineStore.cachedNow()
    }

    private var visibleNudgeBars: [NowNudgeBarData] {
        Array((cachedNow?.nudge_bars ?? []).prefix(4))
    }

    private var taskLane: NowTaskLaneData? {
        cachedNow?.task_lane
    }

    var body: some View {
        ScrollView {
            LazyVStack(alignment: .leading, spacing: 12) {
                todayHeader

                if let statusRow = cachedNow?.status_row {
                    compactNowStatusRow(statusRow)
                }

                if let contextLine = cachedNow?.context_line {
                    SurfaceSectionCard("Context") {
                        Text(contextLine.text)
                            .font(.caption)
                            .foregroundStyle(contextLine.fallback_used ? .secondary : .primary)
                            .frame(maxWidth: .infinity, alignment: .leading)
                    }
                }

                if let meshSummary = cachedNow?.mesh_summary {
                    compactNowMeshSummary(meshSummary)
                }

                if !visibleNudgeBars.isEmpty {
                    SurfaceSectionCard("Nudges") {
                        VStack(alignment: .leading, spacing: 8) {
                            ForEach(visibleNudgeBars) { bar in
                                compactNowBar(bar)
                            }
                        }
                    }
                }

                if let taskLane {
                    compactTaskLane(taskLane)
                }

                compactDockedInputShell

                DisclosureGroup("More context and controls") {
                    VStack(alignment: .leading, spacing: 12) {
                        if let voiceSummary = voiceModel.continuitySummary(using: store) {
                            todaySection("Voice continuity") {
                                VStack(alignment: .leading, spacing: 6) {
                                    Text(voiceSummary.headline)
                                    if let detail = voiceSummary.detail {
                                        Text(detail)
                                            .font(.caption)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                            }
                        }

                        todaySection("Next action") {
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

                        todaySection("Current context") {
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

                        todaySection("Project context") {
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
                    }
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 14)
            .padding(.top, 8)
            .padding(.bottom, 120)
        }
        .modifier(TopAlignedScrollContent())
        .refreshable { await store.refresh() }
    }

    @ViewBuilder
    private var todayHeader: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack(alignment: .top, spacing: 12) {
                VStack(alignment: .leading, spacing: 4) {
                    Text(cachedNow?.header?.title ?? "Vel Now")
                        .font(.system(.largeTitle, design: .rounded, weight: .bold))
                    Text(cachedNow?.status_row?.date_label ?? Date.now.formatted(.dateTime.weekday(.wide).month(.abbreviated).day()))
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                }

                Spacer()

                HStack(spacing: 10) {
                    Menu {
                        Button {
                            onOpenCapture()
                        } label: {
                            Label("Open capture", systemImage: "camera")
                        }

                        Button {
                            onOpenVoice()
                        } label: {
                            Label("Open voice", systemImage: "waveform")
                        }

                        Button {
                            onOpenChat()
                        } label: {
                            Label("Open chat", systemImage: "message")
                        }
                    } label: {
                        Image(systemName: "plus")
                    }
                    .velActionButtonStyle()

                    Button {
                        Task { await store.refresh() }
                    } label: {
                        if store.isSyncing {
                            ProgressView()
                        } else {
                            Image(systemName: "arrow.clockwise")
                        }
                    }
                    .velActionButtonStyle()
                }
            }

            if let buckets = cachedNow?.header?.buckets, !buckets.isEmpty {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(buckets) { bucket in
                            HStack(spacing: 6) {
                                Text(nowHeaderBucketLabel(bucket.kind))
                                if showNowBucketCount(bucket) {
                                    Text("\(bucket.count)")
                                        .font(.caption2)
                                        .padding(.horizontal, 6)
                                        .padding(.vertical, 2)
                                        .background(Color.white.opacity(0.08), in: Capsule())
                                }
                            }
                            .font(.caption)
                            .foregroundStyle(bucket.urgent ? Color.orange : .secondary)
                            .padding(.horizontal, 10)
                            .padding(.vertical, 6)
                            .background(Color.white.opacity(0.04), in: Capsule())
                        }
                    }
                }
            }

            ConnectionSummaryRow(store: store)
        }
        .padding(14)
        .background(
            RoundedRectangle(cornerRadius: 18, style: .continuous)
                .fill(
                    LinearGradient(
                        colors: [
                            Color.white.opacity(0.08),
                            Color.white.opacity(0.03)
                        ],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
        )
        .overlay(
            RoundedRectangle(cornerRadius: 18, style: .continuous)
                .stroke(Color.white.opacity(0.08), lineWidth: 1)
        )
    }

    @ViewBuilder
    private func todaySection<Content: View>(
        _ title: String,
        @ViewBuilder content: () -> Content
    ) -> some View {
        SurfaceSectionCard(title, content: content)
    }

    private func projectLabel(for projectID: String?) -> String? {
        guard let projectID else { return nil }
        return store.offlineStore.cachedProjects().first(where: { $0.id == projectID })?.name
    }

    @ViewBuilder
    private func compactNowStatusRow(_ statusRow: NowStatusRowData) -> some View {
        SurfaceSectionCard("Status") {
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text(statusRow.date_label)
                    Spacer()
                    Text(statusRow.time_label)
                }
                .font(.caption)
                .foregroundStyle(.secondary)

                Text(statusRow.context_label)
                    .font(.headline)

                Text(statusRow.elapsed_label)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    @ViewBuilder
    private func compactNowMeshSummary(_ meshSummary: NowMeshSummaryData) -> some View {
        SurfaceSectionCard("Trust") {
            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(meshSummary.authority_label)
                    Spacer()
                    Text(nowMeshStateLabel(meshSummary.sync_state))
                        .foregroundStyle(meshSummary.urgent ? Color.orange : .secondary)
                }
                .font(.caption)

                Text("\(meshSummary.linked_node_count) linked · \(meshSummary.queued_write_count) queued")
                    .font(.caption2)
                    .foregroundStyle(.secondary)

                if let repairRoute = meshSummary.repair_route {
                    Text(repairRoute.summary)
                        .font(.caption2)
                        .foregroundStyle(meshSummary.urgent ? Color.orange : .secondary)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    @ViewBuilder
    private func compactNowBar(_ bar: NowNudgeBarData) -> some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack(alignment: .top) {
                VStack(alignment: .leading, spacing: 4) {
                    Text(bar.title)
                    Text(bar.summary)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                Spacer()
                Text(nowBarKindLabel(bar.kind))
                    .font(.caption2)
                    .foregroundStyle(bar.urgent ? Color.orange : .secondary)
            }

            if !bar.actions.isEmpty {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(bar.actions) { action in
                            Text(action.label)
                                .font(.caption2)
                                .padding(.horizontal, 8)
                                .padding(.vertical, 5)
                                .background(Color.white.opacity(0.05), in: Capsule())
                        }
                    }
                }
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(10)
        .background(bar.urgent ? Color.orange.opacity(0.10) : Color.white.opacity(0.03), in: RoundedRectangle(cornerRadius: 12, style: .continuous))
    }

    @ViewBuilder
    private func compactTaskLane(_ taskLane: NowTaskLaneData) -> some View {
        SurfaceSectionCard("Tasks") {
            VStack(alignment: .leading, spacing: 8) {
                if let active = taskLane.active {
                    compactTaskRow(active, emphasis: "Active")
                }
                ForEach(taskLane.pending) { item in
                    compactTaskRow(item, emphasis: nil)
                }
                ForEach(taskLane.recent_completed) { item in
                    compactTaskRow(item, emphasis: "Done")
                }
                if taskLane.overflow_count > 0 {
                    Text("+\(taskLane.overflow_count) more")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
                if taskLane.active == nil && taskLane.pending.isEmpty && taskLane.recent_completed.isEmpty {
                    Text("No current tasks are surfaced right now.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    @ViewBuilder
    private func compactTaskRow(_ item: NowTaskLaneItemData, emphasis: String?) -> some View {
        HStack(alignment: .top, spacing: 10) {
            Image(systemName: item.state == "completed" ? "checkmark.circle.fill" : "circle")
                .foregroundStyle(item.state == "completed" ? Color.green : .secondary)
            VStack(alignment: .leading, spacing: 3) {
                HStack {
                    if let emphasis {
                        Text(emphasis)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    Text(item.task_kind.rawValue)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
                Text(item.text)
                    .strikethrough(item.state == "completed")
                if let project = item.project {
                    Text(project)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
            Spacer()
            if item.task_kind == .commitment && item.state != "completed" {
                Button {
                    Task {
                        await store.markCommitmentDone(id: item.id)
                    }
                } label: {
                    Image(systemName: "checkmark.circle")
                }
                .buttonStyle(.borderless)
                .foregroundStyle(.orange)
            }
        }
        .padding(.vertical, 2)
    }

    @ViewBuilder
    private var compactDockedInputShell: some View {
        SurfaceSectionCard("Input") {
            VStack(alignment: .leading, spacing: 10) {
                if let dockedInput = cachedNow?.docked_input {
                    Text("Quick entry and voice stay shell wrappers over backend-owned routing.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    Text(dockedInput.supported_intents.map(\.rawValue).joined(separator: " · "))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                HStack {
                    Button("Open capture") {
                        onOpenCapture()
                    }
                    .velActionButtonStyle()

                    Button("Open voice") {
                        onOpenVoice()
                    }
                    .velActionButtonStyle()

                    Button("Open chat") {
                        onOpenChat()
                    }
                    .velActionButtonStyle()
                }

                TextField("New commitment", text: $commitmentText)
                    .textInputAutocapitalization(.sentences)
                    .textFieldStyle(.roundedBorder)
                Button("Create commitment") {
                    let text = commitmentText.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !text.isEmpty else { return }
                    Task {
                        await store.createCommitment(text: text)
                        commitmentText = ""
                    }
                }
                .velProminentActionButtonStyle()

                TextField("Quick capture", text: $captureText)
                    .textInputAutocapitalization(.sentences)
                    .textFieldStyle(.roundedBorder)
                Button("Save capture") {
                    let text = captureText.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !text.isEmpty else { return }
                    Task {
                        await store.createCapture(text: text)
                        captureText = ""
                    }
                }
                .velActionButtonStyle()
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }
}

private struct ChatTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel
    let onOpenCapture: () -> Void
    let onOpenVoice: () -> Void
    let onOpenThreads: () -> Void
    let onOpenSettings: () -> Void
    let onOpenLinking: () -> Void

    @State private var composerText = ""
    @State private var sending = false
    @State private var conversationID: String?
    @State private var routeTarget: String?
    @State private var lastStatus: String?
    @State private var conversationHistory: [ChatHistoryRow] = []

    var body: some View {
        List {
            Section("Assistant") {
                if let routeTarget {
                    Text("Route: \(routeTarget)")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                if let lastStatus, !lastStatus.isEmpty {
                    Text(lastStatus)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                TextEditor(text: $composerText)
                    .frame(minHeight: 130)

                HStack {
                    Button("Send") {
                        Task { await submitMessage() }
                    }
                    .velProminentActionButtonStyle()
                    .disabled(composerText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty || sending)

                    if let transcript = latestVoiceTranscript {
                        Button(sending ? "Sending voice..." : "Send voice") {
                            Task { await submitMessage(rawText: transcript, clearComposer: false) }
                        }
                        .velActionButtonStyle()
                        .disabled(sending)

                        Button("Use voice text") {
                            appendVoiceTranscript()
                        }
                        .velActionButtonStyle()
                        .disabled(sending)
                    }

                    Button(sending ? "Sending..." : "Open voice") { onOpenVoice() }
                        .velActionButtonStyle()
                        .disabled(sending)

                    Button("Open capture") {
                        onOpenCapture()
                    }
                    .velActionButtonStyle()

                    Button("Open threads") {
                        onOpenThreads()
                    }
                    .velActionButtonStyle()

                    Button("Settings") {
                        onOpenSettings()
                    }
                    .velActionButtonStyle()

                    Button("Link nodes") {
                        onOpenLinking()
                    }
                    .velActionButtonStyle()
                }

                if let conversationID {
                    Text("Using thread: \(conversationID)")
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                        .lineLimit(1)
                }
            }

            if let now = store.offlineStore.cachedNow() {
                Section("Context") {
                    let hints = threadHints
                    if let contextLine = now.context_line {
                        Text(contextLine.text)
                            .font(.caption)
                            .foregroundStyle(contextLine.fallback_used ? .secondary : .primary)
                    } else {
                    Text("No cached context line available yet.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    }

                    if hints.isEmpty {
                        Text("No thread hint in cache.")
                            .font(.caption)
                            .foregroundStyle(.tertiary)
                    } else {
                        Text("Thread hints:")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                        ScrollView(.horizontal, showsIndicators: false) {
                            HStack(spacing: 8) {
                                ForEach(hints, id: \.id) { hint in
                                    Button {
                                        useThreadHint(hint)
                                    } label: {
                                        Text(hint.label)
                                            .font(.caption2)
                                    }
                                    .buttonStyle(.bordered)
                                    .controlSize(.small)
                                    .tint(.orange)
                                }
                            }
                        }
                    }
                    if let activeID = conversationID {
                        Text("Using thread: \(activeID)")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                            .lineLimit(1)
                    }
                }
            }

            if !conversationHistory.isEmpty {
                Section("Conversation history") {
                    ForEach(Array(conversationHistory.suffix(12).enumerated()), id: \.element.id) { index, row in
                        VStack(alignment: .leading, spacing: 4) {
                            HStack(alignment: .firstTextBaseline, spacing: 6) {
                                Text(row.actorLabel)
                                    .font(.caption2)
                                    .fontWeight(.semibold)
                                    .foregroundStyle(row.actorColor)
                                Text(formatDate(row.timestamp))
                                    .font(.caption2)
                                    .foregroundStyle(.tertiary)
                                Spacer()
                            }
                            Text(row.text)
                                .font(row.role == .system ? .caption2 : .body)
                                .foregroundStyle(row.role == .system ? .secondary : .primary)
                                .fixedSize(horizontal: false, vertical: true)
                            if let detail = row.detail, !detail.isEmpty {
                                Text(detail)
                                    .font(.caption2)
                                    .foregroundStyle(.tertiary)
                            }
                        }
                        .padding(.vertical, 2)

                        if index < conversationHistory.suffix(12).count - 1 {
                            Divider()
                        }
                    }
                }
            }
        }
        .velCompactListStyle()
        .refreshable { await store.refresh() }
    }

    private func submitMessage() async {
        await submitMessage(rawText: nil, clearComposer: true)
    }

    private func submitMessage(rawText: String?, clearComposer: Bool) async {
        let text = (rawText ?? composerText).trimmingCharacters(in: .whitespacesAndNewlines)
        guard !text.isEmpty else { return }

        sending = true
        let currentConversationID = conversationID ?? resolveConversationID(from: store.offlineStore.cachedNow())
        conversationHistory.append(.init(role: .user, text: text))

        if let response = await store.submitAssistantEntry(
            text: text,
            conversationID: currentConversationID
        ) {
            if let resolved = response.conversation?.id {
                conversationID = resolved
            }
            routeTarget = response.route_target ?? "inline"
            if let error = response.assistant_error, !error.isEmpty {
                lastStatus = error
                conversationHistory.append(
                    .init(role: .system, text: "Assistant returned an error.", detail: error)
                )
            } else {
                lastStatus = "Message sent to \(routeTarget ?? "inline")."
                conversationHistory.append(
                    .init(
                        role: .assistant,
                        text: "Message acknowledged for \(routeTarget ?? "inline").",
                        detail: conversationHistory.count < 2 ? "Conversation starts now." : nil
                    )
                )
            }
        } else {
            if let currentConversationID {
                routeTarget = "threads (queued)"
                conversationID = currentConversationID
            } else {
                routeTarget = "inline (queued)"
            }
            if let error = store.errorMessage, !error.isEmpty {
                lastStatus = error
            } else {
                lastStatus = "Message queued for sync."
            }
            conversationHistory.append(
                .init(role: .system, text: "Send failed; queued for sync.", detail: routeTarget)
            )
        }

        if clearComposer {
            composerText = ""
        }
        sending = false
        await store.refresh()
    }

    private func appendVoiceTranscript() {
        guard let transcript = latestVoiceTranscript else { return }
        if composerText.isEmpty {
            composerText = transcript
        } else {
            composerText = "\(composerText)\n\(transcript)"
        }
    }

    private var latestVoiceTranscript: String? {
        let trimmedCurrent = voiceModel.transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        if !trimmedCurrent.isEmpty {
            return trimmedCurrent
        }
        if let historyTop = voiceModel.history.first?.transcript.trimmingCharacters(in: .whitespacesAndNewlines), !historyTop.isEmpty {
            return historyTop
        }
        return nil
    }

    private var cachedNow: NowData? {
        store.offlineStore.cachedNow()
    }

    private var threadHints: [ThreadHint] {
        var hints: [ThreadHint] = []
        var seen: Set<String> = []

        func appendHint(id: String?, label: String) {
            let trimmed = id?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
            guard !trimmed.isEmpty else { return }
            if seen.contains(trimmed) { return }
            seen.insert(trimmed)
            hints.append(ThreadHint(threadID: trimmed, label: "\(label): \(threadIDShort(trimmed))"))
        }

        if let now = cachedNow {
            appendHint(id: now.docked_input?.raw_capture_thread_id, label: "Docked")
            appendHint(id: now.context_line?.thread_id, label: "Context")
            if let taskLane = now.task_lane {
                appendHint(id: taskLane.active?.primary_thread_id, label: "Active")
                for item in taskLane.pending.prefix(2) {
                    appendHint(id: item.primary_thread_id, label: "Pending")
                }
                for item in taskLane.recent_completed.prefix(2) {
                    appendHint(id: item.primary_thread_id, label: "Done")
                }
            }
        }

        if !hints.isEmpty {
            return hints
        }

        if let fallback = resolveConversationID(from: cachedNow) {
            hints.append(ThreadHint(threadID: fallback, label: "Current: \(threadIDShort(fallback))"))
        }

        return hints
    }

    private func useThreadHint(_ hint: ThreadHint) {
        conversationID = hint.threadID
        routeTarget = "threads"
        lastStatus = "Conversation locked to \(hint.label)."
        conversationHistory.append(
            .init(role: .system, text: "Thread switched to \(hint.threadID)")
        )
    }

    private func threadIDShort(_ threadID: String) -> String {
        if threadID.count <= 10 {
            return threadID
        }
        return String(threadID.prefix(6)) + "…" + String(threadID.suffix(4))
    }

    private func resolveConversationID(from now: NowData?) -> String? {
        guard let now else { return nil }
        if let threadID = now.docked_input?.raw_capture_thread_id {
            return threadID
        }
        if let threadID = now.context_line?.thread_id {
            return threadID
        }
        if let threadID = now.task_lane?.active?.primary_thread_id {
            return threadID
        }
        if let first = now.task_lane?.pending.first(where: { $0.primary_thread_id != nil }) {
            return first.primary_thread_id
        }
        if let first = now.task_lane?.recent_completed.first(where: { $0.primary_thread_id != nil }) {
            return first.primary_thread_id
        }
        return nil
    }

    private enum ChatRole: String {
        case user
        case assistant
        case system
    }

    private struct ChatHistoryRow: Identifiable {
        let id = UUID()
        let role: ChatRole
        let text: String
        let detail: String?
        let timestamp: Date

        init(role: ChatRole, text: String, detail: String? = nil, timestamp: Date = Date()) {
            self.role = role
            self.text = text
            self.detail = detail
            self.timestamp = timestamp
        }

        var actorLabel: String {
            role.rawValue.capitalized
        }

        var actorColor: Color {
            switch role {
            case .user:
                return .blue
            case .assistant:
                return .green
            case .system:
                return .orange
            }
        }
    }

    private struct ThreadHint: Identifiable, Hashable {
        let threadID: String
        let label: String

        var id: String { threadID + label }
    }
}

private func showNowBucketCount(_ bucket: NowHeaderBucketData) -> Bool {
    switch bucket.count_display {
    case .always_show:
        return true
    case .show_nonzero, .hidden_until_active:
        return bucket.count > 0
    }
}

private func nowHeaderBucketLabel(_ kind: NowHeaderBucketKindData) -> String {
    switch kind {
    case .threads_by_type:
        return "Threads"
    case .needs_input:
        return "Needs input"
    case .new_nudges:
        return "Nudges"
    case .search_filter:
        return "Filter"
    case .snoozed:
        return "Snoozed"
    case .review_apply:
        return "Review"
    case .reflow:
        return "Reflow"
    case .follow_up:
        return "Follow up"
    }
}

private func nowMeshStateLabel(_ state: NowMeshSyncStateData) -> String {
    switch state {
    case .synced:
        return "Synced"
    case .stale:
        return "Stale"
    case .local_only:
        return "Local only"
    case .offline:
        return "Offline"
    }
}

private func nowBarKindLabel(_ kind: NowNudgeBarKindData) -> String {
    switch kind {
    case .nudge:
        return "Nudge"
    case .needs_input:
        return "Needs input"
    case .review_request:
        return "Review"
    case .reflow_proposal:
        return "Reflow"
    case .thread_continuation:
        return "Thread"
    case .trust_warning:
        return "Trust"
    case .freshness_warning:
        return "Freshness"
    }
}

private struct TopAlignedScrollContent: ViewModifier {
    func body(content: Content) -> some View {
        if #available(iOS 17.0, *) {
            content.contentMargins(.top, 0, for: .scrollContent)
        } else {
            content
        }
    }
}

private struct SurfaceSectionCard<Content: View>: View {
    let title: String
    let content: Content

    init(_ title: String, @ViewBuilder content: () -> Content) {
        self.title = title
        self.content = content()
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text(title)
                .font(.headline)
                .foregroundStyle(.secondary)
            content
        }
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 14, style: .continuous)
                .fill(Color.white.opacity(0.04))
        )
        .overlay(
            RoundedRectangle(cornerRadius: 14, style: .continuous)
                .stroke(Color.white.opacity(0.08), lineWidth: 1)
        )
    }
}

private struct NudgesTab: View {
    @ObservedObject var store: VelClientStore

    var body: some View {
        let active = store.nudges.filter { $0.state == "active" || $0.state == "snoozed" }
        let inboxActionCount = store.offlineStore.cachedActionItems()
            .filter { $0.surface == .inbox }
            .count

        ScrollView {
            LazyVStack(alignment: .leading, spacing: 12) {
                SurfaceSectionCard("Inbox") {
                    Text("This is the Apple triage lane. Urgent review stays here; deeper archive and history stay in `Threads`.")
                        .foregroundStyle(.secondary)
                    Text("\(inboxActionCount) backend action items are currently tagged for inbox review.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                SurfaceSectionCard("Active nudges") {
                    if active.isEmpty {
                        Text("No active nudges.")
                            .foregroundStyle(.secondary)
                    } else {
                        ForEach(Array(active.enumerated()), id: \.element.nudge_id) { index, nudge in
                            VStack(alignment: .leading, spacing: 10) {
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
                                    .velProminentActionButtonStyle()

                                    Button("Snooze 10m") {
                                        Task {
                                            await store.snoozeNudge(id: nudge.nudge_id, minutes: 10)
                                        }
                                    }
                                    .velActionButtonStyle()
                                }
                            }
                            .frame(maxWidth: .infinity, alignment: .leading)
                            if index < active.count - 1 {
                                Divider()
                                    .padding(.vertical, 2)
                            }
                        }
                    }
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 14)
            .padding(.top, 8)
            .padding(.bottom, 120)
        }
        .modifier(TopAlignedScrollContent())
        .refreshable { await store.refresh() }
    }
}

private struct ActivityTab: View {
    @ObservedObject var store: VelClientStore
    @ObservedObject var voiceModel: VoiceCaptureModel

    var body: some View {
        let recentSignals = Array(store.signals.prefix(80))

        ScrollView {
            LazyVStack(alignment: .leading, spacing: 12) {
                SurfaceSectionCard("Threads") {
                    Text("Threads are the Apple continuity lane. Use this surface for recent thread-oriented history and deeper follow-up, not daily-use triage.")
                        .foregroundStyle(.secondary)
                    if let urgentThreads = store.context?.context?.message_urgent_thread_count {
                        Text("\(urgentThreads) urgent threads are currently flagged in context.")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }

                SurfaceSectionCard("Voice continuity") {
                    if let summary = voiceModel.continuitySummary(using: store) {
                        Text(summary.headline)
                        if let detail = summary.detail {
                            Text(detail)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                    } else {
                        Text("No local voice continuity is waiting for recovery.")
                            .foregroundStyle(.secondary)
                    }

                    let entries = Array(voiceModel.history.prefix(3))
                    if !entries.isEmpty {
                        Divider()
                            .padding(.vertical, 2)
                        ForEach(entries) { entry in
                            VStack(alignment: .leading, spacing: 4) {
                                HStack {
                                    Text(entry.statusLabel)
                                        .font(.caption2)
                                        .foregroundStyle(entry.statusColor)
                                    Spacer()
                                    Text(formatDate(entry.createdAt))
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                                Text(entry.transcript)
                                    .font(.subheadline)
                                    .lineLimit(2)
                                if let detail = entry.continuityDetail {
                                    Text(detail)
                                        .font(.caption2)
                                        .foregroundStyle(.tertiary)
                                }
                            }
                            .padding(.vertical, 2)
                        }
                    }
                }

                SurfaceSectionCard("Recent signals") {
                    if recentSignals.isEmpty {
                        Text("No signals available yet.")
                            .foregroundStyle(.secondary)
                    } else {
                        ForEach(Array(recentSignals.enumerated()), id: \.element.signal_id) { index, signal in
                            VStack(alignment: .leading, spacing: 6) {
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
                            .frame(maxWidth: .infinity, alignment: .leading)
                            if index < recentSignals.count - 1 {
                                Divider()
                                    .padding(.vertical, 2)
                            }
                        }
                    }
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 14)
            .padding(.top, 8)
            .padding(.bottom, 120)
        }
        .modifier(TopAlignedScrollContent())
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
                .velActionButtonStyle()

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
                .velProminentActionButtonStyle()
                .disabled(!hasDraftContent)

                Button("Clear draft", role: .destructive) {
                    clearDraft()
                }
                .velActionButtonStyle()
                .disabled(!hasDraftContent && selectedPhotoData == nil)

                if let statusMessage, !statusMessage.isEmpty {
                    Text(statusMessage)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
        .velCompactListStyle()
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
    @State private var dailyLoopResponseText = ""
    @State private var dailyLoopStatusMessage: String?

    private var activeDailyLoop: DailyLoopSessionData? {
        store.standupDailyLoop ?? store.morningDailyLoop
    }

    var body: some View {
        List {
            Section("Permissions") {
                PermissionRow(label: "Speech recognition", state: voiceModel.speechPermission)
                PermissionRow(label: "Microphone", state: voiceModel.microphonePermission)

                Button("Request permissions") {
                    Task { await voiceModel.requestPermissions() }
                }
                .velActionButtonStyle()
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
                            .velActionButtonStyle()

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
                .velProminentActionButtonStyle()
                .disabled(!voiceModel.hasTranscript)

                HStack {
                    Button("Save as capture") {
                        Task { await voiceModel.submitAsCapture(using: store) }
                    }
                    .velActionButtonStyle()
                    .disabled(!voiceModel.hasTranscript)

                    Button("Create commitment") {
                        Task { await voiceModel.submitAsCommitment(using: store) }
                    }
                    .velActionButtonStyle()
                    .disabled(!voiceModel.hasTranscript)
                }

                Text("Voice submissions preserve transcript provenance, defer shared product behavior to the backend Apple route, and keep longer follow-up in Threads.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)

                Button("Open multimodal composer") {
                    onOpenCaptureComposer(voiceModel.transcript)
                }
                .velActionButtonStyle()
                .disabled(!voiceModel.hasTranscript)
            }

            Section("Quick commands") {
                ForEach(VoiceCommandExample.defaults) { example in
                    Button(example.label) {
                        voiceModel.applyCommandExample(example.command)
                    }
                    .velActionButtonStyle()

                    Text(example.command)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }

            Section("Daily loop") {
                if let session = activeDailyLoop {
                    VStack(alignment: .leading, spacing: 10) {
                        Text(session.phase == .morningOverview ? "Morning overview" : "Standup")
                            .font(.headline)
                        Text(session.status.rawValue.replacingOccurrences(of: "_", with: " "))
                            .font(.caption)
                            .foregroundStyle(.secondary)

                        if session.state.phase == .morningOverview {
                            if let snapshot = session.state.snapshot, !snapshot.isEmpty {
                                Text(snapshot)
                                    .font(.subheadline)
                            }
                            ForEach(session.state.friction_callouts, id: \.label) { callout in
                                Text("\(callout.label): \(callout.detail)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                        } else {
                            if session.state.commitments.isEmpty {
                                Text("No commitments are locked yet.")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            } else {
                                ForEach(session.state.commitments, id: \.title) { commitment in
                                    Text("\(commitment.bucket.rawValue.uppercased()) · \(commitment.title)")
                                        .font(.caption)
                                }
                            }
                        }

                        if let prompt = session.current_prompt {
                            Text(prompt.text)
                                .font(.subheadline)
                            TextField("Short response", text: $dailyLoopResponseText, axis: .vertical)
                                .textInputAutocapitalization(.sentences)

                            HStack {
                                Button("Submit response") {
                                    Task { await submitDailyLoopTurn(action: .submit) }
                                }
                                .buttonStyle(.borderedProminent)
                                .disabled(dailyLoopResponseText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

                                Button("Skip") {
                                    Task { await submitDailyLoopTurn(action: .skip) }
                                }
                                .buttonStyle(.bordered)
                            }
                        }
                    }
                } else {
                    VStack(alignment: .leading, spacing: 10) {
                        Text("Use the backend daily loop for the bounded morning and standup flow.")
                            .font(.caption)
                            .foregroundStyle(.secondary)

                        HStack {
                            Button("Start morning") {
                                Task { await startDailyLoop(.morningOverview) }
                            }
                            .buttonStyle(.borderedProminent)

                            Button("Start standup") {
                                Task { await startDailyLoop(.standup) }
                            }
                            .buttonStyle(.bordered)
                        }
                    }
                }

                if let message = dailyLoopStatusMessage, !message.isEmpty {
                    Text(message)
                        .font(.caption)
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
                    .velActionButtonStyle()
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
                        .velActionButtonStyle()
                    }
                    .padding(.vertical, 4)
                }
            }
        }
        .velCompactListStyle()
        .onChange(of: activeDailyLoop?.id) { _ in
            dailyLoopResponseText = ""
        }
    }

    private func startDailyLoop(_ phase: DailyLoopPhaseData) async {
        guard store.isReachable else {
            dailyLoopStatusMessage = "Reconnect to Vel to start the backend daily loop. Offline mode only shows cached session state."
            return
        }

        do {
            _ = try await store.client.startDailyLoopSession(
                DailyLoopStartRequestData(
                    phase: phase,
                    session_date: sessionDateForApple(),
                    start: DailyLoopStartMetadataData(source: .manual, surface: .appleVoice)
                )
            )
            await store.refresh()
            dailyLoopStatusMessage = phase == .morningOverview
                ? "Morning overview started."
                : "Standup started."
            dailyLoopResponseText = ""
        } catch {
            dailyLoopStatusMessage = "Could not start the daily loop. \(error.localizedDescription)"
        }
    }

    private func submitDailyLoopTurn(action: DailyLoopTurnActionData) async {
        guard let session = activeDailyLoop else { return }
        guard store.isReachable else {
            dailyLoopStatusMessage = "Reconnect to Vel to continue the backend daily loop. Offline mode does not invent new prompts or commitments."
            return
        }

        do {
            _ = try await store.client.submitDailyLoopTurn(
                sessionID: session.id,
                action: action,
                responseText: action == .submit
                    ? dailyLoopResponseText.trimmingCharacters(in: .whitespacesAndNewlines)
                    : nil
            )
            await store.refresh()
            dailyLoopStatusMessage = action == .submit
                ? "Daily loop response saved."
                : "Daily loop advanced."
            dailyLoopResponseText = ""
        } catch {
            dailyLoopStatusMessage = "Could not continue the daily loop. \(error.localizedDescription)"
        }
    }
}

private struct SettingsTab: View {
    let appEnvironment: VelAppEnvironment
    @ObservedObject var store: VelClientStore
    let initialSection: SettingsLaunchSection
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
    @State private var connectInstanceID = ""
    @State private var connectInput = ""
    @State private var connectStatusMessage: String?
    @State private var connectEvents: [ConnectRunEventData] = []
    @State private var connectStreaming = false
    @State private var connectStreamTask: Task<Void, Never>?
    @State private var resolvingConnectInstance = false
    @State private var connectRecentInstances: [ConnectInstanceData] = []
    @State private var selectedConnectRecentID: String = ""

    var body: some View {
        ScrollViewReader { proxy in
            List {
                runtimeSection
                    .id(SettingsLaunchSection.overview.sectionAnchor)
                embeddedBridgeSection
                connectRuntimeSection
                endpointOverrideSection
                linkingSection
                    .id(SettingsLaunchSection.linking.sectionAnchor)
                linkedDevicesSection
                operatorAuthSection
                docsSection
            }
        .velCompactListStyle()
        .onAppear {
            if selectedDiscoveredNodeID == nil {
                selectedDiscoveredNodeID = store.discoveredWorkers.first?.node_id
            }
            scrollToInitialSection(using: proxy)
        }
        .onChange(of: initialSection) { _ in
            scrollToInitialSection(using: proxy)
        }
        .onDisappear {
            connectStreamTask?.cancel()
            connectStreamTask = nil
            connectStreaming = false
        }
        }
        .onChange(of: store.discoveredWorkers.map(\.node_id)) { nodeIDs in
            if !nodeIDs.contains(selectedDiscoveredNodeID ?? "") {
                selectedDiscoveredNodeID = nodeIDs.first
            }
        }
    }

    private func scrollToInitialSection(using proxy: ScrollViewProxy) {
        guard initialSection != .overview else { return }
        DispatchQueue.main.async {
            withAnimation(.easeInOut(duration: 0.2)) {
                proxy.scrollTo(initialSection.sectionAnchor, anchor: .top)
            }
        }
    }

    private var runtimeSection: some View {
        Section("Advanced operator setup") {
            Text("Apple stays summary-first by default. Daily-use work belongs in `Now`, `Inbox`, and `Threads`; deeper setup and trust detail live here.")
                .font(.caption)
                .foregroundStyle(.secondary)
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
            planningProfileSummary
        }
    }

    private var embeddedBridgeSection: some View {
        let configuration = appEnvironment.embeddedBridge.configuration
        let runtimeStatus = appEnvironment.embeddedBridge.runtimeStatus

        return Section("Embedded Rust bridge") {
            if appEnvironment.featureCapabilities.supportsEmbeddedRustBridge {
                Text("Bridge is enabled for this surface.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } else {
                Text("Bridge is disabled on this device surface.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            Text("Runtime mode: \(configuration.mode.rawValue)")
                .font(.caption2)
                .foregroundStyle(.secondary)
            Text("Target: \(configuration.target.rawValue)")
                .font(.caption2)
                .foregroundStyle(.secondary)
            Text("Runtime status: \(runtimeStatus.isOperational ? "operational" : "not operational")")
                .font(.caption2)
                .foregroundStyle(runtimeStatus.isOperational ? .green : .secondary)
            Text("Library in build: \(configuration.isBridgeAvailableInBuild ? "yes" : "no")")
                .font(.caption2)
                .foregroundStyle(.secondary)
            if let source = runtimeStatus.resolvedSource {
                Text("Runtime source: \(source)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            } else {
                Text("Runtime source: unresolved")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            if !runtimeStatus.attemptedPaths.isEmpty {
                Text("Lookup attempts: \(runtimeStatus.attemptedPaths.count)")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            }

            BoolStatusRow(label: "Cached now hydration", value: configuration.permits(.cachedNowHydration))
            BoolStatusRow(label: "Local quick capture", value: configuration.permits(.localQuickActionPreparation))
            BoolStatusRow(label: "Offline request packaging", value: configuration.permits(.offlineRequestPackaging))
            BoolStatusRow(label: "Domain helpers", value: configuration.permits(.deterministicDomainHelpers))
            BoolStatusRow(label: "Local thread draft packaging", value: configuration.permits(.localThreadDraftPackaging))
            BoolStatusRow(label: "Local voice capture packaging", value: configuration.permits(.localVoiceCapturePackaging))
            BoolStatusRow(label: "Local voice quick action packaging", value: configuration.permits(.localVoiceQuickActionPackaging))

            BoolStatusRow(label: "Cached now symbol loaded", value: runtimeStatus.symbolAvailable(for: .cachedNowHydration))
            BoolStatusRow(label: "Quick capture symbol loaded", value: runtimeStatus.symbolAvailable(for: .localQuickActionPreparation))
            BoolStatusRow(label: "Offline packaging symbol loaded", value: runtimeStatus.symbolAvailable(for: .offlineRequestPackaging))
            BoolStatusRow(label: "Domain helper symbol loaded", value: runtimeStatus.symbolAvailable(for: .deterministicDomainHelpers))
            BoolStatusRow(label: "Thread draft symbol loaded", value: runtimeStatus.symbolAvailable(for: .localThreadDraftPackaging))
            BoolStatusRow(label: "Voice capture symbol loaded", value: runtimeStatus.symbolAvailable(for: .localVoiceCapturePackaging))
            BoolStatusRow(label: "Voice quick action symbol loaded", value: runtimeStatus.symbolAvailable(for: .localVoiceQuickActionPackaging))

            if configuration.approvedFlows.isEmpty {
                Text("No embedded bridge flows are currently permitted.")
                    .font(.caption)
                    .foregroundStyle(.yellow)
            } else {
                let approved = configuration.approvedFlows
                    .map { $0.rawValue }
                    .sorted()
                    .joined(separator: ", ")
                Text("Approved flows: \(approved)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
    }

    @ViewBuilder
    private var planningProfileSummary: some View {
        if let planningProfile = store.planningProfile {
            let profile = planningProfile.profile
            let activeBlocks = profile.routine_blocks.filter { $0.active }.count
            let activeConstraints = profile.planning_constraints.filter { $0.active }.count
            VStack(alignment: .leading, spacing: 6) {
                Text("Planning profile")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Text("Routine blocks: \(activeBlocks) active of \(profile.routine_blocks.count)")
                    .font(.caption)
                Text("Constraints: \(activeConstraints) active of \(profile.planning_constraints.count)")
                    .font(.caption)
                if let firstBlock = profile.routine_blocks.first {
                    Text("Next anchor: \(firstBlock.label) \(firstBlock.start_local_time)-\(firstBlock.end_local_time)")
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                }
                if let proposalSummary = planningProfile.proposal_summary {
                    Text("Proposal continuity: \(proposalSummary.pending_count) pending")
                        .font(.caption)
                    if let latestPending = proposalSummary.latest_pending {
                        Text("Pending: \(latestPending.title)")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    if let latestApplied = proposalSummary.latest_applied {
                        Text("Last applied: \(latestApplied.title)")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                    } else if let latestFailed = proposalSummary.latest_failed {
                        Text("Last failed: \(latestFailed.title)")
                            .font(.caption2)
                            .foregroundStyle(.red)
                    }
                }
            }
        } else {
            Text("Routine blocks and planning constraints load from the backend-owned planning profile used by day plan and reflow.")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }

    private var endpointOverrideSection: some View {
        Section("Connection and trust") {
            TextField("http://host:4130", text: $baseURLOverride)
                .textInputAutocapitalization(.never)
                .autocorrectionDisabled()

            Button("Save and reconnect") {
                store.setBaseURLOverride(baseURLOverride)
                Task { await store.refresh() }
            }
            .velProminentActionButtonStyle()

            Button("Clear override") {
                baseURLOverride = ""
                store.setBaseURLOverride(nil)
                Task { await store.refresh() }
            }
            .velActionButtonStyle()

            Text("Resolution order: vel_tailscale_url, vel_base_url, vel_lan_base_url, localhost.")
                .font(.caption2)
                .foregroundStyle(.tertiary)
        }
    }

    private var connectRuntimeSection: some View {
        Section("Connect runtime console") {
            Text("Use this only for supervised runtime debugging. Daily work should stay in `Now`, `Inbox`, and `Threads`.")
                .font(.caption)
                .foregroundStyle(.secondary)

            TextField("run_...", text: $connectInstanceID)
                .textInputAutocapitalization(.never)
                .autocorrectionDisabled()
                .font(.system(.body, design: .monospaced))

            if !connectRecentInstances.isEmpty {
                Picker("Recent runtime", selection: Binding(
                    get: {
                        selectedConnectRecentID.isEmpty
                            ? (connectRecentInstances.first?.id ?? "")
                            : selectedConnectRecentID
                    },
                    set: { newValue in
                        selectedConnectRecentID = newValue
                        if !newValue.isEmpty {
                            connectInstanceID = newValue
                            if let instance = connectRecentInstances.first(where: { $0.id == newValue }) {
                                connectStatusMessage = "Selected \(instance.display_name) (\(instance.id))."
                            }
                        }
                    }
                )) {
                    ForEach(connectRecentInstances, id: \.id) { instance in
                        Text("\(instance.display_name) • \(instance.id)")
                            .tag(instance.id)
                    }
                }
            }

            Button(resolvingConnectInstance ? "Resolving..." : "Use latest running instance") {
                Task { await useLatestRunningConnectInstance() }
            }
            .velActionButtonStyle()
            .disabled(resolvingConnectInstance)

            HStack {
                Button(connectStreaming ? "Reattach" : "Attach + stream") {
                    Task { await attachAndStartConnectStream() }
                }
                .velProminentActionButtonStyle()
                .disabled(connectInstanceID.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

                Button("Stop stream") {
                    stopConnectStream()
                }
                .velActionButtonStyle()
                .disabled(!connectStreaming)
            }

            HStack {
                TextField("stdin input", text: $connectInput)
                    .textInputAutocapitalization(.never)
                    .autocorrectionDisabled()
                Button("Send") {
                    Task { await sendConnectInput() }
                }
                .velActionButtonStyle()
                .disabled(connectInput.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            }

            Button("Terminate runtime") {
                Task { await terminateConnectRuntime() }
            }
            .velActionButtonStyle()
            .disabled(connectInstanceID.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)

            if let connectStatusMessage, !connectStatusMessage.isEmpty {
                Text(connectStatusMessage)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            if connectEvents.isEmpty {
                Text("No streamed events yet.")
                    .font(.caption2)
                    .foregroundStyle(.tertiary)
            } else {
                ForEach(Array(connectEvents.suffix(40).enumerated()), id: \.element.id) { _, event in
                    VStack(alignment: .leading, spacing: 2) {
                        Text("[\(event.id)] \(event.stream)")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                        Text(event.chunk)
                            .font(.caption2.monospaced())
                            .textSelection(.enabled)
                    }
                }
            }
        }
    }

    private var linkingSection: some View {
        Section("Linking and recovery") {
            scopeToggle(label: "Read context", value: $pairingReadContext)
            scopeToggle(label: "Write safe actions", value: $pairingWriteSafeActions)
            scopeToggle(label: "Execute repo tasks", value: $pairingExecuteRepoTasks)

            discoveredNodeSection

            Button(isIssuingPairingToken ? "Pairing..." : "Pair nodes") {
                Task { await issuePairingToken() }
            }
            .velProminentActionButtonStyle()
            .disabled(isIssuingPairingToken)

            if let prompt = store.localIncomingLinkingPrompt {
                incomingPromptSection(prompt)
            }
            if let pairingToken {
                currentTokenSection(pairingToken)
            }
            if let pairingFeedback, !pairingFeedback.isEmpty {
                Text(pairingFeedback)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }

    @ViewBuilder
    private var discoveredNodeSection: some View {
        if store.discoveredWorkers.isEmpty {
            Text("No unlinked discovered nodes are active right now.")
                .font(.caption)
                .foregroundStyle(.secondary)
        } else {
            Picker("Discovered node", selection: Binding(
                get: { selectedDiscoveredNodeID ?? store.discoveredWorkers.first?.node_id ?? "" },
                set: { selectedDiscoveredNodeID = $0 }
            )) {
                ForEach(store.discoveredWorkers) { worker in
                    Text(worker.node_display_name).tag(worker.node_id)
                }
            }
            if let selectedNodeID = selectedDiscoveredNodeID ?? store.discoveredWorkers.first?.node_id,
               let worker = store.discoveredWorkers.first(where: { $0.node_id == selectedNodeID }) {
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

    private func incomingPromptSection(_ prompt: LinkingPromptData) -> some View {
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

            Button(isRedeemingPairingToken ? "Entering..." : "Enter token") {
                Task { await redeemPairingToken(using: prompt) }
            }
            .velActionButtonStyle()
            .disabled(isRedeemingPairingToken)
        }
    }

    private func currentTokenSection(_ pairingToken: PairingTokenData) -> some View {
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

    private var linkedDevicesSection: some View {
        Section("Linked devices and scopes") {
            if store.linkedNodes.isEmpty {
                Text("No linked devices yet.")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            } else {
                ForEach(store.linkedNodes) { node in
                    linkedDeviceRow(node)
                        .padding(.vertical, 6)
                }
            }
        }
    }

    private func linkedDeviceRow(_ node: LinkedNodeData) -> some View {
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
                    set: { setLinkedScope(nodeID: node.node_id, field: .readContext, value: $0, fallback: node.scopes) }
                )
            )
            scopeToggle(
                label: "Write safe actions",
                value: Binding(
                    get: { (linkedPermissionDrafts[node.node_id] ?? node.scopes).write_safe_actions },
                    set: { setLinkedScope(nodeID: node.node_id, field: .writeSafeActions, value: $0, fallback: node.scopes) }
                )
            )
            scopeToggle(
                label: "Execute repo tasks",
                value: Binding(
                    get: { (linkedPermissionDrafts[node.node_id] ?? node.scopes).execute_repo_tasks },
                    set: { setLinkedScope(nodeID: node.node_id, field: .executeRepoTasks, value: $0, fallback: node.scopes) }
                )
            )

            Button("Request updated access") {
                Task { await renegotiateLinkedNode(node) }
            }
            .velActionButtonStyle()

            if confirmUnpairNodeID == node.node_id {
                HStack {
                    Button(unpairingNodeID == node.node_id ? "Unpairing..." : "Confirm unpair") {
                        Task { await unpair(node) }
                    }
                    .velProminentActionButtonStyle()
                    .disabled(unpairingNodeID == node.node_id)

                    Button("Cancel") {
                        confirmUnpairNodeID = nil
                    }
                    .velActionButtonStyle()
                }
            } else {
                Button("Unpair") {
                    confirmUnpairNodeID = node.node_id
                }
                .velActionButtonStyle()
            }
        }
    }

    private var operatorAuthSection: some View {
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
            .velActionButtonStyle()

            Text("Operator-authenticated /v1 routes send x-vel-operator-token when configured.")
                .font(.caption2)
                .foregroundStyle(.tertiary)
        }
    }

    private var docsSection: some View {
        Section("Docs and deeper detail") {
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
        let normalized = store.normalizeDomainHint(formatPairingTokenInput(pairingCodeInput)).uppercased()
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

    private enum LinkedScopeField {
        case readContext
        case writeSafeActions
        case executeRepoTasks
    }

    private func setLinkedScope(
        nodeID: String,
        field: LinkedScopeField,
        value: Bool,
        fallback: LinkScopeData
    ) {
        var draft = linkedPermissionDrafts[nodeID] ?? fallback
        switch field {
        case .readContext:
            draft = LinkScopeData(
                read_context: value,
                write_safe_actions: draft.write_safe_actions,
                execute_repo_tasks: draft.execute_repo_tasks
            )
        case .writeSafeActions:
            draft = LinkScopeData(
                read_context: draft.read_context,
                write_safe_actions: value,
                execute_repo_tasks: draft.execute_repo_tasks
            )
        case .executeRepoTasks:
            draft = LinkScopeData(
                read_context: draft.read_context,
                write_safe_actions: draft.write_safe_actions,
                execute_repo_tasks: value
            )
        }
        linkedPermissionDrafts[nodeID] = draft
    }

    private func scopeToggle(label: String, value: Binding<Bool>) -> some View {
        Toggle(isOn: value) {
            Text(label)
        }
    }

    private func attachAndStartConnectStream() async {
        let runID = connectInstanceID.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !runID.isEmpty else {
            connectStatusMessage = "Enter a connect runtime id first."
            return
        }
        guard store.isReachable else {
            connectStatusMessage = "Reconnect to Vel before attaching to a runtime."
            return
        }

        do {
            let attach = try await store.client.attachConnectInstance(id: runID)
            connectStatusMessage = "Attached to \(attach.instance.display_name). Streaming from \(attach.latest_event_id.map(String.init) ?? "start")."
            startConnectStream(runID: runID, afterID: attach.latest_event_id)
        } catch {
            connectStatusMessage = "Attach failed. \(error.localizedDescription)"
        }
    }

    private func startConnectStream(runID: String, afterID: Int?) {
        connectStreamTask?.cancel()
        connectStreaming = true
        connectStreamTask = Task {
            do {
                for try await event in store.client.streamConnectInstanceEvents(
                    id: runID,
                    afterID: afterID,
                    limit: 200,
                    pollMS: 500,
                    maxEvents: nil
                ) {
                    await MainActor.run {
                        connectEvents.append(event)
                        if connectEvents.count > 500 {
                            connectEvents.removeFirst(connectEvents.count - 500)
                        }
                    }
                }
                await MainActor.run {
                    connectStreaming = false
                    connectStatusMessage = "Stream ended."
                    connectStreamTask = nil
                }
            } catch {
                if Task.isCancelled {
                    await MainActor.run {
                        connectStreaming = false
                        connectStatusMessage = "Stream stopped."
                        connectStreamTask = nil
                    }
                    return
                }
                await MainActor.run {
                    connectStreaming = false
                    connectStatusMessage = "Stream failed. \(error.localizedDescription)"
                    connectStreamTask = nil
                }
            }
        }
    }

    private func stopConnectStream() {
        connectStreamTask?.cancel()
        connectStreamTask = nil
        connectStreaming = false
        connectStatusMessage = "Stream stopped."
    }

    private func sendConnectInput() async {
        let runID = connectInstanceID.trimmingCharacters(in: .whitespacesAndNewlines)
        let input = connectInput
        guard !runID.isEmpty else {
            connectStatusMessage = "Enter a connect runtime id first."
            return
        }
        guard !input.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
            connectStatusMessage = "Type input before sending."
            return
        }
        guard store.isReachable else {
            connectStatusMessage = "Reconnect to Vel before sending runtime input."
            return
        }

        do {
            let ack = try await store.client.writeConnectInstanceStdin(id: runID, input: input)
            connectInput = ""
            connectStatusMessage = "Sent \(ack.accepted_bytes) bytes."
        } catch {
            connectStatusMessage = "stdin send failed. \(error.localizedDescription)"
        }
    }

    private func terminateConnectRuntime() async {
        let runID = connectInstanceID.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !runID.isEmpty else {
            connectStatusMessage = "Enter a connect runtime id first."
            return
        }
        guard store.isReachable else {
            connectStatusMessage = "Reconnect to Vel before terminating runtime."
            return
        }

        do {
            _ = try await store.client.terminateConnectInstance(id: runID)
            stopConnectStream()
            connectStatusMessage = "Runtime terminated."
        } catch {
            connectStatusMessage = "Terminate failed. \(error.localizedDescription)"
        }
    }

    private func useLatestRunningConnectInstance() async {
        guard store.isReachable else {
            connectStatusMessage = "Reconnect to Vel before resolving runtime instances."
            return
        }

        resolvingConnectInstance = true
        defer { resolvingConnectInstance = false }

        do {
            let instances = try await store.client.listConnectInstances()
            let running = sortedRunningConnectInstances(from: instances)
            connectRecentInstances = Array(running.prefix(8))
            guard let latest = running.first else {
                connectStatusMessage = "No running connect instances found."
                return
            }
            selectedConnectRecentID = latest.id
            connectInstanceID = latest.id
            connectStatusMessage = "Selected \(latest.display_name) (\(latest.id))."
        } catch {
            connectStatusMessage = "Could not resolve connect instances. \(error.localizedDescription)"
        }
    }

    private func sortedRunningConnectInstances(from instances: [ConnectInstanceData]) -> [ConnectInstanceData] {
        instances
            .filter { $0.status == "ready" }
            .sorted { (lhs, rhs) in
                let left = lhs.last_seen_at ?? ""
                let right = rhs.last_seen_at ?? ""
                if left == right {
                    return lhs.id > rhs.id
                }
                return left > right
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
    let threadID: String?
    let mergedAt: Date?

    var statusLabel: String {
        if threadID != nil {
            return "Saved in Threads"
        }
        if mergedAt != nil {
            return "Merged"
        }

        switch status {
        case "pending_review":
            return "Local draft"
        case "queued":
            return "Queued locally"
        case "capture_only":
            return "Capture queued"
        case "answered_cached":
            return "Cached reply"
        case "backend_required":
            return "Backend required"
        case "needs_clarification":
            return "Needs clarification"
        case "submitted":
            return "Sent"
        case "answered":
            return "Answered"
        default:
            return status.replacingOccurrences(of: "_", with: " ").capitalized
        }
    }

    var continuityDetail: String? {
        if threadID != nil {
            return "Canonical follow-up now lives in Threads."
        }
        if let mergedAt {
            return "Recovered into canonical state at \(formatDate(mergedAt))."
        }

        switch status {
        case "queued", "capture_only":
            return "Waiting to merge once the daemon is reachable again."
        case "answered_cached":
            return "This came from cached backend state and was not re-answered locally."
        case "backend_required":
            return "Reconnect to route this through the backend-owned voice path."
        default:
            return nil
        }
    }

    var statusColor: Color {
        if threadID != nil || mergedAt != nil {
            return .green
        }

        switch status {
        case "queued", "capture_only", "pending_review":
            return .orange
        case "backend_required", "needs_clarification":
            return .yellow
        default:
            return .secondary
        }
    }
}

private struct VoiceResponse {
    let summary: String
    let detail: String?
}

private struct VoiceContinuitySummary {
    let headline: String
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
    private let offlineStore: VelOfflineStore
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private var didSaveCurrentSession = false

    var hasTranscript: Bool {
        !transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    init(offlineStore: VelOfflineStore = VelOfflineStore()) {
        self.offlineStore = offlineStore
        super.init()
        loadHistory()
        restoreDraft()
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
        persistDraft()
    }

    func clearTranscript() {
        transcript = ""
        suggestedText = ""
        suggestedIntent = .capture
        errorMessage = nil
        offlineStore.clearVoiceDraft()
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
        offlineStore.clearVoiceDraft()
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
                status: result.historyStatus,
                threadID: result.threadID
            )
            errorMessage = result.errorMessage
            offlineStore.clearVoiceDraft()
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
            status: result.historyStatus,
            threadID: result.threadID
        )
        errorMessage = result.errorMessage
        offlineStore.clearVoiceDraft()
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
        let threadID: String?
        let errorMessage: String?
    }

    private func submitBackendVoiceTurn(
        using store: VelClientStore,
        transcript: String,
        primaryText: String,
        intent: VoiceIntent
    ) async -> VoiceSubmitResult {
        guard let appleIntent = intent.appleIntent, let operation = intent.appleOperation else {
            return VoiceSubmitResult(committedIntent: nil, historyStatus: "unsupported", threadID: nil, errorMessage: "This voice action is not supported by the Apple backend route.")
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
                    threadID: response.thread_id,
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
                threadID: nil,
                errorMessage: store.isReachable ? nil : "Voice transcript queued for sync."
            )
        case .commitmentCreate:
            let packet = voiceQuickActionPacket(
                intent: intent,
                primaryText: primaryText,
                targetID: nil
            )
            await store.createCommitment(text: packet?.text ?? primaryText)
            setResponse(
                summary: store.isReachable ? "Created commitment." : "Commitment queued for sync.",
                detail: primaryText
            )
            return VoiceSubmitResult(
                committedIntent: .commitment,
                historyStatus: historyStatus(for: intent, isReachable: store.isReachable),
                threadID: nil,
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
                threadID: nil,
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
                threadID: nil,
                errorMessage: fallbackErrorMessage(prefix: "Transcript capture queued for sync.", underlyingError: underlyingError)
            )
        case .commitmentDone:
            let target = primaryText.trimmingCharacters(in: .whitespacesAndNewlines)
            guard !target.isEmpty else {
                setResponse(summary: "Commitment target is missing.", detail: "Try phrasing like “mark meds done.”")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "needs_clarification",
                    threadID: nil,
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
                    threadID: nil,
                    errorMessage: fallbackErrorMessage(prefix: "No local commitment match for offline queueing.", underlyingError: underlyingError)
                )
            }
            if isAmbiguousTopMatch(matches) {
                let options = matches.prefix(3).map { $0.commitment.text }.joined(separator: " | ")
                setResponse(summary: "Ambiguous commitment target.", detail: "Could match: \(options)")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "needs_clarification",
                    threadID: nil,
                    errorMessage: fallbackErrorMessage(prefix: "Commitment target was ambiguous.", underlyingError: underlyingError)
                )
            }
            if let packet = voiceQuickActionPacket(
                intent: intent,
                primaryText: primaryText,
                targetID: best.id
            ) {
                applyVoiceQuickActionPacket(packet, using: store.offlineStore)
            } else {
                store.offlineStore.enqueueCommitmentDone(id: best.id)
            }
            await store.refresh()
            setResponse(summary: "Commitment completion queued.", detail: best.text)
            return VoiceSubmitResult(
                committedIntent: .commitmentDone,
                historyStatus: "queued",
                threadID: nil,
                errorMessage: fallbackErrorMessage(prefix: "Commitment completion queued for backend replay.", underlyingError: underlyingError)
            )
        case .nudgeDone:
            guard let nudgeID = firstActionableNudgeID(from: store.nudges) else {
                setResponse(summary: "No active nudge found.", detail: "Transcript capture was queued for sync.")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "capture_only",
                    threadID: nil,
                    errorMessage: fallbackErrorMessage(prefix: "No active nudge available for offline queueing.", underlyingError: underlyingError)
                )
            }
            if let packet = voiceQuickActionPacket(
                intent: intent,
                primaryText: primaryText,
                targetID: nudgeID
            ) {
                applyVoiceQuickActionPacket(packet, using: store.offlineStore)
            } else {
                store.offlineStore.enqueueNudgeDone(id: nudgeID)
            }
            await store.refresh()
            setResponse(summary: "Top nudge resolution queued.", detail: nil)
            return VoiceSubmitResult(
                committedIntent: .nudgeDone,
                historyStatus: "queued",
                threadID: nil,
                errorMessage: fallbackErrorMessage(prefix: "Top nudge resolution queued for backend replay.", underlyingError: underlyingError)
            )
        case .nudgeSnooze:
            guard let nudgeID = firstActionableNudgeID(from: store.nudges) else {
                setResponse(summary: "No active nudge found.", detail: "Transcript capture was queued for sync.")
                return VoiceSubmitResult(
                    committedIntent: nil,
                    historyStatus: "capture_only",
                    threadID: nil,
                    errorMessage: fallbackErrorMessage(prefix: "No active nudge available for offline queueing.", underlyingError: underlyingError)
                )
            }
            let minutes = intent.minutes ?? 10
            if let packet = voiceQuickActionPacket(
                intent: intent,
                primaryText: primaryText,
                targetID: nudgeID
            ) {
                applyVoiceQuickActionPacket(packet, using: store.offlineStore)
            } else {
                store.offlineStore.enqueueNudgeSnooze(id: nudgeID, minutes: minutes)
            }
            await store.refresh()
            setResponse(summary: "Top nudge snooze queued.", detail: "\(minutes) minutes")
            return VoiceSubmitResult(
                committedIntent: .nudgeSnooze(minutes),
                historyStatus: "queued",
                threadID: nil,
                errorMessage: fallbackErrorMessage(prefix: "Top nudge snooze queued for backend replay.", underlyingError: underlyingError)
            )
        case .morningBriefing, .currentSchedule, .queryNextCommitment:
            let cached = offlineCachedScheduleResponse(for: intent, offlineStore: store.offlineStore)
            setResponse(cached)
            return VoiceSubmitResult(
                committedIntent: intent,
                historyStatus: cached.summary.contains("Unavailable") ? "backend_required" : "answered_cached",
                threadID: nil,
                errorMessage: fallbackErrorMessage(prefix: "Showing cached backend schedule state only.", underlyingError: underlyingError)
            )
        case .behaviorSummary:
            let cached = offlineCachedBehaviorResponse(offlineStore: store.offlineStore)
            setResponse(cached)
            return VoiceSubmitResult(
                committedIntent: intent,
                historyStatus: cached.summary.contains("Unavailable") ? "backend_required" : "answered_cached",
                threadID: nil,
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
                threadID: nil,
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

    private func voiceQuickActionPacket(
        intent: VoiceIntent,
        primaryText: String,
        targetID: String?
    ) -> EmbeddedVoiceQuickActionPacket? {
        appEnvironment.embeddedBridge.voiceQuickActionBridge.packageVoiceQuickAction(
            intentStorageToken: intent.storageToken,
            primaryText: primaryText,
            targetID: targetID,
            minutes: intent.minutes
        )
    }

    private func applyVoiceQuickActionPacket(_ packet: EmbeddedVoiceQuickActionPacket, using offlineStore: VelOfflineStore) {
        guard let kind = QueuedAction.Kind(rawValue: packet.queueKind) else { return }

        switch kind {
        case .captureCreate:
            offlineStore.enqueueCaptureCreate(text: packet.text ?? "")
        case .commitmentCreate:
            offlineStore.enqueueCommitmentCreate(text: packet.text ?? "")
        case .commitmentDone:
            guard let targetID = packet.targetID else { return }
            offlineStore.enqueueCommitmentDone(id: targetID)
        case .nudgeDone:
            guard let targetID = packet.targetID else { return }
            offlineStore.enqueueNudgeDone(id: targetID)
        case .nudgeSnooze:
            guard let targetID = packet.targetID else { return }
            offlineStore.enqueueNudgeSnooze(id: targetID, minutes: packet.minutes ?? 10)
        }
    }

    private func refreshBackendCaches(using store: VelClientStore) async {
        if let now = try? await store.client.now() {
            store.offlineStore.saveCachedNow(now)
        }
        if let morning = try? await store.client.activeDailyLoopSession(
            sessionDate: sessionDateForApple(),
            phase: .morningOverview
        ) {
            store.offlineStore.saveCachedDailyLoopSession(morning)
        } else {
            store.offlineStore.clearCachedDailyLoopSession(phase: .morningOverview)
        }
        if let standup = try? await store.client.activeDailyLoopSession(
            sessionDate: sessionDateForApple(),
            phase: .standup
        ) {
            store.offlineStore.saveCachedDailyLoopSession(standup)
        } else {
            store.offlineStore.clearCachedDailyLoopSession(phase: .standup)
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
        let threadParts = response.thread_id.map { _ in ["Saved in Threads for follow-up."] } ?? []
        let detail = (detailParts + reasonParts + evidenceParts + threadParts)
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
                    ?? embeddedCachedNowSummary(from: now)
                return VoiceResponse(
                    summary: "Next event: \(next.title).",
                    detail: detail ?? now.schedule.empty_message
                )
            }
            return VoiceResponse(
                summary: now.schedule.empty_message ?? "No upcoming schedule is cached.",
                detail: embeddedCachedNowSummary(from: now) ?? now.reasons.first
            )
        case .queryNextCommitment:
            if let next = now.tasks.next_commitment {
                return VoiceResponse(
                    summary: "Next commitment: \(next.text).",
                    detail: next.due_at ?? embeddedCachedNowSummary(from: now)
                )
            }
            return VoiceResponse(
                summary: "No next commitment is cached.",
                detail: embeddedCachedNowSummary(from: now) ?? now.schedule.empty_message
            )
        default:
            return VoiceResponse(
                summary: "Unavailable offline.",
                detail: "Reconnect to fetch a backend-owned reply."
            )
        }
    }

    private func embeddedCachedNowSummary(from now: NowData) -> String? {
        guard appEnvironment.embeddedBridge.configuration.permits(.cachedNowHydration) else {
            return nil
        }
        let snapshot = VelContextSnapshot(
            mode: now.summary.mode_label,
            nextEventTitle: now.schedule.next_event?.title,
            nudgeCount: now.attention.total_count
        )
        let parts = appEnvironment.embeddedBridge.nowBridge.hydrateCachedNowSummary(from: snapshot)
            .filter { !$0.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty }
        guard !parts.isEmpty else { return nil }
        return parts.joined(separator: " · ")
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
        let trimmedTranscript = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmedTranscript.isEmpty else { return "" }
        return appEnvironment.embeddedBridge.voiceCaptureBridge.prepareVoiceCapturePayload(
            transcript: trimmedTranscript,
            intentStorageToken: intent.storageToken
        )
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
            status: "pending_review",
            threadID: nil
        )
        persistDraft()
    }

    private func appendHistoryEntry(
        transcript: String,
        suggestedIntent: VoiceIntent,
        committedIntent: VoiceIntent?,
        status: String,
        threadID: String?
    ) {
        let normalizedTranscript = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !normalizedTranscript.isEmpty else { return }

        if let first = history.first,
           first.transcript == normalizedTranscript,
           first.status == "pending_review"
        {
            history[0] = VoiceCaptureEntry(
                id: first.id,
                createdAt: first.createdAt,
                transcript: normalizedTranscript,
                suggestedIntent: suggestedIntent,
                committedIntent: committedIntent,
                status: status,
                threadID: threadID ?? first.threadID,
                mergedAt: first.mergedAt
            )
        } else {
            let entry = VoiceCaptureEntry(
                id: UUID(),
                createdAt: Date(),
                transcript: normalizedTranscript,
                suggestedIntent: suggestedIntent,
                committedIntent: committedIntent,
                status: status,
                threadID: threadID,
                mergedAt: nil
            )
            history.insert(entry, at: 0)
            history = Array(history.prefix(40))
        }
        saveHistory()
    }

    private func firstActionableNudgeID(from nudges: [NudgeData]) -> String? {
        nudges.first(where: { $0.state == "active" || $0.state == "snoozed" })?.nudge_id
    }

    private func loadHistory() {
        history = offlineStore.cachedVoiceContinuityHistory().compactMap { entry in
            guard let suggestedIntent = VoiceIntent(storageToken: entry.suggested_intent) else {
                return nil
            }
            let committedIntent = entry.committed_intent.flatMap(VoiceIntent.init(storageToken:))
            return VoiceCaptureEntry(
                id: entry.id,
                createdAt: entry.created_at,
                transcript: entry.transcript,
                suggestedIntent: suggestedIntent,
                committedIntent: committedIntent,
                status: entry.status,
                threadID: entry.thread_id,
                mergedAt: entry.merged_at
            )
        }
    }

    private func saveHistory() {
        let persisted = history.map { entry in
            AppleVoiceContinuityEntryData(
                id: entry.id,
                created_at: entry.createdAt,
                transcript: entry.transcript,
                suggested_intent: entry.suggestedIntent.storageToken,
                committed_intent: entry.committedIntent?.storageToken,
                status: entry.status,
                thread_id: entry.threadID,
                merged_at: entry.mergedAt
            )
        }
        offlineStore.saveVoiceContinuityHistory(persisted)
    }

    func reconcileRecoveryState(using store: VelClientStore) {
        guard store.isReachable, store.pendingActionCount == 0 else { return }

        var didChange = false
        history = history.map { entry in
            guard entry.threadID == nil, entry.mergedAt == nil else { return entry }
            guard entry.status == "queued" || entry.status == "capture_only" else { return entry }

            didChange = true
            return VoiceCaptureEntry(
                id: entry.id,
                createdAt: entry.createdAt,
                transcript: entry.transcript,
                suggestedIntent: entry.suggestedIntent,
                committedIntent: entry.committedIntent,
                status: entry.status,
                threadID: nil,
                mergedAt: Date()
            )
        }

        if didChange {
            saveHistory()
        }
    }

    func continuitySummary(using store: VelClientStore) -> VoiceContinuitySummary? {
        if offlineStore.cachedVoiceDraft() != nil {
            return VoiceContinuitySummary(
                headline: "Voice draft ready to resume.",
                detail: "Your latest local transcript is still on device and can be resumed without reopening a separate thread."
            )
        }

        if let threaded = history.first(where: { $0.threadID != nil }) {
            return VoiceContinuitySummary(
                headline: "Voice follow-up saved in Threads.",
                detail: threaded.transcript
            )
        }

        let pendingRecovery = history.filter { $0.mergedAt == nil && ($0.status == "queued" || $0.status == "capture_only") }
        if !pendingRecovery.isEmpty {
            let detail = store.isReachable
                ? "Local voice recovery is waiting on canonical replay."
                : "Reconnect to merge \(pendingRecovery.count) local voice entr\(pendingRecovery.count == 1 ? "y" : "ies") back into canonical state."
            return VoiceContinuitySummary(
                headline: "Voice recovery pending.",
                detail: detail
            )
        }

        if let merged = history.first(where: { $0.mergedAt != nil }) {
            return VoiceContinuitySummary(
                headline: "Local voice recovery merged.",
                detail: merged.transcript
            )
        }

        return nil
    }

    private func restoreDraft() {
        guard let draft = offlineStore.cachedVoiceDraft(),
              !draft.transcript.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
        else {
            return
        }

        transcript = draft.transcript
        suggestedText = draft.suggested_text
        suggestedIntent = VoiceIntent(storageToken: draft.suggested_intent) ?? .capture
    }

    private func persistDraft() {
        let cleanTranscript = transcript.trimmingCharacters(in: .whitespacesAndNewlines)
        if cleanTranscript.isEmpty {
            offlineStore.clearVoiceDraft()
            return
        }

        offlineStore.saveVoiceDraft(
            AppleVoiceDraftData(
                transcript: cleanTranscript,
                suggested_intent: suggestedIntent.storageToken,
                suggested_text: suggestedText.trimmingCharacters(in: .whitespacesAndNewlines)
            )
        )
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

private func sessionDateForApple(_ date: Date = .now) -> String {
    let formatter = DateFormatter()
    formatter.calendar = Calendar(identifier: .gregorian)
    formatter.locale = Locale(identifier: "en_US_POSIX")
    formatter.dateFormat = "yyyy-MM-dd"
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

private extension View {
    @ViewBuilder
    func velCompactListStyle() -> some View {
        if #available(iOS 26.0, *) {
            self
                .listStyle(.plain)
                .listSectionSpacing(.compact)
                .contentMargins(.top, 0, for: .scrollContent)
                .environment(\.defaultMinListRowHeight, 44)
                .environment(\.defaultMinListHeaderHeight, 28)
        } else if #available(iOS 17.0, *) {
            self
                .listStyle(.plain)
                .listSectionSpacing(.compact)
                .contentMargins(.top, 0, for: .scrollContent)
                .environment(\.defaultMinListRowHeight, 44)
                .environment(\.defaultMinListHeaderHeight, 28)
        } else {
            self
                .listStyle(.plain)
                .environment(\.defaultMinListRowHeight, 44)
                .environment(\.defaultMinListHeaderHeight, 28)
        }
    }

    @ViewBuilder
    func velLiquidGlassContainer() -> some View {
        if #available(iOS 26.0, *) {
            GlassEffectContainer {
                self
                    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        } else {
            self
        }
    }

    @ViewBuilder
    func velActionButtonStyle() -> some View {
        if #available(iOS 26.0, *) {
            self.buttonStyle(.glass)
        } else {
            self.buttonStyle(.bordered)
        }
    }

    @ViewBuilder
    func velProminentActionButtonStyle() -> some View {
        if #available(iOS 26.0, *) {
            self.buttonStyle(.glassProminent)
        } else {
            self.buttonStyle(.borderedProminent)
        }
    }
}

#Preview {
    ContentView(
        appEnvironment: VelAppEnvironment.bootstrap(
            capabilities: FeatureCapabilityMapper.currentIOSDevice()
        )
    )
        .environmentObject(VelClientStore())
}
