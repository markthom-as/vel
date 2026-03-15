import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelClientStore
    @State private var nudges: [VelAPI.NudgeData] = []
    @State private var context: VelAPI.CurrentContextData?
    @State private var loading = false

    var body: some View {
        NavigationStack {
            Group {
                if store.isReachable == false && store.errorMessage != nil {
                    VStack(spacing: 12) {
                        Image(systemName: "wifi.slash")
                            .font(.largeTitle)
                        Text("Vel daemon unreachable")
                            .font(.headline)
                        Text(store.errorMessage ?? "")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                            .multilineTextAlignment(.center)
                        Text("Run veld and set base URL in Settings if on device.")
                            .font(.caption2)
                            .foregroundStyle(.tertiary)
                    }
                    .padding()
                } else {
                    List {
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
                    }
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
        do {
            async let n: [VelAPI.NudgeData] = store.client.nudges()
            async let c: VelAPI.CurrentContextData = store.client.currentContext()
            let (nudgeList, ctx) = try await (n, c)
            await MainActor.run {
                nudges = nudgeList
                context = ctx
            }
        } catch {
            await MainActor.run { store.errorMessage = error.localizedDescription }
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
                        _ = try? await store.client.nudgeDone(id: nudge.nudge_id)
                        await onAction()
                    }
                }
                .buttonStyle(.borderedProminent)
                Button("Snooze") {
                    Task {
                        _ = try? await store.client.nudgeSnooze(id: nudge.nudge_id, minutes: 10)
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
