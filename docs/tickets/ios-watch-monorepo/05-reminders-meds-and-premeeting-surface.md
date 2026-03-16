---
id: APPLE-005
title: Implement today surface for reminders meds and pre-meeting readiness
status: proposed
owner: agent
priority: p0
area: feature/today
depends_on: [APPLE-004]
---

# Goal

Implement the core mobile utility surface around the exact use case already identified in Vel discussions: meds and temporal proximity reminders before meetings.

# Functional scope

- Show due / overdue tasks
- Show medication state:
  - due
  - taken
  - skipped
  - snoozed
  - unknown / unconfirmed
- Show pre-meeting readiness windows:
  - e.g. "meeting in 60m"
  - meds not confirmed
  - leave soon / prepare soon
- Show risk/severity tone derived from existing policy logic

# Requirements

- derive display from shared policy outputs where possible
- avoid hardcoding logic in view layer
- support quick actions:
  - mark taken
  - snooze X minutes
  - skip with reason
  - open details

# Product note

The user explicitly called out the case where they sometimes forget to mark meds taken versus sometimes have actually not taken them. The UI should preserve this ambiguity rather than flatten it into fake certainty.

Model separate concepts:

- reminder delivered
- user acknowledged
- user confirmed taken
- inferred unresolved

# Acceptance criteria

- at least one fixture and one live-path model for pre-meeting meds reminder
- state transitions generate local events
- distinction between "not checked off" and "not taken" is preserved in data model and UI
