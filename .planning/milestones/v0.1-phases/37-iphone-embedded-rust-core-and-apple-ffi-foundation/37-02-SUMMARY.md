# 37-02 Summary

## Outcome

Stood up the first embedded-capable Apple bridge seam in the shared Apple package without replacing the current HTTP-first `VelAPI` path.

## Shipped

- widened [Package.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Package.swift) with a new `VelEmbeddedBridge` target and product
- added [EmbeddedBridge.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Sources/VelEmbeddedBridge/EmbeddedBridge.swift) with:
  - embedded-safe flow vocabulary
  - explicit bridge configuration and fail-closed gating
  - narrow bridge protocols for cached `Now` hydration and quick-action preparation
  - noop bridge implementations that preserve daemon-backed current truth
- widened [FeatureCapabilities.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Sources/VelFeatureFlags/FeatureCapabilities.swift) so only iPhone currently advertises embedded-bridge capability
- widened [Services.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Sources/VelApplication/Services.swift) so the shared Apple environment can carry an embedded bridge seam without making it the default path
- updated [apple-architecture.md](/home/jove/code/vel/clients/apple/Docs/apple-architecture.md) to describe the new package-level bridge boundary

## Verification

- `make check-apple-swift`

## Notes

- this is still a scaffold slice; the bridge implementation is intentionally noop and daemon-backed by default
- no Apple app target was switched to local embedded execution yet
