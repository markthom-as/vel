# vel_current_context_spec.md

Status: Canonical current-context specification  
Audience: coding agent implementing Vel  
Purpose: define the persistent current-context object, its recomputation rules, storage model, timeline behavior, and inspection surfaces

---

# 1. Purpose

The **current context** is Vel’s continuously maintained model of the present.

It is the load-bearing object that lets Vel answer questions like:

- What is happening right now?
- What matters next?
- What is at risk?
- What state is the morning in?
- What commitments are active?
- Which nudges are currently relevant?
- Which project/thread is most salient?

The current-context subsystem must be:

- persistent
- inspectable
- incrementally updated
- derived from signals + commitments + dependencies + nudges
- cheap enough to recompute frequently

It should be the single source of truth for the present state of the system.

---

# 2. Design Principles

## 2.1 One canonical current context
Vel should maintain **one canonical current-context object** at any given time.

Do not allow multiple shadow context models to emerge in different modules.

## 2.2 Context is derived, not manually edited
Current context is inferred from:
- signals
- commitments
- dependencies
- risk state
- active nudges
- time

Users may steer policies and suggestions, but the context itself is derived.

## 2.3 Context changes should be inspectable
Whenever the context changes materially, Vel should be able to show:
- what changed
- what caused it
- what it now means

## 2.4 Context is not a giant junk drawer
The current-context object should contain only **current** high-value state, not every possible metric. Historical detail belongs in:
- signals
- context timeline
- runs
- artifacts
- synthesis

---

# 3. Canonical Responsibilities

The current-context subsystem is responsible for maintaining:

- current day-state / mode
- current morning-state
- current inferred activity state
- next commitment
- active prep window
- active commute window
- meds status
- active nudge ids
- highest-risk commitments
- open thread ids relevant now
- current global risk level
- interpersonal response pressure summary
- context timestamps and provenance references

It is **not** responsible for:
- computing deep reflective synthesis
- deciding nudge policies directly
- storing all historical data
- talking to external systems directly

---

# 4. Canonical Data Shape

The current context should be stored as structured JSON, but the subsystem should expose typed domain models in code.

## 4.1 Suggested JSON shape

```json
{
  "computed_at": 1700000000,
  "mode": "morning_mode",
  "morning_state": "underway",
  "inferred_activity": "computer_active",
  "next_commitment_id": "com_123",
  "next_commitment_due_at": 1700001800,
  "active_prep_commitment_id": "com_456",
  "prep_window_active": true,
  "active_commute_commitment_id": "com_789",
  "commute_window_active": false,
  "meds_status": "pending",
  "active_nudge_ids": ["nud_001"],
  "top_risk_commitment_ids": ["com_123", "com_456"],
  "open_thread_ids": ["thr_45", "thr_46"],
  "message_waiting_on_me_count": 2,
  "message_scheduling_thread_count": 1,
  "message_urgent_thread_count": 0,
  "global_risk_level": "high",
  "global_risk_score": 0.78,
  "trigger_signal_id": "sig_888"
}
```

## 4.2 Required fields

At minimum, implement:

- `computed_at`
- `mode`
- `morning_state`
- `inferred_activity`
- `next_commitment_id`
- `prep_window_active`
- `commute_window_active`
- `meds_status`
- `active_nudge_ids`
- `top_risk_commitment_ids`
- `message_waiting_on_me_count`
- `global_risk_level`
- `global_risk_score`

## 4.3 Optional fields for early versions

- `open_thread_ids`
- `message_scheduling_thread_count`
- `message_urgent_thread_count`
- `trigger_signal_id`
- `focus_mode_reason`
- `context_confidence`

These can be added incrementally.

---

# 5. Storage Model

## 5.1 current_context table

The latest current context should live in a singleton table.

### Table

```sql
CREATE TABLE current_context (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  computed_at INTEGER NOT NULL,
  context_json TEXT NOT NULL
);
```

### Rule
There must only ever be one current row.

Use `id = 1` as the singleton key.

---

## 5.2 context_timeline table

Material context changes should also be recorded in an append-only timeline.

### Table

```sql
CREATE TABLE context_timeline (
  id TEXT PRIMARY KEY,
  timestamp INTEGER NOT NULL,
  context_json TEXT NOT NULL,
  trigger_signal_id TEXT,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (trigger_signal_id) REFERENCES signals(id)
);
```

### Purpose
This supports:
- daily timeline views
- debugging state transitions
- later synthesis over context changes

---

# 6. Context Recompute Triggers

Current context should be recomputed:

## 6.1 On relevant signal ingestion
Examples:
- calendar event created/updated
- task completed
- shell login
- capture created if capture could affect current state
- nudge resolved/snoozed

## 6.2 On important commitment changes
Examples:
- commitment done
- commitment cancelled
- dependency added
- due time changed

## 6.3 On active nudge state changes
Examples:
- active nudge escalated
- nudge snoozed
- nudge resolved

## 6.4 On periodic reconciliation later
A light periodic recompute loop is acceptable later, for example every 5 minutes, but event-driven updates are primary.

### Important rule
If nothing relevant changed, do not thrash.

---

# 7. Context Recompute Inputs

The context reducer should consume:

- latest relevant signals
- open commitments
- dependency graph
- latest risk snapshots
- active nudges
- current wall-clock time

It should not query external APIs directly.

---

# 8. Context Reducer Responsibilities

Suggested reducer function:

```text
recompute_context(signals, commitments, dependencies, risks, nudges, now) -> CurrentContext
```

The reducer should:

1. determine current mode
2. determine morning state
3. determine inferred activity state
4. select next commitment
5. identify active prep/commute windows
6. compute meds status
7. identify active nudges
8. identify top risk commitments
9. derive a global risk score/level
10. return the canonical current context

