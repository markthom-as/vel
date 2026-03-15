
# vel_agent_next_implementation_steps.md

Audience: Coding agent implementing Vel  
Purpose: Provide explicit instructions for the next implementation phase.

The system currently has scaffolding for:
- commitments
- nudges
- policies
- current context
- distributed architecture
- signal ingestion
- artifacts and synthesis specs

However several **core functional layers remain incomplete**.

This document defines the **required implementation order and constraints** so the system evolves safely without architectural drift.

---

# Implementation Order (Strict)

Implement the following in this order:

1. **Integration Tests**
2. **Risk Engine Compute**
3. **Suggestions / Steering Loop**
4. **Project Synthesis (`vel synthesize project`)**

Do not change this order.

---

# 1. Integration Tests (First)

Before adding more logic, add integration tests that enforce behavioral invariants.

These tests must exist before implementing risk compute and suggestions.

## 1.1 Commute Policy Tests

Test behavior for the commute policy.

Required cases:

### No commute when travel_minutes missing
Input:
- calendar event exists
- `travel_minutes` not set

Expected:
- commute nudge must **not trigger**

### Threshold escalation

Simulate time progression.

Expected sequence:

gentle → warning → danger

### Snooze suppression

When snoozed:

- no escalation occurs until `snoozed_until` passes

### Resolution

If commute commitment marked done:

- escalation stops
- nudge resolved

### Event start

When event start time passes:

- commute nudge resolves automatically

---

## 1.2 Context Explain Tests

Tests for `/v1/explain/context`.

Response must include:

- signal IDs used in computation
- commitment IDs used in computation
- factors that influenced context

Tests:

### Change detection
When input signals change:
- context explain output must change

### Isolation
Unrelated signals/commitments must **not appear** in explain output.

---

## 1.3 Resolution Order Tests

Resolution must occur before escalation.

Required behaviors:

### Resolved nudge never escalates
If resolved:
- escalation logic must not re-trigger

### Completed commitment suppresses nudge
If commitment done:
- no further escalation

### Done dominates snooze
If both appear in replication stream:
- `done` wins

---

# 2. Risk Engine Compute

After tests pass, implement risk computation.

Reference: `vel_risk_engine_spec.md`

Implement only the following components initially:

- consequence
- proximity
- dependency pressure

Do NOT implement yet:

- uncertainty scoring
- progress penalty
- adaptive learning
- LLM evaluation

Keep the first implementation deterministic and inspectable.

---

## 2.1 Risk Inputs

Risk engine consumes:

- commitments
- dependency graph
- due times
- current context
- wall clock time

---

## 2.2 Consequence Mapping

Initial heuristic mapping:

| Commitment Type | Consequence |
|-----------------|------------|
| calendar event with participants | high |
| operational dependency (prep/commute) | inherit parent * 0.8 |
| self task | medium |
| meds linked to commitment | high |

These mappings should be configurable later.

---

## 2.3 Proximity

Compute proximity pressure based on distance to due time.

Simplify initially using buckets:

| Time Remaining | Proximity |
|----------------|----------|
| > 2 hours | low |
| 30m – 2h | medium |
| < 30m | high |
| overdue | critical |

Avoid fake precision.

---

## 2.4 Dependency Pressure

Simple one-level propagation.

If:

parent risk >= high  
AND child unresolved

Then:

child risk increases proportionally.

No recursive graph traversal yet.

---

## 2.5 CLI Inspection

Add commands:

```
vel risk
vel risk <commitment_id>
```

Output must include:

- risk score
- risk level
- contributing factors
- dependency ids
- reason strings

---

# 3. Suggestions / Steering Loop

Implement after risk engine is functioning.

Reference: `suggestions` table from migration 0017.

The goal is to allow Vel to propose improvements to configuration or planning heuristics.

---

## 3.1 First Suggestion Types

Implement only two suggestion types initially:

### increase_commute_buffer

Triggered when:
- repeated danger commute nudges
- same route/context repeatedly late

Payload example:

```
{
  "type": "increase_commute_buffer",
  "current_minutes": 20,
  "suggested_minutes": 30
}
```

### increase_prep_window

Triggered when:
- repeated prep warnings near meeting start

Payload example:

```
{
  "type": "increase_prep_window",
  "current_minutes": 30,
  "suggested_minutes": 45
}
```

Suggestions must be triggered only by **repeated evidence**, not single events.

---

## 3.2 Suggestion CLI

Implement:

```
vel suggestions
vel suggestion inspect <id>
vel suggestion accept <id>
vel suggestion reject <id>
vel suggestion modify <id> --payload ...
```

States:

- pending
- accepted
- rejected
- modified

Accepted suggestions should update config values.

---

## 3.3 Steering Constraint

Do NOT implement:

- natural language steering
- LLM interpretation of suggestions

All payloads must remain structured.

---

# 4. Project Synthesis

After risk + suggestions exist.

Implement:

```
vel synthesize project <project_slug>
```

or

```
vel synthesize week --project <slug>
```

---

## 4.1 Inputs

Use:

- project commitments
- project nudges
- thread graph links
- captures tagged with project
- transcripts tagged with project
- latest risk snapshots

---

## 4.2 Artifact Output

Artifact type:

`project_synthesis`

Sections required:

### Open commitments

### Active threads

### Repeated drift

### Ideation without execution

### Suggested next actions

Each section must include **evidence references**:

- commitment ids
- signal ids
- thread ids
- capture ids

---

## 4.3 Initial Focus

Ensure synthesis works well for the **Vel project itself** before generalizing.

Do not over-generalize early.

---

# Global Constraints

Do not introduce:

- LLM risk logic
- uncertainty scoring
- progress penalty model
- additional suggestion types
- natural-language steering
- opaque inference

The system must remain **inspectable via CLI/API**.

---

# Goal of This Phase

After these steps Vel should have:

- deterministic pressure model (risk engine)
- validated escalation logic (tests)
- adaptive improvement loop (suggestions)
- reflective project analysis (synthesis)

At that point Vel begins functioning as:

> a self-improving executive-function prosthetic rather than a static reminder engine.
