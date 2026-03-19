# 16-01 Summary

## Outcome

Completed the Phase 16 transition-contract slice by making `check_in` and `reflow` expose backend-owned typed transition vocabularies through the existing core, service, DTO, and web seams.

## What Changed

- Added canonical transition types in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) for:
  - `CheckInTransition`
  - `CheckInTransitionKind`
  - `CheckInTransitionTargetKind`
  - `ReflowTransition`
  - `ReflowTransitionKind`
  - `ReflowTransitionTargetKind`
- Extended `CheckInCard` and `ReflowCard` to carry typed `transitions`, while preserving the existing submit/edit/escalation fields for current shell compatibility.
- Updated [crates/veld/src/services/check_in.rs](/home/jove/code/vel/crates/veld/src/services/check_in.rs) so daily-loop-derived check-ins now publish canonical `submit`, `bypass`, and `escalate` transitions.
- Updated [crates/veld/src/services/reflow.rs](/home/jove/code/vel/crates/veld/src/services/reflow.rs) so reflow cards now publish canonical `accept` and `edit` transitions, with confirmation carried as typed backend metadata.
- Updated [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs), [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs), [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts), and [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts) so the widened `Now` contract stays end-to-end aligned.
- Updated [docs/product/operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md) and [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md) to lock the Phase 16 rule that shells consume typed transitions instead of inferring lifecycle semantics from labels alone.

## Why It Matters

Phase 15 proved the seams, but Phase 16 needed one canonical answer for valid next moves before lifecycle behavior widened. This slice makes the backend-owned transition contract explicit so later `check_in` and `reflow` handlers can implement submit/bypass/accept/edit behavior without reopening shell-boundary questions.

## Verification

- `cargo fmt --all`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p veld check_in -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts`

## Notes

- `veld` still emits pre-existing dead-code warnings during targeted test runs.
- No UAT was performed, per user instruction.
