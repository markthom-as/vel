# vel_operator_cockpit_spec.md

Status: Canonical operator cockpit specification  
Audience: coding agent implementing Vel  
Purpose: define the minimum CLI/operator surfaces required to dogfood and debug Vel before Apple/voice clients become the main interaction surfaces

---

# 1. Purpose

Before iPhone, Watch, and voice become primary user surfaces, Vel must be usable and inspectable from the terminal.

This spec defines the **operator cockpit**:
the set of CLI commands and outputs that let the user understand:

- what Vel thinks is happening
- what is at risk
- what nudges are active
- what suggestions exist
- what synthesis says
- why any of the above is true

The operator cockpit is not just for debugging.  
It is the first serious dogfooding interface.

---

# 2. Design Principles

## 2.1 Show, don't mystify
Every major Vel subsystem should have a corresponding inspectable CLI surface.

## 2.2 Structured first, readable second
Commands should support:
- readable human output by default
- `--json` where appropriate

## 2.3 One command per domain
Do not overload one command to reveal everything badly.

## 2.4 Operator cockpit before glossy clients
If the CLI cannot make the system legible, the Apple clients will just become prettier confusion.

---

# 3. Required Commands

These are the minimum commands Vel must provide now.

## 3.1 Context

```bash
vel context
vel context --json
vel context timeline
```

Purpose:
- inspect current present-state model
- inspect recent material state transitions

### Default human output should show:
- mode
- morning state
- attention/drift state
- next commitment
- prep window status
- commute window status
- meds status
- active nudges count
- top risk commitments
- global risk level

Example:

```text
Mode: morning_mode
Morning state: underway
Attention: drifting (prep_drift, high)
Next commitment: Meeting with Dimitri at 11:00
Prep window: active
Commute window: active
Meds: pending
Active nudges: 2
Top risk commitments:
  - com_prep_dimitri (critical)
  - com_commute_dimitri (high)
Global risk: high (0.81)
```

## 3.2 Explain context

```bash
vel explain context
vel explain context --json
```

Must include:
- signal ids used
- commitment ids used
- risk snapshot ids used
- reasons
- drift reasons if present

Example:

```text
Context computed at: 10:25
Signals used:
  - sig_calendar_dimitri
  - sig_shell_login_0920
  - sig_nudge_snooze_prep
Commitments used:
  - com_meds
  - com_prep_dimitri
  - com_commute_dimitri
Risk used:
  - risk_meeting_1025
  - risk_prep_1025
Reasons:
  - upcoming external commitment at 11:00
  - prep dependency unresolved
  - commute leave-by threshold reached
  - prep nudge snoozed once
```

## 3.3 Risk

```bash
vel risk
vel risk <commitment_id>
vel risk --json
```

### `vel risk`
Show top risk commitments ordered by score.

### `vel risk <commitment_id>`
Show:
- score
- level
- factor breakdown
- dependency ids
- reasons

Example:

```text
Commitment: Prepare for Meeting with Dimitri
Risk: critical (0.88)

Factors:
  consequence: 0.72
  proximity: 0.95
  dependency_pressure: 0.84

Dependencies:
  - depends on meeting_dimitri

Reasons:
  - meeting starts in 30 minutes
  - prep unresolved
  - parent meeting is external/high consequence
```

## 3.4 Nudges

```bash
vel nudges
vel nudges --active
vel nudge inspect <id>
vel done <nudge_or_commitment>
vel snooze <nudge_or_commitment> <minutes>
```

### `vel nudges`
Show:
- active
- snoozed
- recently resolved

Each row should show:
- nudge type
- level
- state
- related commitment
- snoozed_until if applicable

Example:

```text
nud_001  meds_not_logged       warning   active    com_meds
nud_002  meeting_prep_window   danger    snoozed   com_prep_dimitri until 10:35
nud_003  commute_leave_time    danger    active    com_commute_dimitri
```

### `vel nudge inspect <id>`
Show:
- metadata
- reasons
- event history
- related commitment/thread/suggestion ids

## 3.5 Suggestions

```bash
vel suggestions
vel suggestion inspect <id>
vel suggestion accept <id>
vel suggestion reject <id>
vel suggestion modify <id> --payload ...
```

### `vel suggestions`
Show:
- suggestion type
- state
- evidence count / context hint
- created_at

Example:

```text
sug_001  increase_commute_buffer  pending  route:SLC weekday_morning
sug_002  increase_prep_window     pending  meeting_kind:external
```

### `vel suggestion inspect <id>`
Must include:
- structured payload
- evidence ids
- rationale

## 3.6 Project synthesis

```bash
vel synthesize project <project_slug>
vel artifact latest --type project_synthesis
```

For `vel` specifically this should produce a readable artifact summary.

The operator cockpit must make it easy to see whether Vel can reflect on itself.

---

# 4. Recommended Additional Commands

These are very helpful, even if not all are fully mature.

## 4.1 Threads

```bash
vel threads
vel thread inspect <id>
vel thread open
```

## 4.2 Signals

```bash
vel signals
vel signals --type calendar_event
vel signals --recent
```

## 4.3 Commitments

```bash
vel commitments
vel commitment inspect <id>
```

## 4.4 Explain drift

```bash
vel explain drift
```

This can initially be an alias or filtered explain-context surface.

---

# 5. Output Conventions

## 5.1 Human output
Readable and compact.
Do not dump raw JSON unless requested.

## 5.2 JSON output
Every core command should support `--json` eventually.

Important first commands to support:
- `vel context --json`
- `vel explain context --json`
- `vel risk --json`
- `vel nudges --json`
- `vel suggestions --json`

This matters for automation, testing, and dogfooding scripts.

## 5.3 IDs
Use ids consistently and visibly enough that the operator can inspect linked entities.

---

# 6. Command Responsibilities

## Context commands
Show present state.

## Explain commands
Show evidence and reasoning.

## Risk commands
Show prioritization pressure.

## Nudge commands
Show active intervention layer and allow direct acknowledgement.

## Suggestion commands
Show adaptive loop and policy tuning proposals.

## Synthesis commands
Show reflective output.

This domain separation is important.  
Do not collapse them into one "status" blob.

---

# 7. Canonical Operator Workflow

The operator cockpit should support this daily loop.

## Morning
```bash
vel context
vel explain context
vel risk
```

## During drift / confusion
```bash
vel nudges
vel nudge inspect <id>
vel explain context
```

## During adaptation
```bash
vel suggestions
vel suggestion inspect <id>
```

## Reflection
```bash
vel synthesize project vel
vel artifact latest --type project_synthesis
```

If this workflow feels bad, the system is not ready for broader clients.

---

# 8. Explainability Rules

The cockpit must make it possible to answer:
- why is this the next commitment?
- why is this high risk?
- why did this nudge fire?
- why did this suggestion appear?
- what signals were used?
- what evidence is missing?

This is the heart of user trust.

---

# 9. Testing Requirements

Add CLI-level or integration-level tests for:

## 9.1 Context output
- includes required fields
- reflects state changes

## 9.2 Explain context
- includes relevant ids
- excludes unrelated ids

## 9.3 Risk output
- top commitments ordered by risk
- inspect shows factors and reasons

## 9.4 Nudge actions
- done resolves
- snooze delays
- inspect shows history

## 9.5 Suggestions
- list shows pending suggestions
- inspect shows rationale/evidence
- accept/reject updates state

## 9.6 Synthesis
- project synthesis command generates artifact
- latest artifact retrievable

---

# 10. Minimum Dogfooding Standard

Vel is not ready for wider client development until the operator cockpit can clearly support:

1. seeing current context
2. understanding current risk
3. seeing active nudges
4. resolving/snoozing nudges
5. understanding why the system acted
6. seeing adaptive suggestions
7. generating project synthesis

This is the minimum viable operator surface.

---

# 11. Practical Engineering Rules

1. Every operator command should map to one subsystem.
2. Human output first; JSON optional but important.
3. Explanations must cite ids and reasons.
4. Keep command semantics stable.
5. The cockpit should be usable before Apple clients exist.
6. Do not hide load-bearing state behind uninspectable APIs.

---

# 12. Final Summary

The operator cockpit is Vel's first serious control room.

It is how you prove that:
- the system is coherent
- the state is inspectable
- nudges are explainable
- suggestions are grounded
- synthesis is useful

In short:

> if the terminal cannot make Vel legible, the rest of the clients will only make it prettier, not better.
