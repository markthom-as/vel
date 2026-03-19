# 15-01 Summary

## Outcome

Completed the Phase 15 contract-tightening slice for canonical operator-action ownership.

This slice did not implement `check_in` or `reflow` behavior yet. It tightened the shared backend contract so later slices can add those semantics without route-local or shell-local drift.

## What Changed

- Extended the core operator action contract in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) with:
  - `ActionPermissionMode`
  - `ActionScopeAffinity`
  - `permission_mode` on `ActionItem`
  - `scope_affinity` on `ActionItem`
- Re-exported the new core types from [crates/vel-core/src/lib.rs](/home/jove/code/vel/crates/vel-core/src/lib.rs).
- Updated the backend queue synthesizer in [crates/veld/src/services/operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so all current derived action items now assign explicit permission and scope defaults.
- Extended the transport DTO boundary in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with:
  - `ActionPermissionModeData`
  - `ActionScopeAffinityData`
  - matching `ActionItemData` fields and mapping
- Updated the route fixture in [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs) to match the tightened core contract.
- Updated the canonical checked-in example at [config/examples/operator-action-item.example.json](/home/jove/code/vel/config/examples/operator-action-item.example.json).
- Recorded the migration rule in:
  - [cross-surface-contract-vocabulary.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md)
  - [operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md)

## Why This Matters

- The operator action seam is now explicitly:
  - core-owned in `vel-core`
  - queue-synthesized in `veld::services::operator_queue`
  - read-model-consumed by surfaces such as `Now`
  - DTO-mapped in `vel-api-types`
- Project ownership and permission posture now have typed places to live before `check_in`, `reflow`, and readiness logic widen the model.

## Verification

- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p vel-api-types action_item_timestamps_serialize_as_rfc3339_strings -- --nocapture`
- `cargo test -p veld operator_queue -- --nocapture`
- `rg -n "Operator Action Contract Migration Rule|Phase 15 Migration Rule|permission_mode|scope_affinity" docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md docs/product/operator-action-taxonomy.md crates/vel-core/src/operator_queue.rs crates/vel-api-types/src/lib.rs config/examples/operator-action-item.example.json`

All passed. `veld` still emits pre-existing dead-code warnings during test builds.
