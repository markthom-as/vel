import SwiftUI
import VelApplePlatform
import VelApplication
import VelAPI
import VelEmbeddedBridge
#if canImport(UIKit)
import UIKit
#endif

@main
struct VelApp: App {
    @StateObject private var client: VelClientStore
    private let appEnvironment: VelAppEnvironment

    init() {
        let environment = VelAppEnvironment.bootstrap(
            capabilities: FeatureCapabilityMapper.currentIOSDevice()
        )
        self.appEnvironment = environment
        _client = StateObject(
            wrappedValue: VelClientStore(embeddedBridge: environment.embeddedBridge)
        )

#if canImport(UIKit)
        if #available(iOS 15.0, *) {
            UITableView.appearance().sectionHeaderTopPadding = 0
        }
#endif
    }

    var body: some Scene {
        WindowGroup {
            ContentView(appEnvironment: appEnvironment)
                .environmentObject(client)
                .preferredColorScheme(.dark)
                .accentColor(.orange)
        }
    }
}

@MainActor
final class VelClientStore: ObservableObject {
    let client: VelClient
    let offlineStore = VelOfflineStore()
    private let embeddedBridge: any EmbeddedBridgeSurface

    @Published var isReachable = false
    @Published var isSyncing = false
    @Published var errorMessage: String?
    @Published var activeBaseURL: String?
    @Published var activeTransport: String?
    @Published var authorityLabel: String?
    @Published var pendingActionCount = 0
    @Published var lastSyncAt: Date?
    @Published var clusterBootstrap: ClusterBootstrapData?
    @Published var clusterWorkers: ClusterWorkersData?
    @Published var linkedNodes: [LinkedNodeData] = []

    @Published var context: CurrentContextData?
    @Published var nudges: [NudgeData] = []
    @Published var commitments: [CommitmentData] = []
    @Published var signals: [SignalData] = []
    @Published var morningDailyLoop: DailyLoopSessionData?
    @Published var standupDailyLoop: DailyLoopSessionData?
    @Published var planningProfile: PlanningProfileResponseData?

    init(embeddedBridge: any EmbeddedBridgeSurface = NoopEmbeddedBridgeSurface()) {
        self.embeddedBridge = embeddedBridge
        let initial = VelEndpointResolver.candidateBaseURLs().first
            ?? URL(string: "http://127.0.0.1:4130")!
        client = VelClient(baseURL: initial)
        applyCachedState()
    }

    func refresh() async {
        isSyncing = true
        defer { isSyncing = false }

        applyCachedState()
        var lastError: Error?

        for candidate in VelEndpointResolver.candidateBaseURLs() {
            client.baseURL = candidate
            do {
                _ = try await client.health()
                _ = await offlineStore.drainQueuedActions(using: client)

                let bootstrap = try await client.syncBootstrap()
                let workers = try? await client.clusterWorkers()
                let linkedNodes = (try? await client.linkingStatus()) ?? bootstrap.linked_nodes
                let planningProfile = try? await client.planningProfile()
                let morningDailyLoop = try? await client.activeDailyLoopSession(
                    sessionDate: currentDailyLoopSessionDate(),
                    phase: .morningOverview
                )
                let standupDailyLoop = try? await client.activeDailyLoopSession(
                    sessionDate: currentDailyLoopSessionDate(),
                    phase: .standup
                )
                offlineStore.hydrate(from: bootstrap)
                offlineStore.saveCachedLinkedNodes(linkedNodes)
                if let morningDailyLoop {
                    offlineStore.saveCachedDailyLoopSession(morningDailyLoop)
                } else {
                    offlineStore.clearCachedDailyLoopSession(phase: .morningOverview)
                }
                if let standupDailyLoop {
                    offlineStore.saveCachedDailyLoopSession(standupDailyLoop)
                } else {
                    offlineStore.clearCachedDailyLoopSession(phase: .standup)
                }

                let recentSignals = try await client.signals(limit: 80)
                offlineStore.saveCachedSignals(recentSignals)

                isReachable = true
                errorMessage = nil
                activeBaseURL = candidate.absoluteString
                activeTransport = bootstrap.cluster.sync_transport
                authorityLabel = bootstrap.cluster.node_display_name
                lastSyncAt = Date()
                clusterBootstrap = bootstrap.cluster
                clusterWorkers = workers

                context = bootstrap.current_context ?? offlineStore.cachedContext()
                nudges = offlineStore.cachedNudgesApplyingPendingActions()
                commitments = offlineStore.cachedCommitmentsApplyingPendingActions()
                self.linkedNodes = linkedNodes
                self.planningProfile = planningProfile
                signals = recentSignals
                self.morningDailyLoop = morningDailyLoop
                self.standupDailyLoop = standupDailyLoop
                pendingActionCount = offlineStore.pendingActionCount()
                return
            } catch {
                lastError = error
                continue
            }
        }

        isReachable = false
        activeBaseURL = nil
        activeTransport = nil
        authorityLabel = nil
        clusterBootstrap = nil
        clusterWorkers = nil
        planningProfile = nil
        applyCachedState()

        if let lastError {
            errorMessage = "Offline cache in use. \(lastError.localizedDescription)"
        } else {
            errorMessage = "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url."
        }
    }

