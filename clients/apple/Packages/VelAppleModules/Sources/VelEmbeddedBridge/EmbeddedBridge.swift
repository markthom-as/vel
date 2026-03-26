import Foundation
import VelDomain
import VelFeatureFlags
#if canImport(Darwin)
import Darwin
#endif

public enum EmbeddedAppleFlow: String, Sendable, CaseIterable {
    case cachedNowHydration = "cached_now_hydration"
    case localQuickActionPreparation = "local_quick_action_preparation"
    case offlineRequestPackaging = "offline_request_packaging"
    case deterministicDomainHelpers = "deterministic_domain_helpers"
    case localThreadDraftPackaging = "local_thread_draft_packaging"
}

public struct EmbeddedBridgeRuntimeStatus: Sendable {
    public let resolvedSource: String?
    public let attemptedPaths: [String]
    public let freeBufferAvailable: Bool
    public let cachedNowHydrationSymbolAvailable: Bool
    public let localQuickActionPreparationSymbolAvailable: Bool
    public let offlineRequestPackagingSymbolAvailable: Bool
    public let deterministicDomainHelpersSymbolAvailable: Bool
    public let localThreadDraftPackagingSymbolAvailable: Bool

    public init(
        resolvedSource: String?,
        attemptedPaths: [String],
        freeBufferAvailable: Bool,
        cachedNowHydrationSymbolAvailable: Bool,
        localQuickActionPreparationSymbolAvailable: Bool,
        offlineRequestPackagingSymbolAvailable: Bool,
        deterministicDomainHelpersSymbolAvailable: Bool,
        localThreadDraftPackagingSymbolAvailable: Bool
    ) {
        self.resolvedSource = resolvedSource
        self.attemptedPaths = attemptedPaths
        self.freeBufferAvailable = freeBufferAvailable
        self.cachedNowHydrationSymbolAvailable = cachedNowHydrationSymbolAvailable
        self.localQuickActionPreparationSymbolAvailable = localQuickActionPreparationSymbolAvailable
        self.offlineRequestPackagingSymbolAvailable = offlineRequestPackagingSymbolAvailable
        self.deterministicDomainHelpersSymbolAvailable = deterministicDomainHelpersSymbolAvailable
        self.localThreadDraftPackagingSymbolAvailable = localThreadDraftPackagingSymbolAvailable
    }

    public static let unavailable = EmbeddedBridgeRuntimeStatus(
        resolvedSource: nil,
        attemptedPaths: [],
        freeBufferAvailable: false,
        cachedNowHydrationSymbolAvailable: false,
        localQuickActionPreparationSymbolAvailable: false,
        offlineRequestPackagingSymbolAvailable: false,
        deterministicDomainHelpersSymbolAvailable: false,
        localThreadDraftPackagingSymbolAvailable: false
    )

    public var isBridgeLoaded: Bool {
        resolvedSource != nil && freeBufferAvailable
    }

    public var hasUsableSymbols: Bool {
        cachedNowHydrationSymbolAvailable
            || localQuickActionPreparationSymbolAvailable
            || offlineRequestPackagingSymbolAvailable
            || deterministicDomainHelpersSymbolAvailable
            || localThreadDraftPackagingSymbolAvailable
    }

    public var isOperational: Bool {
        isBridgeLoaded && hasUsableSymbols
    }

    public var discoveredSymbolCount: Int {
        [cachedNowHydrationSymbolAvailable, localQuickActionPreparationSymbolAvailable, offlineRequestPackagingSymbolAvailable, deterministicDomainHelpersSymbolAvailable, localThreadDraftPackagingSymbolAvailable]
            .filter(\.self)
            .count
    }

    public func symbolAvailable(for flow: EmbeddedAppleFlow) -> Bool {
        switch flow {
        case .cachedNowHydration:
            cachedNowHydrationSymbolAvailable
        case .localQuickActionPreparation:
            localQuickActionPreparationSymbolAvailable
        case .offlineRequestPackaging:
            offlineRequestPackagingSymbolAvailable
        case .deterministicDomainHelpers:
            deterministicDomainHelpersSymbolAvailable
        case .localThreadDraftPackaging:
            localThreadDraftPackagingSymbolAvailable
        }
    }
}

