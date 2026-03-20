# Phase 24 Research

## Domain

Applying approved assistant proposals through the existing operator-action, execution-review, thread, and writeback seams without weakening supervision.

## Locked Inputs

- Phase 20 made the grounded assistant the default entry path across `Now`, `Inbox`, and `Threads`.
- Phase 21 unified voice provenance and shared assistant continuity across web/desktop and Apple.
- Phase 22 made daily loop, closeout, and longer resolution flows assistant-capable while preserving typed backend authority.
- Phase 23 made assistant-mediated actions safe to stage, route through trust/review, and preserve approval-thread continuity.
- Existing writeback, execution handoff review, SAFE MODE, and trust/readiness behavior already exists and must remain canonical.

## Problem

The assistant can now propose bounded actions safely, but approved work still stops at the proposal boundary. That leaves a gap between supervised intent and applied outcome. If this is closed carelessly, Vel risks inventing a second mutation system or letting assistant-originated work bypass the explicit operator controls already established elsewhere.

Phase 24 should close that gap by making approval actually lead to a real applied result through the same runtime seams that already govern writeback and supervised execution.

## Required Truths

1. Approved application
   - confirmed assistant proposals can advance from staged to applied through canonical backend state transitions
   - the backend, not shells, owns proposal outcome state

2. Review-gated execution and writeback
   - repo-local or other supervised writes still depend on existing approval and trust gates
   - applied outcomes preserve continuity from proposal -> review -> execution/writeback -> result

3. Provenance and reversibility
   - assistant-applied outcomes remain inspectable from thread, review, trust, and resulting state surfaces
   - reversibility is reused where the underlying contract already supports it instead of inventing assistant-only undo rules

## Recommended Execution Shape

Phase 24 should be executed in four slices:

1. publish the approved-application contract and canonical proposal state transitions
2. complete review-gated execution and writeback application for approved proposals
3. preserve applied outcome provenance, reversibility, and operator follow-through projections
4. align shell/docs/verification so the shipped story is explicit and honest

## Code Context

- `crates/vel-core/src/operator_queue.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/operator_queue.rs`
- `crates/veld/src/services/writeback.rs`
- `crates/veld/src/services/execution_routing.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/execution.rs`
- `crates/veld/src/routes/threads.rs`
- `clients/web/src/types.ts`
- `crates/vel-cli/src/commands/agent.rs`
- `crates/vel-cli/src/commands/review.rs`
- `docs/api/chat.md`
- `docs/api/runtime.md`
- `docs/user/daily-use.md`

## Risks

- inventing assistant-only apply state outside canonical review/writeback/runtime seams
- letting approval imply ambient mutation without the existing execution or writeback contracts running
- losing provenance between staged proposal, review approval, applied result, and any reversible follow-through
- exposing reversal semantics where the underlying product contract does not actually support them

## Success Condition

Phase 24 is complete when explicitly approved assistant proposals can produce real applied outcomes through the existing supervised lanes, while blocked paths still fail closed and resulting state remains inspectable and reversible where the product already supports reversal.
