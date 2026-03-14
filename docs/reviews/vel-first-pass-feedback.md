# vel — First-Pass Feedback on the Current Implementation

## Overall read

This is a **good first pass**.

It has crossed the line from “concept repo with aspirations” into “real runtime taking shape.” The most important thing is that the repo now has:

- migrations for runs/events/refs
- a visible CLI/operator surface
- a daemon/API boundary
- an actual storage layer with tests
- docs that are trying to articulate system identity rather than merely list endpoints

That is real progress.

That said, the codebase is now entering the dangerous middle period:  
the architecture is *promising enough to grow fast*, but not yet *disciplined enough to survive that growth without sludge*.

So my feedback is mostly about **tightening the spine before you add more flesh**.

---

# Executive summary

## What feels strong

- clean workspace/crate split
- local-first stance remains coherent
- run/event substrate exists now in schema and API
- CLI additions (`doctor`, `runs`, `run inspect`) are the correct operator moves
- tests exist and are not purely decorative
- the repo voice is increasingly legible

## What I would change next

1. **Fix crate boundary drift immediately**
2. **Move context generation onto the run substrate for real**
3. **Clarify the relationship between `events`, `run_events`, and `refs`**
4. **Stop shipping raw JSON strings across boundaries**
5. **Reduce doc sprawl and create one canonical “current state” document**
6. **Tighten style around domain transitions and typed results**
7. **Decide whether Vel is “distributed personal system” or “local-first cognition runtime” and make the docs stop competing**

That is the real work now.

---

# What is working well

## 1. The repo feels like a system now, not just a scaffold

You now have enough moving parts that the repo communicates intent:

- `vel-core`
- `vel-storage`
- `vel-cli`
- `veld`
- migrations
- runtime docs

That matters. The repo is no longer only “future architecture fanfic.”

## 2. The new operator surface is directionally correct

Adding:

- `vel doctor`
- `vel runs`
- `vel run inspect`

was exactly the right instinct.

This is one of the fastest ways to make a local-first system feel like a runtime instead of a pile of endpoints.

## 3. The migration story is getting real

`0004_runs_and_events.sql` and `0005_refs_and_artifact_metadata.sql` are the right kind of moves.

Even where the implementation is still partial, the schema is beginning to articulate durable concepts:

- runs
- run events
- refs / provenance
- artifact metadata

That is the correct layer to be investing in.

## 4. The code is readable

This repo does not currently suffer from “Rust as performance art,” which is a blessing.

The code generally reads plainly. That is worth preserving. Cleverness is cheap; legibility is compound interest.

---

# Highest-priority architectural feedback

## 1. Fix the crate boundary leak: `vel-storage` should not depend on `vel-api-types`

This is the single biggest architecture smell in the current pass.

Right now `vel-storage` imports API-layer types:

- `ContextCapture`
- `SearchResult`

from `vel-api-types`.

That inverts your layering.

### Why this is a problem

Your intended architecture is roughly:

```text
CLI / HTTP DTOs
    ↓
core domain / application logic
    ↓
storage
```

But the current dependency means storage is returning transport-shaped objects upward.

That will quietly rot everything.

Storage should not know what your API looks like.  
Storage should know about **domain models** or storage rows.  
API types should be derived at the boundary.

### Recommendation

Refactor so that:

- `vel-core` owns domain structs for captures/search/context results
- `vel-storage` returns those core/domain structs
- `vel-api-types` maps those domain structs into wire DTOs

### Why this matters now

This is still easy to fix.  
Six weeks from now it becomes glue-gunk everywhere.

---

## 2. The run substrate exists, but the interesting flows are still not actually using it

This is the most important “not yet” in the repo.

You added:

- `runs`
- `run_events`
- CLI/API inspection

But `today` / `morning` / `end_of_day` are still directly computed in the route layer from `orientation_snapshot()`.

That means the runtime spine is **present**, but the product-relevant behavior is still bypassing it.

### Why this matters

The whole point of the run model is not to have a nice `/v1/runs` endpoint.  
The point is that meaningful operations become:

- inspectable
- replayable enough for debugging
- provenance-linked
- durable

Right now context generation is still “handler logic with vibes.”

### Recommendation

Refactor context generation into an application/domain service that does something like:

1. construct run input
2. create run
3. transition to running
4. generate context
5. persist output artifact
6. create refs/provenance
7. emit events
8. mark terminal status
9. return result

This is the next real architectural milestone.

---

## 3. `events` vs `run_events` vs `refs` is currently under-explained and under-disciplined

You now have three adjacent mechanisms:

- global `events`
- `run_events`
- `refs`

This is fine in principle, but the semantics are not yet crisp enough.

### Current risk

