
# vel — Fifth-Round Architectural Feedback and Next-Step Plan

## Overall assessment

This iteration represents the most significant architectural step so far.

The key milestone has now been reached:

> **Context generation is run-backed.**

That single change collapses several previously parallel ideas into a single runtime pathway:

- runs
- run events
- artifacts
- provenance refs
- CLI inspection
- API inspection

The system now demonstrates the architectural model it has been building toward.

In earlier iterations the runtime substrate existed but did not carry the product's primary behavior.  
In this revision, it does.

This is a meaningful transition from **architecture scaffolding** to **architecture exercising itself**.

However, once that milestone is reached, the next set of issues naturally emerges. They are not structural failures — they are the kinds of refinement tasks that appear once the core system is actually being used.

The focus now shifts from *creating the runtime model* to *making the runtime model operationally robust*.

---

# What is clearly stronger in this round

## 1. Context generation now flows through the runtime

This is the most important improvement in the repository's history.

The workflow now resembles the architecture originally described:

```
request
  -> run created
  -> run_started
  -> snapshot computation
  -> context generation
  -> artifact write
  -> refs created
  -> run_succeeded
```

That alignment between architecture and execution is the central goal of the system.

The runtime is no longer theoretical.

---

## 2. Artifact generation is now meaningful

Context outputs are now represented as **managed artifacts** rather than ephemeral response payloads.

This accomplishes several things:

- makes orientation outputs durable
- allows provenance linking
- enables operator inspection
- prepares the system for future synthesis runs

The artifact layer now has a concrete purpose rather than being a generic attachment registry.

---

## 3. Provenance relationships now demonstrate value

The run → artifact → capture relationship graph is now visible.

This is important because it demonstrates the **lineage model** the system is designed to support.

It will become increasingly useful once:

- synthesis
- agent runs
- external artifacts
- document ingestion

begin to exist.

---

## 4. Inspection tools are now justified

`vel run inspect` and `/v1/runs/:id` now return meaningful information rather than empty runtime scaffolding.

The operator surface now exposes:

- run metadata
- events
- artifacts
- provenance

This is exactly what a local runtime system should allow.

---

## 5. The repository narrative is now coherent

The documentation now accurately describes what the system actually does.

The following structure works well:

```
README
status.md
runtime-concepts.md
data-model.md
specs/context-runs.md
reviews/
```

This provides:

- canonical docs
- planning docs
- historical advisory material

without mixing them.

---

# The next architectural focus

Now that context runs exist, the next improvements should focus on **runtime robustness and repeatability** rather than expanding feature surface.

There are five areas where improvements will have the highest impact.

---

# 1. Make run events richer and more intentional

The current run events cover the minimal lifecycle.

That is good for the initial implementation.

However, the runtime would benefit from slightly richer semantic events for major steps.

Suggested additions:

```
context_generated
artifact_written
refs_created
```

The event stream would then resemble:

```
run_created
run_started
context_generated
artifact_written
refs_created
run_succeeded
```

Why this matters:

- clearer debugging
- easier operator inspection
- more expressive run timelines

These events should remain lightweight; they are observational markers rather than heavy payload containers.

---

# 2. Add explicit run timing metrics

Runs currently track:

```
created_at
started_at
finished_at
```

This is good, but the system should also expose runtime duration.

Suggested addition:

```
duration_ms
```

This can be derived rather than stored, but should be exposed in:

- API run responses
- CLI inspection output

Example:

```
Run: run_81
Kind: context_generation
Status: succeeded
Duration: 47ms
```

This provides early performance observability.

---

# 3. Strengthen artifact writing guarantees

Managed artifact writes should now adopt a more robust pattern.

Recommended approach:

```
write temp file
flush
fsync
rename to final location
persist metadata
```

This ensures:

- crashes during write do not produce partial artifacts
- run success reflects actual artifact durability

The first implementation may already approximate this, but it should become a documented invariant for **managed artifacts**.

---

# 4. Improve run inspection usability

Inspection currently exposes the necessary data, but operator ergonomics could be improved.

