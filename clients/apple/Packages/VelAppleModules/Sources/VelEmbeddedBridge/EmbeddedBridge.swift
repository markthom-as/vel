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

private struct OfflineBridgeEnvelope: Decodable {
    let kind: String?
    let payload: String?
}

public protocol EmbeddedBridgeSurface: Sendable {
    var configuration: EmbeddedBridgeConfiguration { get }
    var nowBridge: any EmbeddedNowBridge { get }
    var quickActionBridge: any EmbeddedQuickActionBridge { get }
    var offlineRequestBridge: any EmbeddedOfflineRequestBridge { get }
    var domainHelpersBridge: any EmbeddedDomainHelpersBridge { get }
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

public struct NoopEmbeddedBridgeSurface: EmbeddedBridgeSurface {
    public let configuration: EmbeddedBridgeConfiguration
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge

    public init(configuration: EmbeddedBridgeConfiguration = .daemonBackedDefault()) {
        self.configuration = configuration
        self.nowBridge = NoopEmbeddedNowBridge()
        self.quickActionBridge = NoopEmbeddedQuickActionBridge()
        self.offlineRequestBridge = NoopEmbeddedOfflineRequestBridge()
        self.domainHelpersBridge = NoopEmbeddedDomainHelpersBridge()
    }
}

#if canImport(Darwin)
private typealias VelEmbeddedCachedNowSummaryFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPrepareQuickCaptureFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedPackageOfflineRequestFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedNormalizeDomainHelpersFn = @convention(c) (UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>?
private typealias VelEmbeddedFreeBufferFn = @convention(c) (UnsafeMutablePointer<CChar>?) -> Void

private struct VelEmbeddedRustBindings: @unchecked Sendable {
    let handle: UnsafeMutableRawPointer
    let cachedNowSummary: VelEmbeddedCachedNowSummaryFn?
    let prepareQuickCapture: VelEmbeddedPrepareQuickCaptureFn?
    let packageOfflineRequest: VelEmbeddedPackageOfflineRequestFn?
    let normalizeDomainHelpers: VelEmbeddedNormalizeDomainHelpersFn?
    let freeBuffer: VelEmbeddedFreeBufferFn
}

private enum VelEmbeddedRustBridge {
    private static let symbolNames = (
        cachedNowSummary: "vel_embedded_cached_now_summary",
        prepareQuickCapture: "vel_embedded_prepare_quick_capture",
        packageOfflineRequest: "vel_embedded_package_offline_request",
        normalizeDomainHelpers: "vel_embedded_normalize_domain_helpers",
        freeBuffer: "vel_embedded_free_buffer"
    )

    static let bindings: VelEmbeddedRustBindings? = {
        let flags = RTLD_NOW | RTLD_LOCAL

        if let handle = dlopen(nil, flags) {
            guard let freeBuffer = lookup(candidate: symbolNames.freeBuffer, from: handle) else {
                return nil
            }

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

            if cachedNowSummary != nil || prepareQuickCapture != nil || packageOfflineRequest != nil || normalizeDomainHelpers != nil {
                return VelEmbeddedRustBindings(
                    handle: handle,
                    cachedNowSummary: cachedNowSummary,
                    prepareQuickCapture: prepareQuickCapture,
                    packageOfflineRequest: packageOfflineRequest,
                    normalizeDomainHelpers: normalizeDomainHelpers,
                    freeBuffer: freeBuffer
                )
            }
        }

        let candidates = resolveRustLibraryPaths()
        for candidate in candidates {
            guard let handle = dlopen(candidate, flags) else {
                continue
            }

            guard let freeBuffer = lookup(candidate: symbolNames.freeBuffer, from: handle) else {
                _ = dlclose(handle)
                continue
            }

            let bindings = VelEmbeddedRustBindings(
                handle: handle,
                cachedNowSummary: lookup(candidate: symbolNames.cachedNowSummary, from: handle),
                prepareQuickCapture: lookup(candidate: symbolNames.prepareQuickCapture, from: handle),
                packageOfflineRequest: lookup(candidate: symbolNames.packageOfflineRequest, from: handle),
                normalizeDomainHelpers: lookup(candidate: symbolNames.normalizeDomainHelpers, from: handle),
                freeBuffer: freeBuffer
            )

            if bindings.cachedNowSummary != nil
                || bindings.prepareQuickCapture != nil
                || bindings.packageOfflineRequest != nil
                || bindings.normalizeDomainHelpers != nil
            {
                return bindings
            }

            _ = dlclose(handle)
        }

        return nil
    }()

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

    static func decodeOfflineRequest(_ value: String) -> OfflineRequestPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(OfflineRequestPacket.self, from: data)
    }

    static func decodeDomainHint(_ value: String) -> DomainHintPacket? {
        guard let data = value.data(using: .utf8) else { return nil }
        return try? JSONDecoder().decode(DomainHintPacket.self, from: data)
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

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        guard let bindings = VelEmbeddedRustBridge.bindings else {
            return nil
        }

        self.configuration = configuration

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

        let isEmbedded = configuration.permits(.cachedNowHydration)
            || configuration.permits(.localQuickActionPreparation)
            || configuration.permits(.offlineRequestPackaging)
            || configuration.permits(.deterministicDomainHelpers)

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

public struct VelEmbeddedRustBridgeSurface: EmbeddedBridgeSurface, @unchecked Sendable {
    public let configuration: EmbeddedBridgeConfiguration
    public let nowBridge: any EmbeddedNowBridge
    public let quickActionBridge: any EmbeddedQuickActionBridge
    public let offlineRequestBridge: any EmbeddedOfflineRequestBridge
    public let domainHelpersBridge: any EmbeddedDomainHelpersBridge

    public init?(configuration: EmbeddedBridgeConfiguration) {
        return nil
    }
}
#endif
