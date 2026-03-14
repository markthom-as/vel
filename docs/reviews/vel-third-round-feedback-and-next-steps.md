# vel — Third-Round Feedback and Next-Step Plan

## Overall verdict

Yes — this is **better again**.

This pass did not just shuffle the furniture. It tightened several of the exact places that were still soft:

- domain/API payloads are now typed as `serde_json::Value`
- `Run` transitions actually return updated state
- the unique `(run_id, seq)` guarantee is now enforced in schema
- docs are much more truthful about current reality
- `doctor` is now structured internally
- `storage_kind` exists and is documented
- route modules are thinner than before

That is real progress.

The repo now feels noticeably less speculative and more self-consistent. The architecture is starting to stop narrating itself and start behaving like an actual runtime.

That said, the next round is now very clear:

> **make the runtime substrate carry the orientation flows for real**

You have done enough cleanup that the remaining architectural gap is no longer diffuse. It is concentrated.

---

# What is clearly stronger now

## 1. Typed JSON was the right correction

Moving `Run`, `RunEvent`, and API run payloads to `serde_json::Value` was absolutely the right move.

That buys you:

- structural assertions in tests
- better CLI/API rendering options
- less re-parsing nonsense
- easier evolution of run payload shape

You have stopped paying JSON taxes without getting JSON benefits. Good.

## 2. `Run` now behaves more like a domain object

The immutable transition style is better than the earlier “semantic hall monitor” version.

`start`, `succeed`, `fail`, and `cancel` returning updated `Self` is a meaningful improvement because it makes `vel-core` feel like the owner of runtime semantics rather than a passive validator.

That was the right call.

## 3. The docs are more honest

The combination of:

- `README.md`
- `docs/status.md`
- `docs/specs/context-runs.md`

is much better now.

The repo has a more credible split between:

- implemented
- partial
- planned next

That reduces one of the big earlier risks: contributors inferring current reality from a cloud of aspirational prose.

## 4. The service layer exists in a more concrete way

`services/context_generation.rs` and `services/doctor.rs` are a step forward, even if one of them is still more helper-like than truly orchestration-owning.

The route modules are thinner, which is good. Thin routes age better.

## 5. Artifact storage semantics are at least named now

Adding `storage_kind` was worthwhile.

You do not fully exploit it yet, but naming the distinction between managed and external artifacts is an important step toward making artifact behavior legible.

---

# The main thing still missing

## Context generation is still not actually runtime-backed

This is still the center of gravity.

The docs now correctly admit it, which is good. But the code also makes it clear that:

- routes fetch `orientation_snapshot()`
- `context_generation` is still a pure transformation helper
- no run is created
- no artifact is written
- no refs are linked
- no run events are emitted for context flows

In other words:

> the runtime spine is ready enough that its absence from the product-critical path is now the dominant gap

This is actually progress. Earlier there were many blurry issues. Now there is one obvious one.

---

# Highest-priority next steps

These are the changes I would make next, in order.

---

# 1. Turn `context_generation` into a true application service, not a formatting helper

## What I see now

`crates/veld/src/services/context_generation.rs` is still basically a pure transformation module over `OrientationSnapshot`.

That is fine as internal logic, but it is not yet the actual “service” the repo now needs.

Right now it:

- accepts snapshot data
- returns API-shaped structs
- does not own run lifecycle
- does not persist artifacts
- does not write refs
- does not emit events

So the file name says “service,” but architecturally it is still a helper.

## What I would change

Promote it into a real orchestration service that owns the context use case.

Suggested direction:

```text
ContextGenerationService
  - generate_today(state/config/storage context)
  - generate_morning(...)
  - generate_end_of_day(...)
```

or a more generic:

```text
generate(ContextKind, AppState/Storage/Config, optional params)
```

### That service should own:

1. run creation
2. run transition to running
3. snapshot loading
4. context computation
5. artifact write
6. ref creation
7. event append
8. run success/failure transition

### The pure helper logic can remain

Keep the extraction/scoring code, but make it subordinate to the use-case layer.

That is the right split:

- pure derivation functions
- plus one orchestration service that actually drives the runtime

---

# 2. Make context runs first-class artifacts, not just returned JSON

## Why this is the next decisive move

The repo identity is now strong enough that you should stop treating orientation output as merely response payload and start treating it as a durable object.

For a successful `today` / `morning` / `end_of_day` run, I would want:

- one run row
- ordered run events
- one managed artifact
- run → artifact ref
- optional artifact → source capture refs
- output_json containing a compact summary, not the whole universe

## Artifact format recommendation

I would pick one of these and be explicit:

### Option A — JSON artifact
Good if you want machine-friendly reproducibility first.

### Option B — Markdown artifact
Good if you want human-readable orientation output first.

### Option C — both, later
Not yet.