This should remain deterministic.

---

# 9. Context Submodels

The current context may internally consist of several submodels.

## 9.1 Mode
Examples:
- `morning_mode`
- `meeting_mode`
- `commute_mode`
- `focus_mode`
- `end_of_day_mode`

Only one dominant mode is required initially.

## 9.2 Morning state
Examples:
- `inactive`
- `awake_unstarted`
- `underway`
- `engaged`
- `at_risk`

## 9.3 Inferred activity
Examples:
- `inactive`
- `computer_active`
- `away_from_workstation`
- `unknown`

## 9.4 Commitment focus
Which commitment most deserves near-term attention.

## 9.5 Risk summary
A global view of:
- what is most at risk
- whether danger escalation is likely needed

---

# 10. How to Choose the “Next Commitment”

The context subsystem should choose the next commitment using deterministic ordering.

Suggested factors:

1. external commitments with other people
2. commitments with imminent due time
3. prep/commute prerequisites for those commitments
4. high-risk self commitments that materially affect the above
5. other open commitments

This ordering should be refined later by the risk engine, but the current-context model should expose a single most relevant next commitment if possible.

If ambiguity is too high, it may expose:
- `next_commitment_id`
- plus `top_risk_commitment_ids`

---

# 11. Global Risk Level

The current context should include:
- a numeric `global_risk_score`
- a categorical `global_risk_level`

Suggested levels:
- `low`
- `medium`
- `high`
- `critical`

This should summarize:
- risk of the next commitment
- active dependencies
- unresolved blockers
- time proximity
- uncertainty

The policy engine can then use this as an input, not recompute its own private global view.

---

# 12. Material Change Detection

Not every recomputation should create a new timeline row.

A new `context_timeline` entry should be created when **material context changed**.

Examples of material changes:
- morning state changed
- mode changed
- next commitment changed
- prep/commute window activated or deactivated
- meds status changed
- global risk level changed
- active nudge set changed

Examples of non-material changes:
- recompute ran but produced same state
- timestamp-only differences
- cosmetic ordering differences in arrays

The agent should implement a context comparison function that ignores non-semantic noise.

---

# 13. Explainability Requirements

The current-context subsystem must support explanation.

Possible commands later:
- `vel context`
- `vel context timeline`
- `vel explain context`

For each current context, the system should be able to show:
- which signals mattered
- which commitments were selected as relevant
- why the current mode was chosen
- why the current morning state was chosen
- why the current global risk level is what it is

A minimal implementation may store a small explanation payload in memory or alongside context recomputation traces.

---

# 14. Integration with Other Subsystems

## 14.1 Signals
Signals are the primary facts the reducer consumes.

## 14.2 Commitments
Open commitments and dependencies shape the context.

## 14.3 Risk
Latest risk snapshots should influence:
- top risk commitments
- global risk summary

## 14.4 Nudges
Active and snoozed nudges must be reflected in context.

## 14.5 Threads
Open thread ids may enrich context later but do not block first implementation.

## 14.6 Runs / Artifacts
Context recomputation may optionally be recorded through runs later if useful, but that is not required for every recompute.

---

# 15. CLI / API Surfaces

The current-context subsystem should support these interfaces.

## 15.1 CLI

```bash
vel context
vel context timeline
vel explain context
```

### `vel context`
Show a concise current-state summary.

Example:

```text
Mode: morning_mode
Morning state: underway
Next commitment: meeting with Dimitri at 11:00
Prep window: active
Meds: pending
Global risk: high
Active nudges: 1
```

### `vel context timeline`
Show recent material context transitions.

Example:

```text
08:40 morning_state -> awake_unstarted
08:55 meds_status -> pending
09:05 mode -> morning_mode
09:20 prep_window_active -> true
09:25 global_risk_level -> high
```

## 15.2 API

Suggested endpoints later:
- `GET /v1/context/current`
- `GET /v1/context/timeline`

These should expose structured JSON.

---

# 16. Testing Requirements

The current-context subsystem must be tested independently.

## 16.1 Unit tests
Test reducer behavior for:
- morning state transitions
- next commitment selection
- prep/commute activation
- meds status transitions
- global risk aggregation

## 16.2 Replay tests
Given a signal sequence, the context reducer should produce a deterministic context trajectory.

Example:
- calendar event imported
- meds task incomplete
- shell login
- prep window starts

Expected:
- morning_state changes in correct order
- prep_window_active becomes true
- next_commitment selected correctly

## 16.3 Material change tests
Ensure timeline rows are only added when context changes meaningfully.

---

# 17. Minimal First Slice

The first end-to-end current-context slice should support:

1. next commitment selection from calendar/task data
2. morning state inference
3. meds status
4. prep window activation
5. global risk level
6. timeline entry on state change

That is enough to support the first nudge policies.

---

# 18. Practical Engineering Rules

1. Keep the current-context model small and high-value.
2. Do not duplicate logic across policy engine and context reducer.
3. Persist only one latest context snapshot.
4. Store timeline snapshots only on material change.
5. Use deterministic recomputation.
6. Prefer derived state over manual mutation.
7. Make the context cheap to inspect.
8. Keep explanation support in mind from the start.

---

# 19. Success Criteria

The current-context subsystem is successful when:

- Vel can always answer “what is happening right now?”
- it can identify the next meaningful commitment
- it can detect prep/commute windows
- it can represent current morning state
- it can summarize current risk
- it can show a timeline of meaningful state changes
- the policy engine can depend on it as the canonical present-state object

---

# 20. Final Summary

The current context is Vel’s **present tense**.

Signals are the raw facts.
Artifacts are reflective outputs.
But current context is the thing that lets Vel know what matters now.

In short:

> signals are history arriving; current context is the continuously maintained story of the present.
