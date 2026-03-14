# vel — Follow-up Feedback and Next-Step Plan

## Overall verdict

Yes — this is **better**.

The important improvements are real, not cosmetic:

- the crate boundary leak is fixed
- `(run_id, seq)` uniqueness exists now
- docs have a clearer hierarchy
- `README`, `status.md`, and `AGENTS.md` are much more honest about what is implemented vs planned
- the repo identity is tighter: **local-first cognition runtime** is now the dominant frame instead of a chorus of competing selves

That is the right direction.

The repo now feels less like a proliferating idea-field and more like a system with a center of gravity.

But — and here comes the useful part — the next tranche of work should be less about adding surface area and more about **making the runtime substrate actually own the product-relevant flows**.

Right now the architecture has improved enough that the remaining gaps are easier to name.

---

# The big picture

You have successfully moved Vel from:

> “promising architecture notes with some implementation”

to something more like:

> “a real local-first system with an emerging runtime spine”

The next move is to make sure that the runtime spine is not just present in schema and inspection commands, but is the thing that **actually carries the important operations**.

In other words: the skeleton exists. Now it has to start bearing weight.

---

# What is clearly better now

## 1. Boundary hygiene is materially improved

The most important fix landed:

- `vel-storage` no longer depends on `vel-api-types`
- domain-ish structs such as `ContextCapture`, `SearchResult`, `OrientationSnapshot` live in `vel-core`
- API DTOs now look like boundary objects again

That is a real architectural correction, not a style tweak.

## 2. Documentation authority is much less murky

The addition of:

- `docs/status.md`
- stronger `README` status language
- `docs/reviews/`

helps a lot.

Before, the docs risked becoming a little ecclesiastical — too many scriptures, unclear canon. Now there is at least a plausible hierarchy.

## 3. `AGENTS.md` is sharper

The boundary rules are much more useful now. That file is no longer just a philosophical weather report.

## 4. The runtime model is starting to feel intentional

`runs`, `run_events`, uniqueness on sequence, CLI inspection — this is the right substrate.

The repo now has enough structure that future work can be layered instead of improvised.

---

# The highest-value next steps

These are the changes I would make next, in order.

---

# 1. Move context generation onto the run substrate for real

## Why this is now the main event

This is still the biggest architectural gap.

The docs now correctly say that:

- context endpoints exist
- but they are not yet run-backed

And the code confirms that `today`, `morning`, and `end_of_day` are still computed directly in `routes/context.rs` from `orientation_snapshot()`.

That was acceptable earlier. It is now the next obvious thing to fix.

If Vel’s product identity is about **capture, recall, and daily orientation**, then orientation is not a side quest. It is the center of the thing.

That means it should be run-backed.

## What to implement

Create an application/service layer for context generation.

Suggested shape:

- `ContextKind` enum
  - `Today`
  - `Morning`
  - `EndOfDay`

- `ContextGenerationInput`
  - `context_kind`
  - `requested_at`
  - optional parameters

- `ContextGenerationOutput`
  - structured result data
  - produced artifact metadata
  - source ids / provenance

- `ContextGenerationService`
  - loads orientation inputs
  - creates run
  - transitions run to running
  - computes result
  - persists output artifact
  - writes refs/provenance
  - appends run events
  - transitions run terminally

## What this buys you

This single change would finally unify:

- the runtime model
- the artifact system
- provenance
- operator inspection
- context generation

Right now those are still adjacent ideas. This change makes them one thing.

## Concrete run event sequence I’d want

For a successful morning run:

1. `run_created`
2. `run_started`
3. `context_generated`
4. `artifact_written`
5. `run_succeeded`

And on failure:

1. `run_created`
2. `run_started`
3. `run_failed`

That is boring. Good. Boring is how runtimes earn trust.

---

# 2. Replace stringified JSON in domain/API with typed JSON values

## Why this matters now

The current code still has:

- `Run.input_json: String`
- `Run.output_json: Option<String>`
- `Run.error_json: Option<String>`
- `RunEvent.payload_json: String`

and API DTOs like:

- `RunDetailData.input: String`
- `RunDetailData.output: Option<String>`
- `RunEventData.payload: String`

This is fine as bootstrap plumbing. It is not a good permanent contract.

You’re currently paying the taxes of JSON while declining the benefits of JSON.

## Recommendation

Use:

- `serde_json::Value` in `vel-core`
- `serde_json::Value` in `vel-api-types`
- `TEXT` in SQLite remains fine
- parsing/serialization happens at the storage boundary

## Why this is worth doing before run-backed context

Because once context runs start producing more interesting inputs and outputs, you will want to:

- assert on them in tests
- pretty-print them in CLI
- version them
- inspect them structurally
- avoid re-parsing strings everywhere

This is one of those changes that is cheap now and irritating later.

## Suggested migration strategy

No schema migration needed if DB stays `TEXT`.

Refactor steps:

1. change core types to `serde_json::Value`
2. update storage mapping to serialize/deserialize
3. update DTOs to use `Value`
4. only stringify in CLI formatting

That’s clean and surgical.

---

# 3. Introduce a real application/service layer in `veld`

## Why

The routes are still carrying too much logic.

`routes/context.rs` currently does:

- snapshot retrieval
- tokenization/focus extraction
- commitment extraction
- response assembly

That is a lot for an HTTP route module.

If you proceed directly from here into run-backed context, provenance, or synthesis without introducing a service layer, the route modules will start to become ceremonial glue-balls.

## Recommendation

Introduce an application service layer, likely in `veld` or a new crate if you want to get fancy later.

For now, `veld` is enough.

Suggested modules:

```text
crates/veld/src/services/
  context_generation.rs
  doctor.rs
  runs.rs
```

### Rule

Routes should mostly do:
- parse request
- call service
- map to DTO
- return

### Why

This will make:
- tests easier
- run-backed context straightforward
- reuse possible for future worker/background execution

It also gives you a clearer place for “application logic that is not pure core domain and not storage.”

That layer is currently missing.

---

# 4. Make `Run` transitions actually own state transitions

## Current issue

`Run::start`, `Run::succeed`, `Run::fail`, and `Run::cancel` still mostly validate and return `Ok(())`.

That means the type checks the law but does not actually perform it.

The real transition currently still lives elsewhere.

That leaves `vel-core` in an awkward state: semantic bouncer, not semantic owner.

## Recommendation

Move to one of these patterns:

### Option A — immutable transitions

```rust
fn start(self, now: OffsetDateTime) -> Result<Self, VelCoreError>
fn succeed(self, now: OffsetDateTime, output: Value) -> Result<Self, VelCoreError>
fn fail(self, now: OffsetDateTime, error: Value) -> Result<Self, VelCoreError>
fn cancel(self, now: OffsetDateTime) -> Result<Self, VelCoreError>
```

### Option B — domain transition helpers

If you dislike methods on the model, create a core transition service that returns updated runs plus event intents.

## Why I prefer A right now

It is smaller, more obvious, and pushes actual semantics back into the domain model where they belong.

Right now `Run` knows the allowed transition graph but not how to inhabit it. That’s a slightly neurotic state for a runtime object.

---

# 5. Make doctor structured internally

## Current state

`DoctorData` is still basically:

- `daemon: String`
- `db: String`
- `artifact_dir: String`
- `schema_version: u32`
- `version: String`

This is easy to render but weak as a data model.

## Recommendation

Introduce:

```rust
enum DiagnosticStatus {
    Ok,
    Warn,
    Fail,
}

struct DiagnosticCheck {
    name: String,
    status: DiagnosticStatus,
    message: String,
}
```

Then `DoctorData` becomes:

- `checks: Vec<DiagnosticCheck>`
- `schema_version`
- `version`

The CLI can still print plain text. The API becomes more machine-usable. The type system stops being decorative.

## Why this matters

Because diagnostics are a real operator interface. You will eventually want to:
- add more checks
- render warnings distinctly
- maybe consume diagnostics programmatically
- test them structurally

Strings like `"ok (created)"` are a nice first pass and a mediocre substrate.

---

# 6. Tighten artifact semantics before they spread

## Current state

Artifacts are improved, but still semantically fuzzy.

You currently have fields like:

- `storage_uri`
- `content_hash`
- `size_bytes`
- `metadata_json`

but the repo still does not fully distinguish between:

1. artifacts Vel **manages**
2. artifacts Vel merely **references**

This is where subtle confusion breeds.

## Recommendation

Define artifact provenance classes explicitly.

### Suggested split

#### Managed artifact
Vel writes and owns the file/object.
- canonical relative path
- checksum required
- size required
- atomic write expectations apply

#### External artifact
Vel references something elsewhere.
- URI may be external
- checksum optional/best-effort
- size optional/best-effort
- not fully durability-controlled by Vel

