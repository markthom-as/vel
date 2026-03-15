# vel — v1 Nudge Spec and Morning State Machine

## Status

Proposed  
Audience: implementation planning / coding agent / product design  
Scope: first proactive layer for Vel, with emphasis on daily orientation, morning state, commitment-aware nudges, and escalation behavior

---

# 1. Purpose

This spec defines the **first proactive behavior layer** for Vel.

Vel already has the beginnings of a runtime substrate, context generation, artifacts, provenance, and inspection. The next step is to let Vel become a **context-aware commitment assistant** that can:

- ingest a small number of high-value signals
- infer a minimal morning state
- generate nudges when drift is detected
- escalate nudges by time proximity and risk
- accept a tiny acknowledgement protocol:
  - **Done**
  - **Snooze**

The goal is not to build an omniscient life OS.  
The goal is to ship the smallest system that is already useful in daily practice.

---

# 2. Product framing

Vel is not trying to replace:

- Todoist / Reminders as a task store
- Calendar as an event store
- Obsidian as a PKM
- chat as the primary interaction model

Vel’s job is narrower and stronger:

> **Vel helps keep commitments visible in context, detects drift, and nudges with graded intensity.**

This means Vel is responsible for:

- context-aware orientation
- prep-window awareness
- commitment visibility
- lightweight executive-function scaffolding
- storing artifacts and runtime traces of what it inferred and why

---

# 3. v1 scope

The first proactive layer should support exactly three nudge classes:

1. **Meds not logged**
2. **First meeting prep window approaching**
3. **Morning routine drift**

These were chosen because they are:

- high-value
- time-sensitive
- frequent enough to matter
- supported by realistic input signals
- simple enough to implement without a massive inference engine

---

# 4. v1 input signals

Only ingest the smallest signal set needed to earn trust.

## 4.1 Calendar signals

Use calendar events as the source of timed obligations.

Required fields:

- event id
- title
- start time
- end time
- location (optional)
- travel minutes (optional, manual field at first)
- prep minutes (optional, manual field or system default)

Primary use:

- detect first meaningful commitment of the day
- compute prep window
- compute leave-by time if commute exists

---

## 4.2 Task/reminder signals

For now, tasks may come from Todoist. Later, Apple Reminders may be the preferred source for certain domains such as medication.

Vel should treat task sources as **domain-specific authoritative systems**, not one universal truth source.

Required fields:

- task id
- task text
- completion state
- due time (optional)
- labels / project (optional)
- source name (`todoist`, `reminders`, etc.)

Primary use:

- detect whether meds are logged
- detect task-like commitments due today
- map completion to `done`

---

## 4.3 Computer activity signals

Use workstation activity as a strong signal that morning has begun moving toward engagement.

Possible signals:

- machine login time
- shell start time
- first `vel` invocation
- first keyboard activity if available later

Primary use:

- infer “engaged at workstation”
- distinguish “awake but unstarted” from “actually in morning flow”

---

# 5. Out-of-scope signals for v1

These should be explicitly deferred, not forgotten:

- Apple Watch biometric/wake data
- smart speaker acknowledgement
- office presence sensors
- phone location/geofencing
- messaging inbox response-state
- motion / smart-home events

These are promising later signals, but not required for first usefulness.

---

# 6. Core domain concepts

Vel should model four distinct layers.

## 6.1 Facts

Observed data from systems Vel trusts for that domain.

Examples:

- calendar event at 11:00
- Todoist meds task incomplete
- shell login at 08:42
- task marked complete in Reminders

Facts should be stored as imported or observed signals/events, not guesses.

---

## 6.2 Inferences

Interpretations produced from facts plus time.

Examples:

- morning has likely started
- meds likely still pending
- prep window active
- user likely behind schedule

Inferences should be explicit and time-bounded.

---

## 6.3 Commitments

Things that matter and may need completion or response.

Examples:

- take meds
- prepare for meeting
- leave by 10:30
- reply to message

Vel is not necessarily the system of record for commitments, but it should be able to reason about them.

---

## 6.4 Nudges

Proactive prompts emitted by Vel when an inference and policy threshold are met.

