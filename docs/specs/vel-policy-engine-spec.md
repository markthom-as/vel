# vel_policy_engine_spec.md

Status: Canonical policy engine specification  
Audience: coding agent implementing Vel  
Purpose: define how Vel evaluates current context, computes policy triggers, maps risk to escalation, and produces nudges/suggestions in a deterministic and inspectable way

---

# 1. Policy Engine Philosophy

The policy engine is the **deterministic decision layer** of Vel.

Its role is to answer:

- given the current signals, commitments, dependencies, context, and risk state,
- what should Vel do **now**?

The policy engine may decide to:

- do nothing
- create a nudge
- escalate a nudge
- resolve a nudge
- create a suggestion
- request clarification later
- queue a reflective synthesis task later

The policy engine must **not**:

- use LLMs for real-time decisions
- invent facts not supported by signals/context
- silently mutate user commitments beyond explicit allowed operational inference
- become an opaque expert system nobody can debug

The engine should be:

- deterministic
- inspectable
- configurable
- conservative under uncertainty
- easy to test with replayed signals/context snapshots

---

# 2. Core Inputs

The policy engine consumes the following durable inputs:

- `signals`
- `current_context`
- `commitments`
- `commitment_dependencies`
- `commitment_risk`
- `nudges`
- `suggestions`
- current wall-clock time

The engine should not query raw external systems directly.  
All facts must come through normalized stores.

---

# 3. Core Outputs

The policy engine may produce these outputs:

## 3.1 Nudge Actions
- create nudge
- escalate nudge
- snooze no-op (if still snoozed)
- auto-resolve nudge
- suppress nudge

## 3.2 Suggestion Actions
- create suggestion
- expire suggestion
- suppress suggestion

## 3.3 Context/Trace Outputs
- explanation payload
- decision event / run event
- optional context timeline entry if state changed

The engine should not directly send notifications.  
It emits decisions that the nudge system / notification adapters then deliver.

---

# 4. Policy Evaluation Cycle

The policy engine should run:

- on every relevant signal ingestion
- on current-context recomputation
- on scheduled reconciliation ticks later

Recommended sequence:

1. load current context
2. load relevant commitments
3. load active nudges
4. load current risk snapshots
5. evaluate policy rules in deterministic order
6. emit decision actions
7. persist decision traces/events

---

# 5. Policy Evaluation Order

Evaluate policies in this order.

## 5.1 Resolution policies
First determine whether active nudges should resolve.

Examples:
- meds task now completed
- prep commitment now completed
- meeting started / ended
- user marked `done`

## 5.2 Suppression / cooldown policies
Determine whether a candidate or existing nudge should remain suppressed.

Examples:
- still within snooze window
- confidence too low
- conflicting higher-severity nudge already active
- focus mode suppresses low-priority interruptions

## 5.3 Creation policies
Create new nudges or suggestions if conditions are met.

## 5.4 Escalation policies
Escalate existing nudges if risk/time has worsened.

## 5.5 Suggestion policies
Emit steerable suggestions such as:
- increase commute buffer
- add prep default
- plan focus block

## 5.6 Queue reflective synthesis (optional later)
Only if thresholds are crossed.

This ordering matters.  
Resolution must happen before escalation, or the system becomes haunted.

---

# 6. Policy Configuration Model

Policies should be configurable, not hardcoded everywhere.

Store policy defaults in configuration files or structured config, for example:

```yaml
policies:
  meds_not_logged:
    enabled: true
    consequence: high
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

  commute_leave_time:
    enabled: true
    default_travel_minutes: 30
    danger_after_leave_by_minutes: 0

  morning_drift:
    enabled: true
    gentle_after_wake_minutes: 20
    warning_after_wake_minutes: 40
    danger_after_wake_minutes: 60
```

Configuration values should be overridable later by:
- project-specific rules
- user steering
- learned policy suggestions

---

# 7. Policy Entities

Each policy should be represented conceptually as:

- policy name
- enable/disable state
- preconditions
- confidence conditions
- severity/escalation thresholds
- output action type
- explanation template

The coding agent does not need a fully generic rules DSL immediately.  
A structured set of policy evaluators with a shared interface is enough.

Suggested interface:

```text
evaluate(policy_context) -> PolicyDecision[]
```

Where `PolicyDecision` is one of:
- CreateNudge
- EscalateNudge
- ResolveNudge
- CreateSuggestion
- NoOp

---

# 8. First Required Policies

Implement these first.

## 8.1 meds_not_logged

### Preconditions
- meds-related commitment exists
- commitment is open
- morning context relevant
- no completion signal yet

