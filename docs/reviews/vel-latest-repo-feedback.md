# vel_latest_repo_feedback.md

Audience: Vel coding agent / repo maintainer  
Purpose: architecture, code quality, product, and implementation feedback on the latest repository snapshot

---

# Overall verdict

This is **materially better** than the earlier iterations.

The repo now clearly has real shape:

- a coherent workspace
- explicit crate boundaries
- current-context scaffolding
- policy configuration
- risk engine scaffolding
- suggestion scaffolding
- project synthesis scaffolding
- Apple client directory established
- CLI / daemon separation preserved

That is all good.

The bad news is the classic next-stage problem:

> the repo is now at risk of becoming a collection of mostly-correct subsystems with overlapping responsibility and quietly divergent truth.

That is a much better problem than “there is no architecture,” but it is still the problem to solve next.

The strongest theme in this review is:

> **finish convergence before adding breadth.**

There is enough here to build the first truly dogfoodable Vel loop. The main risk now is not missing primitives. It is duplicated logic, side effects in read paths, and placeholder implementations hardening into production behavior.

---

# What is clearly strong

## 1. Repo structure is disciplined

The workspace layout is sane:

- `vel-core`
- `vel-storage`
- `vel-config`
- `vel-api-types`
- `veld`
- `vel-cli`
- `clients/apple`

This still feels like the right decomposition.

## 2. Apple client placement is right

Keeping Apple inside the same repo but outside Cargo is the correct move **for now**.  
Do not split it yet.

## 3. Specs have successfully shaped the codebase

That is visible now. The code is not just concept cosplay anymore; it is starting to reflect:
- current context
- risk
- policies
- explanations
- synthesis
- suggestions

That is real progress.

## 4. The system is close to a first operational loop

The following now exist in recognizable form:

```text
signals
→ inference/context
→ risk
→ policy/nudges
→ artifact/synthesis
```

The next task is making those pieces stop competing and start composing.

---

# The biggest architectural issue

## Duplicated state logic still exists

The repo still appears to compute related truth in multiple places.

The most obvious hotspot is:

- `services/inference.rs`
- `services/risk.rs`
- `services/nudge_engine.rs`
- `routes/explain.rs`

The code is much closer to the intended architecture, but it still leaks responsibility across subsystem boundaries.

### Desired contract

#### Inference / current context
Owns:
- morning state
- meds status
- inferred activity
- prep/commute window activation
- drift state
- next commitment
- current-mode summary

#### Risk engine
Owns:
- commitment risk scores
- risk levels
- risk factors
- dependency pressure

#### Nudge engine
Owns:
- reading current context + risk + active nudges
- deciding create/escalate/resolve/suppress

#### Explain endpoints
Own:
- exposing existing reasoning/state
- never recomputing core state with side effects

This needs to be enforced harder.

---

# Specific code / architecture feedback

## 1. `explain_commitment` should not call `risk::run()`

This is the most important concrete smell I saw.

In `routes/explain.rs`, `explain_commitment` appears to call:

```rust
let risk_snapshots = risk::run(&state.storage, now_ts).await?;
```

That is extremely problematic for a read/explain endpoint because:

- it recomputes risk during read
- it persists new risk snapshots during read
- it makes explanation endpoints state-mutating
- it can create duplicate / noisy risk history
- it blurs “show me what the system thinks” with “make the system think again”

### Required change
Refactor explain endpoints so they only:
- read latest persisted risk snapshots
- or explicitly call a separate evaluation endpoint/command if recomputation is desired

### Rule
**Explain routes must be read-only.**

If you want on-demand recomputation:
- make it an explicit `evaluate` / `refresh` command
- not a side effect of explanation

This one matters a lot.

---

## 2. `inference.rs` is carrying too much policy-ish behavior

`services/inference.rs` currently does a lot:

- fetches today’s signals
- derives meds status
- chooses first event
- defaults `prep_minutes`
- defaults `travel_minutes`
- computes prep/commute window activation
- selects next commitment
- derives global risk fallback
- derives attention/drift state
- emits timeline state
- writes inferred state
- writes current context
- emits events

That file is becoming a very crowded little monarchy.

Some of that belongs there. Some does not.

### What belongs there
- current-context reduction
- state derivation
- material-change detection

### What should move or become delegated
- default policy values should come from config, not ad hoc literals
- risk fallback heuristics should come from the risk layer or a well-defined fallback contract
- next commitment selection should become a clearer helper with explicit ordering
- drift heuristics should be isolated as subfunctions, not accreted inline

### Recommendation
Refactor `inference.rs` into internal subfunctions or modules:
- `derive_meds_status(...)`
- `derive_temporal_windows(...)`
- `derive_attention_state(...)`
- `select_next_commitment(...)`
- `build_context_json(...)`

