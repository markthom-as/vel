# 16-05 Summary

Completed the final Phase 16 slice by making project-scoped action routing and longer-form thread escalation canonical in backend-owned seams instead of shell-local navigation guesses.

## What changed

- Extended the core action contract in [operator_queue.rs](/home/jove/code/vel/crates/vel-core/src/operator_queue.rs) and the transport boundary in [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with a typed optional `thread_route` hint for action items.
- Added canonical project-thread routing helpers in [projects.rs](/home/jove/code/vel/crates/veld/src/services/projects.rs), so project review and provisioning actions can point to filtered `Threads` views while remaining semantically project-owned.
- Reworked [operator_queue.rs](/home/jove/code/vel/crates/veld/src/services/operator_queue.rs) so project review, project provisioning, and execution-handoff review items now carry typed thread-routing metadata, while non-threaded actions stay explicit with `thread_route = None`.
- Widened [threads.rs](/home/jove/code/vel/crates/veld/src/routes/threads.rs) so the threads list route accepts typed `project_id` and `thread_type` filters and can resolve project-scoped thread views from backend metadata/links.
- Kept the `Now` contract aligned in [now.rs](/home/jove/code/vel/crates/veld/src/routes/now.rs), refreshed the checked-in example in [operator-action-item.example.json](/home/jove/code/vel/config/examples/operator-action-item.example.json), and updated the web decoder/tests in [types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts).
- Recorded the boundary rule in [operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md) and [now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md): project-scoped actions can surface in shared queues, but longer-form follow-up should use typed backend routing hints into `Threads`.

## Verification

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts`
- `cargo test -p vel-core operator_queue -- --nocapture`
- `cargo test -p vel-api-types action_item_timestamps_serialize_as_rfc3339_strings -- --nocapture`
- `cargo test -p veld operator_queue -- --nocapture`
- `cargo test -p veld list_threads_filters_by_project_id_and_thread_type -- --nocapture`
- `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture`
- `cargo test -p veld project_thread_route_preserves_project_scope -- --nocapture`

## Why this matters

Phase 15 preserved project identity, but shells still had to guess where longer-form project follow-up belonged. This slice closes that gap: project actions stay project-owned, shared queues can still surface them, and backend-owned thread-routing hints now tell shells how to move into filtered `Threads` views without inventing their own routing logic.