public struct EmbeddedBridgeConfiguration: Sendable {
    public let isBridgeAvailableInBuild: Bool
    public let mode: EmbeddedRuntimeMode
    public let target: EmbeddedRuntimeTarget
    public let approvedFlows: Set<EmbeddedAppleFlow>

    public init(
        isBridgeAvailableInBuild: Bool,
        mode: EmbeddedRuntimeMode,
        target: EmbeddedRuntimeTarget,
        approvedFlows: Set<EmbeddedAppleFlow>
    ) {
        self.isBridgeAvailableInBuild = isBridgeAvailableInBuild
        self.mode = mode
        self.target = target
        self.approvedFlows = approvedFlows
    }

    public func permits(_ flow: EmbeddedAppleFlow) -> Bool {
        isBridgeAvailableInBuild
            && mode == .embeddedCapable
            && target == .iphoneOnly
            && approvedFlows.contains(flow)
    }

    public static func daemonBackedDefault() -> EmbeddedBridgeConfiguration {
        EmbeddedBridgeConfiguration(
            isBridgeAvailableInBuild: false,
            mode: .daemonBacked,
            target: .iphoneOnly,
            approvedFlows: []
        )
    }
}

public protocol EmbeddedNowBridge: Sendable {
    func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String]
}

public protocol EmbeddedQuickActionBridge: Sendable {
    func prepareQuickCapture(_ text: String) -> String
}

public protocol EmbeddedOfflineRequestBridge: Sendable {
    func packageOfflineRequest(_ payload: String) -> String
}

public protocol EmbeddedDomainHelpersBridge: Sendable {
    func normalizeDomainHint(_ input: String) -> String
}

public struct EmbeddedThreadDraftPacket: Sendable {
    public let payload: String
    public let requestedConversationID: String?

    public init(payload: String, requestedConversationID: String?) {
        self.payload = payload
        self.requestedConversationID = requestedConversationID
    }
}

public protocol EmbeddedThreadDraftBridge: Sendable {
    func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket
}

private struct OfflineBridgeEnvelope: Decodable {
    let kind: String?
    let payload: String?
}

public protocol EmbeddedBridgeSurface: Sendable {
    var configuration: EmbeddedBridgeConfiguration { get }
    var runtimeStatus: EmbeddedBridgeRuntimeStatus { get }
    var nowBridge: any EmbeddedNowBridge { get }
    var quickActionBridge: any EmbeddedQuickActionBridge { get }
    var offlineRequestBridge: any EmbeddedOfflineRequestBridge { get }
    var domainHelpersBridge: any EmbeddedDomainHelpersBridge { get }
    var threadDraftBridge: any EmbeddedThreadDraftBridge { get }
}

public struct NoopEmbeddedNowBridge: EmbeddedNowBridge {
    public init() {}

    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        [
            "Mode: \(context.mode ?? "unknown")",
            "Next: \(context.nextEventTitle ?? "none")",
            "Nudges: \(context.nudgeCount)"
        ]
    }
}

public struct NoopEmbeddedQuickActionBridge: EmbeddedQuickActionBridge {
    public init() {}

    public func prepareQuickCapture(_ text: String) -> String {
        text.trimmingCharacters(in: .whitespacesAndNewlines)
    }
}

public struct NoopEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge {
    public init() {}

    public func packageOfflineRequest(_ payload: String) -> String {
        guard let data = payload.data(using: .utf8),
              let envelope = try? JSONDecoder().decode(OfflineBridgeEnvelope.self, from: data),
              let envelopePayload = envelope.payload else {
            return payload.trimmingCharacters(in: .whitespacesAndNewlines)
        }
        return envelopePayload
    }
}

public struct NoopEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge {
    public init() {}

    public func normalizeDomainHint(_ input: String) -> String {
        input
            .trimmingCharacters(in: .whitespacesAndNewlines)
            .lowercased()
    }
}