CLI suggestions:

Example output:

```
Run: run_81
Kind: context_generation
Status: succeeded
Started: 2026‑03‑14T09:01:02Z
Finished: 2026‑03‑14T09:01:02Z
Duration: 41ms

Artifacts:
  art_120  context_brief  3.4KB

Events:
  0 run_created
  1 run_started
  2 context_generated
  3 artifact_written
  4 run_succeeded
```

Small formatting improvements make the runtime feel far more usable.

---

# 5. Normalize artifact metadata generation

Artifact metadata should now become systematic.

Managed artifacts should always populate:

```
size_bytes
content_hash
metadata_json
```

Recommended metadata fields for context artifacts:

```
{
  "generator": "context-v1",
  "context_kind": "today",
  "snapshot_window": "7d"
}
```

Consistency here will matter later when artifacts become more diverse.

---

# 6. Clarify the system-wide event table

The repository now clearly uses:

```
run_events
refs
artifacts
```

The `events` table still exists but is lightly used.

The architecture documentation should explicitly define its role.

Two options remain viable:

Option A — System audit log

Examples:

```
capture_created
daemon_started
config_updated
schema_migrated
```

Option B — De-emphasize it

Treat `run_events` as the primary runtime event system.

Either approach is valid, but the docs should clearly state which role it plays.

---

# 7. Add one canonical runtime integration test

Now that the runtime workflow exists, a single integration test should guarantee its integrity.

Test scenario:

```
insert captures
call context generation endpoint
verify run created
verify run status transitions
verify artifact created
verify refs created
verify run succeeded
```

Failure scenario:

```
simulate artifact write failure
verify run_failed event
verify no artifact ref created
```

This test becomes the guardrail that protects the runtime architecture from regression.

---

# Additional architectural improvements (lower priority)

## 1. Context service may eventually need internal result types

Currently the service returns API DTO types.

This is acceptable while the service lives inside `veld`.

However, if orchestration logic grows, introducing service-level result types could further decouple application logic from transport formats.

This is not urgent.

---

## 2. Consider eventual job queue integration

The run model naturally supports future asynchronous execution.

Eventually runs may support:

```
Queued
Running
RetryScheduled
Blocked
```

But this should only be introduced once there is a second run-backed workflow (e.g., synthesis or embedding generation).

Premature job infrastructure is rarely beneficial.

---

## 3. Context artifact formats may expand

Currently context outputs are JSON artifacts.

Later you may want:

```
JSON artifact
Markdown artifact
Human readable summary
```

But the JSON artifact should remain the canonical representation.

---

# Documentation improvements

## 1. Update `runtime-concepts.md` with the real workflow

Now that context runs exist, the runtime document should include the actual execution model:

```
capture
  ↓
snapshot
  ↓
context run
  ↓
artifact
  ↓
inspection
```

This is the core system story.

---

## 2. Update `status.md`

Context generation should now be marked as:

```
Implemented (run-backed)
```

This keeps the documentation truthful and useful for contributors.

---

## 3. Expand the data model documentation

Add a small lineage diagram:

```
Capture
  ↓
Run
  ↓
Artifact
  ↓
Refs
```

This clarifies how the system tracks provenance.

---

# Suggested development sequence from here

Phase 1
- enrich run events
- improve CLI inspection formatting

Phase 2
- strengthen artifact write semantics
- normalize artifact metadata generation

Phase 3
- add canonical runtime integration test

Phase 4
- clarify role of global `events` table

Phase 5
- continue runtime-backed workflows (synthesis, ingestion)

---

# Final assessment

This revision marks a turning point.

Vel now demonstrates the architectural loop it was designed to support:

```
input
  → run
  → artifact
  → provenance
  → inspection
```

The architecture is no longer preparatory.

It is operational.

Future work should focus on **stability, observability, and additional run-backed workflows** rather than architectural restructuring.

At this stage the most valuable improvements are the boring ones:

- stronger invariants
- better inspection tools
- clearer event semantics
- comprehensive tests

Those are the qualities that transform a clever system into a dependable one.
