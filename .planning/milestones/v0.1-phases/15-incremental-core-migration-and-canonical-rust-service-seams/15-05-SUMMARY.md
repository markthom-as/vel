# 15-05 Summary

## Outcome

Completed the project-scoped operator-action seam for Phase 15 and closed the migration phase.

This slice preserves compact project identity through the canonical action contract so project-owned actions can surface in shared queues without becoming anonymous global items. The seam now carries enough project context for later Phase 16 logic and Phase 17 shell embodiment without requiring shells to rediscover project ownership from separate lookups.

## What Changed

- Extended the core action contract in [crates/vel-core/src/operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) so `ActionItem` can carry:
  - `project_label`
  - `project_family`
- Extended the transport DTO in [crates/vel-api-types/src/lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with the same project identity fields on `ActionItemData`.
- Updated the canonical queue builder in [crates/veld/src/services/operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so project-scoped actions now preserve compact project identity across:
  - execution handoff reviews
  - pending writebacks
  - open conflicts
  - project review/provisioning items
  - commitment-derived project actions
- Kept the `Now` route fixture aligned in [crates/veld/src/routes/now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs).
- Updated the checked-in action contract example in [operator-action-item.example.json](/home/jove/code/vel/config/examples/operator-action-item.example.json).
- Fixed the web transport boundary in [types.ts](/home/jove/code/vel/clients/web/src/types.ts) so `ActionItemData` now actually decodes:
  - `permission_mode`
  - `scope_affinity`
  - `project_label`
  - `project_family`
- Updated web contract coverage in [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts).
- Recorded the project-identity rule in [operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md) and [now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md).

## Why This Matters

- Project-scoped actions remain semantically project-owned even when they render in `Now` or `Inbox`.
- Shells no longer need to infer project markers from a separate projects query just to render a correct queue item.
- The Phase 15 migration lane now ends with backend-owned seams for:
  - `check_in`
  - `reflow`
  - trust/readiness
  - project-scoped action identity
- That leaves Phase 16 free to implement product logic on top of stable seams instead of reopening boundary questions.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p vel-api-types action_item_timestamps_serialize_as_rfc3339_strings -- --nocapture`
- `cargo test -p veld operator_queue -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`

All passed. `veld` still emits pre-existing dead-code warnings during test builds.