public struct NoopEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge {
    public init() {}

    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        let normalizedConversationID = conversationID?
            .trimmingCharacters(in: .whitespacesAndNewlines)
        return EmbeddedThreadDraftPacket(
            payload: text.trimmingCharacters(in: .whitespacesAndNewlines),
            requestedConversationID: normalizedConversationID?.isEmpty == true ? nil : normalizedConversationID
        )
    }
}

public struct NoopEmbeddedBridgeSurface: EmbeddedBridgeSurface {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge

    public init(configuration: EmbeddedBridgeConfiguration = .daemonBackedDefault()) {
        self.configuration = configuration
        self.runtimeStatus = .unavailable
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
    }
}

#if canImport(Darwin)
private typealias VelEmbeddedCachedNowSummaryFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareQuickCaptureFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPackageOfflineRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedNormalizeDomainHelpersFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareThreadDraftFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedFreeBufferFn = @convention(c) (UnsafeMutablePointer<CChar>?) -> Void

private struct VelEmbeddedRustBindings: @unchecked Sendable {
    let handle: UnsafeMutableRawPointer
    let cachedNowSummary: VelEmbeddedCachedNowSummaryFn?
    let prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn?
    let packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn?
    let normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn?
    let prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn?
    let freeBuffer: VelEmbeddedFreeBufferFn
}

private enum VelEmbeddedRustBridge {
    private static let symbolNames = (
        cachedNowSummary: "vel_embedded_cached_now_summary",
        prepareQuickCapture: "vel_embedded_prepare_quick_capture",
        packageOfflineRequest: "vel_embedded_package_offline_request",
        normalizeDomainHelpers: "vel_embedded_normalize_domain_helpers",
        prepareThreadDraft: "vel_embedded_prepare_thread_draft",
        freeBuffer: "vel_embedded_free_buffer"
    )

    private typealias BindingResolution = (
        bindings: VelEmbeddedRustBindings?,
        status: EmbeddedBridgeRuntimeStatus
    )

    static let resolution: BindingResolution = {
        let flags = RTLD_NOW | RTLD_LOCAL
        var attemptedPaths: [String] = []

        func makeStatus(
            source: String?,
            freeBuffer: VelEmbeddedFreeBufferFn?,
            cachedNowSummary: VelEmbeddedCachedNowSummaryFn?,
            prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn?,
            packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn?,
            normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn?,
            prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn?
        ) -> EmbeddedBridgeRuntimeStatus {
            EmbeddedBridgeRuntimeStatus(
                resolvedSource: source,
                attemptedPaths: attemptedPaths,
                freeBufferAvailable: freeBuffer != nil,
                cachedNowHydrationSymbolAvailable: cachedNowSummary != nil,
                localQuickActionPreparationSymbolAvailable: prepareQuickCapture != nil,
                offlineRequestPackagingSymbolAvailable: packageOfflineRequest != nil,
                deterministicDomainHelpersSymbolAvailable: normalizeDomainHelpers != nil,
                localThreadDraftPackagingSymbolAvailable: prepareThreadDraft != nil
            )
        }

        func bindingsIfUsable(
            from handle: UnsafeMutableRawPointer,
            source: String
        ) -> BindingResolution {
            let freeBuffer = lookup(candidate: symbolNames.freeBuffer, from: handle)
            let cachedNowSummary: VelEmbeddedCachedNowSummaryFn? = lookup(
                candidate: symbolNames.cachedNowSummary,
                from: handle
            )
            let prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn? = lookup(
                candidate: symbolNames.prepareQuickCapture,
                from: handle
            )
            let packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn? = lookup(
                candidate: symbolNames.packageOfflineRequest,
                from: handle
            )
            let normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn? = lookup(
                candidate: symbolNames.normalizeDomainHelpers,
                from: handle
            )
            let prepareThreadDraft: VelEmbeddedPrepareThreadDraftFn? = lookup(
                candidate: symbolNames.prepareThreadDraft,
                from: handle
            )

            let status = makeStatus(
                source: freeBuffer == nil ? nil : source,
                freeBuffer: freeBuffer,
                cachedNowSummary: cachedNowSummary,
                prepareQuickCapture: prepareQuickCapture,
                packageOfflineRequest: packageOfflineRequest,
                normalizeDomainHelpers: normalizeDomainHelpers,
                prepareThreadDraft: prepareThreadDraft
            )

            guard freeBuffer != nil else {
                return (nil, status)
            }

            if cachedNowSummary == nil
                && prepareQuickCapture == nil
                && packageOfflineRequest == nil
                && normalizeDomainHelpers == nil
                && prepareThreadDraft == nil
            {
                return (nil, status)
            }

            return (
                VelEmbeddedRustBindings(
                    handle: handle,
                    cachedNowSummary: cachedNowSummary,
                    prepareQuickCapture: prepareQuickCapture,
                    packageOfflineRequest: packageOfflineRequest,
                    normalizeDomainHelpers: normalizeDomainHelpers,
                    prepareThreadDraft: prepareThreadDraft,
                    freeBuffer: freeBuffer
                ),
                status
            )
        }

        if let handle = dlopen(nil, flags) {
            attemptedPaths.append("main process")
            let primary = bindingsIfUsable(from: handle, source: "main process")
            if let primaryBindings = primary.bindings {
                return (primaryBindings, primary.status)
            }

            _ = dlclose(handle)
        }

        let candidates = resolveRustLibraryPaths()
        for candidate in candidates {
            attemptedPaths.append(candidate)
            guard let handle = dlopen(candidate, flags) else {
                continue
            }

            let discovered = bindingsIfUsable(from: handle, source: candidate)
            guard let rustBindings = discovered.bindings else {
                _ = dlclose(handle)
                continue
            }

            return (rustBindings, discovered.status)
        }

        return (
            nil,
            EmbeddedBridgeRuntimeStatus(
                resolvedSource: nil,
                attemptedPaths: attemptedPaths,
                freeBufferAvailable: false,
                cachedNowHydrationSymbolAvailable: false,
                localQuickActionPreparationSymbolAvailable: false,
                offlineRequestPackagingSymbolAvailable: false,
                deterministicDomainHelpersSymbolAvailable: false,
                localThreadDraftPackagingSymbolAvailable: false
            )
        )
    }()

