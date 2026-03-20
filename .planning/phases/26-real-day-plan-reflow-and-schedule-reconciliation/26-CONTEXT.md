# Phase 26 Context

## Title

Real day-plan reflow and schedule reconciliation

## Why this phase exists

Phase 25 closed the bounded local-recall and grounded-assistant story, but Vel still stops short of turning `reflow` into a truly useful daily-use recovery lane.

This phase should explicitly learn from the operator's already-working scheduler model in `/home/jove/code/codex-workspace` rather than inventing a parallel approximation. The relevant current-truth references there are:

- `docs/scheduler.md`
- `docs/tui.md`
- `schemas/task-schema.md`
- `schemas/todoist-obsidian-alignment.md`
- `scripts/plan-todoist-to-calendar.js`

It should also preserve the relevant product feedback already captured in `/home/jove/Downloads/Vel.csv`, especially the items that affect daily-use recovery and schedule trust:

- stale / aging / fresh inputs should feel more like timed freshness bands than an obtrusive degraded-state warning
- thread continuity should stay easy to reopen from the latest relevant work
- subtle top-level context and status should remain visible without taking over the surface
- project context should stay rich enough to anchor schedule and action meaning when a reflow changes what belongs where
- settings and docs should support recovery contextually instead of forcing the operator into raw implementation detail first

The product already has:

- typed `reflow` cards and status
- `check_in` and thread escalation
- assistant entry and voice entry
- supervised action and approval seams

What is still missing is the practical day-repair behavior:

- detect when today has gone stale in a way the operator actually feels
- recompute the remaining plan instead of only surfacing a warning
- explain what moved, what no longer fits, and what still needs judgment
- keep this behavior backend-owned so Apple, web, CLI, and later desktop shells do not invent separate scheduling logic

The existing `codex-workspace` behavior that should inform Phase 26 includes:

- `block:<name>` targeting against routine free windows
- `cal:free` / `@cal:free` for transparent calendar events
- duration parsing from `@Nm` / `@Nh` markers and duration labels like `30m` / `1h`
- time-window labels such as `time:prenoon`, `time:afternoon`, `time:evening`, `time:day`, and `time:night`
- local urgent/defer flags that affect scheduling without mutating upstream Todoist labels
- fixed-start treatment for due datetimes
- "Didn't Fit" handling for tasks that no longer fit the day
- completion sync and scheduled-status reconciliation against actual surviving calendar events
- schedule rerun expectations when the day has already been fully scheduled but reality has changed

The `Vel.csv` feedback that should shape embodiment and policy for this phase includes:

- "the some inputs are degraded view should be less obtrusive... things should automatically refresh instead"
- "fresh / aging / stale should show some sort of time: Now, <1m, <5m, <30m, >1hr. etc"
- "the threads view should auto open the latest thread"
- "there should some sort of top navbar that always shows time, status, active jobs, link to help, connected clients, etc (should be kept subtle)"
- "documentation should be able to be rendered on every page from its markdown files and be contextual based on route / focused section"
- "projects should have description field ... tags ... color coded ... project specific view ..."
- "there should be template viewing and editing in settings ui"

This is the most direct usability step after bounded recall because it helps Vel stay trustworthy once the day diverges from the original plan.

## Intended outcome

Vel should be able to treat `reflow` as a real recovery lane for missed events, slipped blocks, stale schedule state, and changed task/calendar reality.

The backend should own:

- reflow trigger detection
- recomputed remaining-day proposals
- explicit moved / unscheduled / blocked outcomes
- operator-visible explanations and follow-through paths
- preservation of the operator's practical scheduling semantics from `codex-workspace` where they map cleanly into Vel's current project/action/timing model

The shells should mainly:

- surface the reflow suggestion
- show the compact diff/summary
- route the operator into inline accept or thread-based edit/review

This phase should also include the Settings-facing implementation needed to make the new recovery model usable:

- Settings should reflect the real freshness and recovery posture behind reflow instead of only generic assistant-readiness text
- the operator should be able to see relevant schedule-recovery context from Settings without dropping into raw runtime internals first
- contextual setup/help copy that affects reflow trust should be updated in the same phase rather than deferred into a later doc-only cleanup

## Phase boundary

In scope for planning:

- stale schedule and missed-event recovery behavior
- current-day recomputation and reconciliation contracts
- explainable reflow output shapes
- bounded shell embodiment for the new output
- matching Settings-page embodiment for freshness, recovery, and schedule-trust guidance

Out of scope:

- broad multi-day planning
- speculative autonomous calendar mutation without supervision
- hosted scheduling infrastructure
- a brand-new parallel planner subsystem outside the current daily-loop / `Now` / `Threads` model
- blindly copying all astrology- or workspace-specific behavior before the core reflow contract is stable

## Key references

- `.planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams/15-03-SUMMARY.md`
- `.planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces/16-03-SUMMARY.md`
- `docs/product/operator-action-taxonomy.md`
- `docs/product/operator-mode-policy.md`
- `docs/user/daily-use.md`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/now.rs`
- `/home/jove/Downloads/Vel.csv`
- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/docs/tui.md`
- `/home/jove/code/codex-workspace/schemas/task-schema.md`
- `/home/jove/code/codex-workspace/schemas/todoist-obsidian-alignment.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`

## Initial requirement direction

- `REFLOW-REAL-01`: backend-owned reflow should produce an explicit recomputed remaining-day proposal rather than only a warning/state marker
- `REFLOW-REAL-02`: reflow output should explain what changed, what moved, and what no longer fits
- `SCHED-RECON-01`: stale schedule, missed event, and slipped-block recovery should use one canonical reconciliation path
- `SCHED-RECON-02`: reflow follow-through should stay supervised and inspectable through the existing `Now` / `Threads` / approval seams

Derived migration note from `codex-workspace`:

- the exact Todoist tagging and local scheduling-rule system should be mapped intentionally during planning, not rediscovered ad hoc during implementation

Derived product note from `Vel.csv`:

- reflow and schedule recovery should improve trust and actionability without making the shell noisier; freshness, context, and recovery cues should become clearer and more subtle at the same time
- the Settings page changes that expose or explain recovery posture should ship as implementation work in this lane, not as an afterthought

## Next step

Plan the phase into execution slices before implementation.
