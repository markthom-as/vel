# 37-03 Summary

## Outcome

Routed the first bounded iPhone-local flows through the new embedded bridge seam without changing daemon-backed authority.

## Shipped

- widened [VelApp.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/VelApp.swift) so the iPhone client store now receives the shared embedded bridge from `VelAppEnvironment`
- widened [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift) so offline cached-`Now` voice/status hints use the embedded cached-summary helper when the bridge gate permits it
- widened [VelApp.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/VelApp.swift) capture submission so quick-capture text is prepared through the embedded bridge before submit/queue on iPhone
- widened [Services.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Sources/VelApplication/Services.swift) bootstrap so iPhone advertises an embedded-capable bridge configuration while other Apple surfaces stay daemon-backed/unavailable
- kept [EmbeddedBridge.swift](/home/jove/code/vel/clients/apple/Packages/VelAppleModules/Sources/VelEmbeddedBridge/EmbeddedBridge.swift) narrow and fail-closed: local helpers only, no local authority rewrite

## Verification

- `nix-shell --run 'cd /home/jove/code/vel/clients/apple/Packages/VelAppleModules && swift build'`
- `make check-apple-swift`

## Notes

- the shipped local flows are still helper-level: cached summary hydration and quick-action preparation
- actual backend fetch, sync, review/apply, and heavier authority work remain daemon-backed
