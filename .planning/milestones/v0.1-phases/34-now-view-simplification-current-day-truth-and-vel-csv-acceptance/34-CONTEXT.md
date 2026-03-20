# Phase 34 Context

## Title

Now-view simplification, current-day truth, and Vel.csv acceptance

## Why this phase exists

Phases 20 through 33 materially improved backend seams, planning, continuity, assistant support, and supervised apply paths. The product brain is far stronger than the daily-use shell experience.

The operator has now explicitly called out that the current `Now` surface is overcomplicated, duplicated, and misaligned with both prior specs and `Vel.csv` usability pressure:

- too much low-value status and sync emphasis
- incorrect or duplicated schedule display
- incorrect next-event truth
- broken or missing actions
- weak calendar and Todoist rendering
- too much unclear, non-actionable text

This phase exists to repair that gap before more architectural expansion widens the wrong product shape.

## Product problem

`Now` should be the primary current-day control surface, but it currently behaves too much like a noisy dashboard.

The operator wants:

- a compact context bar
- clear current-status truth
- one dominant ask/capture/talk affordance
- a correct next event
- one unified today lane with commitments first and tasks second
- compressed attention indicators

The operator does not want:

- persistent sync posture or runtime noise in the main surface
- duplicated sections
- thread-centric clutter
- verbose status prose
- routine blocks polluting upcoming-event truth

## Phase goal

Turn `Now` into a compact, execution-first current-day surface and use `Vel.csv` as a regression/acceptance input while fixing the major truth, rendering, and action problems already visible today.

## Must stay true

- product authority comes from operator interview decisions plus prior specs, not `Vel.csv`
- `Vel.csv` is an acceptance and regression input
- `Now` follows perception → action → execution ordering
- the today lane is unified but commitment-first
- threads only resurface contextually and at most one at a time
- sync/debug posture moves to secondary surfaces instead of dominating `Now`

## Locked operator decisions

- top-to-bottom order:
  1. compact context bar
  2. current status
  3. ask/capture/talk
  4. next event
  5. today lane
  6. compressed attention strip
- active-item precedence:
  1. active calendar event
  2. active commitment
  3. routine block
  4. inferred activity
- next event remains strictly calendar-driven and excludes routine/noise blocks
- today lane order remains execution-first:
  1. active item
  2. next up
  3. must-do commitments
  4. should-do commitments
  5. pullable tasks
  6. recently completed
- primary quick actions are limited to:
  - complete
  - snooze/defer
  - promote to commitment
  - open

## External inputs to use

- `~/Downloads/Vel.csv`
- prior `Now` and shell taxonomy docs already in repo
- `codex-workspace` day/scheduler conventions only where they affect shipped current-day truth and filtering expectations

## Likely touch points

- `clients/web/src/components/NowView.tsx`
- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/types.ts`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `docs/user/daily-use.md`
- `docs/api/runtime.md`

## Expected next step

Phase 34 planning should break this into:

1. contract and acceptance publication for the corrected `Now`
2. truth repair for current status and next-event/calendar behavior
3. unified today-lane and primary-action cleanup
4. `Vel.csv`-anchored verification and remaining slop removal