    func refreshSignals() async {
        guard isReachable else {
            signals = offlineStore.cachedSignals()
            return
        }

        do {
            let recentSignals = try await client.signals(limit: 80)
            offlineStore.saveCachedSignals(recentSignals)
            signals = recentSignals
            lastSyncAt = Date()
            errorMessage = nil
        } catch {
            signals = offlineStore.cachedSignals()
            errorMessage = "Could not refresh activity feed. \(error.localizedDescription)"
        }
    }

    func markNudgeDone(id: String) async {
        await performAction(
            queuedMessage: "Queued nudge completion for sync.",
            remote: {
                _ = try await client.nudgeDone(id: id)
            },
            queueFallback: {
                offlineStore.enqueueNudgeDone(id: id)
            }
        )
    }

    func snoozeNudge(id: String, minutes: Int = 10) async {
        await performAction(
            queuedMessage: "Queued nudge snooze for sync.",
            remote: {
                _ = try await client.nudgeSnooze(id: id, minutes: minutes)
            },
            queueFallback: {
                offlineStore.enqueueNudgeSnooze(id: id, minutes: minutes)
            }
        )
    }

    func markCommitmentDone(id: String) async {
        await performAction(
            queuedMessage: "Queued commitment completion for sync.",
            remote: {
                _ = try await client.markCommitmentDone(id: id)
            },
            queueFallback: {
                offlineStore.enqueueCommitmentDone(id: id)
            }
        )
    }

    func createCommitment(text: String) async {
        await performAction(
            queuedMessage: "Queued commitment for sync.",
            remote: {
                _ = try await client.createCommitment(text: text)
            },
            queueFallback: {
                offlineStore.enqueueCommitmentCreate(text: text)
            }
        )
    }

    func createCapture(
        text: String,
        type: String = "note",
        source: String = "apple"
    ) async {
        let preparedText: String
        if embeddedBridge.configuration.permits(.localQuickActionPreparation) {
            preparedText = embeddedBridge.quickActionBridge.prepareQuickCapture(text)
        } else {
            preparedText = text
        }
        await performAction(
            queuedMessage: "Queued capture for sync.",
            remote: {
                _ = try await client.createCapture(text: preparedText, type: type, source: source)
            },
            queueFallback: {
                offlineStore.enqueueCaptureCreate(
                    text: queuedCaptureText(text: preparedText, type: type, source: source)
                )
            }
        )
    }

    func submitAssistantEntry(
        text: String,
        conversationID: String? = nil
    ) async -> AssistantEntryResponseData? {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return nil }

