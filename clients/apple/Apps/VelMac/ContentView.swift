import SwiftUI
import VelApplication
import VelAPI

struct ContentView: View {
    let appEnvironment: VelAppEnvironment
    @EnvironmentObject var store: VelClientStore
    @State private var nudges: [VelAPI.NudgeData] = []
    @State private var commitments: [VelAPI.CommitmentData] = []
    @State private var context: VelAPI.CurrentContextData?
    @State private var cachedNow: VelAPI.NowData?
    @State private var projects: [VelAPI.ProjectRecordData] = []
    @State private var linkedNodes: [VelAPI.LinkedNodeData] = []
    @State private var captureText = ""
    @State private var commitmentText = ""

    var body: some View {
        NavigationSplitView {
            List {
                Section("Now") {
                    Text(cachedNow?.header?.title ?? "Vel Now")
                    if let buckets = cachedNow?.header?.buckets, !buckets.isEmpty {
                        Text(buckets.map { "\(macNowBucketLabel($0.kind)) \($0.count)" }.joined(separator: " · "))
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    if let statusRow = cachedNow?.status_row {
                        Text("\(statusRow.date_label) · \(statusRow.time_label)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text(statusRow.context_label)
                        Text(statusRow.elapsed_label)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    if let contextLine = cachedNow?.context_line {
                        Text(contextLine.text)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    if let meshSummary = cachedNow?.mesh_summary {
                        Text("\(meshSummary.authority_label) · \(macNowMeshStateLabel(meshSummary.sync_state))")
                            .font(.caption)
                            .foregroundStyle(meshSummary.urgent ? .orange : .secondary)
                        Text("\(meshSummary.linked_node_count) linked · \(meshSummary.queued_write_count) queued")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                        if let repairRoute = meshSummary.repair_route {
                            Text(repairRoute.summary)
                                .font(.caption2)
                                .foregroundStyle(meshSummary.urgent ? .orange : .secondary)
                        }
                    } else {
                        Label(store.isReachable ? "Connected" : "Offline cache", systemImage: store.isReachable ? "checkmark.circle" : "wifi.slash")
                    }
                    if let firstBar = cachedNow?.nudge_bars?.first {
                        Text(firstBar.title)
                            .font(.caption)
                        Text(firstBar.summary)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    if let activeTask = cachedNow?.task_lane?.active {
                        Text("Active: \(activeTask.text)")
                            .font(.caption)
                        if let project = activeTask.project {
                            Text(project)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    if let message = store.errorMessage, !message.isEmpty {
                        Text(message)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    quickEntrySection
                }
                Section("Inbox") {
                    Text("This is the Mac triage lane. Urgent nudges and open commitments stay here; longer follow-through should move into the backend-owned continuity lanes.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    ForEach(nudges.filter { $0.state == "active" || $0.state == "snoozed" }) { nudge in
                        VStack(alignment: .leading) {
                            Text(nudge.message)
                            HStack {
                                Button("Done") {
                                    Task {
                                        await store.markNudgeDone(id: nudge.nudge_id)
                                        await load()
                                    }
                                }
                                Button("Snooze") {
                                    Task {
                                        await store.snoozeNudge(id: nudge.nudge_id, minutes: 10)
                                        await load()
                                    }
                                }
                            }
                        }
                    }
                    HStack {
                        TextField("Add commitment", text: $commitmentText)
                        Button("Add") {
                            let text = commitmentText.trimmingCharacters(in: .whitespacesAndNewlines)
                            guard !text.isEmpty else { return }
                            Task {
                                await store.createCommitment(text: text)
                                await MainActor.run { commitmentText = "" }
                                await load()
                            }
                        }
                    }
                    ForEach(commitments.filter { $0.status == "open" }.prefix(8), id: \.id) { commitment in
                        HStack {
                            Text(commitment.text)
                            Spacer()
                            Button("Done") {
                                Task {
                                    await store.markCommitmentDone(id: commitment.id)
                                    await load()
                                }
                            }
                        }
                    }
                }
                Section("Threads") {
                    Text("Threads stay the continuity lane on Apple too. Mac should summarize shared continuation and trust state here without inventing separate product logic.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    if let firstBar = cachedNow?.nudge_bars?.first(where: { $0.primary_thread_id != nil }) {
                        Text("Open target: \(firstBar.primary_thread_id ?? "thread-backed")")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    Text("Linked nodes: \(linkedNodes.count)")
                    if let firstLinkedNode = linkedNodes.first {
                        Text("First linked node: \(firstLinkedNode.node_display_name)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text("Status: \(firstLinkedNode.status.rawValue)")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text("Scopes: \(scopeSummary(firstLinkedNode.scopes))")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    } else {
                        Text("No linked-node continuity is cached yet.")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    planningProfileSummarySection
                }
                Section("Projects") {
                    Text("Projects are a secondary surface for durable roots and project-specific context.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    if projects.isEmpty {
                        Text("No cached projects.")
                            .foregroundStyle(.secondary)
                    }
                    ForEach(Array(projects.prefix(5)), id: \.id) { project in
                        VStack(alignment: .leading, spacing: 4) {
                            Text(project.name)
                            Text("Primary repo: \(project.primary_repo.path)")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                            Text("Notes root: \(project.primary_notes_root.path)")
                                .font(.caption2)
                                .foregroundStyle(.tertiary)
                        }
                    }
                }
                Section("Settings") {
                    Text("Settings is the support lane for trust, docs, and deeper setup. It should not compete with `Now`, `Inbox`, or `Threads` for first-contact attention.")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    DocumentationListView()
                }
            }
            .velMacCompactSidebarStyle()
            .navigationTitle("Vel")
            .refreshable { await load() }
        } detail: {
            Text("Select an item or pull to refresh.")
                .foregroundStyle(.secondary)
        }
        .task {
            await store.checkReachability()
            await load()
        }
    }

    private func load() async {
        await MainActor.run {
            nudges = store.offlineStore.cachedNudgesApplyingPendingActions()
            commitments = store.offlineStore.cachedCommitmentsApplyingPendingActions()
            context = store.offlineStore.cachedContext()
            cachedNow = store.offlineStore.cachedNow()
            projects = store.offlineStore.cachedProjects()
            linkedNodes = store.offlineStore.cachedLinkedNodes()
            store.pendingActionCount = store.offlineStore.pendingActionCount()
        }
        do {
            let bootstrap = try await store.client.syncBootstrap()
            let now = try? await store.client.now()
            store.offlineStore.hydrate(from: bootstrap)
            if let now {
                store.offlineStore.saveCachedNow(now)
            }
            await MainActor.run {
                nudges = bootstrap.nudges
                commitments = bootstrap.commitments
                context = bootstrap.current_context
                cachedNow = now ?? store.offlineStore.cachedNow()
                projects = store.offlineStore.cachedProjects()
                linkedNodes = store.offlineStore.cachedLinkedNodes()
                store.pendingActionCount = store.offlineStore.pendingActionCount()
            }
        } catch {
            await MainActor.run {
                store.errorMessage = error.localizedDescription
                store.isReachable = false
                nudges = store.offlineStore.cachedNudgesApplyingPendingActions()
                commitments = store.offlineStore.cachedCommitmentsApplyingPendingActions()
                context = store.offlineStore.cachedContext()
                cachedNow = store.offlineStore.cachedNow()
                projects = store.offlineStore.cachedProjects()
                linkedNodes = store.offlineStore.cachedLinkedNodes()
                store.pendingActionCount = store.offlineStore.pendingActionCount()
            }
        }
    }

    private func scopeSummary(_ scopes: VelAPI.LinkScopeData) -> String {
        var labels: [String] = []
        if scopes.read_context {
            labels.append("read_context")
        }
        if scopes.write_safe_actions {
            labels.append("write_safe_actions")
        }
        if scopes.execute_repo_tasks {
            labels.append("execute_repo_tasks")
        }
        return labels.isEmpty ? "none" : labels.joined(separator: ", ")
    }

    @ViewBuilder
    private var quickEntrySection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Quick entry stays a shell-native wrapper over backend-owned capture and commitment routes.")
                .font(.caption)
                .foregroundStyle(.secondary)
            HStack {
                TextField("Quick capture", text: $captureText)
                Button("Save") {
                    let text = captureText.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !text.isEmpty else { return }
                    Task {
                        await store.createCapture(text: text)
                        await MainActor.run { captureText = "" }
                        await load()
                    }
                }
            }
        }
    }

    @ViewBuilder
    private var planningProfileSummarySection: some View {
        if let planningProfile = store.planningProfile {
            let profile = planningProfile.profile
            let activeBlocks = profile.routine_blocks.filter { $0.active }.count
            let activeConstraints = profile.planning_constraints.filter { $0.active }.count
            VStack(alignment: .leading, spacing: 4) {
                Text("Planning profile")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Text("Routine blocks: \(activeBlocks) active of \(profile.routine_blocks.count)")
                    .font(.caption)
                Text("Constraints: \(activeConstraints) active of \(profile.planning_constraints.count)")
                    .font(.caption)
                if let firstBlock = profile.routine_blocks.first {
                    Text("Next profile anchor: \(firstBlock.label) \(firstBlock.start_local_time)-\(firstBlock.end_local_time)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
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
            Text("Planning profile loads from the same backend-owned routine and constraint profile used by day plan and reflow.")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }
}

private func macNowBucketLabel(_ kind: VelAPI.NowHeaderBucketKindData) -> String {
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

private func macNowMeshStateLabel(_ state: VelAPI.NowMeshSyncStateData) -> String {
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

private extension View {
    @ViewBuilder
    func velMacCompactSidebarStyle() -> some View {
        if #available(macOS 13.0, *) {
            self
                .listStyle(.sidebar)
                .environment(\.defaultMinListRowHeight, 28)
                .environment(\.defaultMinListHeaderHeight, 20)
        } else {
            self.listStyle(.sidebar)
        }
    }
}

struct DocumentationListView: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Core docs")
                .font(.caption)
                .foregroundStyle(.secondary)
            ForEach(VelAPI.VelDocumentationCatalog.core) { doc in
                VStack(alignment: .leading, spacing: 2) {
                    Text(doc.title)
                    Text(doc.path)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                    Text(doc.summary)
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                }
            }
            Text("Your Vel docs")
                .font(.caption)
                .foregroundStyle(.secondary)
                .padding(.top, 4)
            ForEach(VelAPI.VelDocumentationCatalog.user) { doc in
                VStack(alignment: .leading, spacing: 2) {
                    Text(doc.title)
                    Text(doc.path)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                    Text(doc.summary)
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                }
            }
        }
    }
}
