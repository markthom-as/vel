# Phase 26 Research

## Domain

Turning `reflow` from a warning card into a real backend-owned day-repair lane with remaining-day recomputation, schedule reconciliation, and operator-visible explanation.

## Locked Inputs

- Phase 15 introduced the first backend-owned `reflow` seam.
- Phase 16 implemented typed `reflow` apply/edit behavior and thread-backed escalation.
- Phase 17 embodied heavier `reflow` treatment in the shell without moving plan logic into the UI.
- Phases 20 through 25 made assistant entry, voice entry, thread continuity, supervised action lanes, and bounded local recall usable enough that schedule recovery now has a practical front door.
- The operator already has a proven rule/tagging model in `/home/jove/code/codex-workspace` for single-day scheduling and re-scheduling.
- `Vel.csv` feedback already points at the same problem: stale schedule trust, subtle freshness signaling, contextual recovery, thread reopenability, and settings/help that support recovery instead of dumping raw internals first.

## Problem

Vel currently knows that the day has drifted, but it does not yet repair the day in a way that is useful enough for repeated real use:

- `reflow` mainly surfaces warning/state posture rather than a recomputed remaining-day proposal
- stale schedule, missed event, slipped block, and "this no longer fits" logic are not yet one canonical backend path
- schedule semantics from the operator's existing `codex-workspace` scheduler are not yet mapped into durable Vel-side fields/facets
- Settings still does not reflect real recovery posture and freshness/reflow trust clearly enough

Phase 26 should make `reflow` materially useful without widening into a separate planner product.

## Required Truths

1. Canonical reflow contract
   - `reflow` must produce a typed remaining-day proposal, not just a warning/status marker
   - stale schedule, missed event, slipped block, and "didn't fit" outcomes should converge on one backend-owned reconciliation path

2. Intentional scheduler-rule mapping
   - `codex-workspace` rules like `block:*`, duration markers, `cal:free`, time windows, urgent/defer flags, fixed-start tasks, and "Didn't Fit" handling must be mapped deliberately
   - raw upstream labels should not become Vel's long-term product model; canonical Vel-side fields/facets should own the semantics

3. Explainable recovery
   - reflow output must tell the operator what changed, what moved, what no longer fits, and what still needs judgment
   - shell surfaces should consume typed explanation instead of re-deriving schedule deltas locally

4. Recovery posture in Settings
   - Settings should expose freshness/recovery posture and contextual guidance relevant to reflow trust
   - this should remain summary-first and recovery-oriented, not runtime-debug-first

## Recommended Execution Shape

Phase 26 should be executed in four slices:

1. publish the canonical reflow/reconciliation contract and scheduler-rule mapping seam
2. implement real backend-owned remaining-day recomputation and explicit moved/unscheduled outcomes
3. embody the new reflow output in `Now`, `Threads`, and `Settings` without breaking thin-shell boundaries
4. align docs and verification with honest current-day recovery limits

## Code Context

- `crates/vel-core/src/context.rs`
- `crates/vel-core/src/operator_queue.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/daily_loop.rs`
- `crates/veld/src/services/check_in.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/routes/threads.rs`
- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `docs/product/operator-action-taxonomy.md`
- `docs/product/operator-mode-policy.md`
- `docs/user/daily-use.md`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/docs/tui.md`
- `/home/jove/code/codex-workspace/schemas/task-schema.md`
- `/home/jove/code/codex-workspace/schemas/todoist-obsidian-alignment.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`

## Risks

- over-copying `codex-workspace` behavior without mapping it into Vel's architecture
- letting raw Todoist labels become core scheduling truth instead of typed Vel semantics
- widening into multi-day planning, autonomous mutation, or a separate planner subsystem
- putting schedule-diff logic into shells rather than keeping it backend-owned
- making freshness/recovery louder and noisier instead of clearer and more subtle

## Success Condition

Phase 26 is complete when the product can honestly say:

- `reflow` can recompute the remaining day instead of only surfacing drift
- operators can see what moved, what no longer fits, and what still needs judgment
- the scheduling semantics are backend-owned, explainable, and aligned with the proven `codex-workspace` rules where they fit
- `Now`, `Threads`, and `Settings` expose schedule recovery clearly without becoming the owners of the schedule logic
