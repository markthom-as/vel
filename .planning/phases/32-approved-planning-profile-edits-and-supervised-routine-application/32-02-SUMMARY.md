# 32-02 Summary

## Completed

Implemented backend approval and application of staged planning-profile proposals through the canonical mutation seam.

## What changed

- added a typed backend apply path in [planning_profile.rs](/home/jove/code/vel/crates/veld/src/services/planning_profile.rs) that:
  - loads a staged `planning_profile_edit` thread
  - reconstructs the typed `PlanningProfileEditProposal`
  - applies the canonical `PlanningProfileMutation`
  - records `approved`, `applied`, or `failed` lifecycle outcomes back into thread metadata
- widened [planning_profile.rs](/home/jove/code/vel/crates/veld/src/routes/planning_profile.rs) and [lib.rs](/home/jove/code/vel/crates/vel-api-types/src/lib.rs) with a dedicated proposal-apply response shape over the existing planning-profile route family
- mounted the new backend route in [app.rs](/home/jove/code/vel/crates/veld/src/app.rs):
  - `POST /v1/planning-profile/proposals/:id/apply`
- added focused success and failure integration coverage in [planning_profile_api.rs](/home/jove/code/vel/crates/veld/tests/planning_profile_api.rs)

## Verification

- `cargo fmt --all`
- `cargo test -p veld planning_profile -- --nocapture`
- `cargo test -p veld --test planning_profile_api -- --nocapture`
- `cargo test -p veld --test chat_assistant_entry assistant_entry_stages_planning_profile_edit_with_thread_continuity -- --nocapture`

## Result

Staged planning-profile proposals can now resolve through a real backend-owned approval/application path instead of stopping at continuity-only staging. The next logical step is `32-03`: surface review pressure and applied outcomes across shipped surfaces without turning them into planners.