### Confidence inputs
- high if authoritative task source says incomplete
- medium if derived from weak capture only

### Escalation model
- gentle: meds expected window passed
- warning: significantly delayed
- danger: still unresolved near higher-risk commitment

### Outputs
- nudge type: `meds_not_logged`

### Resolution conditions
- task completed
- user marks done
- commitment cancelled

---

## 8.2 meeting_prep_window

### Preconditions
- upcoming meeting commitment exists
- prep window known or defaulted
- prep dependency unresolved

### Confidence inputs
- high if event + prep_minutes known
- medium if prep derived from default only

### Escalation model
- gentle: prep window starts soon
- warning: prep window active
- danger: insufficient time remains

### Outputs
- nudge type: `meeting_prep_window`

### Resolution conditions
- prep done
- meeting started (may auto-resolve or transform)
- user marks done/cancel

---

## 8.3 commute_leave_time

### Preconditions
- event requires travel
- travel_minutes known or defaulted
- commute dependency unresolved

### Confidence inputs
- high if travel explicitly tagged
- medium if location + default commute heuristic
- low if guess is weak

### Escalation model
- gentle: leave-by approaching
- warning: leave-by reached
- danger: leave-by missed and event at risk

### Outputs
- nudge type: `commute_leave_time`

### Resolution conditions
- user marks done / leaving
- event starts / ends
- commitment cancelled

---

## 8.4 morning_drift

### Preconditions
- there is a meaningful morning commitment or meds expectation
- current state is `awake_unstarted` or otherwise low-progress
- insufficient progress signals present

### Confidence inputs
- based on signals such as shell login, meds completion, task completion, wake proxy window

### Escalation model
- gentle: morning should have started
- warning: likely delayed
- danger: downstream commitment risk now high

### Outputs
- nudge type: `morning_drift`

### Resolution conditions
- current state moves to `underway` or `engaged`
- critical downstream commitment removed
- user marks done on relevant blocker

---

# 9. Risk → Escalation Mapping

The policy engine should combine rule conditions with current risk state.

Use both:

- **local rule threshold**
- **global/current risk level**

A simple model is acceptable.

## 9.1 Inputs to escalation
- time proximity
- consequence of failure
- dependency pressure
- uncertainty
- existing nudge level
- snooze history
- whether user has ignored prior nudges

## 9.2 Example mapping
- low/moderate risk + first threshold -> gentle
- increasing risk or active prep window -> warning
- high risk + imminent consequence -> danger

## 9.3 Rule
Never escalate solely because time passed.  
Escalate because:
- consequence is increasing
- time-to-action is shrinking
- or downstream commitments are endangered

---

# 10. Nudge Suppression and Cooldowns

The policy engine must avoid noisy repetition.

## 10.1 Suppression cases
- nudge currently snoozed
- user already marked done
- equivalent or stronger nudge already active
- confidence below threshold
- system is missing critical data
- context mode suppresses low-priority alerts

## 10.2 Cooldown behavior
Policies should define minimum intervals between repeat sends.

Example:

```yaml
cooldowns:
  gentle_repeat_minutes: 15
  warning_repeat_minutes: 10
  danger_repeat_minutes: 5
```

The engine should not repeatedly recreate equivalent nudges if an active one already exists.

---

# 11. Suggestion Policies

Suggestions are not nudges.  
They are steerable proposals.

Implement these only after basic nudge flow works.

Examples:

## 11.1 increase_commute_buffer
Trigger when:
- commute-related danger nudges recur
- user often arrives at leave-by late
- user repeatedly steers commute time upward

Output:
- suggestion to increase default commute buffer

## 11.2 increase_prep_window
Trigger when:
- meeting prep is repeatedly too tight
- danger nudges recur before meetings

## 11.3 add_operational_commitment
Trigger when:
- calendar event repeatedly implies same operational subtask
- user repeatedly accepts the same inferred prep/commute commitment

## 11.4 schedule_focus_block
Trigger later when:
- stated priorities receive no calendar allocation
- repeated drift exists on project commitments

These suggestions should default to **pending** and require acceptance / steering.

---

# 12. Explanation Model

Every policy decision must be explainable.

The policy engine should produce structured explanation data for each decision.

Suggested explanation schema:

```json
{
  "policy": "meeting_prep_window",
  "decision": "create_nudge",
  "level": "warning",
  "signals_used": ["sig_1", "sig_2"],
  "commitments_used": ["com_10"],
  "risk_snapshot_id": "risk_33",
  "reasons": [
    "prep window active",
    "prep dependency unresolved",
    "meeting with external counterparty"
  ],
  "suppressed_reasons": []
}
```

