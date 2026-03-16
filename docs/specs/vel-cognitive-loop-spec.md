# Vel Cognitive Loop Architecture

## Goal

Define Vel's core cognition cycle.

Vel should operate as a continuous loop:

Observe → Evaluate → Suggest → Act → Reflect

This makes Vel a persistent cognitive prosthetic rather than a reactive chatbot.

---

# Core Loop

1 Observe
2 Evaluate
3 Suggest
4 Act
5 Reflect

Loop runs continuously.

---

## Observe

Inputs collected from:

- calendar
- reminders
- health logs
- sensors
- travel estimates
- user activity

Observation events stored in event store.

Example

{
 event_type: "calendar_meeting",
 start_time: "2026‑03‑15T10:00"
}

---

## Evaluate

Evaluation agents classify observations.

Example checks

- missed commitments
- medication timing
- travel risk
- cognitive load
- habit patterns

Outputs risk levels.

Example

{
 risk: "moderate",
 reason: "meeting soon and commute not started"
}

---

## Suggest

Vel generates non‑intrusive suggestions.

Examples

- "leave now to arrive on time"
- "you forgot medication"
- "you haven't moved in 2 hours"

Suggestions stored in suggestions table.

---

## Act

If permission exists, Vel may act.

Examples

- set reminder
- adjust schedule
- message collaborator
- prepare meeting notes

Actions require capability tokens.

---

## Reflect

Vel reviews its own decisions.

Reflection tasks

- suggestion success rate
- false alarms
- missed opportunities

Reflection output

{
 improvement: "reduce commute alerts by 10 minutes"
}

---

# Agent Types

Vel runs specialized agents.

Observer agents
Evaluation agents
Suggestion agents
Execution agents
Reflection agents

Each agent is implemented as a subagent with limited tools.

---

# Memory Compression

Vel periodically compresses logs.

Process

1 summarize past events
2 extract patterns
3 update topic pads
4 archive raw logs

---

# Cognitive Scheduling

Loops run on different cadences.

real‑time loop: seconds
behavior loop: minutes
reflection loop: hours
learning loop: days

---

# Safety Principles

Vel must follow constraints

- suggestions before actions
- human override always possible
- audit logs immutable
- high‑risk actions require confirmation

---

# Result

Vel becomes

- proactive
- reflective
- context aware
- continuously improving
