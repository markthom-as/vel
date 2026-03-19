# Apple Subtree Agent Guide

This file defines durable rules for work under `clients/apple`.

## Scope

- `VelAPI` is the shared transport/domain boundary for Apple clients.
- `Apps/VeliOS`, `Apps/VelMac`, and `Apps/VelWatch` are thin surfaces.
- Core policy/inference logic stays in Rust (`veld`), not in SwiftUI views.

## Directory Map

- `Vel.xcodeproj`: iOS app project and local package references.
- `VelAPI/Sources/VelAPI`: shared API client, wire models, offline cache/queue, local export utilities.
- `Apps/VeliOS`: iPhone shell for context/nudges/commitments/capture.
- `Apps/VelMac`: macOS shell + local source snapshot export.
- `Apps/VelWatch`: watch bootstrap surface.

## Dependency Rules

- `Apps/*` may depend on `VelAPI`.
- `VelAPI` must not import or depend on app targets.
- UI layers must not redefine backend domain enums/types with alternate spellings.
- SwiftUI views must not contain backend policy rules (risk, inference, nudge generation).
- Phase 05 continuity fields (`projects`, `action_items`, `linked_nodes`) are backend-owned and must not be re-ranked or re-triaged in Swift.
- Widgets/complications and notifications should route actions through shared queue/sync paths.

## Contract and Mapping Workflow

- **Canonical Truth**: Backend contracts and architectural status live in **`docs/MASTER_PLAN.md`**.
- Treat API DTOs in Rust crates as the wire format authority.
- When backend wire shapes change, update `VelAPI` models and mapping in the same change.
- Add or refresh fixture-backed decode checks when adding fields with non-trivial mapping.
- Keep wire models, persisted local models, and view models conceptually separate.

## Build and Run

- `make check-apple-swift`: build the shared `VelAPI` package with Swift.
- `make apple-setup-simulator`: run first-launch setup and ensure iOS + watchOS simulator runtimes exist.
- `make apple-build`: build `VeliOS` for the first available iPhone simulator.
- `make apple-build-watch-sim`: build `VelWatch` for the first available Apple Watch simulator.
- `make apple-run`: build, install, and launch `VeliOS` in Simulator.
- `make apple-list-devices`: list connected physical Apple devices known to `devicectl`.
- `make apple-build-ios-device`: signed build for physical iPhone/iPad (`APPLE_DEVELOPMENT_TEAM` and optional `APPLE_IOS_BUNDLE_ID`).
- `make apple-install-ios-device`: install/launch iOS build on a physical iPhone/iPad (`APPLE_DEVICE_ID`).
- `make apple-build-watch-device`: signed build for physical Apple Watch (`APPLE_DEVELOPMENT_TEAM` and optional `APPLE_WATCH_BUNDLE_ID`).
- `make apple-install-watch-device`: install/launch watch build on a physical Apple Watch (`APPLE_WATCH_DEVICE_ID`).
- `make apple-open`: open `clients/apple/Vel.xcodeproj` in Xcode.

Override defaults when needed:

- `APPLE_IOS_SCHEME=<scheme>` for non-default scheme builds.
- `APPLE_SIM_DEVICE_ID=<simulator-udid>` to pin a specific simulator.

## Do Not Do

- No direct network mutations from widgets/complications without explicit approval.
- No ad hoc Xcode-only fixes that bypass scripts/docs.
- No business-policy forks in Apple clients that diverge from daemon behavior.
