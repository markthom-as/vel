# vel_canonical_day_fixture_spec.md

Status: Canonical integration fixture specification  
Audience: coding agent implementing Vel  
Purpose: define one end-to-end realistic day scenario that every major Vel subsystem must pass before the system is considered dogfoodable

---

# 1. Purpose

Vel now has many specs and partial subsystems:
- signals
- current context
- risk
- policy / nudges
- suggestions
- project synthesis
- distributed/offline assumptions

This fixture exists to force those pieces into one coherent operational loop.

Vel should be able to survive **one believable day** before more architectural expansion happens.

This fixture is that day.

---

# 2. Core Principle

The canonical day fixture is not just test data.

It is the first integrated proof that Vel can:

- ingest signals
- compute context
- compute risk
- detect drift
- fire nudges
- accept Done / Snooze
- update state correctly
- emit suggestions from repeated evidence
- produce project synthesis

If the system cannot handle this scenario, it is not ready for:
- Apple clients
- voice orchestration
- broader distributed behavior
- richer synthesis

---

# 3. Canonical Day Scenario

Use this exact baseline scenario.

## Date / context

- weekday morning
- user starts in Ogden
- first important commitment is in Salt Lake City
- user has not yet taken meds
- workstation activity begins later than ideal
- there is one repeated Vel-related project thread active in the background

## Main external commitment

Calendar event:

- title: `Meeting with Dimitri`
- start_time: `11:00`
- location: `Salt Lake City`
- prep_minutes: `30`
- travel_minutes: `40`
- external / high consequence: yes

Derived expectations:
- prep_start = `10:30`
- leave_by = `10:20`

## Related commitments

### Commitment A
- title: `Take meds`
- kind: `medication`
- state: open
- linked as prerequisite for stable morning functioning

### Commitment B
- title: `Prepare for Meeting with Dimitri`
- kind: `prep`
- state: open
- dependency: required for meeting

### Commitment C
- title: `Commute to Meeting with Dimitri`
- kind: `commute`
- state: open
- dependency: required for meeting

### Project thread
- project: `vel`
- one open commitment linked to it, e.g. `Finish risk engine`
- one recent capture linked to it
- one recent transcript message linked to it

---

# 4. Signal Timeline

Use this exact timeline for replay/integration tests.

## 08:30
Signals:
- calendar event already present
- meds task open
- no workstation activity yet

Expected:
- current context exists
- next commitment = Meeting with Dimitri
- meds_status = pending
- prep_window_active = false
- commute_window_active = false
- morning_state likely `awake_unstarted` or equivalent
- global risk low/medium, not yet high

## 09:00
Signals:
- still no meds completion
- still no workstation activity

Expected:
- morning drift may begin emerging
- no prep nudge yet
- no commute nudge yet
- risk rising slowly

## 09:20
Signals:
- first workstation/shell activity appears

Expected:
- morning_state moves toward `underway` / `engaged`
- meds still pending
- no prep window yet
- context explanation references shell activity signal

## 09:35
Signals:
- meds still not done
- user invokes Vel or context queried
- no prep done

Expected:
- `meds_not_logged` nudge may exist depending on thresholds
- if user snoozes, nudge enters snoozed state
- context reflects active nudge

## 10:10
Signals:
- no prep completion
- no commute completion
- meds still pending or maybe now resolved depending on branch
- one snooze already occurred

Expected:
- commute nudge gentle or warning depending on thresholds
- prep risk rising
- current context global risk increasing
- drift may transition from morning drift to prep drift

## 10:20
Signals:
- leave_by threshold reached
- commute not marked done

Expected:
- commute nudge enters danger
- explanation shows explicit `travel_minutes`
- risk for commute + meeting high/critical

## 10:30
Signals:
- prep window active
- prep unresolved

Expected:
- prep drift active
- `meeting_prep_window` nudge warning or danger depending on branch
- risk engine shows meeting/prep dependency pressure

## 10:35
User action branch A:
- user marks meds done
- user snoozes prep

Expected:
- meds nudge resolves
- prep nudge remains active/snoozed
- context updates
- resolution happens before escalation in any replay

## 10:45
User action branch B:
- user marks commute done or indicates leaving
- prep still unresolved

