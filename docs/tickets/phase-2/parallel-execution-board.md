# Phase 2 Parallel Execution Board

This board defines Phase 2 parallel execution ownership with non-overlapping primary write scopes and a three-sub-phase rollout.

This board is execution guidance, not shipped-behavior authority.
Shipped behavior remains anchored in `docs/MASTER_PLAN.md`.

## Sub-Phase 1: Contract Alignment & Visibility Closure

Goal: remove Phase 2 doc/code contradictions and close low-risk observability gaps before deeper feature work.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Contracts & Queue Shape | `004`, `005`, `012`, `019` | `docs/MASTER_PLAN.md`, `docs/tickets/`, `docs/user/` | active | Normalize ticket scope/status and terminology. |
| B: Sync/Cluster Visibility | `019` | `crates/veld/src/routes/`, `crates/vel-api-types/`, `clients/web/src/components/` | active | Close capability/freshness data-shape gaps in operator surfaces. |
| C: Connect Surface Consistency | `006` | `crates/vel-cli/src/`, `crates/veld/src/app.rs`, `docs/api/` | active | Remove CLI/runtime mismatch for connect endpoint behavior. |

Sub-phase 1 merge gate:

- phase and ticket status text are aligned across master plan and queue index
- connect surface no longer advertises unsupported runtime behavior
- operator diagnostics include currently available sync/capability visibility

## Sub-Phase 2: Core Runtime Delivery

Goal: ship the highest-value Phase 2 backend seams with deterministic behavior and explicit capability boundaries.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Reducer Extraction | `004` | `crates/veld/src/services/inference*`, `crates/vel-core/src/` | queued | Keep inference loop thin, reducers isolated, replay deterministic. |
| B: Connect Lifecycle MVP | `006` | `crates/veld/src/routes/`, `crates/veld/src/services/`, `crates/vel-api-types/` | queued | Launch/heartbeat/terminate with lease expiry and terminal persistence. |
| C: Capability Broker MVP | `016` | `crates/vel-core/src/`, `crates/veld/src/services/` | queued | Brokered execution with scoped checks and fail-closed denials. |

Sub-phase 2 merge gate:

- reducer registry and ordering are explicit and covered by tests
- connect MVP lifecycle is end-to-end testable
- capability broker enforces scoped mediation with denial traces

## Sub-Phase 3: Onboarding, Conflict Hardening & Cross-Surface Closure

Goal: finish user-facing setup and deterministic sync conflict behavior on top of sub-phase 2 seams.

| Lane | Primary Tickets | Primary Write Scope | Ready State | Notes |
| --- | --- | --- | --- | --- |
| A: Onboarding Completion | `012` | `crates/vel-cli/src/commands/`, `clients/web/src/`, `clients/apple/` | queued | Add `vel node link`, pairing flow, and trust/config visibility. |
| B: Sync Conflict Baseline | `005` | `crates/vel-core/src/`, `crates/vel-storage/src/`, `crates/veld/src/services/client_sync.rs` | queued | Implement deterministic ordering primitive + reconciliation policy integration. |
| C: Accessibility/Config Closure | `019` | `clients/web/src/`, `clients/apple/`, `crates/vel-cli/src/`, `docs/user/` | queued | Finish remaining terminology and recoverability gaps across surfaces. |

Sub-phase 3 merge gate:

- tester linking flow is usable through at least CLI and one GUI surface
- sync conflict handling is deterministic under replay and multi-node tests
- cross-surface terminology and recovery guidance are consistent

## Dependency Order

1. Sub-phase 1 closes contradictions and enables clean interface contracts.
2. Sub-phase 2 delivers core runtime seams (`004/006/016`) needed by onboarding and sync-hardening work.
3. Sub-phase 3 completes UX and deterministic conflict handling (`012/005/019`) on top of those seams.

## Coordination Rules

- Do not overlap primary write scopes in the same patch unless it is an explicit cross-lane integration slice.
- Keep route handlers thin; put policy and orchestration in services.
- If a lane reshapes a durable contract, update ticket text and user/operator docs in the same slice.
- Every lane must report command-backed verification evidence with test names and outcomes.

## Suggested Verification Commands

- `cargo test -p veld`
- `cargo test -p vel-cli`
- `cargo test -p vel-storage`
- `npm -C clients/web test -- --run src/components/SettingsPage.test.tsx`
