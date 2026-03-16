---
id: VSM-017
title: Autonomy Budgets and Rate Limits
status: proposed
priority: P1
owner: platform
labels: [budgets, rate-limits, autonomy, safety]
---

## Summary
Implement per-day, per-subsystem, per-class quotas for autonomous changes.

## Why
Even good autonomous changes can become pathological in volume. Nothing says “I helped” like 37 prompt rewrites before breakfast.

## Scope
- Budget definitions for class/subsystem/environment.
- Hard blocks and operator override path.
- Metrics and alerts on budget exhaustion.

## Implementation tasks
1. Define quota model.
2. Add counter storage and reset semantics.
3. Integrate checks into apply decision path.
4. Emit metrics/events on exhaustion.
5. Add override mechanism for approved emergencies.

## Acceptance criteria
- Auto-apply halts when quota exceeded.
- Budgets are configurable.
- Operators can see why a proposal stopped.
- Exhaustion events are auditable.

## Dependencies
- VSM-006, VSM-011.

