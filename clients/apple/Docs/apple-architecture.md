# Apple Architecture Scaffold

Status: active boundary map for iPhone, iPad, Watch, and future macOS role-aware flows.

## Device Roles

- iPhone: daily negotiation surface (capture, voice, quick decisions)
- iPad: planning and structured review workspace
- Watch: edge client, sensor/haptic node, and glance-and-confirm interrupt surface
- macOS: scaffold path for future ambient/HUD integrations

## Shared Module Scaffold

`clients/apple/Packages/VelAppleModules` now contains boundary-first targets:

- `VelDomain`: domain models and commands
- `VelApplication`: use-case/service protocols
- `VelInfrastructure`: API/sync/audit protocols
- `VelUIShared`: shared view-model/token primitives
- `VelApplePlatform`: Apple-specific adapter protocols
- `VelFeatureFlags`: capability model
- `VelEmbeddedBridge`: iPhone-first embedded-capable bridge seam and flow gate vocabulary

These are intentionally thin stubs to keep architecture explicit before deep feature migration.

## Embedded-Capable Foundation

Phase 37 now adds the first package-level seam for the additive iPhone embedded Rust path:

- `VelFeatureFlags` owns whether a surface may support the embedded bridge at all
- `VelEmbeddedBridge` owns the narrow embedded-safe flow vocabulary and fail-closed bridge protocols
- `VelApplication` can depend on that seam without treating embedded execution as the default path

Current truth is still daemon-backed HTTP via `VelAPI`. The embedded bridge exists so later slices can add real Rust-backed implementation without changing the shell/core ownership model.

## Current Client Wiring

- `VeliOS` now uses `VelApplication.VelAppEnvironment` + `VelFeatureFlags.FeatureCapabilities` to switch between:
  - iPhone shell with exactly three first-class surfaces: `Now`, `Threads`, and `System`
  - iPad shell (`NavigationSplitView`) over those same three surfaces
- `VelWatch` remains intentionally lean with quick-loop, haptics, and capture focus.
- `VelMac` is a live target with a placeholder sidebar shell and shared environment wiring.
- `VelWidgetExtension` and `VelIntentsExtension` are scaffolded targets for Apple-native affordances.

## Watch edge-client contract (Wave 3)

`VelWatch` is intentionally narrow for this phase:

- architectural role: edge client of `veld`, with iPhone as the local bridge/cache/reconciliation proxy
- primary objective: expose active nudges, compact current-state posture, and quick capture/append actions
- thread path: keyboard/text append into the active thread and voice transcript append only when canonical thread continuity already exists
- reduced objective: no dedicated thread management/listing, project views, or settings hub
- behavior: if no active thread is available, input is queued as watch capture with recoverable provenance rather than inventing a local thread flow
- mapping: actions go through existing `VelWatchStore` call paths (`markTopNudgeDone`, `snoozeTopNudge`, `submitThreadText`) to preserve backend/API contract boundaries
- authority rule: watch does not own synthesis, policy, or heavy decision-making; it sends bounded signals and renders compact snapshots

Canonical authority for this stance lives in [apple-watch-edge-client-contract.md](../../../docs/cognitive-agent-architecture/apple/apple-watch-edge-client-contract.md).

## Next Moves

1. Tighten `VelWatch` around event-first edge-client behavior instead of growing more watch-local UI.
2. Route widget/complication timeline state from durable backend snapshots and bounded iPhone bridge cache.
3. Expand watch-originated signal vocabulary only through typed event-log contracts.
4. Expand App Intents from placeholder intent execution to auth-aware action routing.
