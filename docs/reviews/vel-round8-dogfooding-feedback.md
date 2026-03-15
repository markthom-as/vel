# vel — Dogfooding-Phase Review and Next-Step Plan (Round 8)

## Overall assessment

This round is the first one where Vel feels like it is becoming a **tool you could plausibly begin using every day**, not just a well-composed runtime substrate.

That is a real milestone.

The codebase now has a practical operator shell:

- `vel capture`
- `vel recent`
- `vel review`
- `vel inspect`
- `vel artifact latest`
- `vel export`
- `vel import`
- run-backed context flows
- a coherent docs hierarchy
- a stable runtime/data model underneath it all

So the center of gravity has shifted again.

Earlier, the main question was:

> is the architecture real?

Now the question is:

> does the daily loop have enough coverage and enough trust to become habit-forming?

That is a much better problem to have.

My short verdict is:

> **Yes, this is better.**  
> But the next work should now be driven much more by the lived dogfooding loop you described — especially commitments, prep windows, source integration, and tiered nudging — than by additional generic CLI surface area.

The repo is at the stage where more commands alone will not make it more useful. What will make it more useful is a tighter model of **what matters today**, **what is drifting**, and **what deserves a nudge**.

---

# What is clearly stronger in this round

## 1. The operator shell is materially more complete

Compared to earlier rounds, the CLI now has a much more credible day-to-day shape.

The additions around:

- `recent`
- `review`
- `import`
- `export`
- `backup`
- `artifact latest`

are exactly the kind of commands that make the system feel inhabitable.

This is good. It means you are no longer only building runtime internals; you are building an actual shell around them.

---

## 2. Dogfooding is now visible in the code

The repo is beginning to reflect the “use Vel to improve Vel” strategy in concrete ways:

- there is a daily-use doc
- there are review commands
- export exists
- import exists
- inspection flows are broader
- artifacts are no longer only backend objects

That is the correct directional move.

---

## 3. Context as a run-backed workflow is now a real foundation

This remains the strongest architectural achievement in the repo.

Because context generation is run-backed, you now have a stable pattern for future high-value features:

```text
request
  → run
  → artifact
  → refs
  → inspection
```

You should aggressively reuse this pattern for anything non-trivial going forward.

That includes:
- synthesis
- review artifacts
- eventual nudge generation logs
- project summaries
- import jobs later

---

## 4. The docs are much closer to operational truth

`README.md`, `docs/status.md`, and `docs/using-vel-daily.md` are much better than several iterations ago.

They now mostly reflect what the code actually does, which is not glamorous but is one of the most trust-building traits a repo can have.

---

# The most important new reality

The codebase is now entering the **dogfooding phase**, which means the next missing pieces are no longer primarily architectural. They are product-behavioral.

Your recent product answers clarified that Vel is not trying to be:

- a PKM
- a replacement task manager
- a generic chat agent
- an everything app

It is trying to be:

> **a day-centered, commitment-aware, context-sensitive assistant with gentle escalation**

That is a much sharper product identity.

And it changes what “next steps” should mean.

---

# Where the current implementation still falls short for real daily use

The repo now has many commands, but the actual day loop is still thin in the most important places.

The biggest gaps are:

1. there is still no explicit **commitments layer**
2. source integration for your real daily truths is not there yet
3. synthesis is still placeholder text
4. review is present, but still fairly shallow
5. backup/export exist, but are still trust gestures more than robust trust features
6. there is no signal/inference/nudge system yet

That is where the next effort should go.

---

# The single biggest product gap now

## Vel still knows about captures better than it knows about commitments

This is the central thing I would fix next.

From your actual use case, the most important objects are not simply captures. They are:

- things you need to do
- things that are time-sensitive
- things that require prep
- things that drift out of mind
- things that recur and reveal patterns

That means Vel needs a first-class concept of **commitments**.

Right now, typed capture helps, but typed capture is not yet the same thing.

A capture is raw input.  
A commitment is an actionable, reviewable, time-relevant object.

Those should not remain collapsed forever.

---

# Strongest architectural recommendation

## Add a commitment layer next

I would treat this as the next major architectural milestone for dogfooding.

### Minimal commitment model

A commitment should probably include at least:

- `id`
- `text`
- `source_type`
- `source_id`
- `status`
- `due_at` optional
- `project` optional
- `commitment_kind` optional
- `created_at`
- `resolved_at` optional

### Minimal statuses

- `open`
- `done`
- `cancelled`
- maybe `snoozed` later, but not necessarily as a commitment status if snoozing belongs to nudges

### Where commitments come from

