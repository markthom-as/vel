# vel_attention_and_drift_detection_spec.md

Status: Canonical attention and drift detection specification  
Audience: coding agent implementing Vel  
Purpose: define how Vel detects attention drift, distinguishes productive engagement from avoidance or fragmentation, and turns those observations into context, nudges, and reflective artifacts

---

# 1. Purpose

Vel is not only a commitment tracker. It is also an **attention-aware executive-function prosthetic**.

This spec defines the first version of Vel’s attention and drift model.

The system should help answer:

- what am I actually attending to?
- am I drifting away from what matters?
- when is drift harmless versus costly?
- what patterns of fragmentation or avoidance are recurring?
- how should drift affect context, risk, nudges, and weekly synthesis?

This subsystem must begin modestly and remain explainable.

---

# 2. Design Principles

## 2.1 Drift is not always failure
Attention drift can mean:
- normal transition time
- context switching
- recuperation
- avoidance
- overload
- distraction

Vel should not moralize drift.  
It should detect it, classify it cautiously, and act proportionally.

## 2.2 Attention is inferred, not known directly
Vel never has perfect access to “what the user is really doing.”
It only has signals.

Therefore:
- attention state should be probabilistic / heuristic
- explanations must mention contributing signals
- danger-level conclusions need stronger evidence

## 2.3 Operational drift and reflective drift are different
Operational drift:
- relevant in the next minutes/hours
- affects immediate nudges and prep logic

Reflective drift:
- visible over days/weeks
- affects synthesis, planning, and self-model tuning

## 2.4 Start narrow
The first version should use only a few signals:
- calendar
- commitments
- computer activity
- nudge history
- optionally capture/transcript mentions later

Do not begin with full device telemetry archaeology.

---

# 3. First-Phase Signal Inputs

The first attention/drift model should use these signals only.

## 3.1 Commitment signals
- open commitments
- commitment due time
- commitment kind
- project linkage
- dependency linkage

## 3.2 Calendar signals
- active or upcoming meetings
- prep windows
- commute windows
- external commitments

## 3.3 Activity signals
- shell login
- recent Vel activity
- workstation activity
- idle state if available

## 3.4 Nudge history
- repeated snoozes
- ignored nudges
- resolved nudges
- timing of acknowledgement

These are enough for a useful first model.

---

# 4. What “Attention” Means in Vel

Vel’s first operational definition of attention should be practical, not philosophical.

Attention means:
> what the user appears to be engaging with right now, relative to what currently matters.

This may be represented in terms of:

- focused on next commitment
- engaged in morning routine
- active at workstation
- in transition / commuting
- not yet engaged
- drifting away from the relevant commitment
- unknown

Vel should not claim mind-reading powers.

---

# 5. Core Attention States

Introduce a simple inferred attention state model.

## 5.1 aligned
Observed activity appears consistent with the most relevant current commitment or mode.

Examples:
- workstation activity while prep window active
- meds done during morning routine
- active work during planned focus block later

## 5.2 neutral_transition
No strong evidence of alignment or harmful drift yet.

Examples:
- brief inactivity
- gap between commitments
- switching between locations or modes

## 5.3 drifting
Evidence suggests the user is not engaging with what matters and time/risk is increasing.

Examples:
- prep window active, no relevant progress, repeated snooze
- morning underway but no movement toward first commitment

## 5.4 fragmented
Multiple signals suggest switching or dispersion without meaningful progress.

This is more reflective than operational at first and can remain low-confidence initially.

## 5.5 unknown
Not enough evidence.

This state should be common early and should suppress overconfident intervention.

---

# 6. Drift Types

Vel should distinguish at least these first drift types.

## 6.1 morning drift
The morning routine is not progressing toward known commitments.

## 6.2 prep drift
Prep/commute time is being consumed without the prerequisite action being completed.

## 6.3 task drift
An active or high-risk commitment remains unresolved while activity signals suggest no engagement.

## 6.4 thread drift (later / reflective)
A project or thread is repeatedly discussed but not advanced.

This one is mostly for synthesis at first.

---

# 7. Drift Severity Model

Drift should have a severity separate from nudge level.

Suggested severity classes:
- low
- medium
- high
- critical

Inputs:
- current risk level
- time proximity
- consequence
- repeated snoozes
- lack of progress signals
- whether the commitment is external or internal

Example:
- mild wandering 2 hours before a meeting = low/medium
- no prep 5 minutes before leave-by = critical

---

# 8. Drift Detection Heuristics (First Version)

Implement only simple, explainable rules first.

## 8.1 Morning drift rule
If:
- current mode is morning
- meds still open or first operational commitments unresolved
- no meaningful workstation or completion signal
- elapsed time exceeds morning thresholds

Then:
- attention_state = drifting
- drift_type = morning_drift

## 8.2 Prep drift rule
If:
- prep window active
- prep dependency unresolved
- no relevant completion signal
- repeated snooze or no engagement

Then:
- attention_state = drifting
- drift_type = prep_drift

## 8.3 Task drift rule
If:
- top-risk commitment remains open
- no progress signals
- multiple reminders ignored/snoozed
- time pressure increasing

Then:
- attention_state = drifting
- drift_type = task_drift

Do not try to infer “social media distraction” or “wrong project” yet from nonexistent data.

---

# 9. Relationship to Current Context

Attention/drift should be incorporated into `current_context`, not as a separate hidden subsystem.

Suggested new fields in current context:

