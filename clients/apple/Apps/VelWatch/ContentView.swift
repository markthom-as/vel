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
                if store.pendingActionCount > 0 {
                    Text("Queued: \(store.pendingActionCount)")
                        .font(.caption2)
                        .foregroundStyle(.orange)
                }
                Button("Refresh") {
                    Task { await store.refresh() }
                }
            }
            Section("Now") {
                if let mode = store.mode, !mode.isEmpty {
                    Text("Mode: \(mode)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                } else {
                    Text("Mode unavailable")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                if let next = store.nextCommitmentText, !next.isEmpty {
                    Text(next)
                        .font(.caption)
                        .lineLimit(3)
                } else {
                    Text("No open commitment")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
            if store.activeNudgeID != nil {
                Section("Actions") {
                    Button("Done") {
                        Task { await store.markTopNudgeDone() }
                    }
                    Button("Snooze 10m") {
                        Task { await store.snoozeTopNudge(minutes: 10) }
                    }
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
