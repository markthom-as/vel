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
                .tint(.orange)
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
    @Published var clusterWorkersStatusMessage: String?
    @Published var fallbackDiscoveredWorkers: [WorkerPresenceData] = []
    @Published var linkedNodes: [LinkedNodeData] = []

    @Published var context: CurrentContextData?
    @Published var nudges: [NudgeData] = []
    @Published var commitments: [CommitmentData] = []
    @Published var signals: [SignalData] = []
    @Published var morningDailyLoop: DailyLoopSessionData?
    @Published var standupDailyLoop: DailyLoopSessionData?
    @Published var planningProfile: PlanningProfileResponseData?
    @Published var operatorSettings: AppleSettingsData?
    @Published var operatorIntegrations: AppleIntegrationsData?
    @Published var integrationConnections: [AppleIntegrationConnectionData] = []

    var configuredBaseURLHint: String? {
        let defaults = UserDefaults.standard
        let keys = [
            "vel_tailscale_url",
            "vel_base_url",
            "vel_lan_base_url",
            "tailscale_base_url",
            "base_url",
            "lan_base_url",
        ]
        for key in keys {
            if let value = defaults.string(forKey: key)?.trimmingCharacters(in: .whitespacesAndNewlines),
               value.isEmpty == false {
                return value
            }
        }
        return nil
    }

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
        var attemptedCandidates: [URL] = []

        if let lastError = await refresh(using: VelEndpointResolver.candidateBaseURLs(), attemptedCandidates: &attemptedCandidates) {
            let discoveredCandidates = await VelEndpointResolver.discoverLANBaseURLs()
                .filter { candidate in
                    !attemptedCandidates.contains(candidate)
                }
            if discoveredCandidates.isEmpty == false,
               await refresh(using: discoveredCandidates, attemptedCandidates: &attemptedCandidates) == nil {
                return
            }
            applyOfflineFallback(lastError: lastError)
            return
        }
    }

    private func refresh(
        using candidates: [URL],
        attemptedCandidates: inout [URL]
    ) async -> Error? {
        var lastError: Error?

        for candidate in candidates {
            attemptedCandidates.append(candidate)
            client.baseURL = candidate
            do {
                _ = try await client.health()
                _ = await offlineStore.drainQueuedActions(using: client)

                let bootstrap = try await client.syncBootstrap()
                let workersResult = await loadClusterWorkers(
                    activeCandidate: candidate,
                    localBootstrap: bootstrap.cluster
                )
                let workers = workersResult.workers
                let linkedNodes = (try? await client.linkingStatus()) ?? bootstrap.linked_nodes
                let planningProfile = try? await client.planningProfile()
                let operatorSettings = try? await listOperatorSettings()
                let operatorIntegrations = try? await listOperatorIntegrations()
                let integrationConnections = (try? await listIntegrationConnections(includeDisabled: true)) ?? []
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
                VelEndpointResolver.saveDiscoveredBaseURLs([candidate])
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
                clusterWorkersStatusMessage = workersResult.message
                fallbackDiscoveredWorkers = workersResult.fallbackWorkers

                context = bootstrap.current_context ?? offlineStore.cachedContext()
                nudges = offlineStore.cachedNudgesApplyingPendingActions()
                commitments = offlineStore.cachedCommitmentsApplyingPendingActions()
                self.linkedNodes = linkedNodes
                self.planningProfile = planningProfile
                self.operatorSettings = operatorSettings
                self.operatorIntegrations = operatorIntegrations
                self.integrationConnections = integrationConnections
                signals = recentSignals
                self.morningDailyLoop = morningDailyLoop
                self.standupDailyLoop = standupDailyLoop
                pendingActionCount = offlineStore.pendingActionCount()
                return nil
            } catch {
                lastError = error
                continue
            }
        }

        return lastError
    }

    private func applyOfflineFallback(lastError: Error?) {
        isReachable = false
        activeBaseURL = nil
        activeTransport = nil
        authorityLabel = nil
        clusterBootstrap = nil
        clusterWorkers = nil
        clusterWorkersStatusMessage = nil
        fallbackDiscoveredWorkers = []
        planningProfile = nil
        operatorSettings = nil
        operatorIntegrations = nil
        integrationConnections = []
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

    func listConversations(archived: Bool = false, limit: Int = 40) async throws -> [AppleConversationData] {
        let archivedValue = archived ? "true" : "false"
        let path = "/api/conversations?archived=\(archivedValue)&limit=\(limit)"
        return try await authorizedGet(path, as: [AppleConversationData].self)
    }

    func listConversationMessages(
        conversationID: String,
        limit: Int = 120
    ) async throws -> [AppleMessageData] {
        let encodedID = conversationID.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? conversationID
        return try await authorizedGet("/api/conversations/\(encodedID)/messages?limit=\(limit)", as: [AppleMessageData].self)
    }

    func listOperatorSettings() async throws -> AppleSettingsData {
        try await authorizedGet("/api/settings", as: AppleSettingsData.self)
    }

    func listOperatorIntegrations() async throws -> AppleIntegrationsData {
        try await authorizedGet("/api/integrations", as: AppleIntegrationsData.self)
    }

    func listIntegrationConnections(
        family: String? = nil,
        providerKey: String? = nil,
        includeDisabled: Bool = false
    ) async throws -> [AppleIntegrationConnectionData] {
        var queryItems: [String] = []
        if let family, !family.isEmpty {
            queryItems.append("family=\(family.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? family)")
        }
        if let providerKey, !providerKey.isEmpty {
            queryItems.append("provider_key=\(providerKey.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? providerKey)")
        }
        if includeDisabled {
            queryItems.append("include_disabled=true")
        }
        let suffix = queryItems.isEmpty ? "" : "?\(queryItems.joined(separator: "&"))"
        return try await authorizedGet("/api/integrations/connections\(suffix)", as: [AppleIntegrationConnectionData].self)
    }

    func startGoogleCalendarAuth() async throws -> AppleGoogleCalendarAuthStartData {
        try await authorizedPost("/api/integrations/google-calendar/auth/start", body: EmptyJSONBody())
    }

    func disconnectGoogleCalendar() async throws {
        let integrations = try await authorizedPost("/api/integrations/google-calendar/disconnect", body: EmptyJSONBody(), as: AppleIntegrationsData.self)
        operatorIntegrations = integrations
    }

    func disconnectTodoist() async throws {
        let integrations = try await authorizedPost("/api/integrations/todoist/disconnect", body: EmptyJSONBody(), as: AppleIntegrationsData.self)
        operatorIntegrations = integrations
    }

    func syncIntegrationSource(_ source: String) async throws -> SyncResultData {
        try await authorizedPost("/v1/sync/\(source)", body: EmptyJSONBody(), as: SyncResultData.self)
    }

    func loadLlmProfileHealth(profileID: String) async throws -> AppleLlmProfileHealthData {
        let encodedID = profileID.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? profileID
        return try await authorizedGet("/api/llm/profiles/\(encodedID)/health", as: AppleLlmProfileHealthData.self)
    }

    func chooseLocalIntegrationSourcePath(integrationID: String) async throws -> AppleLocalIntegrationPathSelectionData {
        let encodedID = integrationID.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? integrationID
        return try await authorizedPost("/api/integrations/\(encodedID)/path-dialog", body: EmptyJSONBody(), as: AppleLocalIntegrationPathSelectionData.self)
    }

    func updateLlmSettings(patch: [String: JSONValue]) async throws -> AppleSettingsData {
        let payload: [String: [String: JSONValue]] = ["llm": patch]
        let settings = try await authorizedPatch("/api/settings", body: payload, as: AppleSettingsData.self)
        operatorSettings = settings
        return settings
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
            let defaults = UserDefaults.standard
            if let url = URL(string: trimmed),
               let host = url.host?.lowercased() {
                if host.hasSuffix(".ts.net") {
                    defaults.set(trimmed, forKey: "vel_tailscale_url")
                    defaults.set(trimmed, forKey: "tailscale_base_url")
                    defaults.removeObject(forKey: "vel_base_url")
                    defaults.removeObject(forKey: "base_url")
                    defaults.removeObject(forKey: "vel_lan_base_url")
                    defaults.removeObject(forKey: "lan_base_url")
                    return
                }
                if Self.isPrivateNetworkHost(host) {
                    defaults.set(trimmed, forKey: "vel_lan_base_url")
                    defaults.set(trimmed, forKey: "lan_base_url")
                    defaults.removeObject(forKey: "vel_base_url")
                    defaults.removeObject(forKey: "base_url")
                    return
                }
            }
            defaults.set(trimmed, forKey: "vel_base_url")
            defaults.set(trimmed, forKey: "base_url")
        } else {
            let defaults = UserDefaults.standard
            defaults.removeObject(forKey: "vel_tailscale_url")
            defaults.removeObject(forKey: "tailscale_base_url")
            defaults.removeObject(forKey: "vel_base_url")
            defaults.removeObject(forKey: "base_url")
            defaults.removeObject(forKey: "vel_lan_base_url")
            defaults.removeObject(forKey: "lan_base_url")
        }
    }

    func clearError() {
        errorMessage = nil
    }

    func issuePairingToken(
        scopes: LinkScopeData,
        targetWorker: WorkerPresenceData?
    ) async throws -> PairingTokenData {
        let bootstrap = try await ensureLinkingBootstrap()
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
        let bootstrap = try await ensureLinkingBootstrap()
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

    func userFacingErrorMessage(_ error: Error, context: String? = nil) -> String {
        let detail: String

        switch error {
        case let clientError as VelClientError:
            detail = clientError.errorDescription ?? clientError.description
        case let urlError as URLError:
            switch urlError.code {
            case .cannotConnectToHost:
                let host = urlError.failingURL?.host ?? activeBaseURL ?? "configured Vel endpoint"
                detail = "Could not connect to \(host). Make sure `veld` is running and that the Apple client is pointed at a reachable base URL."
            case .notConnectedToInternet:
                detail = "This device is offline. Reconnect to the network or point the app at a reachable Vel endpoint."
            case .timedOut:
                detail = "The Vel endpoint timed out before responding."
            default:
                detail = urlError.localizedDescription
            }
        default:
            detail = error.localizedDescription
        }

        guard let context, !context.isEmpty else { return detail }
        return "\(context) \(detail)"
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
        let linkedNodeIDs = Set(linkedNodes.filter { $0.status == .linked }.map(\.node_id))
        let candidates = clusterWorkers?.workers ?? fallbackDiscoveredWorkers
        return candidates.filter { worker in
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

    private func loadClusterWorkers(
        activeCandidate: URL,
        localBootstrap: ClusterBootstrapData
    ) async -> (workers: ClusterWorkersData?, fallbackWorkers: [WorkerPresenceData], message: String?) {
        do {
            return (try await client.clusterWorkers(), [], nil)
        } catch {
            let fallbackWorkers = await loadFallbackDiscoveredWorkers(
                activeCandidate: activeCandidate,
                localBootstrap: localBootstrap
            )
            return (nil, fallbackWorkers, clusterWorkersFeedback(for: error, fallbackCount: fallbackWorkers.count))
        }
    }

    private func loadFallbackDiscoveredWorkers(
        activeCandidate: URL,
        localBootstrap: ClusterBootstrapData
    ) async -> [WorkerPresenceData] {
        let candidateURLs = VelEndpointResolver.discoveredBaseURLs()
            .filter { $0 != activeCandidate }

        guard !candidateURLs.isEmpty else { return [] }

        var workers: [WorkerPresenceData] = []
        var seenNodeIDs: Set<String> = []
        let now = Int(Date().timeIntervalSince1970)

        for candidateURL in candidateURLs {
            let probeClient = VelClient(
                baseURL: candidateURL,
                configuration: client.configuration
            )
            guard let bootstrap = try? await probeClient.discoveryBootstrap() else {
                continue
            }
            guard bootstrap.node_id != localBootstrap.node_id else { continue }
            guard seenNodeIDs.insert(bootstrap.node_id).inserted else { continue }

            workers.append(
                WorkerPresenceData(
                    worker_id: "discovered:\(bootstrap.node_id)",
                    node_id: bootstrap.node_id,
                    node_display_name: bootstrap.node_display_name,
                    client_kind: nil,
                    client_version: nil,
                    protocol_version: nil,
                    build_id: nil,
                    worker_classes: [],
                    capabilities: bootstrap.capabilities ?? [],
                    status: "discovered",
                    queue_depth: 0,
                    reachability: "reachable",
                    latency_class: "unknown",
                    compute_class: "unknown",
                    power_class: "unknown",
                    recent_failure_rate: 0,
                    tailscale_preferred: bootstrap.tailscale_base_url != nil,
                    last_heartbeat_at: now,
                    started_at: nil,
                    sync_base_url: bootstrap.sync_base_url,
                    sync_transport: bootstrap.sync_transport,
                    tailscale_base_url: bootstrap.tailscale_base_url,
                    preferred_tailnet_endpoint: bootstrap.tailscale_base_url,
                    tailscale_reachable: bootstrap.tailscale_base_url != nil,
                    lan_base_url: bootstrap.lan_base_url,
                    localhost_base_url: bootstrap.localhost_base_url,
                    ping_ms: nil,
                    sync_status: "discovered_via_public_bootstrap",
                    last_upstream_sync_at: nil,
                    last_downstream_sync_at: nil,
                    last_sync_error: nil,
                    incoming_linking_prompt: nil,
                    capacity: WorkerCapacityData(
                        max_concurrency: 0,
                        current_load: 0,
                        available_concurrency: 0
                    )
                )
            )
        }

        return workers
    }

    private func clusterWorkersFeedback(for error: Error, fallbackCount: Int) -> String {
        if let clientError = error as? VelClientError {
            switch clientError {
            case let .http(statusCode, _) where statusCode == 401 || statusCode == 403:
                let hasOperatorToken = UserDefaults.standard
                    .string(forKey: "vel_operator_token")?
                    .trimmingCharacters(in: .whitespacesAndNewlines)
                    .isEmpty == false
                if fallbackCount > 0 {
                    return "Showing \(fallbackCount) companion node\(fallbackCount == 1 ? "" : "s") from public discovery. Add operator access to load full worker presence."
                }
                if hasOperatorToken {
                    return "Companion routes were discovered, but node presence was rejected by /v1/cluster/workers. Re-save Operator access and refresh."
                }
                return "Companion routes were discovered, but node presence needs operator access. Add an operator token in System -> Operator access."
            default:
                break
            }
        }

        if fallbackCount > 0 {
            return "Showing \(fallbackCount) companion node\(fallbackCount == 1 ? "" : "s") from public discovery while worker presence is unavailable."
        }

        return "Companion routes were discovered, but node presence could not load. \(error.localizedDescription)"
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

    private static func isPrivateNetworkHost(_ host: String) -> Bool {
        if host == "localhost" || host == "127.0.0.1" || host == "::1" {
            return true
        }
        let octets = host.split(separator: ".").compactMap { Int($0) }
        guard octets.count == 4 else { return false }
        switch (octets[0], octets[1]) {
        case (10, _):
            return true
        case (172, 16...31):
            return true
        case (192, 168):
            return true
        case (100, 64...127):
            return true
        default:
            return false
        }
    }

    private func authorizedGet<T: Decodable>(_ path: String, as type: T.Type) async throws -> T {
        try await authorizedRequest(path, method: "GET", body: Optional<EmptyJSONBody>.none, as: type)
    }

    private func authorizedPost<T: Decodable, B: Encodable>(
        _ path: String,
        body: B,
        as type: T.Type = T.self
    ) async throws -> T {
        try await authorizedRequest(path, method: "POST", body: body, as: type)
    }

    private func authorizedPatch<T: Decodable, B: Encodable>(
        _ path: String,
        body: B,
        as type: T.Type = T.self
    ) async throws -> T {
        try await authorizedRequest(path, method: "PATCH", body: body, as: type)
    }

    private func authorizedRequest<T: Decodable, B: Encodable>(
        _ path: String,
        method: String,
        body: B?,
        as type: T.Type
    ) async throws -> T {
        guard let url = URL(string: path, relativeTo: client.baseURL) else {
            throw VelClientError.apiError("Invalid URL path: \(path)")
        }

        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        if body != nil {
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            request.httpBody = try JSONEncoder().encode(body)
        }

        if let bearerToken = client.configuration.bearerToken?.trimmingCharacters(in: .whitespacesAndNewlines),
           !bearerToken.isEmpty {
            request.setValue("Bearer \(bearerToken)", forHTTPHeaderField: "Authorization")
        }

        if let operatorToken = client.configuration.operatorToken?.trimmingCharacters(in: .whitespacesAndNewlines),
           !operatorToken.isEmpty {
            request.setValue(operatorToken, forHTTPHeaderField: "x-vel-operator-token")
        }

        let (data, response) = try await URLSession.shared.data(for: request)
        guard let http = response as? HTTPURLResponse else {
            throw VelClientError.apiError("No response")
        }
        guard (200..<300).contains(http.statusCode) else {
            throw VelClientError.http(statusCode: http.statusCode, message: String(data: data, encoding: .utf8) ?? "")
        }

        let envelope = try JSONDecoder().decode(APIEnvelope<T>.self, from: data)
        guard envelope.ok, let payload = envelope.data else {
            throw VelClientError.apiError(envelope.error?.message ?? "No data")
        }
        return payload
    }

    private struct EmptyJSONBody: Codable {}

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

    private func ensureLinkingBootstrap() async throws -> ClusterBootstrapData {
        if let clusterBootstrap, isReachable {
            return clusterBootstrap
        }

        await refresh()

        if let clusterBootstrap, isReachable {
            return clusterBootstrap
        }

        if let errorMessage, !errorMessage.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            throw VelClientError.apiError(errorMessage)
        }

        throw VelClientError.apiError(
            "Vel is not reachable. Pairing requires a live daemon connection before it can issue or redeem a token."
        )
    }
}

struct AppleThreadContinuationData: Codable, Sendable {
    let escalation_reason: String
    let continuation_context: JSONValue
    let review_requirements: [String]
    let bounded_capability_state: String
    let continuation_category: NowHeaderBucketKindData
    let open_target: String
}

struct AppleConversationContinuationData: Codable, Sendable {
    let thread_id: String
    let thread_type: String
    let lifecycle_stage: String?
    let continuation: AppleThreadContinuationData
}

struct AppleConversationData: Codable, Sendable, Identifiable {
    let id: String
    let title: String?
    let kind: String
    let pinned: Bool
    let archived: Bool
    let call_mode_active: Bool
    let created_at: Int
    let updated_at: Int
    let message_count: Int
    let last_message_at: Int?
    let project_label: String?
    let continuation: AppleConversationContinuationData?
}

struct AppleMessageData: Codable, Sendable, Identifiable {
    let id: String
    let conversation_id: String
    let role: String
    let kind: String
    let content: JSONValue
    let status: String?
    let importance: String?
    let created_at: Int
    let updated_at: Int?
}

struct AppleSettingsData: Codable, Sendable {
    let timezone: String?
    let node_display_name: String?
    let writeback_enabled: Bool?
    let tailscale_base_url: String?
    let lan_base_url: String?
    let llm: AppleLlmSettingsData?
    let web_settings: AppleWebSettingsData?
    let core_settings: AppleCoreSettingsData?
}

struct AppleWebSettingsData: Codable, Sendable {
    let dense_rows: Bool
    let tabular_numbers: Bool
    let reduced_motion: Bool
    let strong_focus: Bool
    let docked_action_bar: Bool
}

struct AppleCoreSettingsData: Codable, Sendable {
    let user_display_name: String?
    let developer_mode: Bool
    let bypass_setup_gate: Bool
}

struct AppleLlmSettingsData: Codable, Sendable {
    let models_dir: String
    let default_chat_profile_id: String?
    let fallback_chat_profile_id: String?
    let profiles: [AppleLlmProfileSettingsData]
}

struct AppleLlmProfileSettingsData: Codable, Sendable, Identifiable {
    let id: String
    let provider: String
    let base_url: String
    let model: String
    let context_window: Int?
    let enabled: Bool
    let editable: Bool
    let has_api_key: Bool?
}

struct AppleLlmProfileHealthData: Codable, Sendable {
    let profile_id: String
    let healthy: Bool
    let message: String
}

struct AppleIntegrationGuidanceData: Codable, Sendable {
    let title: String
    let detail: String
    let action: String
}

struct AppleIntegrationCalendarData: Codable, Sendable, Identifiable {
    let id: String
    let summary: String
    let primary: Bool
    let sync_enabled: Bool
    let display_enabled: Bool
}

struct AppleGoogleCalendarIntegrationData: Codable, Sendable {
    let configured: Bool
    let connected: Bool
    let has_client_id: Bool
    let has_client_secret: Bool
    let calendars: [AppleIntegrationCalendarData]
    let all_calendars_selected: Bool
    let last_sync_at: Int?
    let last_sync_status: String?
    let last_error: String?
    let last_item_count: Int?
    let guidance: AppleIntegrationGuidanceData?
}

struct AppleTodoistWriteCapabilitiesData: Codable, Sendable {
    let completion_status: Bool
    let due_date: Bool
    let tags: Bool
}

struct AppleTodoistIntegrationData: Codable, Sendable {
    let configured: Bool
    let connected: Bool
    let has_api_token: Bool
    let last_sync_at: Int?
    let last_sync_status: String?
    let last_error: String?
    let last_item_count: Int?
    let guidance: AppleIntegrationGuidanceData?
    let write_capabilities: AppleTodoistWriteCapabilitiesData
}

struct AppleLocalIntegrationData: Codable, Sendable {
    let configured: Bool
    let source_path: String?
    let selected_paths: [String]?
    let available_paths: [String]?
    let internal_paths: [String]?
    let suggested_paths: [String]
    let source_kind: String
    let last_sync_at: Int?
    let last_sync_status: String?
    let last_error: String?
    let last_item_count: Int?
    let guidance: AppleIntegrationGuidanceData?
}

struct AppleLocalIntegrationPathSelectionData: Codable, Sendable {
    let source_path: String?
}

struct AppleIntegrationsData: Codable, Sendable {
    let google_calendar: AppleGoogleCalendarIntegrationData
    let todoist: AppleTodoistIntegrationData
    let activity: AppleLocalIntegrationData
    let health: AppleLocalIntegrationData
    let git: AppleLocalIntegrationData
    let messaging: AppleLocalIntegrationData
    let reminders: AppleLocalIntegrationData
    let notes: AppleLocalIntegrationData
    let transcripts: AppleLocalIntegrationData
}

struct AppleGoogleCalendarAuthStartData: Codable, Sendable {
    let auth_url: String
}

struct AppleIntegrationConnectionSettingRefData: Codable, Sendable, Identifiable {
    var id: String { "\(setting_key):\(created_at)" }

    let setting_key: String
    let setting_value: String
    let created_at: Int
}

struct AppleIntegrationConnectionData: Codable, Sendable, Identifiable {
    let id: String
    let family: String
    let provider_key: String
    let status: String
    let display_name: String
    let account_ref: String?
    let metadata: JSONValue
    let created_at: Int
    let updated_at: Int
    let setting_refs: [AppleIntegrationConnectionSettingRefData]
}