Right now it is still readable, but it is on the slope toward becoming sacred mud.

---

## 3. `next_commitment` selection looks too naive

I saw:

```rust
let next_commitment_id = open_commitments.first()
```

That is almost certainly too weak.

If `list_commitments` is not already strongly ordered by actual urgency, this will drift into nonsense quickly.

### Required behavior
Next commitment selection should prioritize something like:

1. external calendar-backed commitments
2. operational prerequisites for those commitments
3. due commitments with nearest due time
4. high-risk open commitments
5. remaining open commitments

### Recommendation
Make next-commitment selection an explicit helper with a tested ordering policy.

Do not let “whatever row came first” become product behavior by accident.

---

## 4. `first_event = min_by_key(timestamp)` is a subtle bug magnet

In `inference.rs`, the “first event” for today appears to be chosen via minimum timestamp among today’s calendar signals.

That can be wrong if:
- an earlier event already happened
- the most relevant event is the next future event, not the earliest event of the day
- multiple events exist and the day has already progressed

### Required behavior
For operational context, you usually want:
- the next relevant future event
not
- the earliest event that happened at any point today

### Recommendation
Change this logic to:
- choose the nearest upcoming event at or after `now`
- possibly fall back to currently active event if inside its window

This is a product correctness issue, not just a code-style issue.

---

## 5. Hardcoded defaults in inference should come from config

I saw patterns like:
- `prep_minutes.unwrap_or(15)`
- `travel_minutes.unwrap_or(0)`

Given that you already have `policies.yaml`, this should not remain scattered.

### Required change
All temporal defaults should come from config:
- default prep minutes
- commute thresholds
- meds thresholds
- morning drift thresholds

### Rule
No silent operational defaults in multiple modules.

Have one authoritative config source and thread it through.

---

## 6. `mode` logic currently looks suspicious / placeholder-ish

I saw something roughly like:

- if prep window active → `meeting_mode`
- else if commute window active → `commute_mode`
- else if certain states → `morning_mode`
- else → `morning_mode`

That final else means the fallback is always `morning_mode`, which is likely not semantically correct long-term.

### Recommendation
If this is placeholder logic, mark it as such in code.
If not, refine it.

Likely future modes:
- morning
- meeting_prep
- commute
- focus
- neutral / day
- end_of_day later

But for now, at least avoid a mode enum that is effectively “morning unless proven otherwise.”

---

## 7. `risk_used` currently appears misnamed

In `inference.rs`, the field called `risk_used` appears to contain commitment ids rather than risk snapshot ids.

That is confusing and will bite explainability.

### Required change
Use distinct fields if needed:
- `risk_snapshot_ids_used`
- `top_risk_commitment_ids`

Do not overload names here. Explanation payloads live or die on semantic precision.

---

## 8. Risk engine: good first pass, but still too write-heavy

The risk engine is directionally good:
- consequence
- proximity
- dependency pressure
- factor JSON
- persisted snapshots

That’s the right shape.

But `risk::run()` currently:
- walks all open commitments
- persists snapshots every run
- seems likely to generate a lot of write churn

This is acceptable briefly, but not as a steady-state strategy.

### Recommendation
Move toward one of:
- only recompute changed / affected commitments
- only persist on material risk change
- or maintain “latest current risk” plus append-only history on meaningful transitions

You do not need to solve this perfectly now, but don’t let “every read or every tick emits all risk rows forever” harden.

---

## 9. Risk engine likely has N+1 dependency queries

I saw a loop that appears to query dependencies per commitment.

That’s a typical early-stage smell.

### Recommendation
Load dependency relationships in one batch if possible.
You’re going to want that anyway once project synthesis and thread logic get denser.

This is not an emergency if scale is tiny, but it is a correctness-of-shape issue.

---

## 10. Nudge resolution logic needs tighter domain specificity

In `nudge_engine.rs`, I saw logic suggesting commute resolution may depend on things like:
- event started
- or `open_commitments.is_empty()`

That second condition is too broad / suspicious.

A commute nudge should resolve because:
- the commute commitment resolved
- the event started/passed
- event cancelled
- commute no longer relevant

Not because the universe ran out of all open commitments.

### Recommendation
Audit every resolution clause and make it domain-specific.

Generic shortcuts here produce haunted nudges later.

---

## 11. Suggestion system should stay tightly constrained

If suggestions are partially implemented, keep them narrow.

The first two are still the right ones:
- `increase_commute_buffer`
- `increase_prep_window`

Do not let the suggestion layer bloom into “Vel has many feelings” before there is strong repeated evidence plumbing.

### Rule
Suggestions must remain:
- evidence-based
- structured
- inspectable
- explicitly accepted/rejected

No natural-language policy drift yet.

