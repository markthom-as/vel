
# vel — Sixth-Round Architectural Review and Next-Step Plan

## Overall assessment

This iteration shows the system settling into its architectural identity.

The previous milestone — **run-backed context generation** — is now clearly integrated with the rest of the runtime model. The architecture no longer feels like a partially assembled system; it now resembles a working runtime with clear data flows and inspection surfaces.

At this stage, the repository demonstrates the following working loop:

```
capture
  → snapshot
  → run
  → artifact
  → provenance
  → inspection
```

This loop is now implemented rather than merely described.

The most important change in this round is not a single feature but a **shift in the development phase**:

The project is moving from **architectural construction** to **runtime maturation**.

That means the priorities change. The next steps are less about adding new structures and more about:

- making invariants explicit
- improving operational observability
- strengthening durability guarantees
- preparing for additional run-backed workflows

The architecture is now stable enough that most future work can happen **within the existing model** rather than reshaping it.

---

# What is clearly stronger in this revision

## 1. The runtime model now feels exercised rather than theoretical

Earlier iterations had the schema and APIs for runs, events, artifacts, and refs, but the system's primary workflow did not use them.

Now the orientation workflow actually exercises the entire model:

```
run_created
run_started
context_generated
artifact_written
refs_created
run_succeeded
```

This demonstrates the runtime lifecycle clearly.

The result is that the runtime is now **self-explanatory** when inspected via CLI or API.

---

## 2. Artifact persistence is no longer incidental

Context artifacts now represent durable outputs rather than transient API responses.

This establishes the artifact layer as a meaningful part of the system architecture.

Artifacts now serve as:

- durable computation outputs
- provenance anchors
- inspection targets
- inputs for future synthesis runs

That role will become increasingly important as additional workflows appear.

---

## 3. The provenance graph is now demonstrably useful

With the current model:

```
capture → run → artifact
```

the repository demonstrates a working lineage system.

This is exactly the type of graph that will support:

- synthesis workflows
- document ingestion
- long-term knowledge traces
- agent execution history

The current implementation proves the model is viable.

---

## 4. Inspection tools now justify the runtime model

The CLI and API inspection surfaces now provide meaningful runtime introspection.

For example:

```
vel run inspect <id>
```

exposes:

- run metadata
- run events
- artifacts
- provenance

This is precisely what a local runtime should allow.

Without inspection surfaces, runtime abstractions tend to become opaque; this repository avoids that trap.

---

## 5. The documentation ecosystem has stabilized

The documentation now shows a clear separation between:

Canonical docs:

```
README
status.md
runtime-concepts.md
data-model.md
```

Planning specs:

```
docs/specs/
```

Advisory / historical material:

```
docs/reviews/
```

This hierarchy reduces conceptual noise and makes the repository easier to navigate.

---

# Architectural direction from this point

At this stage the system's architecture is largely correct.

The next improvements should focus on **operational maturity** rather than structural changes.

The following areas will produce the greatest improvement in system reliability and usability.

---

# 1. Strengthen runtime invariants

Now that runs are the core execution primitive, the repository should explicitly enforce the invariants associated with them.

Important invariants include:

```
run_started must precede run_succeeded
artifact_written must occur before run_succeeded
run_failed must not produce artifact refs
```

These invariants should be documented and partially enforced through code where practical.

Possible approaches:

- run transition validation in `vel-core`
- additional assertions in storage layer
- integration tests verifying event order

The goal is to ensure that the runtime model cannot silently drift into inconsistent states.

---

# 2. Improve run observability

Runs now represent the core execution mechanism.

The system should expose additional runtime information for debugging and performance analysis.

Recommended additions:

### Duration calculation

Expose run duration in:

```
CLI inspection
API run detail
```

Duration can be derived from timestamps but should be presented explicitly.

---

### Event timestamps in CLI output

Displaying timestamps for events will improve debugging clarity.

Example:

```
Events:
  09:01:02 run_created
  09:01:02 run_started
  09:01:02 context_generated
  09:01:02 artifact_written
  09:01:02 run_succeeded
```

---

# 3. Formalize managed artifact write guarantees

Managed artifact persistence should be treated as a critical invariant.

Recommended implementation pattern:

```
write temporary file
flush
fsync
rename to final path
persist metadata
```

This ensures that successful runs always correspond to durable artifacts.

It also prevents partial artifacts from appearing after crashes.

The invariant should be documented as:

```
run_succeeded implies artifact durability
```

---

# 4. Expand artifact metadata consistency

Managed artifacts should always populate the following fields:

```
size_bytes
content_hash
metadata_json
```

Context artifacts should include metadata describing their generation context.

Example:

```
{
  "generator": "context-v1",
  "context_kind": "morning",
  "snapshot_window": "7d"
}
```

This metadata will become useful for debugging and for future workflows.

---

# 5. Clarify the role of the global event log

The repository currently contains:

```
events
run_events
refs
```

The semantics of these tables should be explicitly documented.

Suggested interpretation:

```
run_events → lifecycle of a single run
events → system-wide audit events
refs → durable relationships between entities
```

Examples for the `events` table could include:

```
capture_created
daemon_started
config_updated
schema_migrated
```

Clarifying this distinction prevents confusion as the system grows.

---

# 6. Introduce a canonical runtime integration test

Now that the runtime loop exists, it should be protected by a comprehensive integration test.

Example scenario:

```
insert captures
trigger context generation
verify run created
verify event sequence
verify artifact created
verify refs created
verify run succeeded
```

Failure scenario:

```
simulate artifact write failure
verify run_failed event
verify no artifact refs created
```

This test will protect the runtime architecture from regression.

---

# Additional improvements (lower priority)

## 1. Context service abstraction

The context generation service currently returns API DTOs.

If the service grows more complex in the future, introducing service-level result types could improve separation between application logic and transport layers.

This is not urgent.

---

## 2. Potential future run statuses

The run lifecycle currently supports:

```
Queued
Running
Succeeded
Failed
Cancelled
```

Future workflows may require:

```
RetryScheduled
Blocked
```

However, these should only be introduced once additional run-backed workflows exist.

---

## 3. Artifact format expansion

Context artifacts are currently JSON.

Later improvements may include:

```
JSON canonical artifact
Markdown human-readable artifact
Derived summaries
```

For now JSON should remain the canonical representation.

---

# Documentation improvements

## Update runtime concepts

Now that context runs are implemented, the runtime documentation should describe the actual workflow:

```
capture
  → snapshot
  → context run
  → artifact
  → inspection
```

This diagram should appear in `runtime-concepts.md`.

---

## Update status documentation

`status.md` should mark context generation as:

```
Implemented (run-backed)
```

Keeping this file accurate ensures that contributors understand the system's current capabilities.

---

# Recommended development sequence

Phase 1
- improve run inspection output
- add duration metrics

Phase 2
- strengthen artifact write guarantees
- normalize artifact metadata generation

Phase 3
- add runtime integration tests

Phase 4
- clarify system-wide event semantics

Phase 5
- implement additional run-backed workflows (synthesis, ingestion)

---

# Final assessment

The repository has now crossed a major architectural threshold.

The runtime model is:

- implemented
- exercised
- inspectable
- documented

This means the project can shift its focus from architecture design to **runtime reliability and additional workflows**.

Future improvements should prioritize:

- stronger invariants
- improved observability
- durable artifact guarantees
- comprehensive tests

These are the qualities that turn a clever architecture into a dependable system.

Vel now has the foundation required for that transition.
