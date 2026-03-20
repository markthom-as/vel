# 29-02 Summary

Persisted the durable routine-planning profile in dedicated backend/storage seams so Phase 29 can move from contract-only shape to real backend-owned planning inputs.

Main changes:

- added [migrations/0046_phase29_routine_planning_profiles.sql](/home/jove/code/vel/migrations/0046_phase29_routine_planning_profiles.sql) with dedicated `routine_blocks` and `planning_constraints` tables plus ordering indexes
- added [crates/vel-storage/src/repositories/planning_profiles_repo.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/planning_profiles_repo.rs) with typed load/replace behavior for `RoutinePlanningProfile`
- updated [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs), [crates/vel-storage/src/repositories/mod.rs](/home/jove/code/vel/crates/vel-storage/src/repositories/mod.rs), and [crates/vel-storage/src/lib.rs](/home/jove/code/vel/crates/vel-storage/src/lib.rs) to expose backend-owned storage methods instead of routing this through generic settings or untyped JSON blobs
- added [crates/veld/src/services/planning_profile.rs](/home/jove/code/vel/crates/veld/src/services/planning_profile.rs) and exported it from [crates/veld/src/services/mod.rs](/home/jove/code/vel/crates/veld/src/services/mod.rs) as the thin application seam for runtime consumers
- updated [docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md) so the owner doc now distinguishes shipped persistence from still-pending runtime consumption

Focused verification:

- `cargo fmt --all`
- `cargo test -p vel-storage planning_profiles_repo -- --nocapture`
- `cargo test -p veld planning_profile -- --nocapture`
- baseline checks before edits:
  - `cargo test -p vel-core planning -- --nocapture`
  - `cargo test -p vel-storage settings_repo -- --nocapture`

Notes:

- this slice intentionally adds a replace/load seam, not full shell CRUD; richer operator management remains Phase `29-03` and later
- the new persistence layer preserves explicit ordering for both routine blocks and planning constraints so backend planning stays explainable and deterministic
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds
