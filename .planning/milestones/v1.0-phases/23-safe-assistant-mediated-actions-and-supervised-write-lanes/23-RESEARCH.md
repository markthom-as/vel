# Phase 23 Research

## Domain

Safe assistant-mediated actions over the existing operator-action, review, trust, and writeback seams.

## Locked Inputs

- Phase 11 grounded the assistant over real Vel state and bounded tool awareness.
- Phase 15-16 established canonical `check_in`, `reflow`, trust/readiness, and action-item seams.
- Phase 20-22 established backend-owned assistant entry, voice continuity, typed daily-loop/closeout entry, and durable thread follow-through.
- Existing writeback, review, and SAFE MODE behavior already exists in runtime, operator settings, execution routing, and integration/writeback services.

## Problem

The assistant is now deeply useful for read-only grounding, daily-loop guidance, closeout, and thread-based resolution, but it still stops short of helping stage real bounded actions. If mutations are added carelessly, the product will regress into opaque assistant magic or duplicate the existing review/writeback model.

Phase 23 should close that gap by making assistant-originated proposals first-class inside existing supervision lanes instead of inventing a second mutation system.

## Required Truths

1. Assistant-staged actions
   - the assistant can propose bounded actions through the canonical operator-action model
   - proposals stay explicit, inspectable, and reviewable

2. Review and trust integration
   - assistant-originated proposals land in existing review/trust surfaces
   - SAFE MODE, writeback grants, and review gates still fail closed with explicit blockers

3. Thread-to-action continuity
   - thread resolution can hand off into explicit approval or confirmation paths
   - accepted proposals preserve provenance across thread, review, and writeback surfaces

## Recommended Execution Shape

Phase 23 should be executed in four slices:

1. publish the assistant proposal contract and backend staging seam
2. integrate assistant proposals with review/trust and fail-closed gates
3. connect thread continuity to staged approvals/confirmations and typed follow-through
4. align shell/docs/verification so the shipped story is explicit and honest

## Code Context

- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/chat/assistant.rs`
- `crates/veld/src/services/chat/tools.rs`
- `crates/veld/src/services/operator_queue.rs`
- `crates/veld/src/services/writeback.rs`
- `crates/veld/src/services/execution_routing.rs`
- `crates/veld/src/services/agent_grounding.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/execution.rs`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/types.ts`
- `crates/vel-cli/src/commands/agent.rs`
- `crates/vel-cli/src/commands/review.rs`

## Risks

- inventing assistant-only mutation state instead of reusing the canonical review/writeback model
- letting the assistant silently widen permissions or bypass SAFE MODE
- losing provenance between thread continuity, staged proposal, approval, and applied writeback

## Success Condition

Phase 23 is complete when assistant-originated proposals can be staged and supervised through the existing trust/review lanes, remain blocked when policy says no, and preserve continuity across assistant, thread, review, and writeback surfaces.
