# 29-03 Summary

Switched same-day planning and recovery from inferred-only routine inputs to the durable routine-planning profile, while keeping shells thin and summary-first.

Main changes:

- expanded [crates/veld/src/services/planning_profile.rs](/home/jove/code/vel/crates/veld/src/services/planning_profile.rs) into the runtime seam for:
  - same-day durable routine-block materialization
  - inferred fallback only when no durable blocks are available
  - bounded planning-constraint helpers for default window, calendar buffers, max scheduled items, and overflow judgment
- updated [crates/veld/src/services/day_plan.rs](/home/jove/code/vel/crates/veld/src/services/day_plan.rs) so `day_plan` now consumes durable routine blocks and bounded constraints instead of relying only on inferred blocks and raw calendar windows
- updated [crates/veld/src/services/reflow.rs](/home/jove/code/vel/crates/veld/src/services/reflow.rs) so remaining-day recovery now uses the same durable routine/constraint substrate as proactive planning
- updated [crates/veld/src/services/timezone.rs](/home/jove/code/vel/crates/veld/src/services/timezone.rs) with the internal timezone accessor needed for durable local-time routine materialization
- updated [clients/web/src/components/NowView.tsx](/home/jove/code/vel/clients/web/src/components/NowView.tsx), [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx), and [clients/web/src/components/ThreadView.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.tsx) so shipped surfaces now summarize whether today is using operator-managed routines or inferred fallback without becoming planner-authority surfaces
- updated the matching focused tests in [clients/web/src/components/NowView.test.tsx](/home/jove/code/vel/clients/web/src/components/NowView.test.tsx), [clients/web/src/components/SettingsPage.test.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.test.tsx), and [clients/web/src/components/ThreadView.test.tsx](/home/jove/code/vel/clients/web/src/components/ThreadView.test.tsx)
- updated owner docs in [docs/cognitive-agent-architecture/architecture/day-plan-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/day-plan-contract.md) and [docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md)

Focused verification:

- `cargo fmt --all`
- `cargo test -p veld day_plan -- --nocapture`
- `cargo test -p veld reflow -- --nocapture`
- `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/SettingsPage.test.tsx src/components/ThreadView.test.tsx src/types.test.ts`

Notes:

- durable routine blocks now take precedence over inferred routine shaping, but inferred fallback remains intentional when no durable blocks are configured
- the planner is still explicitly bounded and same-day; this slice does not widen into multi-day optimization or broad autonomous calendar mutation
- `veld` still emits the same pre-existing unused/dead-code warnings during Rust test builds