    static var bindings: VelEmbeddedRustBindings? {
        resolution.bindings
    }

    static var runtimeStatus: EmbeddedBridgeRuntimeStatus {
        resolution.status
    }

    static func resolveRustLibraryPaths() -> [String] {
        var candidates = [
            "@rpath/libvel_embedded_bridge.dylib",
            "@rpath/VelEmbeddedBridge.framework/VelEmbeddedBridge",
            "libvel_embedded_bridge.dylib",
            "libvel_embedded_bridge.so",
            "/usr/lib/libvel_embedded_bridge.dylib"
        ]

        if let executableURL = Bundle.main.executableURL {
            let executableDirectory = executableURL.deletingLastPathComponent()
            candidates.append(executableDirectory.appendingPathComponent("libvel_embedded_bridge.dylib").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/libvel_embedded_bridge.dylib").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/vel_embedded_bridge.framework/vel_embedded_bridge").path)
            candidates.append(executableDirectory.appendingPathComponent("Frameworks/VelEmbeddedBridge.framework/VelEmbeddedBridge").path)
        }

        let bundlePath = Bundle.main.bundlePath
        let bundleURL = URL(fileURLWithPath: bundlePath)
        candidates.append(bundleURL.appendingPathComponent("libvel_embedded_bridge.dylib").path)
        candidates.append(bundleURL.appendingPathComponent("Frameworks/libvel_embedded_bridge.dylib").path)
        candidates.append(bundleURL.appendingPathComponent("Frameworks/VelEmbeddedBridge.framework/VelEmbeddedBridge").path)

        if let appSupport = NSSearchPathForDirectoriesInDomains(.applicationSupportDirectory, .userDomainMask, true).first {
            candidates.append((appSupport as NSString).appendingPathComponent("libvel_embedded_bridge.dylib"))
        }

