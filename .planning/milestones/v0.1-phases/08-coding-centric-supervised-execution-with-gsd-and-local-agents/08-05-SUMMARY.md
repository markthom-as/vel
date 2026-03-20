---
phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
plan: 05
subsystem: runtime
tags: [phase-08, wasm, guest-runtime, sandbox, connect, policy, veld]
requires:
  - phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
    provides: connect lifecycle baseline from 08-03 and execution contracts from 08-01
provides:
  - direct `wasm_guest` runtime kind on the connect transport
  - guest execution through the existing sandbox and broker boundary
  - integration coverage for approved and denied guest execution paths
affects: [phase-08, connect, sandbox, runtime, docs]
tech-stack:
  added: []
  patterns: [shared sandbox mediation, deny-by-default guest scope checks, explicit compatibility path]
key-files:
  created:
    - crates/veld/src/services/wasm_guest_runtime.rs
    - crates/veld/tests/wasm_guest_runtime.rs
    - docs/cognitive-agent-architecture/agents/wasm-local-runtime.md
    - .planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-05-SUMMARY.md
  modified:
    - crates/veld/src/services/mod.rs
    - crates/veld/src/services/connect_runtime.rs
    - docs/api/runtime.md
key-decisions:
  - "Added `runtime_kind: \"wasm_guest\"` to the existing connect lifecycle instead of creating a separate launch surface."
  - "Reused `services::sandbox` as the mediated execution boundary so capability checks, denials, and terminal state stay trace-linked."
  - "Kept the current direct guest seam explicit as a guest-module-spec compatibility path until a concrete binary engine is selected."
patterns-established:
  - "Guest runtimes use the same brokered capability boundary as local runtimes."
  - "Writable-root and network expansion requests are denied before guest execution begins."
requirements-completed: [EXEC-02, LOCAL-01, POLICY-01]
duration: 18m
completed: 2026-03-19
---

# Phase 08 Plan 05: WASM Guest Runtime Summary

**Vel now supports a supervised `wasm_guest` runtime kind on the connect transport without bypassing the existing sandbox and broker policy model**

## Performance

- **Duration:** 18 min
- **Completed:** 2026-03-19
- **Files modified:** 7

## Accomplishments

- Added `services::wasm_guest_runtime` as the dedicated guest-runtime seam and wired `connect_runtime` to accept `runtime_kind: "wasm_guest"`.
- Reused the existing sandbox executor so guest approval, denial, and terminal states remain trace-linked and broker-mediated.
- Added integration coverage for approved guest execution, denied capability requests, and write-scope/network expansion rejection.
- Added nearby owner/runtime docs for the guest-runtime policy and compatibility posture.

## Verification

- `cargo test -p veld wasm_guest -- --nocapture`
- `cargo test -p veld sandbox -- --nocapture`

## Decisions Made

- Kept guest launches on the existing `/v1/connect/instances` surface so runtime supervision stays inspectable in one place.
- Denied any requested guest network hosts up front because no narrower manifest-backed network allowlist exists yet.
- Returned guest policy denials as explicit launch errors while still persisting the backing run and connect-run state.

## Deviations from Plan

### Intentional Compatibility Cut

**1. Used a guest-module spec file as the direct guest input**
- **Reason:** the repository did not yet carry a concrete WASM engine dependency or binary loader seam, but the phase still needed a direct guest-runtime path that exercised the real policy boundary.
- **Implementation:** `wasm_guest_runtime` currently consumes a checked-in guest module spec that expands to sandbox host-call envelopes and runs them through the shared sandbox/broker flow.
- **Impact:** policy, trace, and denial behavior are now real and test-backed; engine-specific binary loading remains a later widening step rather than hidden implied behavior.

## Issues Encountered

- The first integration-test pass used the wrong `build_app` constructor in the new guest-runtime test harness. That was corrected inline before final verification.

## User Setup Required

None.

## Next Phase Readiness

- `08-06` can now document the shipped guest-runtime caveats and SDK/operator workflow honestly.
- `08-04` and `08-06` can reference a real guest-runtime variant when surfacing routing and handoff review.
