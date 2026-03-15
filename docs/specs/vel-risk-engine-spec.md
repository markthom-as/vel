# vel_risk_engine_spec.md

Status: Canonical risk engine specification  
Audience: coding agent implementing Vel  
Purpose: define how Vel computes, stores, explains, and uses risk so that priorities, nudges, and suggestions remain coherent rather than turning into policy spaghetti

---

# 1. Purpose

The risk engine is Vel’s **priority pressure model**.

Its job is to answer:

- how much danger or pressure is attached to a commitment right now?
- which commitments are most at risk?
- which commitments are putting other commitments at risk through dependencies?
- when should a nudge escalate because consequence and time pressure are increasing?
- when should Vel suggest changes because repeated patterns show current policy is failing?

The risk engine must be:

- deterministic
- inspectable
- incremental
- dependency-aware
- conservative under uncertainty
- separate from the policy engine

The risk engine does **not** directly create nudges.  
It produces risk state that the policy engine consumes.

---

# 2. Design Principles

## 2.1 Risk is dynamic
Risk is not a fixed priority label.

Risk changes with:
- time proximity
- consequence of failure
- dependency pressure
- uncertainty
- active context
- observed progress or lack of progress

## 2.2 Risk is not the same as importance
A thing can be important but low-risk right now, or modestly important but high-risk because time is running out.

## 2.3 Risk should be explainable
For every computed score, Vel should be able to say:
- what factors contributed
- which dependencies mattered
- what data was missing
- why the score changed

## 2.4 Risk should support intervention, not theatrics
The goal is not dramatic scoring.  
The goal is better sequencing, nudging, and prioritization.

---

# 3. Inputs

The risk engine consumes:

- commitments
- commitment dependencies
- current context
- relevant signals
- active nudges
- current wall-clock time
- optional user/config policy values

It must not:
- call external systems directly
- use LLMs
- infer facts that should have come from adapters/context

---

# 4. Outputs

The risk engine should produce:

- per-commitment risk score
- per-commitment risk level
- factors/explanation payload
- optional aggregate/global risk summary for current context
- historical snapshots for trend analysis

These outputs are then used by:
- current-context reducer
- policy engine
- weekly synthesis
- self-model / policy tuning

---

# 5. Core Concept: Risk Components

Risk should be computed from a small number of components.

## 5.1 Consequence
How costly is failure?

Examples:
- meeting with another person = high
- self task with no deadline = lower
- meds before important commitment = potentially high because downstream effects
- follow-up email = low to medium unless explicitly important

## 5.2 Proximity
How near is the relevant due point?

Examples:
- event in 10 minutes = very high proximity pressure
- deadline in 3 days = lower proximity pressure
- no due time = low proximity pressure

## 5.3 Dependency pressure
How much does failure here endanger another commitment?

Examples:
- missing prep endangers meeting
- missing commute endangers meeting
- missing budget draft endangers grant submission

## 5.4 Progress / blockage
Has any progress happened?

Examples:
- prep still untouched as prep window opens = rising risk
- meds still open despite morning being underway = rising risk
- commitment already partially completed = lower risk

## 5.5 Uncertainty
How incomplete or unreliable is the state?

Examples:
- travel_minutes explicitly set = lower uncertainty
- travel inferred weakly = higher uncertainty
- stale task source = higher uncertainty

---

# 6. Risk Data Model

Use a stored snapshot model rather than only in-memory scores.

## 6.1 Table

This should align with the schema spec:

```sql
CREATE TABLE commitment_risk (
  id TEXT PRIMARY KEY,
  commitment_id TEXT NOT NULL,
  risk_score REAL NOT NULL,
  risk_level TEXT NOT NULL,
  factors_json TEXT NOT NULL,
  computed_at INTEGER NOT NULL,
  FOREIGN KEY (commitment_id) REFERENCES commitments(id)
);
```

## 6.2 Suggested factors_json shape

```json
{
  "consequence": 0.9,
  "proximity": 0.7,
  "dependency_pressure": 0.8,
  "progress_penalty": 0.6,
  "uncertainty": 0.2,
  "reasons": [
    "meeting with external counterparty",
    "prep dependency unresolved",
    "prep window active"
  ],
  "dependency_ids": ["dep_1"],
  "signal_ids": ["sig_10", "sig_11"]
}
```

---

# 7. First Risk Heuristic

The coding agent should implement a simple weighted heuristic first.

Example:

```text
risk_score =
  consequence_weight * consequence
+ proximity_weight * proximity
+ dependency_weight * dependency_pressure
+ progress_weight * progress_penalty
+ uncertainty_weight * uncertainty
```

Start with explicit default weights in config.

Example defaults:

