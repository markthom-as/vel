import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelWatchStore

    var body: some View {
        List {
            Section("Status") {
                Text(store.message)
                    .font(.caption)
                    .lineLimit(3)
                    .multilineTextAlignment(.leading)
                if store.nudgeCount > 0 {
                    Text("\(store.nudgeCount) nudge(s)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
                if let transport = store.transport {
                    Text(transport)
                        .font(.caption2)
                        .foregroundStyle(.tertiary)
                }
            }
            Section("Docs") {
                Text("Core: docs/status.md")
                    .font(.caption2)
                Text("User: docs/user/README.md")
                    .font(.caption2)
            }
        }
        .task { await store.refresh() }
        .onAppear { Task { await store.refresh() } }
    }
}
