# vel_system_architecture_map.md

Status: Canonical one-page system architecture map  
Audience: coding agent, future contributors, Vel operator  
Purpose: provide a compact but explicit map of the whole Vel system so the project does not drift into overlapping subsystems and duplicated logic

---

# 1. System Identity

Vel is a **stateful, event-driven executive-function prosthetic and reflective assistant**.

It is designed to:

- ingest facts from external systems
- maintain a current model of the present
- compute risk and detect drift
- generate nudges and suggestions
- preserve continuity across projects, threads, and conversations
- synthesize patterns over time
- improve itself through feedback and dogfooding

Vel is not:
- a generic chat wrapper
- a generic task manager
- a generic PKM
- a multi-master distributed cluster

---

# 2. Top-Level Architecture

```text
External Sources
    ↓
Signal Adapters
    ↓
Signals / Events Store
    ↓
Current Context Reducer
    ↓
Risk Engine
    ↓
Policy / Nudge Engine
    ↓
Suggestions / Steering
    ↓
Artifacts / Synthesis
    ↓
Operator + Client Surfaces
```

This is the canonical flow.

---

# 3. Subsystem Responsibilities

## 3.1 Signal Adapters
Own:
- translating external facts into canonical Vel signals

Do not own:
- inference
- risk
- policy
- synthesis

Examples:
- calendar adapter
- Todoist adapter
- activity adapter
- transcript adapter
- feedback adapter

---

## 3.2 Signals / Events Store
Own:
- append-only normalized facts
- event/action history suitable for replay and replication

Examples:
- calendar event changed
- task completed
- shell login
- capture created
- nudge snoozed
- feedback recorded

---

## 3.3 Commitments
Own:
- action-relevant obligations
- due times
- project linkage
- resolution state
- dependency graph linkage

Examples:
- take meds
- prepare for meeting
- commute to meeting
- finish risk engine

Hierarchy:
```text
commitment → project → domain
```

---

## 3.4 Current Context
Own:
- the system's best current model of the present

Includes:
- mode
- morning state
- attention/drift state
- next commitment
- prep window
- commute window
- meds status
- active nudges
- top risk commitments
- global risk level

Important rule:
**Current context is the single present-tense object.**

---

## 3.5 Risk Engine
Own:
- risk score per commitment
- risk level
- risk factors
- dependency pressure
- current prioritization pressure

Uses:
- commitments
- due times
- dependencies
- context

Does not own:
- nudge decisions
- synthesis prose
- LLM reasoning

---

## 3.6 Attention / Drift Detection
Own:
- attention_state
- drift_type
- drift_severity
- attention_reasons

Initial drift types:
- morning_drift
- prep_drift
- task_drift later

Important rule:
Drift belongs in **current context**, not in a separate shadow system.

---

## 3.7 Policy / Nudge Engine
Own:
- create nudge
- escalate nudge
- resolve nudge
- suppress / cooldown
- suggestion trigger conditions

Consumes:
- current context
- risk
- commitments
- dependencies
- active nudges
- policy config

Does not own:
- raw signal interpretation already handled by context
- LLM decisions

---

## 3.8 Suggestions / Steering
Own:
- structured adaptation proposals
- acceptance/rejection/modify loop
- policy tuning proposals

Initial suggestion types:
- increase_commute_buffer
- increase_prep_window

Does not own:
- natural-language steering yet
- silent mutation of core policy without trace

---

## 3.9 Thread Graph
Own:
- unfinished continuity across:
  - projects
  - people
  - conversations
  - themes
  - logistics

Examples:
- Vel
- Dimitri follow-up
- commute logistics
- medication adherence

This is how ideation and obligations stay connected over time.

---

## 3.10 Artifacts / Synthesis
Own:
- durable reflective outputs
- weekly synthesis
- project synthesis
- self-review
- later alignment synthesis

Examples:
- weekly_synthesis
- project_synthesis
- vel_self_review_weekly

Must remain evidence-backed and inspectable.

---

## 3.11 Self-Model
Own:
- Vel's awareness of its own performance

Tracks:
- nudge effectiveness
- suggestion acceptance
- annoyance/helpfulness feedback
- false positives
- successful interventions
- missing-context cases

This powers dogfooding and policy tuning later.

---

# 4. Canonical Data Flow

## 4.1 Operational Loop

```text
signals
 → current context
 → risk
 → policy / nudges
 → done/snooze/actions
 → updated context
```

This is the main loop that must work before broader client work matters.

