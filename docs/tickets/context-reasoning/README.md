---
title: Context Reasoning tickets
status: active
owner: agent
created: 2026-03-16
---

# Context Reasoning Ticket Pack

This pack is a design-and-implementation queue for improving context inspection, explanation, and feedback handling in Vel.

It is **not** a license to create a second source of truth for user state.

## Boundary

- The current runtime authority for present-tense context remains the existing `current_context`, `context_timeline`, and `explain/*` stack described in [docs/status.md](../../status.md).
- Work in this pack should extend or refine that runtime.
- New storage or APIs in this area should be justified as support for explainability, inspection, uncertainty handling, or feedback loops, not as a replacement for the current context reducer.
- Tickets in this pack should assume the existing reducer/runtime remains authoritative unless a broader architecture decision explicitly changes that.

## Use This Pack When

- improving inspectability of current context
- adding structured explanation or trace surfaces
- improving confidence, uncertainty, or feedback workflows around existing context decisions

## Use With Caution

- Do not introduce a parallel belief ledger unless there is an explicit decision to replace or subsume the current context runtime.
- Do not duplicate inference, explainability, or persistence responsibilities already owned by the existing context/reducer flow.