Without very explicit rules, these tables will start overlapping conceptually:

- is a context generation action logged in `events`?
- only `run_events`?
- both?
- if an artifact is produced, is that represented as a `run_event`, a `ref`, or both?

That ambiguity becomes schema mush fast.

### Recommendation: define the contract explicitly

Use something like:

#### `run_events`
Lifecycle of one run.

Examples:
- `run_created`
- `run_started`
- `artifact_written`
- `run_succeeded`

#### `events`
Global timeline/audit stream for system-level events.

Examples:
- `capture_created`
- `search_executed`
- `job_claimed`
- `daemon_started`

#### `refs`
Stable relationships between durable objects.

Examples:
- run → artifact
- artifact → capture
- artifact → artifact

### Suggested rule

> Events describe **what happened**.  
> Refs describe **what is related to what**.  
> Run events describe **what happened during one run**.

Write that down in one canonical doc and stick to it.

---

## 4. The current run model is still too stringly

Right now `RunDetailData` exposes:

- `input: String`
- `output: Option<String>`
- `error: Option<String>`
- `payload: String` for events

This is workable as a bootstrap, but it is already starting to look like “JSON as a string cargo cult.”

### Why this is a problem

Once these values become more important, you will want to:

- inspect them
- transform them
- render them nicely
- test them structurally
- version them

Stringified JSON delays those benefits and invites re-parsing at every boundary.

### Recommendation

Move toward:

- `serde_json::Value` in the domain and API layer
- only stringify at display edges if needed

You can still store `TEXT` in SQLite. That part is fine.  
But the in-memory/domain representation should stop pretending JSON is just a string.

### Related point

`DoctorData` also uses string status fields like:

- `"ok"`
- `"error: ..."`

That makes the CLI easy, but the type system useless.

Use structured diagnostic results and format them in CLI/API as needed.

---

## 5. `Run` transition methods in `vel-core` are too weak to count as real domain logic yet

Right now `Run::start`, `Run::succeed`, `Run::fail`, `Run::cancel` basically validate current status and return `Ok(())`.

That is half a move.

### Why this matters

If `vel-core` is supposed to own semantics, then the core type should do more than merely say “sure, that transition is allowed.”

It should probably be capable of producing the next state.

### Recommendation

Consider one of these approaches:

#### Option A: immutable transition methods
```rust
fn start(self, now: OffsetDateTime) -> Result<Self, VelCoreError>
fn succeed(self, now: OffsetDateTime, output: Value) -> Result<Self, VelCoreError>
```

#### Option B: transition helper/service in core
If you do not want mutation logic on the struct itself, at least keep transition logic centralized in a core service.

### Why

Otherwise storage becomes the real owner of semantics, and `vel-core` becomes decorative.

---

## 6. There is no unique index on `(run_id, seq)` in `run_events`

This is a small thing with large consequences.

Your own design intent wants monotonic sequence ordering per run.  
But the migration does not currently enforce uniqueness on `(run_id, seq)`.

That means your sequencing guarantee is social, not structural.

### Recommendation

Add:

```sql
UNIQUE(run_id, seq)
```

or the SQLite equivalent via unique index.

This is not optional if you want event ordering to mean anything durable.

---

## 7. Artifact metadata implementation is still only partially true

You added `size_bytes`, which is good, but the implementation still looks like a halfway house:

- `content_hash` exists, but appears caller-supplied
- `size_bytes` is added in migration, but not obviously populated in `create_artifact`
- `metadata_json` exists, but is mostly `{}` right now
- artifacts are represented via `storage_uri`, not a more disciplined path/provenance model

### Why this matters

Artifacts are one of the most promising parts of Vel’s design. Right now the artifact layer still feels more like a row registry than a durable artifact system.

### Recommendation

Clarify one of these two models:

#### Model A: Vel manages artifact files
Then Vel should:
- write files
- compute checksum
- compute size
- store canonical relative path
- own durability semantics

#### Model B: Vel only references external artifact locations
Then:
- `storage_uri` is fine
- checksum/size may be optional or best-effort
- docs must explicitly say Vel is not the storage manager

Right now the code and docs flirt with both.

Pick one.

My bias: **Model A for generated artifacts, Model B optionally for imported artifacts**.

---

# Medium-priority architecture feedback

## 8. `doctor` should be structured internally, not string-assembled

The command is useful, but the underlying data shape is too coarse.

Right now `DoctorData` is basically:

- daemon: string
- db: string
- artifact_dir: string
- schema_version: number
- version: string

This makes presentation easy, but conflates:

- status
- message
- machine-readability

### Recommendation

Use something like:

```rust
struct DiagnosticCheck {
    name: String,
    status: DiagnosticStatus, // Ok | Warn | Fail
    message: String,
}
```

