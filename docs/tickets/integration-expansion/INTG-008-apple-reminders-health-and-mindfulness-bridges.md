---
id: INTG-008
title: Apple Reminders, Health, and Mindfulness bridges
status: proposed
priority: P1
estimate: 5-8 days
dependencies:
  - INTG-001
  - INTG-003
  - INTG-004
---

# Goal

Make Apple-native task and wellbeing data first-class and implementable without contaminating Rust core with Apple framework types.

# Scope

- Define bridge contracts for Apple client-side extraction of:
  - Reminders
  - Health samples
  - Mindfulness / meditation sessions
- Add canonical payloads for reminder/task, health metric, and meditation session ingest.
- Preserve local-first, operator-auditable sync flow.

# Deliverables

- bridge envelope schema
- adapter service interfaces
- Apple Reminders mapping to canonical task/commitment fields
- Apple Mindfulness mapping to canonical meditation session fields
- fixture JSON for bridge payloads

# Acceptance criteria

- Apple Reminders can participate in the tasks family alongside Todoist.
- Apple Mindfulness or meditation sessions have a canonical place in wellbeing.
- Rust core remains provider-agnostic and bridge-driven.

# Notes

If Apple support requires importing EventKit or HealthKit concepts into Rust core, the boundary is wrong.
