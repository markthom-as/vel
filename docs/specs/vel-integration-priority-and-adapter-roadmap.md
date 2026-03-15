# vel_integration_priority_and_adapter_roadmap.md

Status: Integration roadmap for Vel

Purpose: Define external data sources and their priority.

## Phase 1 (Required for MVP)

### Calendar
Fields required:
- title
- start_time
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
- due time
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

## Phase 2

### Transcript / Chat Ingestion
Sources:
- ChatGPT logs
- meeting transcripts
- captured voice sessions

Influences:
- thread graph
- ideation linking
- project synthesis

### Feedback Signals
Inputs:
- NPS style rating
- annoyance score
- usefulness score

Influences:
- self‑model
- nudge tuning

## Phase 3

### Apple Signals
- watch acknowledgements
- reminders completion
- wake / activity

Influences:
- signals
- attention detection

### Messaging Awareness
- conversation threads
- response debt
- scheduling negotiation

## Adapter Pattern

All integrations must emit canonical Vel signals:

signal {
  id
  source_type
  entity_type
  entity_id
  payload_json
  created_at
}

Adapters must:
1. normalize external data
2. emit signals
3. avoid business logic

## Sync Strategy

Adapters can run:
- polling
- webhook
- local watcher

All signals enter Vel via the signal ingestion pipeline.
