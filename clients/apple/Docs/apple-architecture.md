# Apple Architecture Scaffold

Status: scaffolded boundary map for iPhone, iPad, Watch, and future macOS role-aware flows.

## Device Roles

- iPhone: daily negotiation surface (capture, voice, quick decisions)
- iPad: planning and structured review workspace
- Watch: glance-and-confirm edge device
- macOS: scaffold path for future ambient/HUD integrations

## Shared Module Scaffold

`clients/apple/Packages/VelAppleModules` now contains boundary-first targets:

- `VelDomain`: domain models and commands
- `VelApplication`: use-case/service protocols
- `VelInfrastructure`: API/sync/audit protocols
- `VelUIShared`: shared view-model/token primitives
- `VelApplePlatform`: Apple-specific adapter protocols
- `VelFeatureFlags`: capability model

These are intentionally thin stubs to keep architecture explicit before deep feature migration.

## Current Client Wiring

- `VeliOS` now uses `VelApplication.VelAppEnvironment` + `VelFeatureFlags.FeatureCapabilities` to switch between:
  - iPhone shell (existing tab-oriented loop)
  - iPad shell (`NavigationSplitView` with role-appropriate sections)
- `VelWatch` remains intentionally lean with quick-loop and capture focus.
- `VelMac` is a live target with a placeholder sidebar shell and shared environment wiring.
- `VelWidgetExtension` and `VelIntentsExtension` are scaffolded targets for Apple-native affordances.

## Next Moves

1. Move one vertical slice (`Quick Capture`) through `VelApplication` + `VelInfrastructure` protocols.
2. Route widget/live activity timeline state from durable backend snapshots.
3. Expand App Intents from placeholder intent execution to auth-aware action routing.
