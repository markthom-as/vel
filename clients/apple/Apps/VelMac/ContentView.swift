import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelClientStore
    @State private var nudges: [VelAPI.NudgeData] = []
    @State private var context: VelAPI.CurrentContextData?

    var body: some View {
        NavigationSplitView {
            List {
                Section("Status") {
                    Label(store.isReachable ? "Connected" : "Disconnected", systemImage: store.isReachable ? "checkmark.circle" : "xmark.circle")
                }
                Section("Context") {
                    if let ctx = context?.context {
                        if let mode = ctx.mode { Text("Mode: \(mode)") }
                        if let state = ctx.morning_state { Text("Morning: \(state)") }
                        if let meds = ctx.meds_status { Text("Meds: \(meds)") }
                    }
                }
                Section("Nudges") {
                    ForEach(nudges.filter { $0.state == "active" || $0.state == "snoozed" }) { nudge in
                        VStack(alignment: .leading) {
                            Text(nudge.message)
                            HStack {
                                Button("Done") {
                                    Task {
                                        _ = try? await store.client.nudgeDone(id: nudge.nudge_id)
                                        await load()
                                    }
                                }
                                Button("Snooze") {
                                    Task {
                                        _ = try? await store.client.nudgeSnooze(id: nudge.nudge_id, minutes: 10)
                                        await load()
                                    }
                                }
                            }
                        }
                    }
                }
            }
            .listStyle(.sidebar)
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
