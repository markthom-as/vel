# 30-03 Summary

Exposed summary-first planning-profile management through typed backend routes and thin shipped surfaces over the canonical durable profile.

Main changes:

- added dedicated planning-profile routes in [crates/veld/src/routes/planning_profile.rs](/home/jove/code/vel/crates/veld/src/routes/planning_profile.rs) and registered them in [crates/veld/src/routes/mod.rs](/home/jove/code/vel/crates/veld/src/routes/mod.rs) and [crates/veld/src/app.rs](/home/jove/code/vel/crates/veld/src/app.rs) so `GET /v1/planning-profile` and `PATCH /v1/planning-profile` are now real shipped seams
- widened [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts) and [clients/web/src/data/operator.ts](/home/jove/code/vel/clients/web/src/data/operator.ts) with typed planning-profile transport decoding and query/mutation helpers instead of embedding profile logic in the view layer
- expanded [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx) with a summary-first routine/planning-profile card that can inspect saved routine blocks and constraints, add new ones, remove existing ones, and invalidate `Now` after successful mutations
- aligned [clients/web/src/components/NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx) and [clients/web/src/components/ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) so both surfaces explicitly frame day-plan and reflow output as coming from the same backend-owned planning profile
- added focused route coverage in [crates/veld/tests/planning_profile_api.rs](/home/jove/code/vel/crates/veld/tests/planning_profile_api.rs), widened decoder coverage in [clients/web/src/types.test.ts](/home/jove/code/vel/clients/web/src/types.test.ts), repaired Settings UI fixtures in [clients/web/src/components/SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx), and updated the owner doc in [docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md)

Focused verification:

- `cargo fmt --all`
- `cargo test -p veld --test planning_profile_api -- --nocapture`
- `cargo test -p veld planning_profile -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx src/components/NowView.test.tsx src/components/ThreadView.test.tsx src/types.test.ts`

Notes:

- this slice ships web/operator management only; Apple/CLI parity for planning-profile editing remains future work
- the web surface stays summary-first and does not derive planning logic locally; all profile authority remains in Rust backend layers
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds
- no UAT was performed
