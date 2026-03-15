
# vel — Next Implementation Instructions (Post‑Phase‑A)

Audience: coding agent continuing the Vel re‑implementation  
Prerequisite: Phase A (Commitments) is implemented and working

This document defines the **next implementation phases** after Commitments.
The goal is to move Vel from “structured capture” toward **context awareness and nudging**.

The phases must be implemented in order. Do not skip ahead.

---

# Phase B — Signal Ingestion

Purpose: Vel must ingest external signals so it can reason about context.

Only implement **three signal adapters initially**.

## B1 Calendar Adapter

Source:
- Google Calendar or Apple Calendar export / API

Minimum fields to ingest:

```
event_id
title
start_time
end_time
location
calendar_name
tags (optional)
```

Derived fields:

```
prep_minutes
travel_minutes
prep_start = start_time - prep_minutes
leave_by = start_time - travel_minutes
```

Implementation tasks:

1. Create signal type `calendar_event`
2. Add adapter module:
   ```
   vel-signals/adapters/calendar
   ```
3. Normalize events into Vel signals
4. Persist signals in `signals` table

Schema suggestion:

```
signals
-------
signal_id
signal_type
source
timestamp
payload_json
```

Example payload:

```
{
  "event_id": "...",
  "title": "Meeting with Dimitri",
  "start": 1700000000,
  "prep_minutes": 30,
  "travel_minutes": 40
}
```

---

## B2 Todoist / Reminders Adapter

Source:

```
data/todoist/snapshot.json
```
or API.

Purpose:

Translate tasks into **commitments or completion signals**.

Required fields:

```
task_id
text
completed
due_time
labels
project
priority
```

Behavior:

1. Import tasks as signals
2. If task is open → ensure commitment exists
3. If task becomes completed → mark commitment `done`

Example mapping:

Todoist task:

```
Take meds
labels: ["health"]
```

Vel commitment:

```
kind: medication
source_type: todoist
state: open
```

Adapter location:

```
vel-signals/adapters/todoist
```

---

## B3 Computer Activity Adapter

Purpose: detect workstation engagement.

Signals to record:

```
machine_login
shell_start
vel_invocation
keyboard_activity (later)
```

Signal example:

```
{
  "type": "shell_login",
  "timestamp": 1700000000
}
```

These signals help infer:

```
morning_started
engaged_at_workstation
```

Adapter location:

```
vel-signals/adapters/activity
```

---

# Phase C — Inference Engine

Purpose: transform signals into inferred context state.

Create module:

```
vel-inference
```

The engine should run periodically.

Inputs:

```
signals
commitments
calendar events
current time
```

Outputs:

```
inferred_state records
```

---

## C1 Morning State Machine

Implement the states defined in the spec.

States:

```
inactive
awake_unstarted
underway
engaged
at_risk
```

Transitions are triggered by:

```
calendar proximity
medication completion
shell login
elapsed time
```

Example:

```
if calendar_event_today and no_activity:
    state = awake_unstarted
```

Persist inferred state:

```
inferred_state
--------------
state_id
state_name
confidence
timestamp
context_json
```

---

## C2 Prep Window Detection

Using calendar signals:

```
prep_start = start_time - prep_minutes
```

Inference rule:

```
if now >= prep_start and commitment incomplete:
    prep_window_active = true
```

---

## C3 Medication Status

Inference rule:

```
if meds_commitment open and time > meds_expected:
    meds_pending = true
```

---

# Phase D — Nudge Engine

Purpose: convert inferred states into actionable nudges.

Create module:

```
vel-nudges
```

Architecture:

```
signals
→ inference engine
→ nudge engine
→ notification adapters
```

---

## D1 Supported Nudge Types (v1)

Only implement three types.

```
meds_not_logged
meeting_prep_window
morning_drift
```

Each has escalation levels:

```
gentle
warning
danger
```

---

## D2 Nudge Schema

```
nudges
------
nudge_id
nudge_type
level
state
related_commitment
created_at
snoozed_until
resolved_at
metadata_json
```

States:

```
pending
active
snoozed
resolved
```

---

## D3 Done / Snooze Protocol

All nudges must support only two responses.

```
Done
Snooze
```

CLI examples:

```
vel done meds
vel snooze meds 10m
```

Effects:

Done:
- mark commitment resolved
- close nudge

Snooze:
- set `snoozed_until`
- requeue nudge

---

# Phase E — Notification Adapters

Separate delivery from logic.

Adapters:

```
cli
desktop_toast
watch_notification
```

Implementation order:

1. CLI messages
2. Desktop notifications
3. Watch support (later integration)

Architecture:

```
nudge engine
→ notification adapter
→ delivery
```

---

# Phase F — Weekly Synthesis

Purpose: allow Vel to improve Vel.

Command:

```
vel synthesize week
```

Input:

```
commitments
nudges
signals
captures
```

Output artifact:

```
weekly_synthesis
```

Possible insights:

```
most frequent unresolved commitments
repeated snoozed tasks
morning drift frequency
projects receiving attention
```

Persist as artifact.

---

# Phase G — Observability

Every important event must be traceable.

Emit events:

```
signal_ingested
commitment_created
commitment_resolved
state_changed
nudge_generated
nudge_escalated
nudge_resolved
```

This enables debugging and later synthesis.

---

# Development Order

The coding agent should implement phases in this order.

```
Phase B — Signal ingestion
Phase C — Inference engine
Phase D — Nudge engine
Phase E — Notification adapters
Phase F — Weekly synthesis
Phase G — Observability
```

Do not attempt to implement sensors, smart home integrations, or complex AI logic yet.

---

# Success Criteria

Vel is considered functional when:

- calendar ingestion works
- Todoist ingestion works
- workstation activity signals exist
- commitments can be auto‑generated
- morning state is inferred
- meds reminder fires
- meeting prep reminder fires
- user can respond Done/Snooze
- nudges escalate by time proximity
- weekly synthesis produces an artifact

At that point Vel becomes usable for daily operation.

---

# Final Guidance

Keep the system **small, predictable, and observable**.

The goal of this phase is not to build a perfect life automation engine.

The goal is to ship a system that can:

- see commitments
- detect drift
- nudge at the right time
- learn from its own history

Everything else can evolve after dogfooding.