My bias for the next patch: **JSON artifact first**, because it aligns more directly with current API output and keeps the first implementation simpler. You can add markdown rendering later.

## Why this matters

Once context runs produce managed artifacts, several pieces of the architecture finally start touching:

- runs
- refs
- artifact metadata
- `storage_kind`
- operator inspection
- provenance

That is the next real systems-level win.

---

# 3. Extend run inspection so it can show linked artifacts / refs

## Current state

`GET /v1/runs/:id` and `vel run inspect <id>` expose:

- run summary
- input/output/error
- run events

That is already useful.

But once context runs are implemented, inspection without artifacts/refs will immediately feel incomplete.

## Recommendation

Extend run detail to optionally include:

- linked artifacts
- maybe linked refs or at least ref summaries

Suggested additions to `RunDetailData`:

- `artifacts: Vec<ArtifactSummaryData>` or similar
- optionally `refs: Vec<RefData>` if that is not too noisy

## Why

Because the point of run-backed context is not only “a run happened.”  
It is “a run happened, produced a durable output, and I can inspect the relationship.”

Without that, the inspection surface will lag behind the runtime model.

---

# 4. Tighten storage APIs around typed run payloads end-to-end

## What improved

The domain and API layer now use `serde_json::Value`. Good.

## What still looks slightly transitional

In storage, `create_run` still takes:

- `input_json: &str`

while other APIs like `append_run_event` already take `&JsonValue`.

That smells like an intermediate state.

## Recommendation

Unify the storage API so run persistence functions also accept structured JSON values and serialize at the storage boundary.

Examples:

```rust
create_run(&self, id: &RunId, kind: RunKind, input_json: &JsonValue)
update_run_status(..., output_json: Option<&JsonValue>, error_json: Option<&JsonValue>)
```

The same principle should hold everywhere:

> storage owns serialization; core/app layers own structure

## Why this matters

It keeps the architectural story consistent and prevents the repo from accumulating a weird half-typed payload surface.

---

# 5. Fix the likely bug / incompleteness in `create_run`

## What I noticed

In `crates/vel-storage/src/db.rs`, `create_run` inserts a `run_created` event and binds `payload_str`, but in the current file there does not appear to be a local `payload_str` definition in that method.

That may be:

- an actual bug
- an incomplete refactor
- or a hidden compile issue that just slipped in

I cannot compile from this environment, so I will not bluff and claim whether it currently builds. But that line is suspicious enough that I would check it immediately.

## Recommendation

Audit `create_run` now and make the initial event payload explicit and typed.

Something like:

```json
{
  "kind": "context_generation"
}
```

or maybe:

```json
{
  "kind": "context_generation",
  "input_summary": { ... }
}
```

Keep it small. Just make it deliberate.

---

# 6. Decide what `events` is actually for, or demote it

## Current state

The documentation now distinguishes:

- `events`
- `run_events`
- `refs`

which is good.

But the implementation still suggests `events` is only lightly used, while `run_events` is the real active timeline for runtime work.

## Recommendation

You now have two good options:

### Option A — commit to `events`
Use it for system-level occurrences such as:
- capture creation
- daemon startup
- schema migration completion
- future worker/job lifecycle events

and document those semantics clearly.

### Option B — demote it for now
Leave it minimal and stop talking about it as if it is already central.

## Why this matters

Because architectural ambiguity around event systems grows mold very quickly.  
If `run_events` is the real thing right now, let it be the real thing.

My bias: **keep `events`, but narrow its documented role sharply**.

---

# 7. Make `doctor` slightly more operational before adding more checks

## What is good now

Structured checks are a real improvement.

## What I would still change

Right now doctor checks:

- daemon
- db
- artifact_dir
- schema_version
- version

That is fine, but still slightly bare.

Before you add lots of new checks, I would make two small quality improvements:

### A. Distinguish “synthetic OK” from “actually verified”
Right now `daemon` is just reported as ok. If it is not actually probed, that should be reflected honestly.

Maybe:
- `Warn: not explicitly probed`
or
- separate “service is running in-process” vs “remote daemon reachable”

### B. Add an explicit `schema` check object
Right now schema version is a field, but not really a first-class diagnostic. Making it one check would fit the model better.

## Why

Because diagnostics become much more useful when they are internally consistent, not partly structured and partly side-channel.

---

# 8. Tighten artifact metadata population once context runs land

## Current state

The docs are honest that artifact metadata is still partial:

- `size_bytes`
- `content_hash`
- provenance linking

are not yet fully populated/used.

That is fine for now.

## Recommendation

Do **not** try to perfect artifact metadata before context runs exist.  
Do it immediately after.

Why? Because the first context-run artifact path is where you can define the behavior cleanly:

- managed artifact path
- canonical relative location
- checksum computed at write time
- size computed at write time
- metadata_json with generator version / context kind

