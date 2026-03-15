# vel_next_instructions_after_latest_review.md

Status: Detailed implementation instructions after latest repo review  
Audience: Coding agent implementing Vel  
Purpose: Finish wiring the existing architecture into a coherent operational loop, and avoid duplicating logic across inference, risk, and policy layers.

---

# Executive Summary

The repository is in a **good intermediate state**.

It now has:

- signal adapters
- commitments
- current context scaffolding
- nudge engine scaffolding
- policy config
- partial risk engine
- explain endpoints
- migrations for suggestions
- synthesis scaffolding

However, the implementation is currently at risk of a very common failure mode:

> **the same logic is being partially recomputed in multiple places**

Right now, the following appear to overlap:

- `services/inference.rs`
- `services/risk.rs`
- `services/nudge_engine.rs`

This is the main thing to fix next.

Vel should not evolve into three different little brains arguing over:
- meds status
- prep windows
- morning state
- risk
- escalation timing

The next phase is therefore **not primarily new features**.  
It is **architectural convergence + the missing adaptive loop**.

---

# What To Do Next (Strict Order)

Implement the following in this exact order:

1. **Refactor architecture boundaries**
2. **Add missing tests**
3. **Finish risk engine compute and wire it into context**
4. **Refactor nudge engine to consume context + risk instead of recomputing facts**
5. **Implement suggestions / steering loop**
6. **Implement project synthesis**
7. **Only then start iOS/Watch/voice client work**

Do not start iOS app work before steps 1–6 are in place.

---

# 1. Refactor Architecture Boundaries First

This is the most important immediate task.

## Problem

The current repo appears to compute overlapping state in multiple services:

- `inference.rs` calculates morning state / meds status / prep window
- `nudge_engine.rs` recalculates meds/policy facts directly from signals
- `risk.rs` computes risk with its own commitment/time logic

That duplication will rot the system quickly.

## Target architecture

Use this contract:

### `vel-signals`
Owns:
- external fact ingestion only

### `context reducer`
Owns:
- current context
- morning state
- inferred activity state
- meds status
- prep/commute window activation
- drift state
- top-level summary of what matters now

### `risk engine`
Owns:
- risk computation for commitments
- risk levels
- risk factors / explanation

### `policy / nudge engine`
Owns:
- consumption of current context + risk + commitments
- creation/escalation/resolution of nudges
- creation of suggestions

### `synthesis`
Owns:
- reflective artifact generation

## Required refactor rule

After this refactor:

- `nudge_engine.rs` must **not** compute meds status directly from raw signals
- `nudge_engine.rs` must **not** compute prep window directly from calendar signals
- `nudge_engine.rs` must **not** guess morning state directly

Instead it should read:
- current context
- latest risk snapshots
- active nudges
- policy config

That is the right architecture.

## Required code change

Refactor `services/nudge_engine.rs` so that it consumes:
- `storage.get_current_context()`
- `storage.list_latest_commitment_risk(...)`
- `storage.list_nudges(...)`
- policy config

and uses those to decide actions.

Do not leave duplicated signal-walking logic inside the nudge engine.

---

# 2. Add the Missing Tests Before Further Logic Expansion

The next logic changes are dangerous without tests.

Implement these tests now.

## 2.1 Commute policy tests

Required cases:

### No commute nudge when `travel_minutes` missing
- event exists
- `travel_minutes` absent
- expected: no `commute_leave_time` nudge

### Gentle / warning / danger threshold tests
- event with explicit `travel_minutes`
- advance time across thresholds
- expected: level transitions correctly

### Snooze suppression
- active commute nudge
- snooze
- expected: no repeat before `snoozed_until`

### Resolution by done
- mark commute commitment done
- expected: nudge resolved

### Resolution by event start
- event start passes
- expected: commute nudge resolved or suppressed permanently

## 2.2 Context explain tests

Required cases:

- explanation includes signal ids used
- explanation includes commitment ids used
- explanation changes when relevant inputs change
- unrelated signals/commitments excluded

## 2.3 Resolution ordering tests

Required cases:

- resolution occurs before escalation
- resolved nudge never escalates
- completed commitment suppresses equivalent new nudge
- done dominates snooze when conflicting events are replayed

## 2.4 Material-change tests

Required cases:

- context timeline row created only on semantic change
- recompute with same semantic context produces no new row

These tests should be written before broadening the engine.

---

# 3. Finish Risk Engine Compute Properly

`services/risk.rs` exists, but now it must become the real risk engine rather than a sidecar heuristic.

Reference: `vel_risk_engine_spec.md`

## Implement now

Implement only these factors:

- consequence
- proximity
- dependency pressure

Do NOT implement yet:

- uncertainty
- progress penalty
- learning/adaptation
- LLM involvement

## 3.1 Consequence mapping

Use this initial mapping:

### External calendar-backed commitment with people
- consequence = high (0.9)

### Operational dependency (`prep`, `commute`)
- inherits parent consequence with slight discount (e.g. 0.8 of parent)

### Self commitment
- medium by default (0.5)

### Medication linked to higher-order commitment
- high (0.9)

Do not get fancy yet.

## 3.2 Proximity mapping

Use simple buckets, not faux precision:

- > 2h: low
- 30m–2h: medium
- < 30m: high
- overdue / at due point: critical

If there is no due time:
- low baseline proximity only

## 3.3 Dependency pressure

One-level propagation only.

If:
- parent risk >= high
- child unresolved

then:
- child gets dependency pressure increase

No recursive graph reasoning yet.

## 3.4 Persist snapshots

Every risk computation should persist:
- score
- level
- factors_json
- computed_at

Do not only compute in memory.

## 3.5 CLI and API

Add or finish:

CLI:
```bash
vel risk
vel risk <commitment_id>
```

API:
- `GET /v1/risk`
- `GET /v1/risk/:commitment_id`

Required output fields:
- risk_score
- risk_level
- factors
- dependency ids
- reasons

---

# 4. Feed Risk Into Context and Refactor Context Reducer

Current context must become the canonical present-state object.

Reference: `vel_current_context_spec.md`

## Required additions to current context

Ensure context includes:
- `top_risk_commitment_ids`
- `global_risk_score`
- `global_risk_level`

## Required behavior

The context reducer must:
- consume latest risk snapshots
- include them in context
- expose them in `vel context`
- expose them in `/v1/explain/context`

## Important rule

Context explain must show:
- signal ids used
- commitment ids used
- risk snapshot ids used
- reasons

Structured first, no chatty prose.

---

# 5. Implement Attention / Drift Detection in Context

Reference: `vel_attention_and_drift_detection_spec.md`

Implement only two drift types first:

- `morning_drift`
- `prep_drift`

## 5.1 Add to current context

Add fields:
- `attention_state`
- `drift_type`
- `drift_severity`
- `attention_reasons`
- `attention_confidence`

## 5.2 First heuristics only

### Morning drift
If:
- morning mode active
- meds unresolved or first operational commitments unresolved
- no meaningful workstation/completion signal
- threshold elapsed

Then:
- attention_state = drifting
- drift_type = morning_drift

### Prep drift
If:
- prep window active
- prep dependency unresolved
- no progress signal
- repeated snooze or no engagement

Then:
- attention_state = drifting
- drift_type = prep_drift

Do not invent richer attention models yet.

## 5.3 Important architecture rule

Drift belongs in current context.  
Risk and policy consume drift.  
They do not each build their own private drift model.

---

# 6. Refactor Nudge Engine to Consume Context + Risk

Once context + risk are wired, rewrite policy evaluation accordingly.

Reference: `vel_policy_engine_spec.md`

## Required rule

`nudge_engine.rs` must make decisions from:

- current context
- risk snapshots
- commitments
- dependencies
- active nudges
- policy config

It should not walk raw signal streams for core facts that context already knows.

## Example

### Bad pattern
- policy engine inspects calendar signals directly to determine prep window

### Required pattern
- current context already says `prep_window_active = true`
- policy engine uses that

## Required first policies

Keep only these policies active:

- `meds_not_logged`
- `meeting_prep_window`
- `commute_leave_time`
- `morning_drift`

Do not add more policies yet.

---

# 7. Implement Suggestions / Steering Loop

The schema exists. The behavior does not.

Reference: `suggestions` table + previous specs.

Implement only two suggestion types initially:

- `increase_commute_buffer`
- `increase_prep_window`

## 7.1 Trigger rules

These should be triggered only by repeated evidence.

### increase_commute_buffer
Trigger when:
- repeated `commute_leave_time` danger nudges
- similar route/context
- evidence that current default is too small

### increase_prep_window
Trigger when:
- repeated prep warnings/danger near meetings
- same meeting/prep context or similar commitment kind

Do not trigger on one-off events.

## 7.2 Add storage/API/CLI

CLI:
```bash
vel suggestions
vel suggestion inspect <id>
vel suggestion accept <id>
vel suggestion reject <id>
vel suggestion modify <id> --payload ...
```

API:
- list suggestions
- inspect suggestion
- accept/reject/modify

## 7.3 Acceptance behavior

When accepted:
- update policy/default value explicitly
- persist state change
- keep auditability

## 7.4 Constraint

Do NOT implement:
- natural-language steering
- LLM-generated suggestion payloads
- broad suggestion taxonomy

Keep it structured.

---

# 8. Implement Project Synthesis Properly

Reference: `vel_weekly_synthesis_spec.md`

The next synthesis milestone is not generic weekly poetry.  
It is project synthesis with evidence.

## Command

Implement one of:

```bash
vel synthesize project <project_slug>
```

or

```bash
vel synthesize week --project <project_slug>
```

Either is acceptable, but it must be consistent.

## Input set

Use:
- project commitments
- project threads
- project nudges
- captures linked/tagged to project
- transcript messages linked/tagged to project
- latest risk snapshots

## Output artifact

Artifact type:
- `project_synthesis`

## Required sections

- open commitments
- active threads
- repeated drift
- ideation without execution
- suggested next actions

## Evidence rule

Every section must cite evidence ids:
- commitment ids
- thread ids
- signal ids
- capture ids
- transcript ids
- risk ids where relevant

Do not generate vibe summaries with no provenance.

## Initial target

Get project synthesis working **well for `vel` first**.  
Do not generalize too early.

---

# 9. Add Minimal Event/Action Logging If Missing

The distributed spec assumes an action/event replication substrate.

If the repo does not yet have a clean event log for:
- captures
- acknowledgements
- suggestion acceptance
- state transitions

then add a minimal durable event log now.

Suggested table if needed:

```sql
events (
  id TEXT PRIMARY KEY,
  event_type TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
)
```

Use this only if current run_events / nudge_events / signals do not cover the replication story cleanly.

Do not duplicate event logs unnecessarily. Inspect current tables first.

---

# 10. CLI / UX Improvements Needed Now

Before iOS, finish these operator surfaces.

Required commands:
```bash
vel context
vel explain context
vel risk
vel risk <commitment_id>
vel suggestions
vel suggestion inspect <id>
vel synthesize project vel
```

These should be usable enough that you can dogfood Vel from the terminal without needing Apple clients yet.

---

# 11. iOS / Apple Watch Repo Recommendation

## Recommendation

**Do not put iOS / Watch in a separate repo yet.**

## Why

Right now:
- APIs are still evolving
- context/risk/policy model is still stabilizing
- shared command semantics are still being solidified
- rapid coordination between core and clients matters more than repo isolation

A separate repo right now would create:
- version skew
- duplicated schemas/contracts
- more coordination overhead
- slower dogfooding

## Better structure for now

Keep Apple client work in the same repository, but outside the Rust workspace, e.g.:

```text
clients/apple/VelApp/
clients/apple/VelWatch/
```

or

```text
apps/apple/...
```

This gives:
- shared specs
- one issue stream
- easier evolution of API/contracts
- simpler dogfooding

## When to split later

Consider a separate repo only when all of these are true:

- core API is reasonably stable
- client release cadence diverges from core runtime cadence
- signing/provisioning/App Store workflow becomes dominant
- multiple contributors need clean repo separation

That is not yet the phase you are in.

## Important note

Even in the same repo:
- keep Apple code outside Cargo workspace
- define explicit API/interface boundaries
- do not let client code import arbitrary internal Rust details

The client should talk to Vel core through stable interfaces, not repo osmosis.

---

# 12. What Not To Do Next

Do NOT spend the next phase on:

- separate iOS repo
- wake word
- smart speaker mode
- advanced uncertainty model
- broad LLM policy logic
- natural-language steering
- more suggestion types
- broader attention telemetry
- new distributed/replication complexity

The system needs the loop finished first.

---

# 13. Milestone Definition

The next milestone is reached when all of these are true:

1. current context includes risk + drift
2. policy engine consumes current context + risk
3. nudge escalation is driven by risk, not just time windows
4. suggestions appear from repeated evidence
5. project synthesis produces evidence-backed artifact
6. operator surfaces make this inspectable from CLI/API

Only after that should Apple client implementation accelerate.

---

# 14. Final Summary

Vel is close to the interesting part.

The remaining work is not “invent more architecture.”

It is:

> finish connecting the existing architecture so Vel can actually notice pressure, detect drift, adapt policy, and reflect usefully.

Do this next:
- refactor boundaries
- add tests
- finish risk
- wire risk into context and policy
- add suggestions
- add project synthesis
- keep Apple in the same repo for now

When Vel can say, with evidence:

> “Prep drift is high, commute buffer has repeatedly been too short, and here is the project-level pattern that explains why,”

then the system is becoming real.