This can be stored:
- in nudge metadata
- in a decision event table later
- or derived on demand from logs + current state

The important rule:
**the system must be able to explain why a decision occurred or did not occur.**

---

# 13. Policy Modes / Context Modes

The policy engine should eventually honor context modes.

Initial support may be minimal, but the schema/config should allow:

- `morning_mode`
- `meeting_mode`
- `focus_mode`
- `commute_mode`
- `end_of_day_mode`

Example effects:
- suppress non-critical nudges in focus mode
- prefer prep/commute nudges in meeting mode
- bias toward self-maintenance in morning mode

For initial implementation, modes may simply be derived from current context rather than manually set.

---

# 14. Persistent Current Context Requirements

The policy engine depends on a persistent current context object.

At minimum it should contain:

- current mode/state
- next commitment
- active prep window
- active commute window
- meds status
- active nudge ids
- current risk level
- inferred activity state
- open thread ids

The policy engine should **never** independently recompute a separate shadow context.
It must depend on the current-context subsystem.

---

# 15. Decision Trace Persistence

To support debugging and self-improvement, persist decision traces/events.

Options:
- append to run_events
- append to nudge_events
- add policy_decisions table later if needed

A minimal starting approach:
- nudge events for nudge decisions
- suggestion state changes for suggestion decisions
- current_context timeline entries when context changes materially

If later needed, add:

```sql
policy_decisions
----------------
id
policy_name
decision_type
entity_type
entity_id
explanation_json
timestamp
created_at
```

Do not add this table unless existing event infrastructure proves insufficient.

---

# 16. Steering Integration

The policy engine must honor steering outcomes.

Examples:
- user adjusted commute buffer to 45m
- user rejected prep suggestion
- user repeatedly snoozes a rule at a certain stage
- user marks certain inferred commitments as wrong

These should modify policy behavior via:
- config updates
- accepted suggestions
- future self-model tuning

The engine should not silently “forget” steering history.

---

# 17. Self-Model Integration

The policy engine should emit enough data for Vel to evaluate itself later.

Examples:
- which nudge level caused completion
- which nudges were ignored
- which suggestion types are often rejected
- where danger nudges occurred too often
- where no nudge fired but downstream failure happened

These become inputs to:
- weekly self-review
- policy tuning suggestions
- adaptive defaults later

The policy engine itself should remain deterministic; the self-model can recommend changes later.

---

# 18. Testing Requirements

The policy engine must be tested with replayable scenarios.

## 18.1 Unit tests
Each policy should have:
- creation case
- suppression case
- escalation case
- resolution case

## 18.2 Replay tests
Given a sequence of signals, the engine should produce a deterministic series of decisions.

Example:
- calendar event ingested
- prep commitment inferred
- no activity
- prep window starts
- nudge created
- snooze signal ingested
- cooldown respected
- danger escalation later

## 18.3 Explanation tests
Every decision should produce structured explanation output that contains:
- policy name
- reasons
- referenced signal ids / commitment ids where applicable

---

# 19. Minimal First End-to-End Slice

The first policy engine slice should be:

## meds_not_logged
- task signal ingested
- meds commitment open
- morning context underway
- policy creates gentle nudge
- user snoozes
- time passes / risk increases
- policy escalates to warning
- task completed
- policy resolves nudge

This slice proves:
- signal → context → policy → nudge → acknowledgement → resolution

After that, implement:
- meeting_prep_window
- commute_leave_time
- morning_drift

---

# 20. Practical Engineering Rules

1. Policies are deterministic.
2. LLMs do not participate in immediate policy decisions.
3. Policies consume normalized state, not raw external APIs.
4. Explanations are part of the product.
5. Suppression/cooldown logic is mandatory.
6. Suggestions are separate from nudges.
7. Steering modifies future policy behavior, but usually via explicit accepted adjustments.
8. Optimize for trust over aggressiveness.

---

# 21. Success Criteria

The policy engine is working when:

- the first three nudge classes behave deterministically
- done/snooze transitions are correct
- escalation follows time + risk, not arbitrary spam
- equivalent duplicate nudges are suppressed
- policy decisions are explainable
- replay tests pass
- downstream weekly synthesis has usable traces to analyze

---

# 22. Final Summary

The policy engine is Vel’s **reflex layer**.

It should be:

- fast
- deterministic
- stateful
- inspectable
- conservative under uncertainty
- configurable
- compatible with later reflection and self-improvement

In short:

> facts become context, context becomes policy decisions, policy decisions become nudges or suggestions, and everything remains explainable.