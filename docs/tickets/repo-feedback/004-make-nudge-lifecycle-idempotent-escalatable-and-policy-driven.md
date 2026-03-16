---
title: "Make the nudge lifecycle idempotent, escalatable, and fully policy-driven"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 001-enforce-evaluate-read-boundary.md
  - 002-refactor-inference-into-deterministic-reducers.md
  - 003-complete-risk-engine-and-make-it-the-only-risk-authority.md
labels:
  - vel
  - nudges
  - policy
  - behavior
---
The nudge engine has crossed the line from toy to meaningful subsystem. That means it now needs lifecycle discipline, not just creation logic.

At the moment the code appears to support create / resolve / snooze, but the behavior still looks closer to "emit helpful rows" than "operate a robust intervention state machine."

## What is missing

Vel needs nudges to behave like durable interventions with clear semantics:

- created once when conditions appear
- escalated when urgency rises
- suppressed when policy says "not now"
- snoozed without becoming forgotten
- auto-resolved when the world changes
- explainable after the fact

## Current concerns

- Existing-nudge checks look simplistic and could harden into "never duplicate, never escalate" behavior.
- Escalation policy for gentle -> warning -> danger is not obviously modeled as a lifecycle transition.
- Snooze semantics may suppress too much or too little depending on condition changes.
- Policy config is present, but some nudge behavior still feels partly hardcoded.

## Tasks

- Introduce explicit nudge lifecycle semantics:
  - active
  - snoozed
  - resolved
  - dismissed
  - escalated (event, not necessarily state)
- Distinguish "same nudge condition still active" from "same condition but materially higher urgency."
- Add append-only nudge events for:
  - created
  - escalated
  - snoozed
  - unsnoozed/reactivated
  - resolved
  - dismissed
- Make snooze behavior policy-aware:
  - snoozed nudges should reappear when snooze expires and condition still holds
  - severe conditions may bypass or shorten snooze depending on policy
- Centralize level thresholds in policy config, not inline branch soup.
- Add CLI/API visibility for current lifecycle and event history if not already surfaced cleanly.

## Acceptance Criteria

- Re-running evaluation is idempotent: it does not spam duplicate nudges.
- Escalation produces a clear state/event transition and updated explanation.
- Snooze + re-evaluate behaves predictably and is tested.
- Every active nudge can answer:
  - why it exists
  - what policy created it
  - what evidence it used
  - whether it has escalated before

## Notes for Agent

A nudge engine that cannot distinguish persistence from repetition becomes ambient guiltware. Vel should be annoying only when reality earns it.