Initially:
- explicit typed capture (`--type todo`)
- imported Todoist/Reminders items
- maybe extracted from review/synthesis later

### Why this matters

Because your actual daily loop is not:

> what did I capture?

It is closer to:

> what matters, what is unresolved, what is next, and what am I at risk of dropping?

That is commitment logic.

---

# Source integration should now be the next big implementation phase

This is the major product gap between “architecturally solid” and “actually useful.”

From your answers, the first three sources should be:

## 1. Calendar

This is the highest leverage source.

Vel needs:
- event start
- event end
- event title
- location
- travel/commute awareness
- maybe manual prep duration

Why:
- first meeting prep window
- commute awareness
- morning planning
- lateness risk

This is probably the most important external integration.

---

## 2. Todoist / Reminders

Even if Todoist is first and Apple Reminders becomes the long-term medication source, the architecture should support **source-by-domain** rather than pretending there is one universal source of truth.

Vel should ingest:
- task text
- due date/time
- completion state
- source identity

Then it can derive:
- open commitments
- meds status
- response obligations
- due-today pressure

This is the second highest value integration.

---

## 3. Computer activity

This is a superb signal source for morning-state inference and daily use.

Vel should begin by ingesting only a minimal set:
- login time
- first shell activity
- maybe last activity heartbeat later

This allows inference like:
- workstation engagement
- morning started
- drift from wake time / first meeting prep

This is low-cost and high signal.

---

# The next major feature area should be signals, inferences, and nudges

The repository is now ready for this.

Your product answers narrowed the first useful proactive system down to something very clean.

## First signal sources

- calendar
- Todoist/Reminders
- computer activity

## First inferred states

- meds_logged / not logged
- first_commitment_upcoming
- prep_window_active
- morning_started / not started
- behind_schedule (carefully inferred)

## First nudges

- meds not logged
- first meeting prep window approaching
- morning routine drift

That is the correct first nudge set. Do not expand beyond that yet.

---

# The most useful design rule to adopt now

> **Every proactive behavior should be grounded in explicit signals and expressed as a reversible, inspectable nudge.**

This means:
- no invisible magic
- no giant blob of “AI context”
- no untraceable reminder logic

A nudge should be something Vel can explain:
- what triggered it
- which signals contributed
- what state was inferred
- whether it was snoozed or completed

That keeps the system legible and trustworthy.

---

# The nudge interaction model is now clear enough to implement

Your recent guidance simplifies this beautifully.

Every nudge should have only two meaningful responses:

- `done`
- `snooze`

That is exactly right.

Do not bloat this.

### Why it works

It maps across:
- watch
- desktop
- CLI
- toast notifications
- speaker later

And it minimizes attentional tax.

So I would formalize the nudge state machine as:

- `pending`
- `active`
- `snoozed`
- `resolved`

That is enough.

---

# The current review surface is useful but still too shallow

`vel review today` and `vel review week` are the right additions, but currently they still look more like:

- recent captures
- latest context artifact
- counts

That is a start, but not yet the review loop you actually described.

Given your stated needs, review should evolve toward exposing:

- chronology
- open commitments
- unresolved threads
- recurring motifs
- “what may need attention tomorrow”
- “what has repeated without being resolved”

That is where Vel becomes psychologically useful, not just operationally convenient.

---

# Synthesis is now the most obvious unfinished feature

Right now `vel synthesize` is still a placeholder.

That is fine — but it is also the clearest stub in the repo.

And given your stated goal of using Vel to improve Vel, this should become the next run-backed workflow after commitment/source integration begins.

## First synthesis features I would actually build

### 1. `vel synthesize week`

Output:
- recurring themes
- unresolved commitments
- repeated pain points
- project imbalance
- suggested priorities

### 2. `vel synthesize project vel`

Output:
- repeated Vel-related friction
- implementation ideas
- unresolved architecture issues
- likely next tasks

This is the recursive loop you explicitly want: Vel helping organize Vel’s evolution.

Because context runs are already real, synthesis should copy the same pattern:

```text
request
  → run
  → artifact
  → refs
  → inspection
```

Do not implement synthesis outside the runtime spine.

---

# Import/export/backup: good start, but still mostly first-pass

## Import

`vel import` is useful, but currently still very thin:
- file import is essentially capture creation from file contents
- line import is capture creation from stdin
- URL import is just typed URL capture

That is fine for now. It is honest and practical.

I would not overbuild connectors yet.

But I **would** make sure imports eventually produce better provenance:
- imported-from path
- imported-at timestamp
- maybe line number or batch id for line imports

That will matter once you rely on imported data more.

