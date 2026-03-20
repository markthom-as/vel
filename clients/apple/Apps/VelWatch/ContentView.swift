import SwiftUI
import VelApplication

struct ContentView: View {
    let appEnvironment: VelAppEnvironment
    @EnvironmentObject var store: VelWatchStore
    @State private var captureText = ""
    @State private var commitmentText = ""

    var body: some View {
        let linkedNodes = store.offlineStore.cachedLinkedNodes()

        List {
            Section("Now") {
                Text("Apple Watch stays summary-first. Show immediate context here; deeper setup stays on iPhone or Mac.")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
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
                Text("Role: \(appEnvironment.featureCapabilities.roleLabel)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
                Text("Linked nodes: \(linkedNodes.count)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
                if let actionTitle = store.topActionTitle, !actionTitle.isEmpty {
                    Text("Top action: \(actionTitle)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
                }
                if let lastActionStatus = store.lastActionStatus, !lastActionStatus.isEmpty {
                    Text(lastActionStatus)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
                }
                if let mode = store.mode, !mode.isEmpty {
                    Text("Mode: \(mode)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                } else {
                    Text("Mode unavailable")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                if let scheduleSummary = store.scheduleSummary, !scheduleSummary.isEmpty {
                    Text(scheduleSummary)
                        .font(.caption)
                        .lineLimit(3)
                }
                if let scheduleDetail = store.scheduleDetail, !scheduleDetail.isEmpty {
                    Text(scheduleDetail)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
                if let scheduleProposalStatus = store.scheduleProposalStatus, !scheduleProposalStatus.isEmpty {
                    Text(scheduleProposalStatus)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
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
                Button("Refresh") {
                    Task { await store.refresh() }
                }
            }
            Section("Advisories") {
                if let headline = store.behaviorHeadline, !headline.isEmpty {
                    Text(headline)
                        .font(.caption)
                        .lineLimit(3)
                } else {
                    Text("No backend behavior summary cached.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
                if let reason = store.behaviorReason, !reason.isEmpty {
                    Text(reason)
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                        .lineLimit(3)
                }
            }
            if store.activeNudgeID != nil {
                Section("Inbox actions") {
                    Button("Done") {
                        Task { await store.markTopNudgeDone() }
                    }
                    Button("Snooze 10m") {
                        Task { await store.snoozeTopNudge(minutes: 10) }
                    }
                }
            }

            Section("Quick entry") {
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

            Section("Commitments") {
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

            Section("Settings and docs") {
                Text("Core: docs/MASTER_PLAN.md")
                    .font(.caption2)
                Text("User: docs/user/daily-use.md")
                    .font(.caption2)
            }
        }
        .velWatchCompactListStyle()
        .velWatchLiquidGlassContainer()
        .velWatchActionButtonStyle()
        .task { await store.refresh() }
        .onAppear { Task { await store.refresh() } }
    }
}

private extension View {
    @ViewBuilder
    func velWatchCompactListStyle() -> some View {
        if #available(watchOS 10.0, *) {
            self
                .listStyle(.plain)
                .listSectionSpacing(.compact)
                .environment(\.defaultMinListRowHeight, 30)
                .environment(\.defaultMinListHeaderHeight, 18)
        } else {
            self
                .listStyle(.plain)
                .environment(\.defaultMinListRowHeight, 30)
                .environment(\.defaultMinListHeaderHeight, 18)
        }
    }

    @ViewBuilder
    func velWatchLiquidGlassContainer() -> some View {
        if #available(watchOS 26.0, *) {
            GlassEffectContainer {
                self
            }
        } else {
            self
        }
    }

    @ViewBuilder
    func velWatchActionButtonStyle() -> some View {
        if #available(watchOS 26.0, *) {
            self.buttonStyle(.glass)
        } else {
            self.buttonStyle(.bordered)
        }
    }
}
