# Day-Plan Reflow Contract

## Status

Implemented same-day reflow/reconciliation over the shared bounded planning substrate, not full planner closure.

This document records the canonical backend-owned contract for day-plan `reflow` after Phase 26 slices `26-01` through `26-03`.

For the proactive same-day planning contract that precedes recovery, see [day-plan-contract.md](./day-plan-contract.md).

## Purpose

`reflow` is the current-day recovery lane that turns schedule drift into an explicit remaining-day proposal.

It exists so shells can render schedule repair without owning the schedule logic themselves.

It is intentionally the recovery side of the same planning substrate rather than a separate planner.

## Current Contract

The backend-owned `ReflowCard` now carries an optional typed `proposal` with:

- aggregate counts for `moved`, `unscheduled`, and `needs_judgment`
- explicit `changes`
- explicit canonical `rule_facets`

The backend now fills that proposal from the current day's persisted commitments, calendar signals, and durable routine-planning profile when available:

- it computes remaining free windows for the rest of the day
- it subtracts durable protected routine blocks first, with inferred fallback only when no durable routine blocks are configured
- it applies bounded planning constraints such as default time-window preference, calendar buffers, and overflow judgment
- it attempts bounded same-day placement for movable work
- it marks work as `unscheduled` when it no longer fits
- it leaves fixed-time or missed-anchor situations in `needs_judgment`

Current rule-facet vocabulary is:

- `block_target`
- `duration`
- `calendar_free`
- `fixed_start`
- `time_window`
- `local_urgency`
- `local_defer`

These are the normalized Vel-side scheduler semantics that later slices can populate from upstream labels, tokens, and local operator flags.

## Mapping Rule

Raw upstream tags are not the durable product model.

The intended mapping direction is:

- Todoist labels or title tokens remain compatibility metadata at ingest
- Vel maps them into canonical scheduler facets and typed fields
- shells consume typed proposal output and explanation

That keeps the operator's proven `codex-workspace` rule system available without making raw provider syntax the long-term authority.

The current canonical normalization seam is documented in [canonical-scheduler-facets.md](./canonical-scheduler-facets.md).

The supervised application-layer contract for turning bounded same-day `reflow` output into explicit commitment scheduling changes is published separately in [day-plan-application-contract.md](./day-plan-application-contract.md).

## Shell Rule

Shells consume this typed output. They do not derive schedule logic locally.

Current embodiment rules:

- `Now` renders the compact reflow summary, counts, change rows, and rule-facet chips
- `Now` may also indicate whether the current day is using operator-managed routine blocks or inferred fallback
- `Threads` is the place for longer disagreement or manual schedule shaping
- `Settings` surfaces recovery posture and trust/freshness summary without becoming a second planner

## Current Limit

The shipped reflow lane is intentionally bounded.

Current limits:

- it is a same-day remaining-day repair path, not multi-day planning
- durable routine materialization is currently bounded to weekday/local-time templates rather than richer recurrence semantics
- fixed-time handling is still conservative and remains operator-supervised
- acceptance/editing continuity still relies on the existing supervised `reflow` / `Threads` path
- shipped supervised application of `reflow` outcomes to commitment scheduling is not yet claimed in this contract slice
- raw upstream tags are still compatibility metadata, even when their normalized meaning is surfaced as canonical `rule_facets`
