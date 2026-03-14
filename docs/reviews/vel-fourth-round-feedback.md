
# vel — Fourth-Round Feedback and Next-Step Plan

## Overall assessment

This iteration continues the positive trajectory.

The repository now shows clear architectural discipline compared to the earliest passes:

Key improvements that are visible in this round:

- consistent typed JSON payloads (`serde_json::Value`)
- better domain ownership of `Run` transitions
- clearer crate boundaries
- docs hierarchy largely stabilized
- improved CLI inspection surface
- artifact storage semantics named
- service modules exist
- migrations and schema discipline intact

The repo now feels much closer to a **runtime substrate** rather than a prototype with runtime ideas layered on top.

However, one architectural gap still dominates the design:

> The runtime substrate exists, but the primary user-facing workflows still bypass it.

This is the same core observation as the previous review — but it now matters **more**, because the rest of the architecture has been cleaned up enough that this remaining gap is the largest conceptual inconsistency.

---

# What is clearly stronger in this round

## 1. Domain types and boundaries are now stable

`vel-core` now clearly owns:

- `Run`
- `RunStatus`
- `RunKind`
- `OrientationSnapshot`
- provenance relationships

Storage is no longer returning API DTOs.  
That was the single most important architectural correction, and it appears to have stuck.

This boundary discipline should be protected aggressively going forward.

---

## 2. Typed payloads significantly improved the runtime model

The migration from raw JSON strings to `serde_json::Value` improves several things simultaneously:

- structural testing
- payload evolution
- CLI formatting flexibility
- future compatibility with richer run inputs/outputs

The runtime payload layer now feels like a real data model rather than a transport hack.

---

## 3. The repository voice is coherent now

The docs are no longer competing with each other.

The combination of:

- `README.md`
- `docs/status.md`
- `docs/runtime-concepts.md`
- `docs/data-model.md`

creates a credible hierarchy of truth.

Review / advisory docs have been moved into `docs/reviews/`, which prevents conceptual drift.

That was an important cleanup.

---

## 4. Artifact semantics are beginning to stabilize

The introduction of:

```
storage_kind: managed | external
```

is an important architectural signal.

You are correctly distinguishing:

- artifacts Vel **controls**
- artifacts Vel merely **references**

This distinction will become extremely important once context and synthesis outputs are persisted.

---

## 5. The service layer is emerging

`services/context_generation.rs` exists, and the route modules are thinner.

This is the correct direction even though the service is not yet fully owning the runtime lifecycle.

---

# The main architectural gap remains

## Context generation is still not runtime-backed

The service module exists, but it still functions as a pure transformation layer:

```
OrientationSnapshot -> TodayData
OrientationSnapshot -> MorningData
OrientationSnapshot -> EndOfDayData
```

There is still:

- no run creation
- no run transitions
- no artifact creation
- no provenance refs
- no run events

The runtime model is therefore still orthogonal to the system’s most important behavior.

This was acceptable when the runtime substrate did not exist.

Now that the runtime substrate **does exist**, the inconsistency is more visible.

---

# The next architectural milestone

The next meaningful change is:

> **Make context generation a run-backed workflow.**

This is the step where:

- runs
- artifacts
- provenance
- run events
- inspection tools

finally intersect.

Until that happens, the runtime substrate remains underutilized.

---

# Recommended implementation plan

## Phase 1 — Promote `context_generation` into a real application service

The current module should evolve from a pure helper into an orchestration layer.

Suggested structure:

```
ContextGenerationService
  generate(ContextKind)
```

Where:

```
enum ContextKind {
    Today,
    Morning,
    EndOfDay
}
```

The orchestration service should:

1. create run
2. emit `run_created`
3. transition to running
4. compute orientation snapshot
5. call existing helper logic
6. write artifact
7. create refs
8. emit events
9. mark run terminal

The existing helper functions can remain unchanged and simply become internal building blocks.

---

## Phase 2 — Persist context outputs as managed artifacts

When a context run succeeds:

Create a **managed artifact**.

Recommended artifact type:

```
artifact_type: context_brief
mime_type: application/json
```

Artifact payload example:

```
{
  "context_kind": "morning",
  "date": "2026-03-14",
  "top_active_threads": [...],
  "pending_commitments": [...],
  "suggested_focus": "..."
}
```

Persist:

- checksum
- size_bytes
- metadata_json

Example metadata:

```
{
  "generator": "context-v1",
  "context_kind": "morning"
}
```

---

## Phase 3 — Create provenance relationships

For a successful context run:

Create refs:

```
run -> artifact
artifact -> capture
```

This gives you a full lineage graph:

```
captures
   ↓
context run
   ↓
artifact
```

Inspection tools can then expose the full lineage.

---

## Phase 4 — Extend run inspection output

Once context runs exist, extend:

```
GET /v1/runs/:id
vel run inspect <id>
```

to include:

- produced artifacts
- artifact summaries
- optionally provenance refs

Example CLI output:

```
Run: run_42
Kind: context_generation
Status: succeeded

Events:
  run_created
  run_started
  context_generated
  artifact_written
  run_succeeded

Artifacts:
  art_81 (context_brief, 4.8KB)
```

---

## Phase 5 — Normalize storage API payload types

Storage APIs should consistently accept `serde_json::Value`.

Currently there are still traces of `&str` JSON payloads in storage interfaces.

Normalize everything so serialization happens only at the storage boundary.

This keeps the architectural layering consistent.

---

# Artifact system improvements (next stage)

Once context runs land, tighten artifact semantics.

Recommended additions:

### Atomic artifact write

For managed artifacts:

```
write temp file
fsync
rename -> final path
persist metadata
```

This prevents phantom success states if a run crashes mid-write.

---

### Canonical artifact paths

Prefer storing **relative paths** in the database:

```
artifacts/context/2026-03-14/run_42.json
```

Resolve against the configured artifact root.

This improves portability and backups.

---

# Run lifecycle improvements

Your current run lifecycle is good but still minimal.

Future additions (not urgent):

Possible additional statuses:

```
Blocked
RetryScheduled
``

Possible additional events:

```
artifact_written
context_generated
retry_scheduled
```

Do not add these until the first run-backed workflow exists.

---

# Documentation changes recommended

## 1. Update runtime concepts doc once context runs land

Add a section describing the canonical run flow:

```
request
  -> run created
  -> run started
  -> computation
  -> artifact written
  -> refs created
  -> run succeeded
```

This becomes the reference pattern for future run-backed features.

---

## 2. Update `status.md` immediately after the milestone

Once implemented:

Change:

```
Context generation: Partial
```

to

```
Context generation: Implemented (run-backed)
```

Keeping `status.md` honest is extremely valuable.

---

## 3. Add a small runtime diagram

A simple diagram in `runtime-concepts.md` would clarify the architecture:

```
Capture
   ↓
Snapshot
   ↓
Context Run
   ↓
Artifact
   ↓
Inspection
```

Even a small diagram reduces documentation ambiguity.

---

# Testing priorities for the next patch

Once context runs exist, add one comprehensive integration test.

Test scenario:

```
create captures
trigger context generation
verify run created
verify event sequence
verify artifact written
verify refs exist
verify run succeeded
```

Failure test:

```
simulate artifact write failure
verify run status = failed
verify error_json set
verify no artifact ref created
```

This test proves the runtime substrate is functioning as intended.

---

# Minor observations

## 1. Context generation service still returns API types

This is acceptable for now because the service lives in `veld`.

But if the orchestration layer grows, you may eventually want service-level result types that are mapped to API DTOs at the route boundary.

Not urgent.

---

## 2. `events` table still lightly used

You should either:

- commit to a system-wide event log
- or document that `run_events` is currently the primary event system

The architecture docs should reflect whichever decision you choose.

---

# Recommended development sequence

If I were steering the next round of work:

Phase 1
- implement run-backed context generation

Phase 2
- persist context artifacts
- create provenance refs

Phase 3
- extend run inspection with artifact summaries

Phase 4
- normalize storage JSON payload APIs

Phase 5
- tighten artifact write semantics

After that milestone the architecture becomes **self-demonstrating**.

---

# Final assessment

This revision shows clear architectural maturation.

The repo now has:

- disciplined crate boundaries
- a credible runtime model
- structured payloads
- an artifact system
- provenance concepts
- inspection tools
- a coherent documentation hierarchy

The final step for this architectural phase is simply:

> **make the most important workflow actually flow through the runtime.**

Once context generation is run-backed, the architecture will stop being preparatory and start being operational.

That is the threshold the project is now approaching.