---

## 12. Project synthesis still feels placeholder-heavy

From what I saw, project/weekly synthesis is structurally present but still quite skeletal.

That is okay.

But for project synthesis to become genuinely useful, it must do more than:
- dump open commitments
- dump open threads
- leave repeated drift / ideation-without-execution empty

### Required next improvement
For `vel` specifically, implement at least:
- ideation without execution:
  - recent transcripts/captures mentioning Vel but no linked resolved commitment
- repeated drift:
  - repeated prep/commute/morning-drift related to Vel work if available
- suggested next actions:
  - derived from open commitments + recent ideation clusters

And every section needs evidence ids.

No vibes, no oracle mist.

---

# Dryness / elegance / style feedback

## What feels good
- naming is mostly clear
- docs/spec influence is visible
- comments are purposeful
- JSON structures are explicit
- crate boundaries are still cleaner than average

## Where it starts to smell
### 1. Big service files
A few service files are becoming “God service” candidates.

### 2. Inline JSON assembly everywhere
This is okay early, but a lot of repeated `serde_json::json!` payload construction across modules will eventually make consistency brittle.

### Recommendation
Introduce small typed explanation/output structs for internal use in hot paths where the shape matters a lot.

Not everywhere, but especially for:
- context explain
- risk explain
- suggestion payloads
- synthesis sections

### 3. Placeholder behavior hiding in real paths
Some fallback behavior still feels more like “good enough for now” than intentionally temporary.
Mark those more explicitly or refine them before they become de facto product.

---

# Product feedback

## 1. Vel is now close enough that wrong defaults matter
Earlier, architectural purity mattered most.
Now, behavioral correctness matters more.

Examples:
- which event counts as “next”
- when commute actually becomes active
- what counts as drift
- when a nudge resolves
- how risk is surfaced

These are now product questions, not just implementation trivia.

## 2. The next “wow” moment is still the same
The product becomes real when it can correctly do something like:

> “Your meeting is at 11. Prep and commute are unresolved. Morning drift is rising. You should leave soon.”

and then:
- let me snooze/done
- update state correctly
- explain why
- later suggest a better buffer if this keeps happening

Everything should still be bent toward that.

## 3. Don’t let synthesis get ahead of the operational loop
Synthesis is compelling, but the system still wins or loses on:
- current context correctness
- risk correctness
- nudge correctness
- explanation correctness

Reflective intelligence after operational truth, not before.

---

# Best-practice recommendations for the next phase

## 1. Refactor for responsibility, not for cleverness
Do not invent a generic engine framework.
Just make each subsystem own one thing clearly.

## 2. Make read paths read-only
This is non-negotiable.
Explain routes should not mutate risk or context.

## 3. Centralize defaults in config
No scattered literals for threshold-ish behavior.

## 4. Prefer one canonical scenario replay test
You already have the right idea with the canonical day.
Make that the spine of truth.

## 5. Finish CLI/operator cockpit before broad client work
Before Apple/voice becomes primary, the terminal needs to be the control room.

Required commands should feel strong:
- `vel context`
- `vel explain context`
- `vel risk`
- `vel nudges`
- `vel suggestions`
- `vel synthesize project vel`

---

# Concrete next steps for the agent

Implement these in order:

## 1. Remove side effects from explain routes
Especially `explain_commitment`.

## 2. Refactor `inference.rs`
Split into explicit helpers:
- next commitment selection
- temporal window derivation
- meds state derivation
- attention/drift derivation
- material change comparison

## 3. Fix “next future event” selection
Do not use earliest event of the day when the day is already underway.

## 4. Move defaults to config
Use `policies.yaml` and inject values consistently.

## 5. Tighten nudge resolution semantics
Make them commitment/event specific, not global-shortcut based.

## 6. Improve project synthesis for `vel`
Make at least one truly useful project synthesis output with evidence-backed “ideation without execution” and “suggested next actions.”

## 7. Add replay tests for the canonical day
That remains the most valuable test asset in the whole system.

---

# iOS / repo guidance

Still: **keep iOS / Watch in the same repo for now**.

Why:
- core contracts are still moving
- a separate repo would amplify drift
- API and command semantics still need close coordination

Recommended layout remains:
- same repo
- outside Cargo workspace
- explicit API boundary
- no direct duplication of business logic in Swift

---

# Final assessment

This repo is no longer vague.  
It is now **specific enough that the main risks are architectural duplication, state mutation in the wrong places, and placeholder behavior becoming product truth**.

That’s a healthy stage to be in.

The next leap is not new scope.  
It is **tightening the one operational loop until it behaves like one assistant instead of several smart subroutines politely disagreeing off-camera**.

When that happens, Vel will stop feeling like a promising system and start feeling like a useful one.