# 15-02 Summary

## Outcome

Completed the first backend-owned `check_in` seam for Phase 15.

This slice introduces a typed `check_in` contract owned by Rust layers and exposes it through the existing `Now` read model. The first source is active daily-loop prompt state, which gives the seam durable backend-owned semantics without inventing new UI-owned logic or a parallel persistence model.

## What Changed

- Extended the core operator contract in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) with:
  - `ActionKind::CheckIn`
  - typed `CheckInCard` / submit-target / escalation metadata
- Re-exported the new core types from [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs).
- Added a dedicated backend service in [crates/veld/src/services/check_in.rs](/home/jove/code/vel/crates/veld/src/services/check_in.rs) that:
  - resolves the local session date from timezone-aware runtime state
  - prefers an active standup session over morning overview
  - derives a typed `check_in` card from the active daily-loop prompt
  - preserves `Threads` escalation as metadata instead of hard-coded UI behavior
- Added a reusable local-date helper in [crates/veld/src/services/timezone.rs](/home/jove/code/vel/crates/veld/src/services/timezone.rs).
- Updated [crates/veld/src/services/now.rs](/home/jove/code/vel/crates/veld/src/services/now.rs) so `Now` consumes the backend-owned `check_in` seam and still returns it even when current context is otherwise empty.
- Extended the route mapping and fixture coverage in [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs).
- Extended the transport boundary in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with:
  - `CheckInCardData`
  - typed submit-target and escalation DTOs
  - `NowData.check_in`
- Updated the web decoder/types in [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the shell can consume the new contract without becoming its semantic owner.
- Updated the operator policy note in [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to record the current migration rule for `check_in`.
- Fixed the affected test fixture in [crates/veld/src/services/execution_context.rs](/home/jove/code/vel/crates/veld/src/services/execution_context.rs) so the new `NowData` contract stays consistent across exported grounding/execution artifacts.

## Why This Matters

- `check_in` now has a canonical Rust-owned landing zone instead of being implied by route-local JSON or shell behavior.
- The first implementation source is durable backend state that already exists: active daily-loop sessions.
- `Now` remains a consumer of the seam, not the owner.
- `Threads` escalation is preserved as typed metadata, which fits the Phase 14 product split and leaves shell embodiment for later phases.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p veld builds_check_in_card_from_daily_loop_prompt -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld formats_local_date_string_using_timezone -- --nocapture`

All passed. `veld` still emits pre-existing dead-code warnings during test builds.
