# Phase 24 Context

## Purpose

Phase 24 exists because Phase 23 made assistant-mediated actions safe to stage, inspect, gate, and route through review, but the product is still missing the next practical step: taking an already-approved proposal through to an actual applied result.

The operator can now ask for real work through the assistant, but approved work still stops at the proposal boundary. That keeps the system safe, but it leaves the assistant one step short of being materially useful for bounded supervised execution.

## Product Direction

The next move is not broader autonomy. It is narrower closure.

This phase should:

- let bounded confirmed assistant actions apply through the existing operator-action lane
- let supervised write proposals advance only after the current review and trust gates are satisfied
- keep applied outcomes inspectable from the same run, thread, and review evidence already used elsewhere in Vel
- preserve reversibility where the underlying writeback or execution contract already supports it

The assistant should become more operationally useful without acquiring a parallel mutation authority.

## Expected Focus

1. Confirmation-to-application closure
   - staged proposals that only need operator confirmation should be able to advance into an applied result
   - the backend should own the state transition from proposed -> approved -> applied
   - shells should render that outcome, not invent it

2. Review-gated execution and writeback closure
   - repo-local or other supervised write proposals should continue to reuse execution handoff review and writeback gates
   - once approved, the same proposal/thread lineage should show what actually ran or applied
   - blocked paths should remain fail-closed and explicit

3. Reversible and explainable outcomes
   - assistant-applied work should preserve provenance across proposal, approval, execution/writeback, and resulting state
   - reversal should reuse existing product contracts where supported instead of introducing assistant-specific undo semantics
   - daily-use surfaces should be able to explain whether a proposal is still pending, was approved, was applied, or failed

## Non-Goals

- broad autonomous writeback beyond current review/trust policy
- ambient assistant authority widening
- inventing a second action model outside the operator queue, review, threads, and writeback seams

## Inputs

- [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md)
- [docs/api/runtime.md](/home/jove/code/vel/docs/api/runtime.md)
- [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- the shipped assistant-entry, thread, review, and SAFE MODE seams from Phases 20-23
- the existing execution handoff and writeback review contracts from Phases 6, 8, 9, and 23

## Exit Condition

Phase 24 is successful when explicitly approved assistant proposals can become real applied outcomes through the existing supervised lanes, with preserved provenance, honest blocked-path behavior, and no weakening of operator control.