### Why this matters

Because once context generation starts writing output artifacts, you want those artifacts to be **managed**.

While imported URLs, linked docs, etc. may be **external**.

Right now `storage_uri` blurs those cases.

## Minimum implementation change

Add one explicit field or enum, such as:
- `artifact_storage_kind: managed | external`

That one bit will save a lot of future ambiguity.

---

# 7. Clarify `events` vs `run_events` vs `refs` in canonical docs

## Current state

This is better than before, but still not fully nailed down.

The repo now has the structure to support:
- global events
- per-run events
- refs/provenance

But I still do not think the semantics are crisp enough in the canonical docs.

## Recommendation

Create a dedicated section in `docs/vel-data-model.md` or `docs/vel-runtime-concepts.md`:

### Events vs Run Events vs Refs

#### `events`
System-wide timeline / audit events.

#### `run_events`
Lifecycle and milestones within a single run.

#### `refs`
Stable relationships among durable entities.

Then give 3–5 concrete examples.

## Why this matters

Because without this, contributors will improvise:
- whether to write both
- when to write refs
- whether artifact production is an event or a ref
- whether search belongs in `events`, `run_events`, or both

That ambiguity metastasizes.

---

# 8. Consolidate documentation names and reduce “vel-*” repetition

## This is minor but worth fixing

The hierarchy is improved, but the doc names still feel a bit like a belt wearing suspenders:

- `vel-architecture.md`
- `vel-runtime.md`
- `vel-runtime-concepts.md`
- `vel-data-model.md`
- `vel-product-spec.md`
- `vel-roadmap.md`

Because they already live under `docs/`, the `vel-` prefix on every filename is doing less than it thinks it is.

## Recommendation

Rename canonical docs more plainly:

- `docs/architecture.md`
- `docs/runtime.md`
- `docs/runtime-concepts.md`
- `docs/data-model.md`
- `docs/product-spec.md`
- `docs/roadmap.md`

Keep review/archive docs prefixed or nested if you want.

## Why

Cleaner navigation, less visual redundancy, easier referencing in `AGENTS.md`.

This is not urgent, but it is a nice time to do it before external contributors or agent tooling start depending on the names.

---

# 9. Sharpen `AGENTS.md` further by removing future-state vagueness

## What improved

The new boundary rules are good.

## What still feels loose

Sections like:

- memory graph
- alignment engine
- execution layer

still drift toward conceptual vapor relative to the repo’s actual current implementation.

I get why they’re there. They’re part of the long arc. But for a coding agent, they are less useful than sharper local rules.

## Recommendation

Add a section like:

```markdown
## Current implementation truth

As of now:
- captures, search, runs, run-events, and doctor are implemented
- context endpoints exist but are not yet run-backed
- run payloads are still stringified JSON
- artifacts have partial metadata/provenance support
- services are still thin or route-local in some areas
```

That small dose of anti-delusion would be very useful for an automated contributor.

Also consider trimming or demoting the more speculative conceptual vocabulary unless a doc is explicitly vision-oriented.

---

# 10. Add one intentionally boring integration test for run-backed context once implemented

This is planning for the next patch, but I’d line it up now.

Once context generation is run-backed, add an integration test that verifies:

1. context request creates run
2. run transitions queued → running → succeeded
3. artifact is written
4. run event sequence is correct
5. refs link run → artifact and artifact → inputs

This is the test that proves the runtime spine is actually carrying the organism.

---

# Proposed implementation sequence from here

If I were sequencing the next patch series, I would do it like this.

## Phase 1 — Runtime payloads and service layer

1. introduce service modules in `veld`
2. refactor context logic out of route handlers into service
3. convert `Run` and `RunEvent` payload fields to `serde_json::Value`
4. update storage serialization/deserialization
5. update DTOs and CLI formatting

### Exit criteria
- no more raw JSON strings at domain/API level
- route modules are thinner
- existing tests still pass

---

## Phase 2 — Run-backed context generation

1. define `ContextKind`
2. define structured run input/output/error payloads
3. create run lifecycle in context service
4. persist generated artifact
5. create provenance refs
6. append run events
7. return result through API/CLI

### Exit criteria
- `today` and `morning` are run-backed
- `end_of_day` can follow in same patch or immediately after
- `vel run inspect` shows meaningful context run detail

---

## Phase 3 — Artifact and provenance tightening

