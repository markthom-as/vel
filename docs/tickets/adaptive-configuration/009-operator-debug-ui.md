---
id: vel-adaptive-config-009
title: Add operator-facing effective config and audit debug UI
status: proposed
priority: P2
owner: frontend
---

## Summary
Expose base vs effective settings, active signals, active policies, provenance, and recent config events in a debug/operator UI.

## Scope
- effective config inspector
- provenance per key
- active profile/policy badges
- recent config event timeline
- replay/simulation panel
- "make temporary change my default" affordance

## Acceptance Criteria
- operators can answer “why did Vel do this?” without spelunking logs
- temporary vs durable values are clearly separated
- redacted evidence remains redacted in user mode

## Notes
Do not bury provenance under five clicks and a blood oath.