## Export

Export is a good trust gesture, but it still feels more like:
- “dump some JSON”
than
- “reliable data portability story”

That is okay for now, but long term you probably want:
- manifest
- captures
- runs
- artifacts metadata
- maybe refs
- maybe bundled artifact files

## Backup

Right now backup is guidance text, not a backup system.

That is not wrong, but it should be documented honestly as:
- backup instructions
not
- backup functionality

This is a place where words matter.

---

# One code-adjacent product issue: the roadmap is stale

`docs/roadmap.md` is now noticeably behind the repo reality.

It still reads like a much earlier phase:

- search across captures
- today and morning context assembly
- worker loop beyond skeleton
- artifact ingestion for files and transcripts

But many of those have already happened in meaningful form.

This matters because roadmap drift quietly erodes clarity.

I would update the roadmap so it now reflects the dogfooding phase:

## Near term should probably become something like

- commitments layer
- calendar integration
- Todoist/Reminders integration
- workstation activity ingestion
- signal/inference/nudge engine
- weekly/project synthesis
- trust improvements (real export/backup)

That would align the roadmap with both the code and your current intent.

---

# The default Vel experience still needs to become more opinionated

You described a future where typing:

```bash
vel
```

gives you the morning briefing automatically.

The CLI currently still requires subcommands.

This is not urgent, but it is worth naming:

> the eventual default entrypoint should feel like entering Vel, not selecting a subcommand from a Swiss Army knife.

I would not implement this before:
- commitments
- source integration
- morning-state logic

But I would keep it as the medium-term shell UX goal.

---

# Concrete next implementation sequence I would recommend

This is the sequence I would use from here.

## Phase 1 — commitments and sources

1. add commitment model
2. ingest Todoist/Reminders into commitments
3. ingest calendar events
4. ingest minimal computer activity signals

### Exit criteria
- Vel can answer “what matters today?” from more than raw captures
- meds/meeting prep can be grounded in real data

---

## Phase 2 — morning planning with prep windows

1. compute first commitment of day
2. support prep duration and commute duration
3. surface:
   - next commitment
   - prep start
   - leave by
   - unresolved commitments
4. expose this in morning context artifact

### Exit criteria
- `vel morning` becomes actually day-shaping, not just context-shaped

---

## Phase 3 — nudge engine v1

1. add signal model
2. add inferred state model
3. add nudge object/state machine
4. implement first three nudges:
   - meds not logged
   - prep window approaching
   - morning drift
5. implement done/snooze actions

### Exit criteria
- Vel can gently intervene using explicit rules and explicit state

---

## Phase 4 — synthesis

1. implement `vel synthesize week`
2. implement `vel synthesize project vel`
3. make both run-backed
4. persist artifacts and refs
5. expose them through inspection surfaces

### Exit criteria
- Vel can help improve Vel from inside its own data

---

## Phase 5 — trust and portability

1. strengthen export
2. convert backup from guidance to actual operation
3. include refs and artifact metadata in exports
4. maybe bundle artifacts or create manifest-based restore path

### Exit criteria
- using Vel daily feels less like “interesting experiment” and more like “recoverable system”

---

# Specific doc changes I would make now

## 1. Update `docs/roadmap.md`

This is the most obviously stale doc now.

## 2. Add a new spec:
```text
docs/specs/signals-inference-nudges.md
```

This should define:
- first signal sources
- first inferred states
- first nudges
- done/snooze protocol
- escalation channels

That would turn a lot of recent product clarity into implementation clarity.

## 3. Add a commitments section to:
- `docs/data-model.md`
- maybe `docs/product-spec.md`
- maybe `docs/using-vel-daily.md`

Because commitments are now clearly the missing center.

---

# Blunt product read

The repo has passed the stage where the most valuable thing is more runtime architecture polish.

What matters now is whether Vel can become:

- a useful morning organizer
- a keeper of unresolved commitments
- a detector of drift
- a generator of weekly reflection
- a recursive project aid for itself

That is the product.

Not “general cognition runtime” in the abstract.

Those abstract words are now only useful if they cash out into:
- better mornings
- fewer dropped commitments
- better weekly reflection
- better Vel planning

---

# Final assessment

This round is stronger. The shell is broader, the docs are mostly honest, and the system feels much closer to inhabitable daily software.

But the next leap will not come from more CLI breadth.

It will come from building the first truly useful internal loop around:

```text
commitments
signals
inferences
nudges
weekly synthesis
```

That is where Vel stops being a capable runtime with useful commands and starts becoming the actual attentional prosthetic you described.

And that is exactly the right next phase.