1. distinguish managed vs external artifacts
2. normalize checksum/size population behavior
3. document artifact semantics
4. ensure context-generated artifacts are managed artifacts

### Exit criteria
- artifact behavior is no longer semantically mushy
- provenance around generated outputs is inspectable

---

## Phase 4 — Diagnostics and docs refinement

1. introduce structured doctor checks
2. update CLI doctor display
3. update API doctor endpoint shape
4. document events/run-events/refs semantics canonically
5. optionally simplify doc filenames

### Exit criteria
- doctor has typed structure
- docs stop forcing readers to triangulate truth from vibes

---

# Planning changes I would make

## 1. Add a short “next milestone” doc

Create something like:

`docs/specs/context-runs.md`

This should be narrow and practical:
- current state
- target state
- event sequence
- artifact behavior
- provenance behavior
- acceptance tests

You now have enough docs. What you need next is not more breadth — it is one tight spec for the next architectural move.

## 2. Make `docs/status.md` the anchor for all work planning

Any feature PR or coding-agent change should answer:
- does this move something from “partial” to “implemented”?
- does it introduce a new “partial” state?
- does it change what is “planned next”?

This would make the repo feel much more self-aware.

## 3. Add one “architecture constraints” section to the roadmap

Something like:

- no API types in storage
- no route-local business logic for runtime-backed operations
- no stringified JSON in core/API
- context generation must be run-backed
- managed artifacts must have checksum and size

Those hard constraints are worth writing down once, centrally.

---

# Architecture changes I would make after the next patch, not before

These are not immediate, but I would keep them in view.

## 1. Consider a small `vel-app` crate later

Not now. Later.

If `veld` begins to accumulate a lot of application services, a dedicated crate for application/use-case orchestration might eventually make sense.

But today that would likely be premature. Right now just introduce service modules in `veld`.

## 2. Introduce job queue / worker semantics only after context runs land

You already have some worker scaffolding. Resist the temptation to build job infrastructure before the system has one or two meaningful run-backed workflows.

Otherwise you end up building a train station before confirming there’s a town.

## 3. Consider typed run payload enums later

Today:
- `serde_json::Value` is enough

Later:
- if run kinds proliferate, consider typed payload enums or wrappers

Do not jump there now unless the code really asks for it.

---

# Concrete doc changes I would make now

## README

Add one sentence clarifying that context endpoints are still computed synchronously and not yet persisted as run outputs.

That honesty is useful.

## `docs/vel-runtime-concepts.md`

Add explicit sections on:
- run lifecycle
- run events
- refs/provenance
- how context generation will fit into the runtime

## `docs/vel-data-model.md`

Add a canonical table of:
- entity
- purpose
- durable?
- event-backed?
- ref-backed?

For example:

| Entity | Purpose | Durable | Notes |
|---|---|---:|---|
| run | execution record | yes | statusful lifecycle |
| run_event | per-run timeline | yes | ordered by seq |
| event | global system event | yes | system-wide observability |
| ref | durable relationship | yes | provenance/links |
| artifact | durable output/reference | yes | may be managed or external |

That kind of table cuts through a lot of ambiguity.

## `AGENTS.md`

Add a “Current implementation truth” section as mentioned above.

That would make it significantly more useful.

---

# One subtle strategic note

You’ve improved the architecture enough that you are now at risk of a different kind of mistake: **premature elegance**.

Be careful not to turn every next step into a grand ontological cleanup.

The next real win is not a more beautiful abstraction tree.

It is this:

> when I call `vel morning`, I should be able to inspect the resulting run, see the produced artifact, and understand its provenance.

That one experience is worth more than six additional conceptual modules.

---

# My blunt recommendation

If I were steering the repo for the next couple of iterations, I would tell the coding agent:

1. stop touching broad docs for a minute
2. build a proper context generation service
3. make it run-backed
4. make payloads typed
5. make artifacts/provenance honest
6. then come back and tighten diagnostics/docs

That is the shortest path from “good architecture” to “architecture actually doing work.”

---

# Final assessment

This revision is definitely stronger.

The repo now feels less fragmented, less speculative, and more internally truthful.

The next challenge is no longer “what should Vel be?”  
The next challenge is:

> can the runtime substrate become the actual carrier of the most important behavior?

That is the right problem to have.

And frankly, it’s a much nicer one than the earlier risk of becoming a very elaborate mood board with migrations.