Then `DoctorData` can contain `Vec<DiagnosticCheck>` plus version/schema info.

The CLI can still print plain text.  
But now you have real structure.

---

## 9. The route layer is carrying too much application logic

`routes/context.rs` currently contains both HTTP handling and actual context-generation logic.

That is acceptable for a toy. It is no longer ideal for this repo.

### Recommendation

Move the extraction / scoring logic into a service/module that can be tested independently of Axum.

Routes should mostly:
- parse request
- call service
- map response/error

### Why

Once you introduce run-backed context, retries, provenance, or synthesis, this route file will otherwise become a ritual chamber of mixed concerns.

---

## 10. The global event log is currently underused

You added `emit_event`, which is promising, but it is not yet clearly part of the operator story.

### Recommendation

Either:
- use it intentionally for system-level observability right away  
or
- leave it out until you have a disciplined reason

Half-using it is how “audit log” tables become orphan mythology.

---

# Documentation feedback

## 1. You now have too many top-level docs with overlapping authority

This is the biggest docs issue.

The repo currently contains a lot of valuable writing, but it is starting to feel like an archive of successive selves:

- architecture
- roadmap
- runtime concepts
- runtime design
- product spec
- full implementation spec
- schema review
- repo review
- bootstrap docs
- MVP docs
- `vel.md`

There is good material there. There is also too much overlap.

### Why this is a problem

A new contributor — human or agent — will not know:

- which doc is normative
- which doc is historical
- which doc is aspirational
- which doc describes current reality

That creates cognitive drag fast.

### Recommendation: create a doc hierarchy

I would split docs into four buckets:

## A. Canonical / current
These should describe what is true now.

- `README.md`
- `docs/current-architecture.md`
- `docs/current-data-model.md`
- `docs/current-runtime-concepts.md`

## B. Implementation planning
These are active design docs for work in progress.

- `docs/specs/runtime-spine.md`
- `docs/specs/context-runs.md`
- `docs/specs/artifact-system.md`

## C. Product / vision
These explain direction, not implementation truth.

- `docs/product-spec.md`
- `docs/roadmap.md`

## D. Historical / generated reviews
These are useful, but not normative.

- repo review
- schema review
- architecture roadmap
- implementation spec drafts

Move those into something like:

```text
docs/notes/
docs/reviews/
docs/archive/
```

That will immediately reduce the “which scripture is canon?” problem.

---

## 2. README should say “implemented now” vs “planned next”

The README is clean, but it is still too bootstrap-y for the repo’s current maturity.

### It should include

- what Vel is in one sharp sentence
- what is implemented **today**
- what is planned but not yet implemented
- how the crates relate
- key operator commands
- where canonical docs live

### Suggested addition

A section like:

```markdown
## Status

Implemented:
- capture storage
- lexical search
- artifacts API
- run/event schema and inspection
- doctor diagnostics

Planned next:
- run-backed context generation
- artifact provenance in context flows
- synthesis jobs
```

That one small addition would cut confusion a lot.

---

## 3. The docs are not fully aligned on Vel’s identity

You currently have multiple identities in motion:

- local-first personal executive system
- local-first cognition runtime
- distributed personal system
- memory graph / alignment engine / execution layer

These are not all wrong. But they are not the same thing.

### Recommendation

Pick one primary identity sentence and make the rest subordinate.

My recommendation:

> **Vel is a local-first cognition runtime for capture, recall, and daily orientation.**

Then architecture docs can say:
- distributed later
- mobile clients later
- execution automation later

But the root identity stays stable.

Right now the “distributed personal system” language in `docs/vel-architecture.md` feels more expansive than the current repo truth.

It reads like the project three incarnations from now.

That is not fatal, but it is slippery.

---

## 4. `AGENTS.md` is directionally good, but too generic relative to the actual repo

`AGENTS.md` has good instincts, but parts of it still speak in broad product-future abstractions like:

- memory graph
- alignment engine
- execution layer

Those are fine as long-term concepts, but a coding agent today would benefit more from sharper concrete rules.

### I would add

- explicit crate boundary rules
- “do not introduce API-type dependencies into storage”
- “new runtime behaviors must emit run events if they are run-backed”
- “prefer structured JSON values over raw JSON strings”
- “docs must mark planned vs implemented”

That would make `AGENTS.md` more operational and less manifesto-like.

---

# Style and code quality feedback

## 1. The style is readable; keep it that way

The repo’s biggest stylistic win is that it is mostly plain.

Please do not “improve” it into abstraction fog.

## 2. Module doc comments are helpful

Files like `run.rs`, `provenance.rs`, and CLI command modules benefit from the explanatory comments. Keep that pattern.

