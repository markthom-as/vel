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
            errorMessage = appShellFeedback(
                scenario: "offline_cache_in_use",
                detail: lastError.localizedDescription
            ) ?? "Offline cache in use. \(lastError.localizedDescription)"
        } else {
            errorMessage = appShellFeedback(
                scenario: "no_reachable_endpoint"
            ) ?? "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url."
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
            errorMessage = appShellFeedback(
                scenario: "refresh_signals_failed",
                detail: error.localizedDescription
            ) ?? "Could not refresh activity feed. \(error.localizedDescription)"
        }
    }

    func markNudgeDone(id: String) async {
        await performAction(
            queuedMessage: appShellFeedback(scenario: "queued_nudge_done") ?? "Queued nudge completion for sync.",
            remote: {
                _ = try await client.nudgeDone(id: id)
            },
            queueFallback: {
                enqueuePreparedQueuedAction(kind: "nudge.done", targetID: id, text: nil, minutes: nil)
            }
        )
    }

    func snoozeNudge(id: String, minutes: Int = 10) async {
        await performAction(
            queuedMessage: appShellFeedback(scenario: "queued_nudge_snooze") ?? "Queued nudge snooze for sync.",
            remote: {
                _ = try await client.nudgeSnooze(id: id, minutes: minutes)
            },
            queueFallback: {
                enqueuePreparedQueuedAction(kind: "nudge.snooze", targetID: id, text: nil, minutes: minutes)
            }
        )
    }

    func markCommitmentDone(id: String) async {
        await performAction(
            queuedMessage: appShellFeedback(scenario: "queued_commitment_done") ?? "Queued commitment completion for sync.",
            remote: {
                _ = try await client.markCommitmentDone(id: id)
            },
            queueFallback: {
                enqueuePreparedQueuedAction(kind: "commitment.done", targetID: id, text: nil, minutes: nil)
            }
        )
    }

    func createCommitment(text: String) async {
        let preparedText = embeddedBridge.configuration.permits(.localQuickActionPreparation)
            ? embeddedBridge.quickActionBridge.prepareQuickCapture(text)
            : text

        await performAction(
            queuedMessage: appShellFeedback(scenario: "queued_commitment_create") ?? "Queued commitment for sync.",
            remote: {
                _ = try await client.createCommitment(text: preparedText)
            },
            queueFallback: {
                enqueuePreparedQueuedAction(
                    kind: "commitment.create",
                    targetID: nil,
                    text: packageOfflineRequestPayload(
                        kind: "commitment.create",
                        payload: preparedText
                    ),
                    minutes: nil
                )
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
        let queueText = packageOfflineRequestPayload(
            kind: "capture.create",
            payload: embeddedBridge.captureMetadataBridge.prepareQueuedCaptureText(
                text: preparedText,
                type: type,
                source: source
            )
        )

        await performAction(
            queuedMessage: appShellFeedback(scenario: "queued_capture_create") ?? "Queued capture for sync.",
            remote: {
                _ = try await client.createCapture(text: preparedText, type: type, source: source)
            },
            queueFallback: {
                enqueuePreparedQueuedAction(kind: "capture.create", targetID: nil, text: queueText, minutes: nil)
            }
        )
    }

    func submitAssistantEntry(
        text: String,
        conversationID: String? = nil
    ) async -> AssistantEntryResponseData? {
        let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return nil }
        let preparedDraft = embeddedBridge.configuration.permits(.localThreadDraftPackaging)
            ? embeddedBridge.threadDraftBridge.prepareThreadDraft(trimmed, conversationID: conversationID)
            : embeddedBridge.threadDraftBridge.prepareThreadDraft(
                embeddedBridge.quickActionBridge.prepareQuickCapture(trimmed),
                conversationID: conversationID
            )
        let preparedText = preparedDraft.payload
        let preparedConversationID = preparedDraft.requestedConversationID

        do {
            let response = try await client.submitAssistantEntry(text: preparedText, conversationID: preparedConversationID)
            pendingActionCount = offlineStore.pendingActionCount()
            await refresh()
            errorMessage = nil
            return response
        } catch {
            let fallbackText = embeddedBridge.assistantEntryFallbackBridge
                .prepareAssistantEntryFallback(
                    text: preparedText,
                    conversationID: preparedConversationID
                )
                .payload

            offlineStore.enqueueCaptureCreate(
                text: packageOfflineRequestPayload(
                    kind: "assistant_entry",
                    payload: embeddedBridge.captureMetadataBridge.prepareQueuedCaptureText(
                        text: fallbackText,
                        type: "assistant_entry",
                        source: "apple_ios_chat"
                    )
                )
            )
            pendingActionCount = offlineStore.pendingActionCount()
            errorMessage = appShellFeedback(
                scenario: "assistant_entry_queued"
            ) ?? "Assistant message queued for sync."
            await refresh()
            return nil
        }
    }

    func normalizeDomainHint(_ input: String) -> String {
        embeddedBridge.domainHelpersBridge.normalizeDomainHint(
            input.trimmingCharacters(in: .whitespacesAndNewlines)
        )
    }

    func normalizePairingTokenInput(_ input: String) -> String {
        embeddedBridge.linkingSettingsBridge.normalizePairingTokenInput(input)
    }

    func collectRemoteRoutes(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> [(label: String, baseURL: String)] {
        embeddedBridge.linkingSettingsBridge.collectRemoteRoutes(
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            publicBaseURL: publicBaseURL
        )
        .map { (label: $0.label, baseURL: $0.baseURL) }
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
        let packet = embeddedBridge.linkingRequestBridge.preparePairingTokenIssueRequest(
            issuedByNodeID: bootstrap.node_id,
            targetNodeID: targetWorker?.node_id,
            targetNodeDisplayName: targetWorker?.node_display_name,
            targetBaseURL: preferredRemoteBaseURL(
                syncBaseURL: targetWorker?.sync_base_url,
                tailscaleBaseURL: targetWorker?.tailscale_base_url,
                lanBaseURL: targetWorker?.lan_base_url,
                publicBaseURL: nil
            )
        )
        let request = PairingTokenIssueRequestData(
            issued_by_node_id: packet.issuedByNodeID,
            ttl_seconds: nil,
            scopes: scopes,
            target_node_id: packet.targetNodeID,
            target_node_display_name: packet.targetNodeDisplayName,
            target_base_url: packet.targetBaseURL
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
        let packet = embeddedBridge.linkingRequestBridge.preparePairingTokenRedeemRequest(
            tokenCode: tokenCode,
            nodeID: bootstrap.node_id,
            nodeDisplayName: bootstrap.node_display_name,
            transportHint: bootstrap.sync_transport,
            syncBaseURL: bootstrap.sync_base_url,
            tailscaleBaseURL: bootstrap.tailscale_base_url,
            lanBaseURL: bootstrap.lan_base_url,
            localhostBaseURL: bootstrap.localhost_base_url,
            publicBaseURL: nil
        )
        let linkedNode = try await client.redeemPairingToken(
            PairingTokenRedeemRequestData(
                token_code: packet.tokenCode,
                node_id: packet.nodeID,
                node_display_name: packet.nodeDisplayName,
                transport_hint: packet.transportHint,
                requested_scopes: requestedScopes,
                sync_base_url: packet.syncBaseURL,
                tailscale_base_url: packet.tailscaleBaseURL,
                lan_base_url: packet.lanBaseURL,
                localhost_base_url: packet.localhostBaseURL,
                public_base_url: packet.publicBaseURL
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

    private func appShellFeedback(scenario: String, detail: String? = nil) -> String? {
        embeddedBridge.appShellFeedbackBridge.prepareAppShellFeedback(
            scenario: scenario,
            detail: detail
        )?.message
    }

    private func currentDailyLoopSessionDate() -> String {
        let formatter = DateFormatter()
        formatter.calendar = Calendar(identifier: .gregorian)
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.dateFormat = "yyyy-MM-dd"
        return formatter.string(from: Date())
    }

    private struct OfflineRequestEnvelope: Codable {
        let kind: String
        let payload: String
    }

    private struct OfflineRequestPacket: Decodable {
        let kind: String
        let payload: String
        let ready: Bool
        let reason: String?
    }

    private func enqueuePreparedQueuedAction(kind: String, targetID: String?, text: String?, minutes: Int?) {
        guard let packet = embeddedBridge.queuedActionBridge.packageQueuedAction(
            kind: kind,
            targetID: targetID,
            text: text,
            minutes: minutes
        ),
        let queuedKind = QueuedAction.Kind(rawValue: packet.queueKind) else {
            return
        }

        switch queuedKind {
        case .captureCreate:
            offlineStore.enqueueCaptureCreate(text: packet.text ?? "")
        case .commitmentCreate:
            offlineStore.enqueueCommitmentCreate(text: packet.text ?? "")
        case .commitmentDone:
            if let targetID = packet.targetID {
                offlineStore.enqueueCommitmentDone(id: targetID)
            }
        case .nudgeDone:
            if let targetID = packet.targetID {
                offlineStore.enqueueNudgeDone(id: targetID)
            }
        case .nudgeSnooze:
            if let targetID = packet.targetID {
                offlineStore.enqueueNudgeSnooze(id: targetID, minutes: packet.minutes ?? 10)
            }
        }
    }

    private func packageOfflineRequestPayload(kind: String, payload: String) -> String {
        guard let encoded = try? JSONEncoder().encode(
            OfflineRequestEnvelope(kind: kind, payload: payload)
        ), let json = String(data: encoded, encoding: .utf8)
        else {
            return payload
        }

        let packaged = embeddedBridge.offlineRequestBridge.packageOfflineRequest(json)
        guard let parsed = parseOfflineRequestPacket(from: packaged), parsed.ready else {
            return payload
        }

        if !parsed.payload.isEmpty { return parsed.payload }
        if let reason = parsed.reason, !reason.isEmpty {
            return "\(payload)\noffline_request_reason:\(reason)"
        }
        return payload
    }

    private func parseOfflineRequestPacket(from value: String) -> OfflineRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(OfflineRequestPacket.self, from: data)
    }

    private func preferredRemoteBaseURL(
        syncBaseURL: String?,
        tailscaleBaseURL: String?,
        lanBaseURL: String?,
        publicBaseURL: String?
    ) -> String? {
        collectRemoteRoutes(
            syncBaseURL: syncBaseURL,
            tailscaleBaseURL: tailscaleBaseURL,
            lanBaseURL: lanBaseURL,
            publicBaseURL: publicBaseURL
        ).first?.baseURL
    }
}