        return candidates
    }

    static func lookup<T>(candidate: String, from handle: UnsafeMutableRawPointer) -> T? {
        guard let symbol = dlsym(handle, candidate) else { return nil }
        return unsafeBitCast(symbol, to: T.self)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedCachedNowSummaryFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareQuickCaptureFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPackageOfflineRequestFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedNormalizeDomainHelpersFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    static func invokeStringResultFunction(
        _ function: VelEmbeddedPrepareThreadDraftFn?,
        freeBuffer: VelEmbeddedFreeBufferFn?,
        payload: String
    ) -> String? {
        guard let function else { return nil }
        guard let result = payload.withCString({ function($0) }) else { return nil }
        defer { freeBuffer?(result) }
        return String(cString: result)
    }

    @inline(__always)
    static func encodeContextPayload(_ context: VelContextSnapshot) -> String {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.sortedKeys]
        return (try? encoder.encode(context)).flatMap { String(data: $0, encoding: .utf8) } ?? "{}"
    }

    @inline(__always)
    static func splitSummary(_ value: String) -> [String] {
        guard let data = value.data(using: .utf8) else { return [] }
        return (try? JSONDecoder().decode([String].self, from: data)) ?? [value]
    }

    struct OfflineRequestPacket: Decodable {
        let kind: String
        let payload: String
        let ready: Bool
        let reason: String?
    }

    struct DomainHintPacket: Decodable {
        let normalized: String
        let kind: String?
        let ready: Bool?
    }

    struct ThreadDraftInput: Encodable {
        let text: String
        let requestedConversationID: String?

        enum CodingKeys: String, CodingKey {
            case text
            case requestedConversationID = "requestedConversationId"
        }
    }

    struct ThreadDraftPacket: Decodable {
        let payload: String
        let requestedConversationID: String?
        let ready: Bool

        enum CodingKeys: String, CodingKey {
            case payload
            case requestedConversationID = "requestedConversationId"
            case ready
        }
    }

    static func decodeOfflineRequest(_ value: String) -> OfflineRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(OfflineRequestPacket.self, from: data)
    }

    static func decodeDomainHint(_ value: String) -> DomainHintPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(DomainHintPacket.self, from: data)
    }

    static func encodeThreadDraftPayload(text: String, conversationID: String?) -> String {
        let payload = ThreadDraftInput(
            text: text,
            requestedConversationID: conversationID
        )
        guard let data = try? JSONEncoder().encode(payload),
              let value = String(data: data, encoding: .utf8) else {
            return "{\"text\":\"\"}"
        }
        return value
    }

    static func decodeThreadDraft(_ value: String) -> ThreadDraftPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(ThreadDraftPacket.self, from: data)
    }
}

public struct RustEmbeddedNowBridge: EmbeddedNowBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.cachedNowSummary != nil else { return nil }
        self.bindings = bindings
    }

    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        guard let response = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.cachedNowSummary,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeContextPayload(context)
        ) else {
            return []
        }
        return VelEmbeddedRustBridge.splitSummary(response)
    }
}

public struct RustEmbeddedQuickActionBridge: EmbeddedQuickActionBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareQuickCapture != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareQuickCapture(_ text: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareQuickCapture,
            freeBuffer: bindings.freeBuffer,
            payload: text
        ) else {
            return text
        }
        return output
    }
}

public struct RustEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.packageOfflineRequest != nil else { return nil }
        self.bindings = bindings
    }

    public func packageOfflineRequest(_ payload: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.packageOfflineRequest,
            freeBuffer: bindings.freeBuffer,
            payload: payload
        ) else {
            return payload
        }

        guard let parsed = VelEmbeddedRustBridge.decodeOfflineRequest(output),
              parsed.ready else {
            return output
        }

        return parsed.payload
    }
}

public struct RustEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.normalizeDomainHelpers != nil else { return nil }
        self.bindings = bindings
    }

    public func normalizeDomainHint(_ input: String) -> String {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.normalizeDomainHelpers,
            freeBuffer: bindings.freeBuffer,
            payload: input
        ) else {
            return input
        }

        guard let parsed = VelEmbeddedRustBridge.decodeDomainHint(output),
              parsed.ready ?? false else {
            return input
        }

        return parsed.normalized
    }
}

public struct RustEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge, @unchecked Sendable {
    private let bindings: VelEmbeddedRustBindings