## 3. Naming is mostly sane, but a few APIs are too low-level

Examples:

- `payload_json: String`
- `input_json: String`
- `output_json: Option<String>`

These are technically accurate, but semantically weak.

A lot of your future maintainability will come from moving from “thing containing JSON” to “structured domain payload.”

## 4. `#[serde(other)]` on `RunKind::Agent` is a little suspect

This is a subtle one.

Using `#[serde(other)]` on `Agent` means unknown values may deserialize into `Agent`, which is a bit semantically cursed.

That is probably not what you really mean.

If an unknown run kind comes in, that should likely be an error — not silently “agent.”

That little move could produce extremely weird debugging later.

I would remove it unless you have a very deliberate compatibility reason.

## 5. `DoctorData` / `RunDetailData` should become more typed before they calcify

The current DTOs are fine for a bootstrap, but they are at risk of becoming the permanent transport contract by inertia.

That is how stringly-typed APIs ossify.

If you are going to fix it, fix it early.

---

# Concrete architectural changes I would make next

## Priority 1 — boundary cleanup

- remove `vel-storage -> vel-api-types` dependency
- move search/context/capture result domain structs into `vel-core`
- map domain structs to API DTOs only in `veld` / `vel-api-types`

## Priority 2 — run-backed context for real

- create an application service for context generation
- `today` / `morning` create and transition runs
- persist output artifact(s)
- create refs linking run -> artifact and artifact -> sources
- emit `run_started`, `context_generated`, `artifact_written`, terminal event

## Priority 3 — typed payloads

- replace string JSON fields in domain/API with `serde_json::Value` where appropriate
- keep DB as `TEXT`, but deserialize at the boundary

## Priority 4 — tighten schema guarantees

- unique constraint on `(run_id, seq)`
- clarify whether `events` table is truly needed now
- if yes, document semantics and start using it consistently

## Priority 5 — docs consolidation

- create one “current state” architecture doc
- move generated review/spec docs into archive/reviews
- update README to distinguish implemented vs planned
- tighten AGENTS with concrete repo rules

---

# Suggested doc restructure

Here is the doc set I would actually want in the repo root of truth.

## Keep as canonical

- `README.md`
- `AGENTS.md`
- `docs/architecture.md`
- `docs/data-model.md`
- `docs/runtime-concepts.md`
- `docs/roadmap.md`

## Move into `docs/reviews/` or `docs/archive/`

- `vel-repo-review.md`
- `vel-schema-review.md`
- `vel-runtime-design.md`
- `vel-architecture-roadmap.md`
- `vel-full-implementation-spec.md`
- other generated/advisory docs

## Create one new doc

`docs/status.md`

This should say:

- what is implemented
- what is partial
- what is next
- what is intentionally deferred

That file would save everyone a lot of psychic friction.

---

# Suggested README revision themes

If you touch only one doc right now, touch the README.

## It should answer these questions in 30 seconds

1. What is Vel?
2. What crates exist and why?
3. What works today?
4. What is the next major implementation target?
5. Which docs are canonical?

That is the whole job.

---

# Suggested changes to `AGENTS.md`

I would add a section like this:

```markdown
## Repository boundary rules

- `vel-core` owns domain semantics and domain types
- `vel-storage` must not depend on `vel-api-types`
- `vel-api-types` contains transport DTOs only
- route handlers should remain thin and defer to services
- run-backed operations must emit run events and persist terminal state
- structured payloads are preferred over raw JSON strings
- docs must distinguish between implemented and planned behavior
```

That is much more useful to a coding agent than broad talk of alignment engines.

---

# Blunt assessment

The repo is better than many first passes because it already has a shape.  
The risk now is not incompetence. The risk is **conceptual creep plus boundary erosion**.

The two places I would be strictest are:

1. **crate boundaries**
2. **documentation authority**

If you do not tighten those now, the code will still “work,” but the repo will become progressively harder to reason about, especially as you add synthesis, provenance, and agent behavior.

Right now the beast is still tameable.

---

# Short version of my recommendation

If I were driving the next patch series, I would do exactly this:

1. boundary cleanup (`vel-storage` no longer depends on API types)
2. add `(run_id, seq)` uniqueness
3. make context generation actually run-backed
4. switch JSON payload fields to structured values
5. consolidate docs into canonical vs advisory/historical
6. sharpen `AGENTS.md`
7. only then add more features

That would turn this from a promising first pass into a much sturdier substrate.

---

# Final verdict

**Yes — this is a solid first pass.**  
But the repo is now at the stage where architecture discipline matters more than adding another clever surface.

In psychoanalytic terms: the project now has an ego. Guard it from fragmentation.