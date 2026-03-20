# Phase 22 Context

## Purpose

Phase 22 exists because the daily loop, closeout flows, and thread-based item resolution are already core product concepts, but they are still fragmented across route-specific and shell-specific entry points. After the grounded assistant seam and voice parity are in place, the next product step is to make those workflows assistant-capable without creating a second planning system.

## Product Direction

The operator explicitly wants:

- morning/daily standing support through the assistant
- end-of-day closeout support
- thread-based resolution of items

This phase should build on the existing typed authorities:

- morning overview and standup stay owned by the daily-loop session model
- check-in and reflow remain canonical backend actions
- threads remain the durable deeper-interaction surface

The assistant should help operators move through those workflows, not replace the underlying product contracts.

## Expected Focus

1. Assistant-capable morning and standup
   - start or resume daily-loop work through the grounded assistant
   - preserve typed prompts, responses, and session continuity
   - keep commitment and deferral outcomes inspectable

2. Assistant-capable closeout
   - end-of-day should become a first-class assistant entry path
   - summaries, unresolved items, and drift remain explainable from persisted state
   - no shell-local closeout heuristics

3. Thread-based resolution
   - longer check-in, reflow, and item-resolution work escalates into durable threads
   - thread history records why an item was resolved, deferred, edited, or left pending
   - `Now` and `Inbox` stay summary/triage surfaces rather than turning into archives

## Non-Goals

- inventing a parallel assistant-only planning system
- flattening all daily-loop work into freeform chat
- widening the assistant into broad writeback authority

## Inputs

- [docs/product/operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md)
- [docs/product/now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md)
- [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- the shipped Phase 10 daily-loop seams
- the shipped Phase 15-16 operator action and thread escalation seams

## Exit Condition

Phase 22 is successful when the operator can enter morning, standup, closeout, and longer item-resolution work through the assistant/thread model without losing typed backend ownership or explainability.
