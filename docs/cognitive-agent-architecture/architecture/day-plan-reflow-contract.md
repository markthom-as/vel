# Day-Plan Reflow Contract

## Status

Implemented same-day reflow/reconciliation over the shared bounded planning substrate, not full planner closure.

This document records the canonical backend-owned contract for day-plan `reflow` for milestone `v0.2`.

For the proactive same-day planning contract that precedes recovery, see [day-plan-contract.md](./day-plan-contract.md).
For the active `v0.2` MVP-loop authority that constrains how reflow appears in overview, escalation, and review, see [mvp-loop-contracts.md](./mvp-loop-contracts.md).

## Purpose

`reflow` is the current-day recovery lane that turns schedule drift into an explicit remaining-day proposal.

It exists so shells can render schedule repair without owning the schedule logic themselves.

It is intentionally the recovery side of the same planning substrate rather than a separate planner.

## Current Contract

The backend-owned reflow lane is expressed through two typed surfaces:

- `ReflowCard` for the live same-day recovery proposal and inline review posture
- `CurrentContextReflowStatus` for the durable applied or escalated state after the operator accepts or moves the work into `Threads`

`ReflowCard` carries an optional typed `proposal` with:

- aggregate counts for `moved`, `unscheduled`, and `needs_judgment`
- explicit `changes`
- explicit canonical `rule_facets`

Its proposal and supervision posture is carried by existing typed fields rather than shell-local state:

- `trigger` and `severity` explain why the recovery lane exists
- `accept_mode` and `transitions` explain whether inline apply remains review-gated
- `edit_target` and later `reflow_status` explain whether the work moved into `Threads`

The backend fills that proposal from the current day's persisted commitments, calendar signals, and durable routine-planning profile when available:

- it computes remaining free windows for the rest of the day
- it subtracts durable protected routine blocks first, with inferred fallback only when no durable routine blocks are configured
- it applies bounded planning constraints such as default time-window preference, calendar buffers, and overflow judgment
- it attempts bounded same-day placement for movable work
- it marks work as `unscheduled` when it no longer fits
- it leaves fixed-time or missed-anchor situations in `needs_judgment`
- it should escalate ambiguous or review-gated cases into `Threads` rather than widening into shell-local planner logic

The intended outcome vocabulary for `v0.2` is:

- `moved`: work can still be placed safely in the remaining day
- `unscheduled`: work no longer fits the remaining day without violating the bounded rules
- `needs_judgment`: the runtime can explain the pressure but should not silently choose the outcome

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
- `Now` may render review-gated inline acceptance only when the backend keeps the case bounded and explicit
- `Threads` is the place for longer disagreement or manual schedule shaping
- ambiguous or review-gated reflow cases should move into thread continuation rather than staying as implicit shell state
- `Settings` surfaces recovery posture and trust/freshness summary without becoming a second planner

## Current Limit

The shipped reflow lane is intentionally bounded.

Current limits:

- it is a same-day remaining-day repair path, not multi-day planning
- it does not add local-calendar milestone work to `v0.2`
- durable routine materialization is currently bounded to weekday/local-time templates rather than richer recurrence semantics
- fixed-time handling is still conservative and remains operator-supervised
- acceptance/editing continuity relies on the existing supervised `reflow` / `Threads` path and typed `reflow_status`
- supervised application of actionable reflow outcomes can reuse the bounded commitment-scheduling seam, but that does not widen reflow into autonomous planner writeback
- raw upstream tags are still compatibility metadata, even when their normalized meaning is surfaced as canonical `rule_facets`
