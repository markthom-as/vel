# Trigger and Scheduling Model

## Trigger classes

The workflow system should support four trigger classes from the beginning.

### 1. Manual triggers

Initiated by user action from:

- CLI
- UI button/menu
- thread command
- keyboard shortcut
- webhook/manual API call

### 2. Scheduled triggers

Time-based orchestration:

- cron-like recurring schedules
- fixed dates and times
- windows such as morning, evening, weekly review
- timezone-aware execution
- missed-run policy (`catch_up`, `skip`, `queue_one`)

### 3. Event triggers

Run on explicit domain events:

- calendar event created/changed
- task overdue
- task completed
- thread idle for N hours
- project status changed
- note added
- session started
- microphone capture ended

### 4. State/condition triggers

Run when derived conditions become true:

- project has overdue blockers and no active nudge
- task lacks metadata and deadline < 24h
- calendar conflict + commitment overlap
- thread unresolved after repeated exchanges

These can be implemented as evaluated rules or as workflows fed by periodic detector workflows.

## Trigger payload contract

Every trigger should produce a normalized payload:

```json
{
  "trigger": {
    "id": "app-day-start",
    "type": "event",
    "firedAt": "2026-03-21T08:00:00-06:00",
    "source": "vel.app"
  },
  "subject": {
    "kind": "session",
    "id": "sess_123"
  },
  "related": [
    {"kind": "project", "id": "proj_home"},
    {"kind": "thread", "id": "thr_today"}
  ],
  "attributes": {
    "day_start": true,
    "timezone": "America/Denver"
  }
}
```

The runtime then resolves context bindings from the payload plus registry/context services.

## Trigger governance

Triggers should be governed by:

- enabled/disabled state
- rate limits
- deduplication key
- cool-down windows
- workspace/user scoping
- quiet hours and Do Not Disturb
- dependency health
- policy allow/deny rules

## Scheduling engine requirements

MVP scheduling engine should support:

- cron + simple intervals
- per-user timezone
- durable pending jobs
- recovery after restart
- at-most-once or idempotent execution semantics
- visible next-run preview

Later:

- calendar-native schedule expressions
- blackout periods
- holiday rules
- follow-the-sun / locale-aware templates
- resource-based rate shaping
