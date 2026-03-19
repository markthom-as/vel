# 15-03 Summary

## Outcome

Completed the first backend-owned `reflow` seam for Phase 15.

This slice introduces a typed `reflow` contract owned by Rust layers and exposes it through the existing `Now` read model. The first implementation derives from typed current-context drift, stale schedule age, and missed-event timing rather than a full scheduler, which keeps the seam migration-safe and daily-loop-adjacent.

## What Changed

- Extended the core operator contract in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) with:
  - `ReflowCard`
  - typed trigger, severity, accept-mode, and edit-target metadata
- Re-exported the new core types from [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs).
- Added a dedicated backend service in [crates/veld/src/services/reflow.rs](/home/jove/code/vel/crates/veld/src/services/reflow.rs) that:
  - derives `reflow` from stale context age
  - detects missed-event conditions from `next_event_start_ts`
  - maps typed drift into slipped-block reflow suggestions
  - stays severity-aware about whether the future accept path should be direct or confirm-required
- Updated [crates/veld/src/services/now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs) so `Now` consumes the backend-owned `reflow` seam alongside `check_in`.
- Extended the route mapping and fixture coverage in [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs).
- Extended the transport boundary in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with:
  - `ReflowCardData`
  - typed trigger, severity, accept-mode, and edit-target DTOs
  - `NowData.reflow`
- Updated the web decoder/types in [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the shell can consume the new contract without becoming its semantic owner.
- Updated the product contract note in [docs/product/onboarding-and-trust-journeys.md](/home/jove/code/vel/docs/product/onboarding-and-trust-journeys.md) to record the current migration rule for `reflow`.
- Fixed the affected `NowData` fixture in [crates/veld/src/services/execution_context.rs](/home/jove/code/vel/crates/veld/src/services/execution_context.rs) so the broader export path stays aligned.

## Why This Matters

- `reflow` now has a canonical Rust-owned landing zone separate from generic nudges and generic action items.
- The initial triggers are explicit and explainable from persisted current-context state rather than inferred in shell code.
- `Accept` / `Edit` branching is preserved as backend metadata, with `Edit` escalating toward `Threads`.
- This creates the seam needed for later scheduler/recalculation work without inventing a fake planner abstraction early.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p veld derives_critical_reflow_for_missed_event -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld execution_context -- --nocapture`

All passed. `veld` still emits pre-existing dead-code warnings during test builds.
