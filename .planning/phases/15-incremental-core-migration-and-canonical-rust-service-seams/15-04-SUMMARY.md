# 15-04 Summary

## Outcome

Completed the first summary-first trust/readiness projection for Phase 15.

This slice composes existing backup trust, freshness state, pending writeback/conflict pressure, and supervised review pressure into one backend-owned `Now` readiness seam. The result stays action-oriented and summary-first instead of forcing shells to recompute trust posture from lower-level fields.

## What Changed

- Extended the core/operator contract in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) with `ReviewSnapshot.pending_execution_reviews`.
- Updated the backend review projection in [crates/veld/src/services/operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so pending execution handoffs are counted in the canonical review snapshot.
- Extended the transport boundary in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with:
  - `TrustReadinessFacetData`
  - `TrustReadinessReviewData`
  - `TrustReadinessData`
  - `NowData.trust_readiness`
- Refactored backup trust reuse in [crates/veld/src/services/backup.rs](/home/jove/code/vel/crates/veld/src/services/backup.rs) so non-route services can consume the same typed backup status/trust classification without duplicating logic.
- Kept doctor aligned in [crates/veld/src/services/doctor.rs](/home/jove/code/vel/crates/veld/src/services/doctor.rs) by reusing the backup trust seam rather than carrying a forked classification path.
- Added the backend-owned readiness projection in [crates/veld/src/services/now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs), where `Now` now folds:
  - backup trust
  - current freshness state
  - open review pressure
  - pending writebacks
  - open conflicts
- Extended route mapping and fixture coverage in [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs).
- Updated the broader export fixture in [crates/veld/src/services/execution_context.rs](/home/jove/code/vel/crates/veld/src/services/execution_context.rs) so execution-preview artifacts stay aligned with the widened `NowData` contract.
- Updated the web transport boundary in [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the shell consumes the new readiness summary without deriving it locally.
- Recorded the migration note in [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) and [docs/product/onboarding-and-trust-journeys.md](/home/jove/code/vel/docs/product/onboarding-and-trust-journeys.md).

## Why This Matters

- Trust/readiness now has a canonical backend-owned summary surface instead of remaining an implicit combination of backup, freshness, and review screens.
- The `Now` shell can render one readiness posture while deeper trust inspection stays progressively disclosed.
- Supervised execution review pressure is now part of the same explainable trust posture as stale inputs and backup degradation.
- This creates the seam Phase 16 needs for summary-first trust behavior without reopening shell-boundary debates.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p vel-api-types review_snapshot_default_serializes_named_counts -- --nocapture`
- `cargo test -p veld trust_readiness_warns_when_review_pressure_exists -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld execution_context -- --nocapture`

All passed. `veld` still emits pre-existing dead-code warnings during test builds.
