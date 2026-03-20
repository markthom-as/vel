# Phase 28 Research

## Domain

Using canonical scheduler rules plus calendar anchors and routine blocks to create a bounded backend-owned same-day plan before drift occurs.

## Locked Inputs

- Phase 26 made `reflow` a real backend-owned same-day recovery lane with explicit `moved`, `unscheduled`, and `needs_judgment` outcomes.
- Phase 27 made scheduler semantics canonical, persisted, and reusable through commitments, recall, and assistant grounding.
- The operator already has a proven single-day shaping model in `/home/jove/code/codex-workspace`, including:
  - `block:*`
  - `cal:free`
  - duration markers
  - `time:*`
  - urgent/defer
  - fixed-start anchors
  - didn't-fit handling
- `Vel.csv` feedback reinforced the need for subtle context, summary-first visibility, strong project context, and Settings surfaces that support trust/recovery without dumping the operator into raw internals.

## Problem

Vel can now:

- normalize scheduler intent
- persist it canonically
- recover the day when drift occurs

But it still does not shape the day proactively from those same semantics.

That leaves a product gap:

- morning/standup can capture intention
- reflow can repair after drift
- but there is no canonical backend-owned initial day plan that explains what was scheduled, deferred, or left out before the day starts moving

## Required Truths

1. Bounded same-day planning
   - This phase should remain same-day and bounded.
   - It should not claim multi-day optimization, automatic wide calendar mutation, or opaque autonomous planning.

2. Routine blocks as typed inputs
   - Routine blocks need to become explicit backend-owned planning inputs rather than shell hints or undocumented assumptions.
   - They should interact with calendar anchors and scheduler rules deterministically.

3. Shared planning/recovery substrate
   - Initial day planning and later `reflow` should feel like one coherent backend-owned story.
   - They should share canonical scheduler semantics instead of creating a second planner model.

4. Explainable outcomes
   - The planner must explain:
     - what was scheduled
     - what was deferred
     - what did not fit
     - what still needs judgment
   - `Now`, `Threads`, assistant entry, CLI, and Apple should consume typed outputs rather than inferring planner meaning locally.

## Recommended Execution Shape

Phase 28 should be executed in four slices:

1. publish the canonical routine-block and day-plan contract
2. implement backend-owned same-day shaping over commitments, calendar anchors, and scheduler rules
3. embody the result in `Now`, `Threads`, and `Settings` without creating a shell planner
4. close with docs, examples, and verification for bounded day planning

## Code Context

- `crates/vel-core/src/scheduler.rs`
- `crates/vel-core/src/commitment.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/daily_loop.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/chat/`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`
- `docs/product/operator-mode-policy.md`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/docs/tui.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`
- `/home/jove/Downloads/Vel.csv`

## Risks

- accidentally widening from same-day shaping into a speculative planner rewrite
- creating separate routine semantics outside the canonical scheduler/planning substrate
- letting shells own prioritization or placement logic
- failing to keep planning outputs explainable enough for operator trust
- overfitting the first routine-block model to one source or one client shell

## Success Condition

Phase 28 is complete when the product can honestly say:

- Vel can create a bounded same-day plan before drift occurs
- routine blocks, calendar anchors, and commitment rules all feed one backend-owned planning contract
- operators can see what was scheduled, deferred, and left out without reverse-engineering label syntax
- shells consume the same typed planning output instead of inventing local planner rules
