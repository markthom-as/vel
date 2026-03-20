# Phase 35 Context

## Title

Sleep-relative day boundary and today-lane correctness

## Why this phase exists

Fixing `Now` presentation without fixing the operator-day model would only harden another shallow shell correction.

The operator has explicitly defined “today” as the chunk between sleeps, not the chunk between midnights. That means late-night work, unfinished commitments, night events, and routine continuity should remain in one operator day until the sleep boundary is crossed.

## Product problem

Right now different surfaces can still drift on what counts as:

- today
- current
- next
- resurfaced

If `Now`, next-event truth, standup output, and thread resurfacing do not share the same day model, the shell will remain confusing even after UI cleanup.

## Phase goal

Make one sleep-relative current-day truth govern `Now`, next-event calculation, commitment ordering, task demotion, and contextual resurfacing across surfaces.

## Must stay true

- calendar event still outranks commitment, routine, and inference for “what is happening now”
- tasks do not outrank commitments unless promoted
- next event stays calendar-driven
- the day can cross midnight until sleep is detected
- if contextual thread confidence is low, show none

## Key operator inputs

- primary day-boundary signals should be multi-signal, with Apple bedtime/routine signals preferred where available
- the system should preserve continuity across midnight instead of fragmenting the day artificially
- `Now` should say:
  - `Free until [event]` when the next event is soon
  - `Unstructured time` when daytime but no stronger structure exists
  - inferred task only when confidence is strong
  - `Between blocks` when truly ambiguous

## Likely touch points

- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/day_plan.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/timezone.rs`
- Apple/runtime current-context seams
- `clients/web/src/components/NowView.tsx`
- `clients/apple`

## Expected next step

Phase 35 planning should break this into:

1. sleep-relative day-boundary contract publication
2. backend implementation of multi-signal day truth
3. commitment-first ordering and resurfacing alignment
4. cross-surface verification of current-day correctness
