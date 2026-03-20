# Phase 33 Context

## Title

Approved day-plan and reflow application over commitment scheduling

## Why this phase exists

Phases 26 through 32 made same-day planning and planning-profile edits explainable, typed, durable, and supervised across surfaces. Vel can now:

- normalize scheduler rules
- shape bounded same-day `day_plan`
- derive real `reflow` recovery proposals
- persist durable routine/planning inputs
- stage and apply supervised planning-profile edits with cross-surface continuity

What is still missing is the operator’s next useful step after seeing a bounded `day_plan` or `reflow` proposal: applying same-day scheduling changes themselves through one backend-owned seam instead of leaving those outputs as explainable but non-applied guidance.

## Product problem

Right now the planning substrate is trustworthy but still incomplete for daily use:

- `day_plan` can explain what should be scheduled, deferred, or left out
- `reflow` can explain how the remaining day should be repaired
- `Threads` can hold longer disagreement and shaping work

But bounded same-day scheduling outcomes are not yet moving through a supervised canonical apply lane the way planning-profile edits now can.

That creates a usability gap:

- the operator can see the right same-day answer
- but Vel cannot yet persist the approved scheduling result through one backend-owned commitment scheduling seam

## Phase goal

Let approved bounded same-day `day_plan` and `reflow` changes apply through canonical backend-owned commitment scheduling mutations with explicit review, lifecycle continuity, and summary-first cross-surface posture.

## Must stay true

- same-day only; this is not a multi-day planner
- bounded commitment scheduling only; this is not broad autonomous calendar editing
- backend-owned mutation seam; shells do not invent planner writes locally
- explicit thread continuity for pending, applied, and failed outcomes
- summary-first shells stay thin: `Now`, `Settings`, CLI, and Apple should report continuity, not become planners

## Likely touch points

- `crates/vel-core`
- `crates/vel-storage`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/vel-api-types`
- `clients/web`
- `crates/vel-cli`
- `clients/apple`
- `docs/api/`
- `docs/user/`
- `docs/product/`

## Expected next step

Phase 33 planning should break this into:

1. contract/lifecycle publication for approved day-plan and reflow application
2. backend apply path over canonical commitment scheduling seams
3. cross-surface continuity and summary embodiment
4. docs/examples/verification closeout