```yaml
risk:
  weights:
    consequence: 0.35
    proximity: 0.30
    dependency_pressure: 0.20
    progress_penalty: 0.10
    uncertainty: 0.05
```

This does not need to be statistically elegant yet.  
It needs to be inspectable and steerable.

---

# 8. Risk Levels

Map numeric risk scores into categories.

Suggested levels:

- `low`
- `medium`
- `high`
- `critical`

Example thresholds:

```yaml
risk:
  thresholds:
    low_max: 0.24
    medium_max: 0.49
    high_max: 0.74
    critical_max: 1.0
```

Rule:
- scores above `high_max` become `critical`

These thresholds should live in configuration, not scattered through code.

---

# 9. Consequence Model

The risk engine should incorporate commitment consequence.

Initial consequence classes can be inferred from commitment attributes.

Suggested mapping:

## 9.1 External commitments
Examples:
- meetings with people
- deadlines promised to others

Default consequence: high

## 9.2 Operational prerequisites
Examples:
- prep
- commute
- leave-by
- meds if linked to higher-order commitments

Default consequence: medium to high depending on parent

## 9.3 Self commitments
Examples:
- self-improvement tasks
- optional routines

Default consequence: lower, unless configured otherwise

## 9.4 Adjustable consequence
Users or later synthesis may steer certain commitments upward/downward.

This should eventually become a first-class field or derived property.

---

# 10. Proximity Model

The risk engine should compute proximity relative to the commitment’s effective due point.

Examples:
- meeting start
- leave-by time
- prep start window
- Todoist due time
- meds expectation window

A simple first model:

## 10.1 If due time exists
Convert time distance into normalized pressure:
- far away -> low
- very near -> high
- overdue -> very high

## 10.2 If no due time exists
Set low baseline proximity unless another policy supplies temporal urgency.

The engine should not invent due times where none exist.

---

# 11. Dependency Pressure Model

Dependencies are mandatory inputs.

Each child commitment should inherit some pressure from its parent.

Examples:

- prep pressure rises as meeting risk rises
- commute pressure rises as meeting start approaches
- unfinished draft pressure rises as submission deadline approaches

A simple initial model:
- if child unresolved and parent risk high -> child dependency pressure increases
- if parent resolved/cancelled -> dependency pressure drops

The engine should support at least one level of propagation initially.  
Recursive deep graph optimization can come later.

---

# 12. Progress / Blockage Model

Risk should rise when expected progress is absent.

Examples:
- meds still open after expected time
- no prep done when prep window active
- no workstation activity while morning at risk
- repeated snoozes on same commitment

Initial implementation may use:
- completion state
- nudge history
- current context markers

Later this can expand to:
- richer activity signals
- actual task progress estimation
- thread movement

---

# 13. Uncertainty Model

Uncertainty prevents false precision.

Examples of uncertainty sources:
- weakly inferred commute time
- stale calendar sync
- no recent task refresh
- no activity signals
- ambiguous next commitment

Initial implementation can use a small additive penalty or modifier.

Rule:
High uncertainty should not automatically cause high danger nudges.  
It should instead:
- reduce confidence
- support suggestions/clarifications
- keep explanations honest

---

# 14. Risk Recompute Triggers

Recompute risk when:

- relevant signal ingested
- commitment created/updated/resolved
- dependency added/removed
- current context materially changed
- active nudge changed state
- periodic reconciliation occurs later

Do not recompute the whole universe on every trivial event if not needed.  
Prefer incremental recalculation for affected commitments first.

---

# 15. Risk Scope

The engine should support two scopes.

## 15.1 Local risk
Risk of one commitment.

## 15.2 Global/current risk
A summary of what is most dangerous or pressing right now.

This global summary should be surfaced through current context, not maintained as a separate hidden ontology.

Suggested current-context fields:
- `global_risk_score`
- `global_risk_level`
- `top_risk_commitment_ids`

---

# 16. Integration with Current Context

The current-context reducer should consume risk snapshots.

It should not privately re-implement risk logic.

Current context may use risk to determine:
- next commitment
- global risk level
- whether system is in `at_risk`
- which open threads matter now

This keeps the system coherent.

---

# 17. Integration with Policy Engine

The policy engine consumes risk; it does not define it.

Examples:
- meeting prep policy escalates because local risk moved from medium to high
- morning drift policy escalates because global risk rose and no progress signals arrived
- suggestions are generated because repeated critical risk appears for similar commitments

The policy engine should be able to inspect:
- latest risk score
- latest risk level
- factors_json

---

# 18. Explainability Requirements

The risk engine must support explanation.

