# 32-03 Summary

## What shipped

`32-03` exposed planning-profile proposal continuity across shipped surfaces without turning those surfaces into planners.

The main backend read-model changes landed in:

- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/planning_profile.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/vel-api-types/src/lib.rs`

The shipped behavior now is:

- planning-profile proposal continuity is summarized once in the backend from `planning_profile_edit` thread metadata
- `Threads` detail reads expose proposal lifecycle stage for planning-profile edits the same way assistant proposals already do
- `/v1/planning-profile` now carries compact proposal continuity for web Settings, CLI, and Apple read parity
- `/v1/now` now carries the same compact planning-profile continuity so `Now` can show pending review or recent applied outcome without becoming a planner

Cross-surface embodiment landed in:

- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `crates/vel-cli/src/commands/planning_profile.rs`
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`
- `clients/apple/Apps/VelMac/VelMacApp.swift`
- `clients/apple/Apps/VelMac/ContentView.swift`
- `clients/apple/Apps/VeliOS/VelApp.swift`
- `clients/apple/Apps/VeliOS/ContentView.swift`

Those surfaces now preserve one backend-owned story:

- `Now` shows compact planning-profile review/apply continuity only
- web `Settings`, CLI, and Apple summary surfaces report pending review and recent applied/failed outcomes from the same backend snapshot
- none of those surfaces silently apply routine/constraint edits outside the canonical approval path

I also fixed a real contract gap in the web boundary: `PlanningProfileEditProposalData` decoding was dropping `state` and `outcome_summary`, so the TypeScript decoder now matches the Rust DTO.

## Verification

Passed:

- `cargo fmt --all`
- `npm --prefix clients/web test -- --run src/types.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx`
- `cargo test -p veld planning_profile -- --nocapture`
- `cargo test -p vel-cli planning_profile -- --nocapture`
- `make check-apple-swift`

Not performed:

- UAT

## Result

Planning-profile proposal review pressure and applied outcome continuity are now visible across shipped surfaces through one backend-owned read model. The next logical step is `32-04`: docs/examples/verification closure for supervised planning-profile application.
