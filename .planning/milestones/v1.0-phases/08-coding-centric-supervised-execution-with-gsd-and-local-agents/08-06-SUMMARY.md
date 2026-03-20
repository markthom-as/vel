---
phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
plan: 06
subsystem: sdk-and-docs
tags: [phase-08, sdk, docs, connect, handoff, repo-local, operator]
requires:
  - phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
    provides: execution contracts, handoff review, and guest-runtime launch from 08-01 through 08-05
provides:
  - reference SDK helpers that mirror the live connect-launch wire contract
  - execution-backed SDK coverage against the shipped `/v1/connect/instances` transport
  - operator-facing repo-local coding workflow docs aligned with the live handoff and connect path
affects: [phase-08, sdk, docs, cli, operator-surfaces]
tech-stack:
  added: []
  patterns: [wire-contract mirroring, execution-backed docs, repo-local supervised workflow]
key-files:
  created:
    - docs/user/coding-workflows.md
    - .planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-06-SUMMARY.md
  modified:
    - crates/vel-agent-sdk/src/lib.rs
    - crates/veld/tests/agent_sdk.rs
    - crates/vel-cli/src/commands/exec.rs
    - crates/vel-cli/src/commands/connect.rs
    - docs/user/README.md
    - docs/api/runtime.md
    - docs/cognitive-agent-architecture/agents/coding-execution-contracts.md
    - clients/web/src/components/SettingsPage.tsx
key-decisions:
  - "Kept `vel-agent-sdk` transport-shaped: the SDK now mirrors the live connect-launch request instead of inventing a second launcher abstraction."
  - "Documented the real shipped repo-local path honestly, including that launch initiation is still API/SDK driven rather than a dedicated `vel connect launch` CLI command."
  - "Closed verification with a live guest-runtime launch through `/v1/connect/instances` so the SDK/docs slice is execution-backed instead of doc-only."
patterns-established:
  - "Reference SDK helpers should track the live wire contract closely and stay small."
  - "Operator docs should describe one reviewable path from context export to handoff review to supervised launch."
requirements-completed: [GSD-01, GSD-02, HANDOFF-02, LOCAL-01, POLICY-01]
duration: 24m
completed: 2026-03-19
---

# Phase 08 Plan 06: SDK And Workflow Closure Summary

**Phase 08 now closes with a real repo-local workflow doc set and an SDK that can build live connect-launch payloads instead of pointing at historical stubs.**

## Performance

- **Duration:** 24 min
- **Completed:** 2026-03-19
- **Files modified:** 8

## Accomplishments

- Added `AgentSdkCapabilityGrant`, `AgentSdkConnectLaunchRequest`, `AgentSdkClient::manifest_reference(...)`, and `AgentSdkClient::connect_launch_request(...)` so the reference SDK mirrors the shipped transport boundary.
- Extended `crates/veld/tests/agent_sdk.rs` with a live `/v1/connect/instances` guest-runtime launch using the SDK-built payload, while keeping the protocol handshake/heartbeat/action-batch coverage intact.
- Wrote `docs/user/coding-workflows.md` as the operator-facing Phase 08 path from execution context to repo-local export to handoff review to supervised launch and inspection.
- Aligned runtime, architecture, CLI, and web discoverability copy with the real shipped workflow and current launch limitations.

## Verification

- `cargo fmt --all`
- `cargo test -p veld agent_sdk -- --nocapture`
- `cargo test -p vel-agent-sdk -- --nocapture`
- `cargo test -p vel-cli exec -- --nocapture`
- `cargo test -p vel-cli connect -- --nocapture`
- `rg -n "repo-local|GSD|handoff|connect|writable|review" docs/user/README.md docs/user/coding-workflows.md docs/api/runtime.md docs/cognitive-agent-architecture/agents/coding-execution-contracts.md`

## Decisions Made

- Reused the existing `ProtocolManifestReference` contract instead of creating a separate SDK-only manifest type.
- Kept launch initiation honest in the docs: inspection is available in CLI, but launch still flows through authenticated connect transport payloads.
- Used `wasm_guest` for the execution-backed SDK test because it exercises the live transport and policy boundary deterministically without depending on a long-running shell process.

## Deviations from Plan

### Light CLI Discoverability Closure

**1. Added workflow hints to existing CLI outputs instead of introducing a new launch subcommand**
- **Reason:** the shipped Phase 08 transport already supports supervised launch, but the repo does not yet have a dedicated operator CLI launch wrapper.
- **Implementation:** `vel exec export` and `vel connect instances` now point operators toward the repo-local review and launch workflow.
- **Impact:** discoverability improved without inventing a second launch surface that would need separate policy handling.

## Next Phase Readiness

- Phase 08 is now complete at the plan level (`08-01` through `08-06`).
- The roadmap can treat Phase 08 as ready for verification/close-out once the active Phase 07 lane is finished.