## 4.2 Adaptive Loop

```text
repeated evidence
 → suggestion
 → accept/reject/modify
 → updated policy/config
```

## 4.3 Reflective Loop

```text
signals + commitments + nudges + threads + self-model
 → synthesis
 → artifact
 → human review / future policy refinement
```

## 4.4 Assistant Continuity Loop

```text
transcripts + captures + threads + project links
 → ideation continuity
 → project synthesis
 → Vel-on-Vel backlog
```

---

# 5. Canonical Present-Tense Loop

This is the loop Vel must nail for daily usefulness:

```text
What is happening now?
What matters next?
What is at risk?
Am I drifting?
Should I nudge?
Did the user resolve or snooze?
What changed?
```

If the system cannot answer those coherently, nothing else matters yet.

---

# 6. Distributed / Ambient Model

Vel should be deployed as:

- one preferred canonical VELD node
- multiple edge clients/nodes
- action/event replication
- offline local action queue
- later reconciliation

Canonical node preference:
1. NAS
2. Desktop
3. VPS

Client roles:
- Watch = interrupt + acknowledge
- iPhone = review + explain + feedback
- desktop voice = command + capture + morning orchestration
- CLI/TUI = deep operator cockpit

---

# 7. Rust / Swift Boundary

## Rust owns
- state
- inference
- context
- risk
- policy
- threads
- synthesis

## Swift owns
- Apple UI
- notifications
- watch actions
- speech
- Apple framework integrations
- device-local UX/cache

Rule:
**Rust thinks, Swift feels.**

Use API boundary first, FFI later if proven necessary.

---

# 8. Voice Role

Voice is a client over Vel core.

Owns:
- speech input/output
- push-to-talk
- command-to-action mapping

Primary use cases:
- morning orchestration
- command/control
- quick capture
- explanation on demand

Voice does not own separate assistant semantics.

---

# 9. Integration Priority

## Phase 1
- Calendar
- Todoist / Reminders
- Computer activity

## Phase 2
- Transcripts / chat logs
- Feedback signals

## Phase 3
- Apple-native signals
- Messaging awareness
- broader ambient sources

All integrations must enter via canonical signals.

---

# 10. Operator Cockpit

The terminal must remain the first serious control room.

Required commands:
- `vel context`
- `vel explain context`
- `vel risk`
- `vel nudges`
- `vel suggestions`
- `vel synthesize project vel`

If the cockpit is weak, the rest of the clients will just hide the confusion.

---

# 11. Canonical Day Fixture

One realistic day must serve as the integration spine:

- 11:00 external meeting
- prep + commute dependencies
- meds unresolved
- workstation activity late
- drift detected
- nudges escalate
- done/snooze works
- suggestion may emerge
- project synthesis still works for Vel

If the system cannot survive this day, it is not ready for expansion.

---

# 12. Hard Architectural Rules

## Rule 1
Do not duplicate logic across:
- context
- risk
- policy

## Rule 2
Signals are facts, not interpretations.

## Rule 3
Current context is the only present-tense model.

## Rule 4
Risk computes pressure; policy decides action.

## Rule 5
Suggestions adapt policy; they do not silently rewrite history.

## Rule 6
Artifacts are durable outputs, not temporary console chatter.

## Rule 7
Everything important must be inspectable.

## Rule 8
LLMs belong in reflective synthesis and explanation enrichment, not immediate operational truth.

---

# 13. What Is Most Important Right Now

The project should currently focus on finishing this chain:

```text
signals
 → current context
 → risk
 → drift
 → policy / nudges
 → suggestion trigger
 → project synthesis
 → operator cockpit
```

Not on:
- more subsystems
- wake word
- extra integrations
- advanced attention telemetry
- separate Apple repo
- richer distributed complexity

---

# 14. Success Condition

Vel becomes real when it can correctly and explainably say something like:

> "Your next meeting is at 11:00. Prep and commute are unresolved. Morning drift is high. Risk is critical. You should leave soon."

and then:

- accept Done / Snooze
- update state correctly
- later reflect on the pattern
- suggest better defaults if this repeats

That is the target.

---

# 15. Final Summary

Vel is one system, not a pile of adjacent concepts.

The shortest correct mental model is:

```text
facts arrive
 → present state is reduced
 → pressure is computed
 → action is chosen
 → history is preserved
 → reflection improves future behavior
```

In short:

> Vel should behave like one stateful assistant with many surfaces, not many clever subsystems hoping the user will unify them in their head.