Examples:

- “Meds not logged yet.”
- “Prep window for your first meeting has started.”
- “Morning appears delayed.”

---

# 7. Nudge classes

v1 supports only these three.

## 7.1 Meds not logged

### Intent

Surface that medication has not yet been completed/logged during the relevant window.

### Required inputs

- medication task exists or is expected for the day
- medication task still incomplete
- time has passed medication expectation threshold

### Example message

- Gentle: “Meds not logged yet.”
- Warning: “Meds still not logged.”
- Danger: “Meds still not logged and your first commitment is approaching.”

---

## 7.2 First meeting prep window approaching

### Intent

Prevent being late or underprepared for the day’s first major commitment.

### Required inputs

- first meeting/event of the day exists
- prep duration known or defaulted
- optional travel duration known or defaulted

### Derived values

- `prep_start = event_start - prep_minutes`
- `leave_by = event_start - travel_minutes`

### Example message

- Gentle: “Prep window begins soon.”
- Warning: “Prep window has started.”
- Danger: “You may be late unless you leave soon.”

---

## 7.3 Morning routine drift

### Intent

Detect that the morning is not progressing in a way that supports known commitments.

### Required inputs

- wake threshold reached or first morning window active
- no meds logged
- no workstation activity
- no relevant morning-progress signals
- time elapsed exceeds thresholds

### Example message

- Gentle: “Morning hasn’t started yet.”
- Warning: “Morning routine appears delayed.”
- Danger: “Morning routine is behind schedule.”

---

# 8. Nudge acknowledgement protocol

Vel should intentionally support only **two user responses**:

## 8.1 Done

Meaning:

- the relevant task/commitment is completed
- the nudge should resolve
- the system should stop escalating this nudge

Examples:

- meds taken
- prep completed
- leave started
- reply sent

### Effects

- mark nudge resolved
- if possible, update linked commitment/task state
- persist acknowledgement event

---

## 8.2 Snooze

Meaning:

- task not completed yet
- remind again later

### Effects

- move nudge into `snoozed`
- schedule next reminder time
- preserve escalation state or slightly decay it, depending on policy

### Default snooze options

For v1, use fixed choices:

- 5 minutes
- 10 minutes
- 30 minutes

On constrained UIs (watch/toast), it is acceptable to expose only one default snooze, such as 10 minutes.

---

# 9. Nudge state model

A nudge should move through a very small state machine.

## States

- `pending`
- `active`
- `snoozed`
- `resolved`

### `pending`
Vel has determined the condition exists but has not yet surfaced it.

### `active`
Vel has surfaced the nudge and is awaiting user action or escalation.

### `snoozed`
User requested delay.

### `resolved`
User marked the issue done, or the external source of truth confirmed completion.

---

# 10. Escalation model

Vel should escalate by **time proximity and consequence**, not by arbitrary annoyance.

This is crucial.

The system should behave like:

- calm
- attentive
- graduated

not like a nagging alarm daemon with no sense of proportion.

---

## 10.1 Escalation levels

Use three levels in v1.

### Level 1 — Gentle

Purpose:
- surface the condition
- low intrusiveness
- early warning

Example channels:
- watch haptic
- watch notification
- desktop toast
- shell message next time Vel is invoked

### Level 2 — Warning

Purpose:
- increase urgency
- indicate relevant time window is active or closing

Example channels:
- repeated watch notification
- more visible desktop toast
- stronger CLI phrasing

### Level 3 — Danger

Purpose:
- convey likely consequence
- reserve for time-sensitive scenarios with high confidence

Example channels:
- repeated haptic
- louder or more intrusive local alert
- “klaxon” style sound acceptable for high-priority deadlines if explicitly enabled

This level must be used sparingly, or the whole system becomes noise.

---

## 10.2 Escalation principle

Escalation should be driven by:

- time until consequence
- confidence in the underlying signals
- whether the user has snoozed recently
- whether the nudge has already been surfaced multiple times

In plain terms:

> Vel should escalate more as time runs out and confidence rises.

---

## 10.3 Time-proximity policy

Suggested generic progression:

