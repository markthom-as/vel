# Phase 19 Context

## Purpose

Phase 19 exists to finish milestone closeout after Phase 18 repaired verification coverage and the requirements ledger. This phase is not product work. It is the final archive-readiness, milestone re-audit, and closeout lane.

## Starting Truth

Phase 18 resolved the first two milestone audit blockers:

- milestone phase verification coverage now exists for Phases `2` through `17`
- [REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md) now reflects reconciled requirement truth instead of a partial stale subset

The milestone is still not archive-ready because the following closeout blockers remain:

- roadmap/archive metadata drift
- no milestone-level integration artifact
- no milestone-level end-to-end flow artifact
- no rerun audit proving the repaired state passes

## Required Inputs

- [v0.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.1-MILESTONE-AUDIT.md)
- [REQUIREMENTS.md](/home/jove/code/vel/.planning/REQUIREMENTS.md)
- [ROADMAP.md](/home/jove/code/vel/.planning/ROADMAP.md)
- [STATE.md](/home/jove/code/vel/.planning/STATE.md)
- [18-PHASE19-HANDOFF.md](/home/jove/code/vel/.planning/phases/18-milestone-verification-backfill-and-requirement-reconciliation/18-PHASE19-HANDOFF.md)
- all Phase `2` through `17` verification artifacts under `.planning/phases/`

## What Phase 19 Must Achieve

1. Archive inputs become internally consistent
   - roadmap/state/analyze output must reflect the real shipped and closed state

2. Milestone-level integration evidence exists
   - one durable artifact must tie backend seams, web shell, Apple shell, and CLI shell together at milestone scope

3. Milestone-level flow evidence exists
   - one durable artifact must prove the required closeout operator flows at milestone scope rather than leaving only plan-local tests

4. Milestone audit is rerun against the repaired state
   - the rerun must explicitly close `CLOSEOUT-04`

5. The milestone is ready for `gsd-complete-milestone`
   - but this phase itself should stop at archive/tag readiness, not silently perform archival unless that is the routed next step

## Constraints

- Preserve the reconciled Phase 18 truth. Do not revert `REQUIREMENTS.md` back to a smaller or simpler but less honest ledger.
- Historical baseline/deferred Phase `2` and `4` items must remain unresolved unless new evidence actually changes product truth.
- Do not introduce new product/runtime implementation work in this phase.
- Prefer command-backed milestone artifacts over prose-only closeout claims.

## Expected Outputs

- Phase 19 planning stack
- milestone-level integration verification artifact
- milestone-level flow verification artifact
- rerun milestone audit artifact
- repaired roadmap/state/archive-readiness metadata

## Exit Condition

Phase 19 is complete when:

- `CLOSEOUT-03` can be marked satisfied truthfully
- `CLOSEOUT-04` can be marked satisfied truthfully
- the next `gsd-next` step is milestone archival rather than more gap repair
