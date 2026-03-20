# Phase 30 Research

## Domain

Turn the durable routine-block and planning-constraint substrate from a persisted backend truth into something operators can actually inspect and manage through typed seams and thin shipped surfaces.

## Locked Inputs

- Phase 27 made scheduler semantics canonical, persisted, and reusable.
- Phase 28 proved the bounded same-day `day_plan` contract and shell embodiment.
- Phase 29 made routine blocks and bounded planning constraints durable backend-owned inputs to `day_plan` and `reflow`.
- `Vel.csv` kept pushing toward trustworthy summary-first posture, subtle status visibility, and Settings/help surfaces that support daily use without dumping operators into raw internals.
- `codex-workspace` already defines the operator’s real rule system, but Vel still lacks a complete operator-facing management path for the durable planning profile.

## Problem

Vel can now:

- persist durable routine blocks
- persist bounded planning constraints
- use those records in `day_plan` and `reflow`
- summarize routine-backed planning posture in shipped surfaces

But the operator still cannot manage that substrate cleanly:

- there is no first-class typed inspect/mutate seam for the planning profile
- Settings can summarize the profile, but not yet own safe management over it
- `Now` and `Threads` can reference planning posture, but cannot rely on one canonical management/read model yet

That leaves a practical usability gap:

- the planning substrate exists
- runtime truth is durable
- the operator still cannot shape or correct that truth directly through the product

## Required Truths

1. One planning profile
   - Routine blocks and bounded planning constraints must remain one backend-owned planning profile.
   - This phase must not create a second planner or ad hoc shell-local planning state.

2. Typed management, not JSON drift
   - Inspect/create/update/delete flows should be explicit typed backend seams.
   - This phase should not fall back to generic settings blobs or shell-owned JSON mutation.

3. Summary-first shells
   - `Settings` can be the main management surface, but it must stay summary-first and consume typed backend DTOs.
   - `Now` and `Threads` should keep referencing planning posture without becoming the authority for editing rules.

4. Bounded same-day scope
   - This phase should improve operability of the current same-day planning substrate.
   - It should not widen into multi-day planning, broad calendar editing, or a separate routine/habit product.

## Recommended Execution Shape

Phase 30 should be executed in four slices:

1. publish the typed planning-profile management contract and read/write seams
2. implement backend/storage/profile mutation seams with validation and persistence
3. embody summary-first management in shipped surfaces over the same canonical profile
4. close with docs/examples/verification for the operator-managed planning-profile model

## Code Context

- `crates/vel-core/src/planning.rs`
- `crates/vel-storage/src/repositories/planning_profiles_repo.rs`
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `docs/cognitive-agent-architecture/architecture/durable-routine-planning-contract.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-contract.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/product/operator-mode-policy.md`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`
- `/home/jove/Downloads/Vel.csv`

## Risks

- creating shell-managed planning semantics instead of typed backend seams
- letting Settings become a second planner surface instead of a summary-first management view
- introducing generic JSON mutation paths that bypass the planning contract
- widening into broad calendar editing, multi-day planning, or a routine-product detour
- making profile editing under-validated enough to erode trust in `day_plan` and `reflow`

## Success Condition

Phase 30 is complete when the product can honestly say:

- operators can inspect and manage the durable planning profile through typed backend seams
- `day_plan`, `reflow`, `Now`, `Threads`, and `Settings` all rely on one canonical planning-profile truth
- shells remain thin management and summary surfaces rather than planner owners
- docs/examples teach the operator-managed planning-profile model and its current limits clearly
