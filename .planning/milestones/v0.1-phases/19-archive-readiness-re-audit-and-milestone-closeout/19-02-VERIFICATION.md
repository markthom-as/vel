# 19-02 Verification

## Goal

Produce one milestone-level integration record showing that the shipped backend seams, transport contracts, and operator shells fit together coherently across the completed milestone.

## Integration Record

### Shared backend-owned truth

- The runtime contract in [runtime.md](/home/jove/code/vel/docs/api/runtime.md) now describes one backend-owned model for:
  - `Now`
  - grounded assistant entry
  - planning profile management
  - same-day day-plan / `reflow`
  - supervised proposal application
- Those seams are backed by canonical Rust services and routes in:
  - [now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs)
  - [planning_profile.rs](/home/jove/code/vel/crates/veld/src/routes/planning_profile.rs)
  - [commitment_scheduling.rs](/home/jove/code/vel/crates/veld/src/routes/commitment_scheduling.rs)
  - [chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs)

### Web shell integration

- Web consumes the same backend-owned contracts through:
  - [types.ts](/home/jove/code/vel/clients/web/src/types.ts)
  - [NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx)
  - [ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx)
  - [SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx)
- Focused web tests verify the operator shell stays aligned to that shared data shape:
  - [NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx)
  - [ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx)
  - [SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx)
  - [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts)

### Apple shell integration

- Apple remains a thin operator shell over the same contracts, documented in [README.md](/home/jove/code/vel/clients/apple/README.md).
- The shipped Apple surfaces and shared transport layer are represented in:
  - [Models.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/Models.swift)
  - [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift)
  - [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VelMac/ContentView.swift)
  - [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VelWatch/ContentView.swift)
- Embedded/local-first additions stayed additive and typed rather than becoming a separate Apple product brain:
  - [apple-embedded-runtime-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md)
  - [apple-local-first-voice-continuity-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/apple/apple-local-first-voice-continuity-contract.md)

### CLI integration

- CLI continues to expose the same backend-owned model rather than inventing alternate flows:
  - [main.rs](/home/jove/code/vel/crates/vel-cli/src/main.rs)
  - [planning_profile.rs](/home/jove/code/vel/crates/vel-cli/src/commands/planning_profile.rs)
  - [review.rs](/home/jove/code/vel/crates/vel-cli/src/commands/review.rs)
  - [threads.rs](/home/jove/code/vel/crates/vel-cli/src/commands/threads.rs)

## Evidence

- Phase verification backfill established per-phase evidence through Phase `18`.
- Later product integration and parity work is recorded in:
  - [17-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/17-shell-embodiment-operator-mode-application-and-surface-simplification/17-04-SUMMARY.md)
  - [21-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/21-cross-surface-voice-assistant-parity-and-desktop-push-to-talk/21-04-SUMMARY.md)
  - [31-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/31-cross-surface-planning-profile-parity-and-assistant-managed-routine-edits/31-04-SUMMARY.md)
  - [33-04-SUMMARY.md](/home/jove/code/vel/.planning/phases/33-approved-day-plan-and-reflow-application-over-commitment-scheduling/33-04-SUMMARY.md)
  - [39-03-SUMMARY.md](/home/jove/code/vel/.planning/phases/39-vel-csv-regression-sweep-and-daily-use-closeout/39-03-SUMMARY.md)

## Verification

- `rg -n "GET /v1/now|planning-profile|commitment-scheduling/proposals|assistant|Apple" docs/api/runtime.md clients/apple/README.md crates/vel-cli/src/commands clients/web/src/components`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/ThreadView.test.tsx src/components/SettingsPage.test.tsx src/types.test.ts`
- `make check-apple-swift`

## Verdict

Passed. The milestone now has one explicit integration artifact tying backend seams, web, Apple, and CLI into the same shipped product record.
