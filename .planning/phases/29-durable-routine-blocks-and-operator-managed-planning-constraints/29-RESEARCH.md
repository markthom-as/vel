# Phase 29 Research

## Domain

Turn routine blocks and a bounded set of planning constraints into durable backend-owned records that can shape the same `day_plan` and `reflow` substrate across surfaces.

## Locked Inputs

- Phase 27 made scheduler semantics canonical, persisted, and reusable.
- Phase 28 proved the bounded same-day `day_plan` contract and shell embodiment.
- `codex-workspace` already provides the operator’s real same-day shaping model, but Vel currently preserves only the scheduler-rule portion durably.
- `Vel.csv` continued to push toward subtle context, summary-first visibility, trustworthy planning posture, and Settings/help surfaces that support recovery without becoming raw internals.

## Problem

Vel can now:

- normalize scheduler intent
- persist it on commitments
- shape a bounded same-day plan
- repair the day through `reflow`

But routine blocks are still too weak:

- they are inferred from current context instead of durably owned
- the operator cannot manage them explicitly through backend-owned records
- bounded planning constraints are not yet their own typed persisted surface

That leaves a trust and repeated-use gap:

- the planning substrate exists
- the operator cannot shape its durable routine assumptions cleanly
- repeated daily-use behavior still depends too much on inferred context rather than explicit planning records

## Required Truths

1. One planning substrate
   - Durable routine blocks must feed the same planning substrate already used by `day_plan` and `reflow`.
   - This phase must not create a second planner model.

2. Durable backend ownership
   - Routine blocks and bounded planning constraints should live in backend/domain/storage seams, not shell-local state.
   - Shells may manage or summarize them, but they must not own their semantics.

3. Bounded same-day scope
   - This phase should improve same-day planning quality.
   - It should not widen into multi-day optimization, broad autonomous mutation, or a habit/lifestyle product.

4. Operator-shaped, not opaque
   - The operator should be able to declare and adjust bounded routine inputs and planning constraints.
   - The product must stay explainable from persisted records, scheduler rules, and planning outputs.

## Recommended Execution Shape

Phase 29 should be executed in four slices:

1. publish the durable routine-block and planning-constraint contract
2. persist backend/storage seams for routine blocks and bounded planning constraints
3. feed durable routine records into `day_plan` and expose summary-first management in shipped shells
4. close with docs, examples, and verification for durable routine-backed planning

## Code Context

- `crates/vel-core/src/scheduler.rs`
- `crates/vel-core/src/commitment.rs`
- `crates/vel-core/src/operator_queue.rs`
- `crates/vel-storage/src/`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `docs/cognitive-agent-architecture/architecture/day-plan-contract.md`
- `docs/cognitive-agent-architecture/architecture/day-plan-reflow-contract.md`
- `docs/cognitive-agent-architecture/architecture/canonical-scheduler-facets.md`
- `docs/product/operator-mode-policy.md`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`
- `/home/jove/Downloads/Vel.csv`

## Risks

- widening routine handling into a vague full habit system
- creating shell-managed routine semantics or ad hoc constraint storage
- drifting away from the proven bounded same-day scheduler model
- making routine records so provider-shaped that they stop being durable product truth
- increasing planning complexity without improving operator trust

## Success Condition

Phase 29 is complete when the product can honestly say:

- routine blocks and bounded planning constraints persist as backend-owned planning records
- `day_plan` and `reflow` consume those durable records over one planning substrate
- shipped surfaces summarize and manage that posture without becoming planners
- docs/examples teach the durable routine-backed planning model and its limits clearly