    public init?(bindings: VelEmbeddedRustBindings) {
        guard bindings.prepareThreadDraft != nil else { return nil }
        self.bindings = bindings
    }

    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        guard let output = VelEmbeddedRustBridge.invokeStringResultFunction(
            bindings.prepareThreadDraft,
            freeBuffer: bindings.freeBuffer,
            payload: VelEmbeddedRustBridge.encodeThreadDraftPayload(text: text, conversationID: conversationID)
        ),
        let parsed = VelEmbeddedRustBridge.decodeThreadDraft(output),
        parsed.ready else {
            return NoopEmbeddedThreadDraftBridge().prepareThreadDraft(text, conversationID: conversationID)
        }

        return EmbeddedThreadDraftPacket(
            payload: parsed.payload,
            requestedConversationID: parsed.requestedConversationID
        )
    }
}

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        guard let bindings = VelEmbeddedRustBridge.bindings else {
            return nil
        }

        self.configuration = configuration
        self.runtimeStatus = VelEmbeddedRustBridge.runtimeStatus

        if let rustNow = RustEmbeddedNowBridge(bindings: bindings), configuration.permits(.cachedNowHydration) {
            self.nowBridge = rustNow
        } else {
            self.nowBridge = NoopEmbeddedNowBridge()
        }

        if let rustQuick = RustEmbeddedQuickActionBridge(bindings: bindings), configuration.permits(.localQuickActionPreparation) {
            self.quickActionBridge = rustQuick
        } else {
            self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        }

        if let rustOffline = RustEmbeddedOfflineRequestBridge(bindings: bindings), configuration.permits(.offlineRequestPackaging) {
            self.offlineRequestBridge = rustOffline
        } else {
            self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        }

        if let rustDomain = RustEmbeddedDomainHelpersBridge(bindings: bindings), configuration.permits(.deterministicDomainHelpers) {
            self.domainHelpersBridge = rustDomain
        } else {
            self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        }

        if let rustThreadDraft = RustEmbeddedThreadDraftBridge(bindings: bindings), configuration.permits(.localThreadDraftPackaging) {
            self.threadDraftBridge = rustThreadDraft
        } else {
            self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
        }

        let isEmbedded = configuration.permits(.cachedNowHydration)
            || configuration.permits(.localQuickActionPreparation)
            || configuration.permits(.offlineRequestPackaging)
            || configuration.permits(.deterministicDomainHelpers)
            || configuration.permits(.localThreadDraftPackaging)

        guard isEmbedded else { return nil }
    }
}
#else
public struct RustEmbeddedNowBridge: EmbeddedNowBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func hydrateCachedNowSummary(from context: VelContextSnapshot) -> [String] {
        []
    }
}

public struct RustEmbeddedQuickActionBridge: EmbeddedQuickActionBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareQuickCapture(_ text: String) -> String {
        text
    }
}

public struct RustEmbeddedOfflineRequestBridge: EmbeddedOfflineRequestBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func packageOfflineRequest(_ payload: String) -> String {
        guard let data = payload.data(using: .utf8),
              let envelope = try? JSONDecoder().decode(OfflineBridgeEnvelope.self, from: data),
              let envelopePayload = envelope.payload else {
            return payload
        }
        return envelopePayload
    }
}

public struct RustEmbeddedDomainHelpersBridge: EmbeddedDomainHelpersBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func normalizeDomainHint(_ input: String) -> String {
        input
    }
}

public struct RustEmbeddedThreadDraftBridge: EmbeddedThreadDraftBridge, @unchecked Sendable {
    public init?(bindings: ()) { return nil }
    public func prepareThreadDraft(_ text: String, conversationID: String?) -> EmbeddedThreadDraftPacket {
        NoopEmbeddedThreadDraftBridge().prepareThreadDraft(text, conversationID: conversationID)
    }
}

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let runtimeStatus: EmbeddedBridgeRuntimeStatus
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge
    public let threadDraftBridge: any EmbeddedThreadDraftBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        self.configuration = configuration
        self.runtimeStatus = .unavailable
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
        self.threadDraftBridge = NoopEmbeddedThreadDraftBridge()
        return nil
    }
}
#endif
