# Phase 27 Context

## Why this phase exists

Phase 26 made `reflow` real enough to repair the remaining day, but its scheduler semantics are still derived from raw labels and title tokens at runtime.

That is useful as a bridge, not as durable product truth.

Vel now needs a canonical scheduler-rule model that:

- preserves the proven `codex-workspace` rule system
- becomes SQL-backed and explainable
- can be consumed consistently by reflow, recall, agent grounding, and future planning logic

## Inputs to preserve

### `codex-workspace`

The exact local rule system lives in:

- `/home/jove/code/codex-workspace/docs/scheduler.md`
- `/home/jove/code/codex-workspace/docs/tui.md`
- `/home/jove/code/codex-workspace/schemas/task-schema.md`
- `/home/jove/code/codex-workspace/schemas/todoist-obsidian-alignment.md`
- `/home/jove/code/codex-workspace/scripts/plan-todoist-to-calendar.js`

Important semantics to preserve intentionally:

- `block:<name>`
- `cal:free` / `@cal:free`
- duration tags and tokens such as `@30m`, `30m`, `@1h`, `1h`
- time windows such as `time:prenoon`, `time:afternoon`, `time:evening`, `time:day`, `time:night`
- local urgent/defer semantics
- fixed-start due datetimes

### Vel user-story pressure

`/home/jove/Downloads/Vel.csv` already reinforced that:

- subtle contextual status is preferable to loud dashboard state
- project context should remain rich enough to anchor task and reflow decisions
- freshness and recovery should be summary-first and explainable
- thread continuity should be easy to reopen from the latest relevant work

## Phase goal

Turn scheduler semantics into canonical Vel-backed fields and facets instead of leaving them as:

- raw provider labels
- freeform title parsing only
- ad hoc runtime interpretation inside reflow

## Non-goals

- not a broad multi-day planner rewrite
- not a new provider-specific label system
- not autonomous schedule mutation beyond the existing supervised lanes
- not a shell-driven scheduling model

## Expected result

After this phase, Vel should be able to say:

- scheduling semantics are persisted in canonical Vel form
- agents and recall can inspect them directly
- reflow no longer depends primarily on re-parsing raw labels each time
- raw external labels remain compatibility metadata, not product truth