- **Gentle**: condition first becomes relevant
- **Warning**: key action window has started or user is clearly drifting
- **Danger**: missing the action likely causes immediate cost

Examples:

### Meds
- gentle: meds expected window opened
- warning: meds still not logged well past expected time
- danger: meds still not logged and first timed commitment is near

### Meeting prep
- gentle: prep window starts in 15 minutes
- warning: prep window has started
- danger: leave-by or final prep threshold is near/past

### Morning drift
- gentle: morning should have started by now
- warning: no meaningful progress signals
- danger: first commitment risk is now high

---

# 11. Morning state machine

Morning should be inferred, not hardcoded to one signal.

Vel should support a minimal inferred morning state.

## States

- `inactive`
- `awake_unstarted`
- `underway`
- `engaged`
- `at_risk`

---

## 11.1 `inactive`

Likely not yet meaningfully awake or no evidence of morning progress.

Possible facts:
- no workstation activity
- no meds logged
- no relevant task completion
- first commitment still distant

---

## 11.2 `awake_unstarted`

Some sign of wakefulness exists, but morning routine has not meaningfully progressed.

Possible facts:
- first morning window reached
- calendar commitments today
- but no meds/logins/completions yet

This is the main state from which “morning drift” can emerge.

---

## 11.3 `underway`

Morning routine appears to be in progress.

Possible evidence:
- meds logged
- first task completed
- first interaction with Vel
- workstation activity detected

This does not mean fully ready; it means the machine has started moving.

---

## 11.4 `engaged`

User is actively at workstation or clearly progressing through day-start behavior.

Possible evidence:
- shell login or first sustained machine interaction
- recent use of Vel
- multiple morning actions completed

This state reduces the need for “morning hasn’t started” style nudges.

---

## 11.5 `at_risk`

Time pressure is now high relative to the first major commitment.

Possible evidence:
- prep window active or missed
- leave-by threshold close or exceeded
- meds still incomplete
- minimal progress signals

This state should make warnings and danger nudges more likely.

---

# 12. Morning state transitions

These should be simple and conservative.

## Example transitions

### `inactive -> awake_unstarted`
Triggered when:
- first meaningful morning window is reached
or
- one wake-related signal is observed later in future versions

### `awake_unstarted -> underway`
Triggered when:
- meds logged
or
- first explicit interaction occurs (`vel`, login, relevant completion)

### `underway -> engaged`
Triggered when:
- workstation activity occurs
or
- multiple morning signals indicate real progress

### any state -> `at_risk`
Triggered when:
- prep window activates without sufficient progress
- first meeting/leave-by threshold approaches
- meds remain incomplete and time pressure rises

### `at_risk -> engaged`
Triggered when:
- critical blockers are resolved
- user logs key completions
- schedule risk materially decreases

---

# 13. Confidence model

Vel should not escalate based on weak evidence alone.

Use a simple informal confidence model in v1.

## Confidence sources

### High confidence
- task marked completed in task source
- calendar event exists at a concrete time
- explicit user response (`done`, `snooze`)
- shell login or Vel invocation occurred

### Medium confidence
- inferred morning drift from lack of expected signals
- task text suggests a commitment but has no due time

### Low confidence
- vague contextual assumptions
- weak pattern matches
- stale sensor data

## Policy

Danger-level nudges should require:
- high confidence in the relevant fact
- high consequence if missed

Example:
- “meeting in 15 minutes and travel still required” can justify danger
- “maybe you meant to do something sometime” absolutely cannot

---

# 14. Notification adapters

Vel should separate **nudge generation** from **delivery channel**.

Architecture:

```text
signals -> inferences -> nudge engine -> notification adapters
```

Adapters in v1 and near-v1:

- CLI / shell
- desktop toast
- watch notification

Later:
- smart speaker
- phone push
- richer haptics / sound

This keeps the logic centralized and channels interchangeable.

---

# 15. Watch-first interaction model

Given the stated user preference, the watch should be treated as the most important away-from-computer acknowledgment interface.

## v1 watch alert shape

Display:

- short title
- short message
- two actions only:
  - Done
  - Snooze

Examples:

