
# vel — Explicit Implementation Directive (Post‑Phase‑A)

Audience: coding agent implementing Vel
Purpose: concrete implementation steps after Commitments (Phase A)

This document translates the behavioral constitution into **explicit modules,
schemas, and implementation order**. Follow this order strictly.

---
# Implementation Order

1. signals/events ingestion
2. persistent current_context
3. commitment dependency graph
4. risk model
5. nudge engine
6. suggestion / steering loop
7. explanation surfaces
8. thread graph
9. weekly synthesis scaffold

Each subsystem depends on the previous ones.

---
# 1. Signals / Events System

Create crate:

crates/vel-signals

Purpose: normalize all incoming facts.

## Migration

signals
-------
id
signal_type
source
timestamp
payload_json
created_at

## Initial signal types

calendar_event
todoist_task
todoist_completed
shell_login
vel_capture
nudge_ack
nudge_snooze

## Requirements

- append-only storage
- deduplicate identical signals
- index by timestamp + type

## API

record_signal()
list_signals()
latest_signal_of_type()

---
# 2. Persistent Current Context

Create crate:

crates/vel-context

Vel must maintain a persistent **current_context** snapshot.

## Schema

current_context
---------------
id
computed_at
context_json

Example:

{
  "morning_state": "...",
  "next_commitment": "...",
  "prep_window_active": true,
  "commute_window_active": false,
  "meds_status": "pending",
  "active_nudges": [],
  "current_risk_level": 0.2,
  "inferred_activity": "computer_active",
  "open_threads": []
}

## Function

recompute_context(signals, commitments)

Triggered on:
- new signal
- periodic reconciliation later

## CLI

vel context
vel context timeline

---
# 3. Commitment Dependencies

Migration:

commitment_dependencies
-----------------------
id
parent_commitment_id
child_commitment_id
dependency_type

Examples

meeting -> prep
meeting -> commute
grant -> finalize_budget

## Functions

add_dependency()
list_dependencies()
compute_dependency_risk()

---
# 4. Risk Model

Create crate:

crates/vel-risk

Purpose: compute dynamic risk score for commitments.

## Function

compute_risk(commitment, context)

## Inputs

- consequence of failure
- time proximity
- dependency pressure
- uncertainty

Example heuristic

risk = consequence * proximity_weight * dependency_factor

## Storage

commitment_risk
---------------
commitment_id
risk_score
computed_at

## CLI

vel risk
vel risk <commitment_id>

---
# 5. Nudge Engine

Create crate:

crates/vel-nudges

Pipeline:

signals
→ context recompute
→ risk recompute
→ nudge policy evaluation

## Initial policies

meds_not_taken
meeting_prep_window
commute_leave_time
morning_drift

## Schema

nudges
------
id
nudge_type
level
state
commitment_id
created_at
snoozed_until
resolved_at

Levels

gentle
warning
danger

## Acknowledgement

Done
Snooze

## CLI

vel nudges
vel done <nudge>
vel snooze <nudge>

---
# 6. Suggestion / Steering Loop

Migration

suggestions
-----------
id
suggestion_type
payload_json
state
created_at
resolved_at

Examples

increase_commute_buffer
increase_prep_window
add_operational_commitment

## CLI

vel suggestions
vel suggestion accept <id>
vel suggestion reject <id>
vel suggestion modify <id>

---
# 7. Explanation Surface

Commands

vel explain nudge <id>
vel explain context
vel explain commitment <id>

Output should include:

signals used
context state
risk factors
rule triggered
decision outcome

Initial output may be JSON.

---
# 8. Thread Graph

Create crate:

crates/vel-threads

## Schema

threads
-------
id
thread_type
title
created_at

thread_links
------------
thread_id
entity_type
entity_id

Entity types

commitments
captures
calendar_events
signals

## CLI

vel threads
vel thread inspect <id>
vel thread open

---
# 9. Weekly Synthesis Scaffold

Command

vel synthesize week

Inputs

signals
commitments
nudges
risk_history
threads

Outputs artifact

weekly_synthesis

LLM integration later.

---
# Engineering Rules

Rule 1

Never mix these layers

signals
context
risk
nudges
synthesis

Rule 2

LLMs are not used for

risk computation
context inference
nudge triggers

LLMs are used only for

reflective synthesis
alignment analysis
natural language explanation

Rule 3

Every state change must be observable.

Rule 4

Signals should be append‑only logs.

---
# End‑to‑End Success Example

calendar event imported
→ prep commitment inferred
→ commute inferred
→ risk increases
→ nudge fires
→ user snoozes
→ risk recalculated
→ escalation occurs

System must be explainable via

vel explain
