import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelWatchStore
    @State private var captureText = ""
    @State private var commitmentText = ""

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
                if let lastActionStatus = store.lastActionStatus, !lastActionStatus.isEmpty {
                    Text(lastActionStatus)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
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

            Section("Quick capture") {
                Button("Meds taken") {
                    Task {
                        await store.createCapture(
                            text: "watch_check_in: meds_taken",
                            type: "watch_check_in",
                            source: "apple_watch"
                        )
                    }
                }

                Button("Start prep now") {
                    Task {
                        await store.createCapture(
                            text: "watch_check_in: prep_started",
                            type: "watch_check_in",
                            source: "apple_watch"
                        )
                    }
                }

                TextField("Quick note", text: $captureText)

                Button("Save note") {
                    let trimmed = captureText.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !trimmed.isEmpty else { return }
                    Task {
                        await store.createCapture(
                            text: trimmed,
                            type: "watch_note",
                            source: "apple_watch"
                        )
                        await MainActor.run { captureText = "" }
                    }
                }
                .disabled(captureText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            }

            Section("Quick commitment") {
                TextField("Add task", text: $commitmentText)

                Button("Add task") {
                    let trimmed = commitmentText.trimmingCharacters(in: .whitespacesAndNewlines)
                    guard !trimmed.isEmpty else { return }
                    Task {
                        await store.createCommitment(text: trimmed)
                        await MainActor.run { commitmentText = "" }
                    }
                }
                .disabled(commitmentText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            }

            Section("Docs") {
                Text("Core: docs/MASTER_PLAN.md")
                    .font(.caption2)
                Text("User: docs/user/README.md")
                    .font(.caption2)
            }
        }
        .task { await store.refresh() }
        .onAppear { Task { await store.refresh() } }
    }
}
