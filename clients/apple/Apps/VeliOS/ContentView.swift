import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelClientStore
    @State private var nudges: [VelAPI.NudgeData] = []
    @State private var commitments: [VelAPI.CommitmentData] = []
    @State private var context: VelAPI.CurrentContextData?
    @State private var loading = false
    @State private var captureText = ""
    @State private var commitmentText = ""

    var body: some View {
        NavigationStack {
            List {
                Section("Connection") {
                    Label(
                        store.isReachable ? "Connected" : "Offline cache",
                        systemImage: store.isReachable ? "checkmark.circle" : "wifi.slash"
                    )
                    if let authority = store.authorityLabel {
                        Label(authority, systemImage: "network")
                    }
                    if let transport = store.activeTransport {
                        Text("Transport: \(transport)")
                    }
                    if let baseURL = store.activeBaseURL {
                        Text(baseURL)
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                    if store.pendingActionCount > 0 {
                        Text("Pending actions: \(store.pendingActionCount)")
                            .font(.caption)
                            .foregroundStyle(.orange)
                    }
                    if let message = store.errorMessage, !message.isEmpty {
                        Text(message)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    if !store.isReachable {
                        Text("Run veld locally or set vel_tailscale_url / vel_base_url in Settings.")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                    }
                }
                if let ctx = context?.context {
                    Section("Context") {
                        if let mode = ctx.mode { Label(mode, systemImage: "brain") }
                        if let state = ctx.morning_state { Label(state, systemImage: "sunrise") }
                        if let meds = ctx.meds_status { Label("Meds: \(meds)", systemImage: "pills") }
                    }
                }
                Section("Nudges") {
                    ForEach(nudges.filter { $0.state == "active" || $0.state == "snoozed" }) { nudge in
                        NudgeRow(nudge: nudge, store: store) { await load() }
                    }
                }
                Section("Commitments") {
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
                            .buttonStyle(.bordered)
                        }
                    }
                }
                Section("Capture") {
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
                Section("Documentation") {
                    DocumentationListView()
                }
            }
            .navigationTitle("Vel")
            .refreshable { await load() }
            .task {
                await store.checkReachability()
                await load()
            }
        }
    }

    private func load() async {
        loading = true
        defer { loading = false }
        await MainActor.run {
            nudges = store.offlineStore.cachedNudgesApplyingPendingActions()
            commitments = store.offlineStore.cachedCommitmentsApplyingPendingActions()
            context = store.offlineStore.cachedContext()
            store.pendingActionCount = store.offlineStore.pendingActionCount()
        }
        do {
            let bootstrap = try await store.client.syncBootstrap()
            store.offlineStore.hydrate(from: bootstrap)
            await MainActor.run {
                nudges = bootstrap.nudges
                commitments = bootstrap.commitments
                context = bootstrap.current_context
                store.pendingActionCount = store.offlineStore.pendingActionCount()
            }
        } catch {
            await MainActor.run {
                store.errorMessage = error.localizedDescription
                store.isReachable = false
                nudges = store.offlineStore.cachedNudgesApplyingPendingActions()
                commitments = store.offlineStore.cachedCommitmentsApplyingPendingActions()
                context = store.offlineStore.cachedContext()
                store.pendingActionCount = store.offlineStore.pendingActionCount()
            }
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

struct NudgeRow: View {
    let nudge: VelAPI.NudgeData
    let store: VelClientStore
    let onAction: () async -> Void
    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(nudge.message)
                .font(.subheadline)
            Text("\(nudge.nudge_type) · \(nudge.level)")
                .font(.caption)
                .foregroundStyle(.secondary)
            HStack {
                Button("Done") {
                    Task {
                        await store.markNudgeDone(id: nudge.nudge_id)
                        await onAction()
                    }
                }
                .buttonStyle(.borderedProminent)
                Button("Snooze") {
                    Task {
                        await store.snoozeNudge(id: nudge.nudge_id, minutes: 10)
                        await onAction()
                    }
                }
                .buttonStyle(.bordered)
            }
        }
        .padding(.vertical, 4)
    }
}

#Preview {
    ContentView()
        .environmentObject(VelClientStore())
}