Expected:
- commute nudge resolves
- meeting risk may remain high because prep unresolved
- current context changes materially

## 11:00
Signal:
- event start passes

Expected:
- unresolved commute nudge auto-resolves or suppresses permanently
- unresolved prep nudge resolves/suppresses according to policy
- current context mode changes out of prep/commute mode

---

# 5. Required System Behaviors

The system must demonstrate these behaviors under the canonical fixture.

## 5.1 Current context
Must correctly compute:
- next commitment
- meds status
- prep window activation
- commute window activation
- morning state
- drift type
- active nudge ids
- top risk commitments
- global risk level

## 5.2 Risk engine
Must compute:
- meeting risk
- prep dependency risk
- commute dependency risk
- reasons/factors
- explainable level transitions

## 5.3 Drift detection
Must detect at least:
- morning drift
- prep drift

Do not require richer drift classes for this fixture.

## 5.4 Policy engine / nudges
Must support:
- meds nudge
- prep nudge
- commute nudge
- Done / Snooze behavior
- escalation ordering
- resolution ordering

## 5.5 Suggestions
Repeated evidence branch should be able to trigger:
- `increase_commute_buffer`
or
- `increase_prep_window`

This may require replaying the same day fixture more than once.

## 5.6 Project synthesis
At the end, `vel synthesize project vel` should include:
- open Vel commitments
- related thread
- any ideation capture/transcript linked to Vel
- suggested next action

---

# 6. Required Assertions

The coding agent should implement these as integration assertions.

## 6.1 Context assertions
- next commitment is the meeting
- prep window false before 10:30, true at/after 10:30
- commute window false before leave-by ladder, then relevant
- meds pending until done
- active nudge ids update correctly

## 6.2 Risk assertions
- meeting risk rises as time approaches
- prep unresolved raises dependency pressure
- commute unresolved raises dependency pressure
- risk explanations cite dependency reasons

## 6.3 Nudge assertions
- no commute nudge without explicit `travel_minutes`
- commute nudge gentle/warning/danger at configured thresholds
- snooze suppresses repeated firing until `snoozed_until`
- done resolves immediately
- event start suppresses or resolves stale commute nudge

## 6.4 Ordering assertions
- resolution occurs before escalation
- resolved nudge never re-escalates from stale state
- completed commitment suppresses equivalent future nudge

## 6.5 Explain assertions
- context explain references relevant signal ids
- context explain references commitment ids
- context explain references risk snapshot ids where applicable
- unrelated entities excluded

## 6.6 Synthesis assertions
- project synthesis artifact created
- includes evidence ids
- includes open commitments
- includes active thread
- includes suggested next action

---

# 7. Fixture Variants

After the canonical base fixture works, add only these variants.

## Variant A: success path
- meds done on time
- prep done before danger
- commute done before leave-by
- low final risk

## Variant B: drift path
- meds delayed
- prep snoozed
- commute danger triggered
- strong drift signatures

## Variant C: suggestion path
Replay repeated drift path over multiple days to trigger:
- commute buffer suggestion
- prep window suggestion

Do not add more scenario families until these three work.

---

# 8. Test Harness Guidance

Implement this fixture as:

- deterministic integration fixtures
- replayable signal/action sequence
- explicit timestamps
- assertion checkpoints after each significant event

Prefer:
- a single fixture builder
- reusable helpers for:
  - calendar event creation
  - commitment creation
  - activity signal creation
  - nudge acknowledgement
  - replay stepping

This fixture should become the system's integration spine.

---

# 9. Success Criteria

Vel passes the canonical day fixture when:

- context, risk, policy, and nudges all behave coherently
- Done / Snooze work correctly
- drift appears where expected
- suggestions can be triggered after repeated evidence
- project synthesis artifact is generated with provenance
- explain surfaces remain inspectable

Only then should broader client and ambient work accelerate.

---

# 10. Final Summary

The canonical day fixture is the first real proof that Vel can survive a day of actual life.

In short:

> if Vel cannot get you from "meeting at 11, meds pending, prep unresolved, commute required" to a coherent set of warnings, state transitions, and explanations, it is not ready for more abstraction.