### Meds
“Vel: Meds not logged yet.”
Buttons:
- Done
- Snooze

### Prep window
“Vel: Prep window started.”
Buttons:
- Done
- Snooze

That’s enough.

---

# 16. Nudge persistence

Nudges should be durable objects, not ephemeral one-off notifications.

At minimum, store:

- nudge id
- nudge type
- related run / commitment / task ids if available
- level
- state
- created_at
- last_sent_at
- snoozed_until
- resolved_at
- channel history if convenient later

Why this matters:

- inspection
- debugging
- analytics
- future synthesis (“what did Vel remind me about repeatedly?”)

---

# 17. Run integration

All proactive workflows should continue using the runtime substrate.

Recommended pattern:

- signal ingestion may be its own lightweight workflow
- nudge generation should produce runtime traces or artifacts where useful
- daily context remains run-backed
- weekly synthesis can later summarize:
  - frequent nudges
  - unresolved commitments
  - repeated drift patterns

This is how Vel eventually uses Vel to improve Vel.

---

# 18. First implementation slice

Implement the smallest end-to-end path that already helps.

## Slice A — passive morning + meds nudge

- ingest calendar
- ingest Todoist tasks
- ingest shell/computer activity
- infer first commitment
- infer meds status
- compute morning state
- generate meds nudge if needed
- support `done` and `snooze`

This is enough to prove the whole loop.

---

## Slice B — meeting prep nudges

- support `prep_minutes`
- support `travel_minutes`
- compute prep window
- generate prep-related nudges
- escalate by time proximity

This is likely the first feature that really earns user trust.

---

## Slice C — morning drift

- infer lack of progress
- generate gentle -> warning -> danger based on elapsed time and first commitment

This is useful, but should come after the first two are working reliably.

---

# 19. Suggested CLI commands

These commands give Vel a coherent operator surface.

## Read / orient

```bash
vel
vel morning
vel recent
vel commitments
```

## Capture / acknowledge

```bash
vel capture "todo: take meds"
vel done meds
vel snooze meds 10m
vel done prep
vel snooze prep 5m
```

## Inspect

```bash
vel runs
vel run inspect <id>
vel inspect artifact <id>
```

Eventually:

```bash
vel nudges
vel nudge inspect <id>
```

---

# 20. Weekly synthesis implications

Once nudges are durable and artifacts exist, Vel can later generate useful reflective summaries.

Examples:

- most frequent unresolved nudges
- repeated morning drift patterns
- commitments repeatedly snoozed
- commitments resolved quickly vs late
- recurring high-danger nudges

This is where Vel becomes not only a reminder system, but a reflective one.

---

# 21. Design principles

These should be treated as hard rules.

## Principle 1
**Done and Snooze are enough.**

Do not explode the acknowledgement model.

## Principle 2
**Escalate by consequence, not by impatience.**

## Principle 3
**Be helpful before being intrusive.**

## Principle 4
**Never pretend certainty you do not have.**

Prefer:
- “You may be late unless you leave soon.”

Not:
- “You are late.”

## Principle 5
**Trust is more valuable than coverage.**

Better to do three nudges well than twelve badly.

---

# 22. Open questions for later

Not blockers for v1:

- exact Apple Watch integration path
- Reminders vs Todoist domain splitting
- smart speaker voice UX
- richer sensor fusion
- message-response nudges
- project-specific recurring thread nudges

These should not delay the first useful version.

---

# 23. Definition of done for v1 proactive layer

Vel v1 proactive support is complete when:

- calendar ingestion works
- task/reminder ingestion works for at least one source
- shell/computer activity is visible as a signal
- morning state is inferred
- meds nudge exists
- meeting prep nudge exists
- `done` / `snooze` works
- at least one notification channel beyond CLI exists
- nudges escalate by time proximity
- inspection/debugging exists for runs and nudges

---

# 24. Final summary

Vel v1 proactivity should be:

- narrow
- trustworthy
- calm
- time-aware
- watch-friendly
- easy to acknowledge

The right first system is not “AI that runs your life.”

It is:

> **a small, reliable engine that notices drift around commitments and helps you recover in time.**