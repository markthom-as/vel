# 35 Research

## Goal

Turn the current-day model from clock-midnight assumptions into a sleep-relative day boundary that matches how `Now`, commitments, routines, and continuity should behave for one working day that can extend past midnight.

## Inputs

- operator interview decisions captured in [35-CONTEXT.md](/home/jove/code/vel/.planning/phases/35-sleep-relative-day-boundary-and-today-lane-correctness/35-CONTEXT.md)
- `codex-workspace` scheduling semantics and late-night continuity model
- `~/Downloads/Vel.csv` pressure around continuity, simplification, and richer context
- shipped `GET /v1/now` / `day_plan` / `reflow` behavior from Phase 34

## Key Findings

- the next correctness step is not more UI rearrangement; it is one backend-owned definition of “today”
- the day boundary should prefer explicit/structured sleep signals when available and only fall back to weaker heuristics later
- `Now`, next-event logic, commitments, reflow, and resurfaced continuity all need the same day-window substrate or the shell will drift again
- routine blocks should continue to shape the active day, but must not be treated as generic upcoming events
- Phase 35 should stay same-day and local-first; it should not widen into multi-day planning or general sleep inference infrastructure

## Risks

- mixing timezone fixes, day-boundary changes, and ranking changes in one patch will make regressions hard to debug
- if the shell invents day-window logic locally, Apple/web parity will regress immediately
- overreaching into heavy inference heuristics before explicit signal handling exists will create untrustworthy edge cases

## Recommended Shape

1. publish a typed day-boundary/current-day contract
2. move `Now` and schedule selection onto that shared contract
3. align continuity and lane ordering to the new current-day substrate
4. close docs and verification with explicit late-night acceptance cases