That gives you one well-defined gold path instead of trying to backfill elegance abstractly.

---

# 9. Consider moving service-return types one layer inward

## This is lower priority, but worth keeping in mind

`context_generation.rs` currently returns API-shaped types:

- `TodayData`
- `MorningData`
- `EndOfDayData`

That is acceptable for now because the service still sits inside `veld`, not `vel-core`.

But if you turn it into a real orchestration layer, you may eventually want a slightly more internal app/service result type, then map that to API DTOs in the route.

## Why this matters

Because the more substantial the service becomes, the more useful it is to keep:

- use-case outputs
- transport DTOs

as distinct concepts.

## Not urgent

I would not block the next patch on this.  
I would just avoid deepening the current coupling too much.

---

# Planning / sequencing recommendation

If I were sequencing the next round of work, I would do it in this exact order.

---

# Phase 1 — Make context generation actually runtime-backed

1. introduce a true orchestration service for context generation
2. create runs for `today`, `morning`, `end_of_day`
3. transition runs through queued → running → terminal
4. append run events
5. persist output artifact
6. create run → artifact ref
7. optionally create artifact → capture refs

### Exit criteria
- `GET /v1/context/today` produces a visible run
- run has meaningful event sequence
- one managed artifact is created
- failure path sets `error_json`

---

# Phase 2 — Extend inspection surfaces

1. update run detail API to include linked artifact summaries
2. update `vel run inspect <id>` to render artifacts
3. optionally show ref summaries

### Exit criteria
- operator can inspect not just a run, but its output object

---

# Phase 3 — Normalize storage payload APIs

1. make `create_run` accept `JsonValue`
2. audit any remaining stringified payload APIs
3. keep serialization pinned to storage boundary

### Exit criteria
- no architectural backsliding on typed payloads

---

# Phase 4 — Make managed artifact behavior the canonical gold path

1. implement write-to-temp + rename for managed artifact output
2. compute checksum and size on write
3. populate metadata_json with context kind / generator version
4. document managed vs external artifact behavior in canonical docs

### Exit criteria
- first-class generated artifacts are genuinely durable

---

# Phase 5 — Clarify the system-wide event story

1. decide active role of `events`
2. document it explicitly
3. either use it more intentionally or demote it in docs

### Exit criteria
- no conceptual overlap fuzz between `events`, `run_events`, and `refs`

---

# Documentation changes I would make next

## 1. Update `docs/status.md` the moment context runs land

This file is now actually useful. Keep it brutally current.

Specifically update:
- Context generation from **Partial** → **Implemented**
- Artifact metadata from partial → more precise statement
- global events status if clarified

## 2. Expand `docs/specs/context-runs.md` into a post-implementation record

Once implemented, either:
- archive it and create a new canonical runtime note
or
- add a short “implemented shape” section showing what shipped

That prevents the spec from lingering as a parallel reality.

## 3. Add one compact diagram to `docs/vel-runtime-concepts.md`

Something as simple as:

```text
context request
  -> run created
  -> run started
  -> context computed
  -> artifact written
  -> refs linked
  -> run succeeded
```

Sometimes one dumb little diagram saves five paragraphs of interpretive labor.

## 4. Consider renaming canonical docs later, not now

You still have the `vel-*` naming pattern in `docs/`. I still think simpler names would eventually be nicer, but I would not spend energy on that before the context-run milestone is complete.

---

# Test plan I would prioritize

Once context runs are implemented, add one intentionally boring integration test that proves the runtime spine is real.

## Required happy-path test

For `today`:

1. seed captures
2. call context generation
3. assert one run exists
4. assert run status == `succeeded`
5. assert event sequence is exactly:
   - `run_created`
   - `run_started`
   - `context_generated`
   - `artifact_written`
   - `run_succeeded`
6. assert one managed artifact exists
7. assert run → artifact ref exists

## Required failure-path test

Simulate artifact write failure or DB failure and assert:

- run exists
- terminal status == `failed`
- `error_json` populated
- no successful artifact ref created

This is the test that graduates the runtime from concept to substrate.

---

# My blunt recommendation

At this point I would tell the coding agent:

1. stop polishing the scaffolding
2. make context generation actually use the runtime
3. make the produced context durable
4. make inspection show the durable result
5. only then worry about broader event theory or additional features

That is the shortest path to making the architecture *earn* its abstraction.

---

# Final assessment

This revision is more coherent than the last one.

The repo now feels like it has:

- a cleaner boundary story
- a truer documentation story
- a more serious runtime story

The remaining work is no longer “figure out what Vel is.”  
The remaining work is:

> **make the most important user-facing behavior actually flow through the runtime you built**

That is exactly where you want to be.

The project has moved out of the mirror stage. Now it needs to walk.