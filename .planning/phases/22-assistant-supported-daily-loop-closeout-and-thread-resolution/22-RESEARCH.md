# Phase 22 Research

## Domain

Assistant-supported daily loop, closeout, and thread resolution over existing canonical backend seams.

## Locked Inputs

- Phase 10 already established typed backend daily-loop authority.
- Phase 15-16 already established canonical `check_in`, `reflow`, trust/readiness, and thread-escalation seams.
- Phase 20-21 already established the grounded assistant entry seam and cross-surface voice continuity rules.
- `Now` remains summary/pressure, `Inbox` remains triage, and `Threads` remains the durable deeper interaction surface.

## Problem

The product now has:

- grounded assistant entry
- typed daily-loop authority
- typed `check_in` / `reflow` actions
- shared thread continuity

But these still behave like adjacent systems instead of one operator workflow. Morning/standup still rely on dedicated daily-loop entry points, end-of-day is still a separate context/review path, and longer item resolution still depends on shell- or route-specific handoff logic.

Phase 22 should close that gap without flattening the system into freeform chat.

## Required Truths

1. Assistant-capable morning and standup
   - assistant entry can start or resume the canonical daily-loop flow
   - typed daily-loop prompts, outcomes, and inspectability remain intact

2. Assistant-capable closeout
   - end-of-day becomes a first-class assistant-capable backend flow
   - the assistant can guide closure without inventing shell-local heuristics

3. Durable thread resolution
   - longer `check_in`, `reflow`, and operator-action work escalates into typed thread continuity
   - thread history preserves why work was resolved, deferred, edited, or left pending

## Recommended Execution Shape

Phase 22 should be executed in four slices:

1. shared assistant-to-daily-loop routing and typed continuity seam
2. assistant-capable end-of-day / closeout seam
3. canonical thread-resolution follow-through for `check_in`, `reflow`, and action items
4. shell/docs verification closure across web, Apple, and CLI

## Code Context

- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/chat/assistant.rs`
- `crates/veld/src/services/chat/tools.rs`
- `crates/veld/src/services/daily_loop.rs`
- `crates/veld/src/services/check_in.rs`
- `crates/veld/src/services/reflow.rs`
- `crates/veld/src/services/context_runs.rs`
- `crates/veld/src/services/threads*`
- `clients/web/src/components/NowView.tsx`
- `clients/apple/Apps/VeliOS/ContentView.swift`
- `crates/vel-cli/src/commands/`

## Risks

- rebuilding a parallel assistant-only planner instead of reusing typed daily-loop and action seams
- letting shell code decide when something is a closeout or thread-resolution flow
- losing the explanation trail for why an item was resolved or deferred

## Success Condition

Phase 22 is complete when morning, standup, end-of-day, and longer action resolution can all be entered through the grounded assistant/thread model while remaining typed, inspectable, and backend-owned.