```json
{
  "attention_state": "drifting",
  "drift_type": "prep_drift",
  "drift_severity": "high",
  "attention_confidence": 0.72,
  "attention_reasons": [
    "prep window active",
    "prep dependency unresolved",
    "nudge snoozed twice"
  ]
}
```

These fields should be optional initially but added early enough that:
- policy engine can use them
- explanation surfaces can show them
- weekly synthesis can aggregate them

---

# 10. Relationship to Risk

Drift should influence risk, but not replace it.

Rule:
- drift is an input to risk pressure
- risk is still the broader prioritization model

Examples:
- prep drift increases risk of meeting commitment
- morning drift increases meds and first-commitment risk
- repeated ignored nudges increase progress-penalty later

In the very first implementation, drift may raise:
- local risk of the affected commitment
- global risk summary in current context

---

# 11. Relationship to Policy Engine

The policy engine may use drift state to:
- create nudge
- escalate nudge
- suppress low-priority interruptions
- produce suggestion later

Examples:
- `morning_drift` nudge triggered from attention drift
- `meeting_prep_window` escalated more confidently when prep drift is high

The policy engine should not independently compute its own private drift model.

---

# 12. Relationship to Suggestions

Repeated drift patterns should eventually generate suggestions.

Examples:
- repeated prep drift → suggest longer prep window
- repeated commute drift → suggest longer commute buffer
- repeated morning drift → suggest earlier wake/start cue
- repeated task drift → suggest schedule block or reclassify commitment

These suggestions should only appear after repeated evidence, not single failures.

---

# 13. Relationship to Weekly Synthesis

Weekly synthesis should summarize drift patterns.

Required future sections:
- most common drift types
- which commitments / threads drift clustered around
- what conditions tended to precede drift
- whether drift was resolved after nudges
- possible adjustments

This is how Vel becomes reflective rather than merely alerting.

---

# 14. Explainability Requirements

Attention and drift must be explainable.

Potential CLI:
```bash
vel explain context
vel explain commitment <id>
vel explain drift
```

Example output:
```json
{
  "attention_state": "drifting",
  "drift_type": "prep_drift",
  "drift_severity": "high",
  "confidence": 0.74,
  "reasons": [
    "prep window active for 15 minutes",
    "prep commitment unresolved",
    "nudge snoozed twice",
    "no workstation progress signal"
  ],
  "signals_used": ["sig_1", "sig_2"],
  "commitments_used": ["com_1", "com_2"]
}
```

No black-box “you are distracted” pronouncements.

---

# 15. Self-Model Integration

Vel should record enough to learn:
- which drift detections were useful
- which were false positives
- which drift warnings caused completion
- which drift states correlate with missed commitments
- whether certain nudge timings reduce drift

This data becomes part of Vel’s self-model and policy-tuning loop.

Examples:
- prep drift warnings were accurate on 80% of critical meeting cases
- morning drift warnings were often too early
- low-confidence task drift was noisy and should be suppressed

---

# 16. Attention Awareness as a Future Expansion

The first version is intentionally narrow.

Later expansions may use:
- directory/project activity
- git commit signals
- app focus
- screen time
- message activity
- watch / presence data
- sleep data

These should feed richer models of:
- project attention distribution
- fragmentation
- intent-vs-behavior gaps

But they are out of scope for first implementation.

---

# 17. Minimal Data Model Additions

No dedicated giant drift table is required immediately.

First implementation may store drift through:
- current_context fields
- context_timeline snapshots
- risk factors
- nudge metadata/events

Later, if needed, add:

```sql
drift_events
------------
id
drift_type
severity
confidence
context_json
timestamp
created_at
```

Do not add this table until existing structures prove insufficient.

---

# 18. Testing Requirements

## 18.1 Unit tests
Test:
- morning drift detection
- prep drift detection
- no false drift under neutral transition
- unknown state when evidence insufficient

## 18.2 Replay tests
Given signal sequences:
- attention_state changes deterministically
- drift severity increases appropriately
- drift feeds context and policy coherently

## 18.3 Explanation tests
Verify:
- reasons are emitted
- signal/commitment ids are included
- unrelated entities are excluded

---

# 19. Minimal First Slice

The first end-to-end slice should be:

1. morning mode active
2. meds unresolved
3. first commitment exists
4. no workstation/completion signal
5. elapsed threshold crossed
6. current_context shows `attention_state=drifting`
7. `morning_drift` policy uses that state
8. nudge fires
9. explanation shows why

After that:
- implement prep drift
- then task drift

---

# 20. Practical Engineering Rules

1. Drift is heuristic, not omniscient.
2. Start with state changes and lack-of-progress, not app-level surveillance.
3. Drift belongs in current context, not as a shadow system.
4. Risk and policy consume drift; they do not privately reinvent it.
5. Explanations are mandatory.
6. Reflective drift analysis comes later.
7. Trust is more important than sensitivity.

---

# 21. Success Criteria

Attention/drift detection is successful when:

- Vel can detect morning drift and prep drift deterministically
- drift appears in current context
- drift can explain why a nudge happened
- drift influences risk coherently
- weekly synthesis can later aggregate drift patterns
- false certainty is avoided

---

# 22. Final Summary

Attention and drift detection make Vel more than a schedule checker.

They allow it to notice:
- when the user is moving with the day
- when the user is falling out of alignment with what matters
- when intervention should become more urgent
- when repeated patterns deserve reflection

In short:

> drift detection gives Vel a first real model of where attention is failing to meet commitment.