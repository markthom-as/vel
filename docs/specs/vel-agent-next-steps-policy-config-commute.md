
# vel_agent_next_steps_policy_config_commute_context_explain.md

Status: Implementation instructions for coding agent  
Audience: Vel coding agent  
Purpose: Define the next implementation tasks in a strict order with constraints.

---

# Implementation Order

Implement the following features **in this exact order**.

1. Policy configuration system
2. `commute_leave_time` nudge policy
3. Context explanation endpoint
4. Targeted unit tests for all above behavior

Each phase should be committed separately so regressions are easy to isolate.

---

# 1. Policy Configuration

## File Location

Use a repository-level config file:

```
config/policies.yaml
```

Do **not** implement user config overrides yet.

No hot reload is required.

Load the config **once at startup** and inject it into:

- policy engine
- context reducer
- risk engine (if present)

If the config file is missing or malformed, the system must fail clearly at startup.

Do not silently invent defaults inside multiple modules.

---

## Initial Config Structure

```yaml
policies:
  meds_not_logged:
    enabled: true
    gentle_after_minutes: 10
    warning_after_minutes: 30
    danger_after_minutes: 60
    default_snooze_minutes: 10

  meeting_prep_window:
    enabled: true
    default_prep_minutes: 30
    gentle_before_minutes: 15
    warning_before_minutes: 0
    danger_before_minutes: -10
    default_snooze_minutes: 10

  commute_leave_time:
    enabled: true
    require_travel_minutes: true
    gentle_before_minutes: 20
    warning_before_minutes: 5
    danger_before_minutes: 0
    default_snooze_minutes: 5

  morning_drift:
    enabled: true
    gentle_after_minutes: 20
    warning_after_minutes: 40
    danger_after_minutes: 60
    default_snooze_minutes: 10
```

---

# 2. Commute Leave Time Policy

Implement the `commute_leave_time` nudge policy next.

## Preconditions

A nudge should only be generated when **all conditions are true**:

- an upcoming calendar-backed commitment exists
- the commitment has a `travel_minutes` value
- the commitment is still active (not done/cancelled)
- the event start time is known
- the current time is within the configured threshold ladder

Do **not** infer travel time from event location yet.

Do **not** automatically create travel minutes.

Those will be implemented later as suggestions.

---

## Leave-By Time

Compute leave-by time as:

```
leave_by = event_start_time - travel_minutes
```

---

## Escalation Ladder

Using the config values above:

| Level | Condition |
|------|-----------|
| Gentle | now >= leave_by - gentle_before_minutes |
| Warning | now >= leave_by - warning_before_minutes |
| Danger | now >= leave_by - danger_before_minutes |

Example using defaults:

- Gentle: 20 minutes before leave-by
- Warning: 5 minutes before leave-by
- Danger: exactly at leave-by

---

## Resolution Conditions

A commute nudge resolves if:

- the user marks the commute done
- the calendar event starts
- the event is cancelled
- the associated commitment resolves

---

## Nudge Messaging

Initial messages should be minimal and factual.

Gentle:

```
Leave-by time is approaching.
```

Warning:

```
You should leave soon.
```

Danger:

```
You may be late unless you leave now.
```

Natural language tuning can happen later.

---

# 3. Context Explain Endpoint

Implement structured explainability for the context reducer.

Add:

```
GET /v1/explain/context
```

CLI command:

```
vel explain context
```

---

## Response Structure

Return structured JSON first.

Do not generate narrative explanations yet.

Example:

```json
{
  "computed_at": 1700000000,
  "mode": "morning_mode",
  "morning_state": "underway",
  "signals_used": ["sig_1", "sig_2"],
  "commitments_used": ["com_1", "com_2"],
  "risk_used": ["risk_1"],
  "reasons": [
    "upcoming external commitment at 11:00",
    "prep window active",
    "meds commitment still open"
  ]
}
```

The reducer must track which entities contributed to the context decision.

This information may be stored temporarily or reconstructed from reducer traces.

---

# 4. Tests

Tests should be added immediately alongside each feature.

Do not defer testing to a later phase.

---

## Policy Config Tests

Verify:

- config loads successfully
- missing config causes clear startup failure
- policy values propagate correctly into the policy engine

---

## Commute Policy Tests

Test cases:

- no nudge when `travel_minutes` is missing
- gentle threshold fires correctly
- warning threshold fires correctly
- danger threshold fires correctly
- snooze suppresses repeats
- resolved commitment resolves nudge
- event start resolves nudge

---

## Context Explain Tests

Verify:

- signal ids appear in explanations
- commitment ids appear in explanations
- explanation updates when inputs change
- unrelated entities are not included

---

## Material Change Tests

Verify:

- context timeline only records semantic changes
- recompute with identical context does not create timeline row

---

## Resolution Order Tests

Verify:

- resolution occurs before escalation
- resolved nudges do not escalate
- completed commitments suppress new equivalent nudges

---

# Implementation Constraints

The agent must respect the following constraints.

Do not implement:

- user-level config paths yet
- config hot reload
- inferred travel time from event location
- LLM-generated explanations
- a generic policy DSL
- silent fallback defaults inside modules

The system should prefer explicit configuration and deterministic behavior.

---

# Success Criteria

This phase is complete when:

- policies load from `config/policies.yaml`
- commute_leave_time nudges fire correctly
- context explanation endpoint works
- all listed tests pass
- escalation and resolution order behave correctly
