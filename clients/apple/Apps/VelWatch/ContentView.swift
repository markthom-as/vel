import SwiftUI

struct ContentView: View {
    @EnvironmentObject var store: VelWatchStore

    var body: some View {
        VStack(spacing: 8) {
            Text(store.message)
                .font(.caption)
                .lineLimit(3)
                .multilineTextAlignment(.center)
            if store.nudgeCount > 0 {
                Text("\(store.nudgeCount) nudge(s)")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
        .padding()
        .task { await store.refresh() }
        .onAppear { Task { await store.refresh() } }
    }
}
