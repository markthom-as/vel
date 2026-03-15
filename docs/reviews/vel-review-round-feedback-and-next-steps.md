
# vel_review_round_feedback_and_next_steps.md

Audience: Vel coding agent
Purpose: Provide architectural feedback and concrete next steps based on the latest repository snapshot.

This document assumes the repository roughly matches the architecture described in the Vel specs already provided:
- risk engine spec
- distributed architecture spec
- attention/drift detection spec
- policy engine spec
- thread graph spec
- synthesis spec

The goal now is **not to add more subsystems**, but to finish wiring the existing architecture into a working operational loop.

---

# High-Level Feedback

The repository structure is strong and moving in the right direction.

Key strengths:

- clean Rust workspace layout
- `vel-core` containing domain types
- `vel-storage` separated from API types
- CLI and daemon separation (`vel-cli` vs `veld`)
- artifact system already present
- run-backed context model
- policies.yaml configuration
- migrations present
- basic CLI ergonomics

This matches the intended architecture well.

However several **critical loops are still incomplete**.

Right now Vel behaves more like a **structured capture/search system** rather than a **stateful executive-function runtime**.

To get Vel to its first real dogfooding milestone, four things must happen:

1. risk engine must become operational
2. policy engine must consume risk
3. drift detection must feed context
4. synthesis must produce actionable reflection

Until these exist together, Vel will not feel like an assistant.

---

# Most Important Missing Loop

Vel currently lacks a fully connected loop:

signals → context → risk → policy → nudge → resolution → synthesis

Pieces exist but are not fully wired.

That loop must be completed before adding new features.

---

# Immediate Next Engineering Tasks

Implement these in strict order.

## 1. Finish Risk Engine Compute

The table and spec exist but the compute layer must be implemented.

Implement in crate:

vel-core or vel-context (depending on your layout)

Required capabilities:

- compute risk score for commitments
- compute risk level
- include explanation factors

Inputs:

- commitment
- dependency graph
- due times
- current time
- drift signals

Initial factors only:

- consequence
- proximity
- dependency pressure

Do NOT implement:

- uncertainty
- progress penalty
- adaptive learning

Expose via:

CLI

```
vel risk
vel risk <commitment_id>
```

API

```
GET /v1/risk
GET /v1/risk/:commitment_id
```

Each risk record must include:

- score
- level
- factors
- dependency ids
- explanation reasons

---

## 2. Feed Risk Into Context

Current context generation must incorporate risk.

Update context structure to include:

```
top_risk_commitments
global_risk_level
global_risk_score
```

Context explain endpoint must reference the risk snapshot used.

---

## 3. Integrate Risk Into Policy Engine

Policies should not rely solely on time windows.

They should read:

- commitment risk
- drift signals
- dependency status

Example:

meeting prep nudge escalation should occur when:

```
prep_dependency unresolved
AND meeting_risk >= high
```

This ensures escalation is tied to actual danger.

---

## 4. Implement Attention / Drift Detection

The spec exists but implementation should begin with only two drift types.

Implement first:

- morning_drift
- prep_drift

These should modify current_context fields:

```
attention_state
drift_type
drift_severity
attention_reasons
```

Do not attempt sophisticated behavioral inference.

Only use:

- unresolved commitments
- time elapsed
- repeated snoozes
- lack of progress signals

---

## 5. Suggestions / Steering Loop

The schema exists but functionality is missing.

Implement only two suggestion types initially.

```
increase_commute_buffer
increase_prep_window
```

Trigger conditions:

- repeated commute danger nudges
- repeated prep warnings

CLI commands required:

```
vel suggestions
vel suggestion inspect <id>
vel suggestion accept <id>
vel suggestion reject <id>
```

Do not add natural language steering yet.

---

## 6. Implement Project Synthesis

Command:

```
vel synthesize project <slug>
```

Input data:

- project commitments
- project threads
- project nudges
- captures linked to project
- recent risk snapshots

Output artifact type:

```
project_synthesis
```

Sections required:

- open commitments
- active threads
- repeated drift patterns
- ideation without execution
- suggested next actions

Each section must reference evidence ids.

---

# Architecture Corrections / Recommendations

## 1. Prevent CLI From Becoming a Shadow Runtime

The CLI must remain an interface layer.

All core logic must live in:

- vel-core
- vel-runtime modules

CLI should call those modules, not reimplement logic.

---

## 2. Ensure Event / Action Logging Exists

The distributed architecture spec assumes an event log.

Verify the repo includes:

- durable action/event records
- replication-safe event ids
- timestamp ordering

If missing, add an `events` table.

Example:

```
events
------
id
event_type
entity_id
payload_json
created_at
```

This simplifies distributed sync later.

---

## 3. Strengthen Context Explain

Context explain must show:

- signals used
- commitments used
- risk factors
- drift signals

This will be critical for debugging the system.

---

## 4. Add Observability CLI

Add:

```
vel explain context
vel explain commitment <id>
vel explain drift
```

Vel will be extremely difficult to debug without this.

---

# Suggested Testing Additions

Add replay tests simulating:

### Morning scenario

Sequence:

wake → meds open → no workstation activity → time passes

Expected:

- drift detected
- morning nudge
- risk increases

---

### Meeting prep scenario

Sequence:

meeting scheduled → prep dependency → prep ignored → repeated snooze

Expected:

- prep drift detected
- escalation occurs

---

### Commute scenario

Sequence:

meeting with travel_minutes → leave-by window → no departure signal

Expected:

- commute nudge
- escalation thresholds

---

# Stop Adding New Subsystems

Do NOT add:

- advanced ML
- complex attention models
- uncertainty models
- new suggestion types
- additional synthesis modes

Until the main operational loop works.

---

# Milestone Definition

Vel reaches the **first real dogfooding milestone** when:

1. morning command produces accurate context
2. risk model highlights real danger
3. drift detection notices misalignment
4. nudges escalate appropriately
5. suggestions occasionally appear
6. weekly/project synthesis produces useful reflection

Only after that should the system expand.

---

# Final Advice

Vel is becoming architecturally sophisticated.

At this stage the biggest risk is **adding more architecture instead of finishing the loop**.

Focus on making the existing pieces interact.

When Vel can correctly tell you:

> "Your meeting prep is at risk and you have been drifting for 20 minutes."

the system will begin to feel real.