Potential commands:
- `vel risk`
- `vel risk <commitment_id>`
- `vel explain commitment <id>`
- later: `vel explain risk <commitment_id>`

Example output:

```json
{
  "commitment_id": "com_123",
  "risk_score": 0.82,
  "risk_level": "critical",
  "factors": {
    "consequence": 0.9,
    "proximity": 0.95,
    "dependency_pressure": 0.7,
    "progress_penalty": 0.6,
    "uncertainty": 0.1
  },
  "reasons": [
    "meeting with external counterparty starts in 20 minutes",
    "prep dependency unresolved",
    "no progress signals detected"
  ]
}
```

This is not optional.  
Risk without explanation will become a black box and lose trust.

---

# 19. Configuration

Add risk config alongside policies.

Suggested location:
- `config/policies.yaml` initially is acceptable
- or `config/risk.yaml` later if split becomes cleaner

Minimal config sections:

```yaml
risk:
  weights:
    consequence: 0.35
    proximity: 0.30
    dependency_pressure: 0.20
    progress_penalty: 0.10
    uncertainty: 0.05

  thresholds:
    low_max: 0.24
    medium_max: 0.49
    high_max: 0.74
    critical_max: 1.0
```

Possible later config:
- default consequence by commitment kind
- consequence multiplier for external commitments
- penalties for repeated snoozes
- uncertainty dampening rules

---

# 20. Steering and Adaptation

The risk engine itself should remain deterministic, but it must accept:
- config changes
- accepted suggestions
- explicit user steering
- later self-model tuning suggestions

Examples:
- user increases prep default from 30m to 45m
- user marks meds as very high consequence
- repeated evidence suggests commute defaults too low

These should affect future risk computation through config/policy inputs, not through magical hidden mutation.

---

# 21. Self-Model Integration

The risk engine should emit useful data for Vel’s self-model.

Examples:
- commitments that repeatedly reach critical risk
- commitments whose risk rises despite repeated nudging
- contexts where high-risk commitments were resolved quickly
- whether certain policy changes reduced critical risk frequency

This supports:
- weekly self-review
- policy tuning suggestions
- overcommitment analysis
- “Vel on Vel” improvement work

---

# 22. Overcommitment

The risk engine should eventually support overcommitment analysis.

Not full implementation yet, but the design must leave room for it.

Potential future factors:
- number of overlapping high-consequence commitments
- total required prep/commute time vs available time
- repeated carryover of unfinished commitments
- accumulated unresolved threads

A later artifact might be:
- `overcommitment_report`
- or `daily_capacity_warning`

Do not fully implement this yet unless needed, but do not block it either.

---

# 23. Testing Requirements

## 23.1 Unit tests
Test:
- consequence mapping
- proximity normalization
- dependency pressure propagation
- progress penalty behavior
- uncertainty behavior
- threshold mapping to risk levels

## 23.2 Replay tests
Given a sequence of signals and state changes:
- risk should rise/fall deterministically
- dependency chains should affect related commitments
- due-time proximity should behave predictably

## 23.3 Explainability tests
Every computed risk should include:
- factor values
- reason strings
- relevant entity references where possible

## 23.4 Integration tests
Verify:
- current context reflects top risk commitments
- policy engine escalates based on risk levels
- resolved commitments drop in risk appropriately

---

# 24. Minimal First Slice

The first end-to-end risk slice should support:

1. one meeting commitment with due time
2. one prep dependency
3. time proximity increases
4. prep unresolved
5. risk rises from medium → high → critical
6. policy engine uses this to escalate nudge

This proves:
- due-time pressure
- dependency pressure
- escalation coherence

After that, add:
- meds-linked risk
- commute-linked risk
- repeated snooze impact

---

# 25. Practical Engineering Rules

1. Keep the first risk function simple.
2. Store snapshots, not just current in-memory values.
3. Keep factor explanations explicit.
4. Avoid hidden defaults across modules.
5. Make thresholding configurable.
6. Let current context and policy engine consume risk, not re-implement it.
7. Do not use LLMs here.
8. Optimize for trust and inspectability over elegance.

---

# 26. Success Criteria

The risk engine is working when:

- commitments receive deterministic risk scores
- risk levels update when time/context changes
- dependencies raise downstream risk correctly
- current context surfaces top-risk commitments
- policies escalate based on risk rather than ad hoc timing alone
- risk is inspectable and explainable
- replay tests pass

---

# 27. Final Summary

The risk engine is Vel’s **pressure interpreter**.

Signals tell Vel what happened.
Current context tells Vel what the present looks like.
The risk engine tells Vel what is becoming dangerous or urgent.

In short:

> risk turns context into prioritization pressure, and it must do so transparently enough that both the user and Vel can learn from it.