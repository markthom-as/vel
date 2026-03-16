# vel_integration_priority_and_adapter_roadmap.md

Status: Integration roadmap for Vel

Purpose: Define external data sources and their priority.

## Implemented Foundations

### Calendar
Fields required:
- title
- start / end
- location
- prep_minutes
- travel_minutes

Influences:
- commitments
- prep windows
- commute windows
- risk

### Todoist / Reminders
Fields:
- task title
- due
- completion status
- labels/project

Influences:
- commitments
- meds tracking
- nudges

### Computer Activity
Signals:
- login
- shell activity
- Vel invocation

Influences:
- morning_state
- drift detection
- attention state

### Git Activity
Signals:
- git activity snapshots
- branch / repo / operation metadata

Influences:
- inferred activity
- workstation/coding evidence
- explain surfaces

### Notes / Local Documents
Signals:
- replay-safe note document captures
- note document signals

Influences:
- recall
- project continuity
- later synthesis/review work

### Transcript / Chat Ingestion
Sources:
- ChatGPT logs
- assistant transcript snapshots

Influences:
- thread graph
- ideation linking
- project synthesis

### Messaging Awareness
- local messaging thread snapshots
- replay-safe `message_thread` signals
- waiting-state / urgency / scheduling metadata

Influences:
- response debt visibility
- scheduling negotiation awareness
- current-context explain surfaces
- deterministic `response_debt` nudges via current-context + policy

## Next Priority Extensions

### Feedback Signals
Inputs:
- NPS style rating
- annoyance score
- usefulness score

Influences:
- self‑model
- nudge tuning

### Calendar Hardening Follow-up
- recurrence fidelity
- richer attendee/travel/prep metadata
- better snapshot/update coverage

## Later Extensions

### Apple Signals
- watch acknowledgements
- reminders completion
- wake / activity

Influences:
- signals
- attention detection

## Adapter Pattern

All integrations must emit canonical Vel signals:

signal {
  id
  signal_type
  source
  source_ref
  timestamp
  payload_json
}

Adapters must:
1. normalize external data
2. emit replay-safe signals where stable external identity exists
3. avoid business logic

## Sync Strategy

Adapters can run:
- polling
- webhook
- local watcher

All signals enter Vel via the signal ingestion pipeline.
