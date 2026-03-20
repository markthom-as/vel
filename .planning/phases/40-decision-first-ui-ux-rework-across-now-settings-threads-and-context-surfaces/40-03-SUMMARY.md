# 40-03 Summary

## Outcome

The durable architecture and surface-taxonomy docs now point at the active `v0.2` MVP loop instead of relying on older cross-surface lane history or implied shell behavior.

The main slice landed in:

- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md`
- `docs/product/operator-surface-taxonomy.md`

## What changed

- Updated the cross-surface adapter authority so it now references `mvp-loop-contracts.md` directly and treats overview, commitments, reflow, threads, and review as the active cross-surface behavior lane.
- Clarified that shells consume those contracts and must not own overview or commitment-selection policy.
- Replaced the old future-phase sequence in the adapter doc with the active Phase 40-45 sequence for the `v0.2` milestone.
- Updated the operator surface taxonomy so `Now` is explicitly tied to overview, inline commitments, and reflow pressure, while `Threads` is explicitly bounded multi-step continuation.
- Removed stale discovery-oriented wording where the surface taxonomy still pointed back to older exploratory framing.

## Verification

- `rg -n "mvp-loop-contracts|overview|commitments|reflow|threads|review|thin shells|shell-owned" docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md docs/product/operator-surface-taxonomy.md`
- `rg -n "v0.2|Phase 40|Phase 41|Phase 45|Phase 13|active roadmap work begins at Phase 5" docs/MASTER_PLAN.md`

## Notes

- `docs/MASTER_PLAN.md` did not need further edits in this slice because the active-lane truth repair already landed in `40-01`.