        do {
            let response = try await client.submitAssistantEntry(text: trimmed, conversationID: conversationID)
            pendingActionCount = offlineStore.pendingActionCount()
            await refresh()
            errorMessage = nil
            return response
        } catch {
            let fallbackText = [
                "queued_assistant_entry:",
                conversationID.map { "requested_conversation_id: \($0)" },
                "",
                trimmed
            ]
                .compactMap { $0 }
                .filter { !$0.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty }
                .joined(separator: "\n")

            offlineStore.enqueueCaptureCreate(
                text: queuedCaptureText(
                    text: fallbackText,
                    type: "assistant_entry",
                    source: "apple_ios_chat"
                )
            )
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = "Assistant message queued for sync."
            await refresh()
            return nil
        }
    }

    func setBaseURLOverride(_ value: String?) {
        let trimmed = value?.trimmingCharacters(in: .whitespacesAndNewlines)
        if let trimmed, !trimmed.isEmpty {
            UserDefaults.standard.set(trimmed, forKey: "vel_base_url")
        } else {
            UserDefaults.standard.removeObject(forKey: "vel_base_url")
        }
    }

    func clearError() {
        errorMessage = nil
    }

    func issuePairingToken(
        scopes: LinkScopeData,
        targetWorker: WorkerPresenceData?
    ) async throws -> PairingTokenData {
        guard let bootstrap = clusterBootstrap else {
            throw VelClientError.apiError("Cluster bootstrap must load before issuing a pairing token.")
        }
        let request = PairingTokenIssueRequestData(
            issued_by_node_id: bootstrap.node_id,
            ttl_seconds: nil,
            scopes: scopes,
            target_node_id: targetWorker?.node_id,
            target_node_display_name: targetWorker?.node_display_name,
            target_base_url: preferredRemoteBaseURL(
                syncBaseURL: targetWorker?.sync_base_url,
                tailscaleBaseURL: targetWorker?.tailscale_base_url,
                lanBaseURL: targetWorker?.lan_base_url,
                publicBaseURL: nil
            )
        )
        let token = try await client.issuePairingToken(request)
        await refresh()
        return token
    }

    func redeemPairingToken(
        tokenCode: String,
        requestedScopes: LinkScopeData
    ) async throws -> LinkedNodeData {
        guard let bootstrap = clusterBootstrap else {
            throw VelClientError.apiError("Cluster bootstrap must load before redeeming a pairing token.")
        }
        let linkedNode = try await client.redeemPairingToken(
            PairingTokenRedeemRequestData(
                token_code: tokenCode,
                node_id: bootstrap.node_id,
                node_display_name: bootstrap.node_display_name,
                transport_hint: bootstrap.sync_transport,
                requested_scopes: requestedScopes,
                sync_base_url: bootstrap.sync_base_url,
                tailscale_base_url: bootstrap.tailscale_base_url,
                lan_base_url: bootstrap.lan_base_url,
                localhost_base_url: bootstrap.localhost_base_url,
                public_base_url: nil
            )
        )
        let updatedLinkedNodes = [linkedNode] + linkedNodes.filter { $0.node_id != linkedNode.node_id }
        self.linkedNodes = updatedLinkedNodes
        offlineStore.saveCachedLinkedNodes(updatedLinkedNodes)
        await refresh()
        return linkedNode
    }

    func revokeLinkedNode(nodeID: String) async throws {
        _ = try await client.revokeLinkedNode(nodeID: nodeID)
        let updatedLinkedNodes = linkedNodes.filter { $0.node_id != nodeID }
        linkedNodes = updatedLinkedNodes
        offlineStore.saveCachedLinkedNodes(updatedLinkedNodes)
        await refresh()
    }

    var discoveredWorkers: [WorkerPresenceData] {
        guard let bootstrap = clusterBootstrap else { return [] }
        let linkedNodeIDs = Set(linkedNodes.map(\.node_id))
        return (clusterWorkers?.workers ?? []).filter { worker in
            worker.node_id != bootstrap.node_id && !linkedNodeIDs.contains(worker.node_id)
        }
    }

    var localIncomingLinkingPrompt: LinkingPromptData? {
        guard let bootstrap = clusterBootstrap else { return nil }
        return clusterWorkers?.workers.first(where: { $0.node_id == bootstrap.node_id })?.incoming_linking_prompt
    }

    private func performAction(
        queuedMessage: String,
        remote: () async throws -> Void,
        queueFallback: () -> Void
    ) async {
        do {
            try await remote()
            pendingActionCount = offlineStore.pendingActionCount()
            await refresh()
        } catch {
            queueFallback()
            applyCachedState()
            errorMessage = queuedMessage
        }
    }

    private func applyCachedState() {
        context = offlineStore.cachedContext()
        nudges = offlineStore.cachedNudgesApplyingPendingActions()
        commitments = offlineStore.cachedCommitmentsApplyingPendingActions()
        linkedNodes = offlineStore.cachedLinkedNodes()
        signals = offlineStore.cachedSignals()
        morningDailyLoop = offlineStore.cachedDailyLoopSession(phase: .morningOverview)
        standupDailyLoop = offlineStore.cachedDailyLoopSession(phase: .standup)
        pendingActionCount = offlineStore.pendingActionCount()
    }

    private func currentDailyLoopSessionDate() -> String {
        let formatter = DateFormatter()
        formatter.calendar = Calendar(identifier: .gregorian)
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.dateFormat = "yyyy-MM-dd"
        return formatter.string(from: Date())
    }

    private func queuedCaptureText(text: String, type: String, source: String) -> String {
        let cleanType = type.trimmingCharacters(in: .whitespacesAndNewlines)
        let cleanSource = source.trimmingCharacters(in: .whitespacesAndNewlines)
        guard cleanType != "note" || cleanSource != "apple" else {
            return text
        }

        return [
            "queued_capture_metadata:",
            "requested_capture_type: \(cleanType)",
            "requested_source_device: \(cleanSource)",
            "",
            text
        ].joined(separator: "\n")
    }

    private func preferredRemoteBaseURL(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> String? {
        for value in [syncBaseURL, tailscaleBaseURL, lanBaseURL, publicBaseURL] {
            let trimmed = value?.trimmingCharacters(in: .whitespacesAndNewlines)
            if let trimmed, !trimmed.isEmpty, !trimmed.contains("127.0.0.1"), !trimmed.contains("localhost") {
                return trimmed
            }
        }
        return nil
    }
}
