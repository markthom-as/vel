# 46-03 Summary

## Outcome

Created the explicit subsystem inventory for the canonical `Now` contract and aligned the milestone-level planning docs so downstream phases inherit one fixed scope and ownership map.

## What Changed

- `.planning/phases/46-canonical-now-contract-boundaries-and-milestone-lock/46-SUBSYSTEM-INVENTORY.md`
  - Added the owner-phase map for every supporting subsystem required by the local source contract, including ranking, intent taxonomy, approval policy, governed config, `day thread`, `raw capture`, mesh summary, and reduced watch consumption.
- `.planning/PROJECT.md`
  - Tightened the `v0.3` target feature and active-scope language so governed config, deterministic ranking, and approval posture are explicitly part of the Rust-owned product-core lane.
- `.planning/REQUIREMENTS.md`
  - Added milestone acceptance language that ranking, intent routing, approval posture, and governed config must be shared subsystem contracts rather than shell policy.
- `.planning/ROADMAP.md`
  - Added explicit source-contract reconciliation to Phase 46 success criteria.
  - Tightened Phase 47 and 48 success criteria around intent taxonomy and approval posture ownership.
- `docs/MASTER_PLAN.md`
  - Updated the top-level authority pointer so post-`v0.2` `Now` behavior now points to the canonical `Now` product and Rust-core architecture docs.

## Verification

- `rg -n "ranking|intent|approval|config|Phase 47|Phase 48|Phase 49|Phase 50|Phase 51|raw capture|day thread" .planning/phases/46-canonical-now-contract-boundaries-and-milestone-lock/46-SUBSYSTEM-INVENTORY.md .planning/PROJECT.md .planning/REQUIREMENTS.md .planning/ROADMAP.md docs/MASTER_PLAN.md`

## Notes

- This slice deliberately stayed in planning/docs. It did not create DTOs, services, or client changes.
