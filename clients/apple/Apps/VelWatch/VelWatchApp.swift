import SwiftUI

@main
struct VelWatchApp: App {
    @StateObject private var store = VelWatchStore()
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(store)
        }
    }
}

final class VelWatchStore: ObservableObject {
    let client = VelAPI.VelClient(baseURL: URL(string: "http://localhost:4242")!)
    @Published var message: String = "Vel"
    @Published var nudgeCount: Int = 0

    func refresh() async {
        do {
            let nudges = try await client.nudges()
            let active = nudges.filter { $0.state == "active" || $0.state == "snoozed" }
            await MainActor.run {
                nudgeCount = active.count
                message = active.first?.message ?? "No nudges"
            }
        } catch {
            await MainActor.run { message = "Offline" }
        }
    }
}
