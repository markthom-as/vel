# vel — Full Implementation Specification
Version: 0.1  
Status: Proposed  
Audience: Coding agent / implementation team  
Scope: Runtime spine, run model, event log, artifact metadata, provenance, context-generation runs, CLI/API/operator surface

---

# 1. Executive Summary

This spec defines the next major implementation tranche for **Vel**.

The goal is to harden Vel’s architecture around a **runtime spine** that makes meaningful operations:

- explicit
- durable
- inspectable
- replayable enough for debugging
- extensible toward synthesis and agent execution

This work is not about adding surface novelty. It is about giving the system a backbone.

The core deliverables are:

1. first-class **runs**
2. append-only **run events**
3. stronger **artifact metadata**
4. explicit **provenance**
5. `today` / `morning` as **run-backed operations**
6. CLI operator commands: `doctor`, `runs`, `run inspect`
7. API support for run inspection
8. test coverage for the runtime substrate
9. documentation updates

This spec is intentionally detailed so a coding agent can implement it with minimal interpretation drift.

---

# 2. Problem Statement

Vel already has promising bones:

- local-first storage
- SQLite
- CLI / daemon split
- captures, search, context
- artifact references beginning to emerge

However, several important operations still risk being “implicit behavior” instead of durable runtime objects.

Without a runtime spine, the system will drift toward one or more of the following failure modes:

- context generation becomes magical and uninspectable
- generated outputs lose provenance
- async/background work later becomes ad hoc
- operator tooling remains weak
- testability declines as behavior hides in endpoint handlers
- future synthesis/agent features require schema violence

The remedy is to introduce **explicit runtime state** now, while the system is still small enough to keep elegant.

---

# 3. Goals

## 3.1 Primary Goals

Implement a runtime substrate that:

- models important operations as **runs**
- records lifecycle transitions as **events**
- strengthens artifact durability and metadata
- links generated outputs to their inputs via provenance
- makes context generation observable and debuggable
- supports operator inspection from CLI and API

## 3.2 Secondary Goals

- create extension points for future job queue work
- support future cancellation / resume semantics without redesign
- improve diagnostics for local-first operation
- make the codebase more legible by clarifying crate responsibilities

## 3.3 Success Criteria

The implementation is successful when:

1. `today` and `morning` generate explicit runs
2. runs can be listed and inspected
3. run lifecycle transitions emit persistent events
4. generated outputs can be traced back to source inputs and originating runs
5. artifacts persist stronger metadata
6. `vel doctor` detects common local failures
7. tests protect the new runtime spine
8. the code remains aligned with existing crate boundaries

---

# 4. Non-Goals

This spec does **not** require implementation of:

- distributed execution
- generic DAG/workflow builder
- graphical orchestration UI
- plugin marketplace
- remote multi-user sync
- rich agent chat UX
- full cancellation / resume
- semantic/vector retrieval redesign
- model-provider abstraction overhaul

Do not smuggle those in “while we’re here.” That is how projects become soup.

---

# 5. Architectural Principles

## 5.1 Local-first before cleverness

Everything in this spec should work fully on one machine with inspectable state.

## 5.2 Explicit state over implicit flow

If an operation matters, it should have a durable record.

## 5.3 Artifacts before vibes

Generated outputs should be persisted as artifacts or structured records, not left as ephemeral handler return values.

## 5.4 Provenance over mysticism

A generated context item should be explainable.

## 5.5 Core domain owns semantics

`vel-core` owns runtime meaning. Other crates consume it.

## 5.6 Keep the system boring in the right places

The runtime substrate should feel more like a small database-backed operating layer than a performance of AI cleverness.

---

# 6. Scope Overview

This implementation introduces the following major slices:

1. **Schema**
   - `runs`
   - `run_events`
   - artifact metadata improvements
   - provenance support

2. **Core domain**
   - `Run`
   - `RunKind`
   - `RunStatus`
   - `RunEvent`
   - provenance relations
   - domain services for run-backed context generation

3. **Storage**
   - repositories for runs/events/provenance/artifact enrichment
   - migration execution
   - transactional guarantees where practical

4. **API**
   - list runs
   - inspect run
   - optional list run events

5. **CLI**
   - `vel doctor`
   - `vel runs`
   - `vel run inspect <id>`

6. **Context operations**
   - `today`
   - `morning`
   become explicit runs with durable outputs

7. **Testing**
   - domain tests
   - storage integration tests
   - CLI tests
   - end-to-end run-backed context tests

8. **Documentation**
   - runtime concepts
   - operator workflows
   - troubleshooting
   - development notes

---

# 7. Crate Responsibility Contract

This section is normative.

## 7.1 `vel-core`

Owns:

- runtime domain types
- statuses and transitions
- provenance semantics
- orchestration rules
- context-generation domain logic
- validation rules
- repository traits/interfaces

Must **not** own:

- SQL
- CLI formatting
- HTTP serialization details specific to transport
- file layout assumptions beyond domain-level references

## 7.2 `vel-storage`

Owns:

- SQL schema mapping
- migration files / migration execution
- repository implementations
- transactional persistence
- filesystem artifact persistence details
- checksum/size persistence wiring

Must **not** own:

- business semantics for run transitions
- CLI behavior
- endpoint behavior decisions

## 7.3 `vel-api-types`

Owns:

- DTOs for HTTP responses/requests
- stable transport shapes
- mapping helpers if needed

Must **not** own:

- core business rules
- storage knowledge

## 7.4 `vel-cli`

Owns:

- command parsing
- terminal formatting
- operator experience
- exit code behavior
- doctor report display

Must **not** own:

- runtime business logic
- SQL
- provenance rules

## 7.5 `veld`

Owns:

- daemon process concerns
- routing to services
- background service setup
- future worker lifecycle

Must remain thin where possible.

---

# 8. Data Model Specification

## 8.1 Run Model

Introduce a first-class `Run`.

### Fields

- `id`
- `kind`
- `status`
- `input_json`
- `output_json` nullable
- `error_json` nullable
- `created_at`
- `started_at` nullable
- `finished_at` nullable

### Recommended SQL shape

```sql
CREATE TABLE runs (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  status TEXT NOT NULL,
  input_json TEXT NOT NULL,
  output_json TEXT,
  error_json TEXT,
  created_at TEXT NOT NULL,
  started_at TEXT,
  finished_at TEXT
);
```

### Notes

- `TEXT` timestamps should be ISO-8601 UTC if that matches current conventions
- `id` may be UUID/ULID/string according to current project conventions; pick one and use consistently
- `kind` and `status` should be validated at the domain layer, not trusted from the DB

## 8.2 RunKind

Minimum enum values:

- `Search`
- `ContextGeneration`
- `ArtifactExtraction`
- `Synthesis`

Reserve compatibility for:
- `Agent`

### String mapping recommendation

Use stable storage values:

- `search`
- `context_generation`
- `artifact_extraction`
- `synthesis`
- `agent`

Avoid title-cased storage values.

## 8.3 RunStatus

Minimum enum values:

- `Queued`
- `Running`
- `Succeeded`
- `Failed`
- `Cancelled`

Recommended future compatibility:
- `Blocked`
- `RetryScheduled`

Storage values:

- `queued`
- `running`
- `succeeded`
- `failed`
- `cancelled`

## 8.4 RunEvent Model

Introduce append-only events per run.

### Fields

- `id`
- `run_id`
- `seq`
- `event_type`
- `payload_json`
- `created_at`

### Recommended SQL shape

```sql
CREATE TABLE run_events (
  id TEXT PRIMARY KEY,
  run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(run_id, seq)
);
```

### Notes

- `seq` must be monotonic per run
- payload can be sparse at first; do not overfit
- this is append-only in practice

## 8.5 Run Event Types

Minimum required values:

- `run_created`
- `run_started`
- `run_succeeded`
- `run_failed`
- `run_cancelled`

Recommended early extension values:

- `artifact_written`
- `search_executed`
- `context_generated`

### Payload examples

#### `run_created`
```json
{
  "kind": "context_generation"
}
```

#### `run_failed`
```json
{
  "error_kind": "validation",
  "message": "template missing"
}
```

#### `artifact_written`
```json
{
  "artifact_id": "art_...",
  "artifact_type": "context_brief"
}
```

Payloads should be useful, but not encyclopedic.

## 8.6 Artifact Metadata

If an artifact table already exists, extend it rather than duplicating it.

Required fields:

- `id`
- `artifact_type`
- `mime_type`
- `path`
- `checksum`
- `size_bytes`
- `created_at`
- `metadata_json`

If the current table lacks these fields, add them via migration.

### Recommended SQL additions

```sql
ALTER TABLE artifacts ADD COLUMN artifact_type TEXT;
ALTER TABLE artifacts ADD COLUMN mime_type TEXT;
ALTER TABLE artifacts ADD COLUMN checksum TEXT;
ALTER TABLE artifacts ADD COLUMN size_bytes INTEGER;
ALTER TABLE artifacts ADD COLUMN metadata_json TEXT;
```

Adjust according to existing schema reality.

### Notes

- backfill nullability carefully
- if existing rows are present, allow temporary nulls then backfill where appropriate
- checksum should ideally be SHA-256 hex

## 8.7 Provenance / References

Add a generic relation/provenance mechanism.

### Preferred generic table

```sql
CREATE TABLE refs (
  id TEXT PRIMARY KEY,
  from_type TEXT NOT NULL,
  from_id TEXT NOT NULL,
  to_type TEXT NOT NULL,
  to_id TEXT NOT NULL,
  relation_type TEXT NOT NULL,
  created_at TEXT NOT NULL
);
```

### If a refs table already exists

Either:
- extend it to support provenance semantics
or
- introduce a new dedicated provenance table with a clear rationale

### Minimum relation types

- `generated_from`
- `derived_from`
- `attached_to`

### Typical examples

- run → artifact (`attached_to` or `produced`)
- artifact → capture (`derived_from`)
- generated context artifact → source capture (`generated_from`)

### Important note

If the project already has `artifact_refs`, avoid redundant overlapping systems. Consolidate if practical.

---

# 9. Migration Plan

## 9.1 Migration ordering

Add a new migration after the current latest migration.

Suggested migration steps:

1. create `runs`
2. create `run_events`
3. add/extend artifact metadata columns
4. add provenance/refs support

## 9.2 Backfill requirements

If artifacts already exist:
- leave new metadata columns nullable initially if necessary
- backfill `artifact_type`, `mime_type`, `size_bytes`, `checksum` where practical
- do not block the whole implementation on perfect backfill unless required by the code path

## 9.3 Migration acceptance criteria

- migration runs on a fresh DB
- migration runs cleanly on an existing DB from prior repo state
- no data loss
- indexes created as needed

## 9.4 Recommended indexes

```sql
CREATE INDEX idx_runs_status ON runs(status);
CREATE INDEX idx_runs_kind ON runs(kind);
CREATE INDEX idx_runs_created_at ON runs(created_at);

CREATE INDEX idx_run_events_run_id ON run_events(run_id);
CREATE INDEX idx_run_events_run_id_seq ON run_events(run_id, seq);

CREATE INDEX idx_refs_from ON refs(from_type, from_id);
CREATE INDEX idx_refs_to ON refs(to_type, to_id);
CREATE INDEX idx_refs_relation_type ON refs(relation_type);

CREATE INDEX idx_artifacts_artifact_type ON artifacts(artifact_type);
CREATE INDEX idx_artifacts_checksum ON artifacts(checksum);
```

Adjust naming to project conventions.

---

# 10. Domain API Specification

## 10.1 Core domain types

Add, at minimum:

- `Run`
- `RunKind`
- `RunStatus`
- `RunEvent`
- `RunEventType`
- provenance relation types

Recommended modules:

- `core::run`
- `core::event`
- `core::provenance`
- `core::context`

Avoid one giant “models.rs” graveyard.

## 10.2 Domain validation rules

### Run creation
- kind required
- status initially `Queued`
- `created_at` required
- `input_json` must be valid serializable payload

### Run start transition
Allowed from:
- `Queued`

Effects:
- status -> `Running`
- set `started_at` if absent
- emit `run_started`

### Run success transition
Allowed from:
- `Running`

Effects:
- status -> `Succeeded`
- set `finished_at`
- set `output_json`
- emit `run_succeeded`

### Run failure transition
Allowed from:
- `Queued`
- `Running`

Effects:
- status -> `Failed`
- set `finished_at`
- set `error_json`
- emit `run_failed`

### Run cancellation transition
Allowed from:
- `Queued`
- `Running`

Effects:
- status -> `Cancelled`
- set `finished_at`
- emit `run_cancelled`

### Rejected transitions
Should produce a domain error.

Example:
- `Succeeded` -> `Running`
- `Failed` -> `Succeeded`

## 10.3 Domain service traits

Introduce repository traits in `vel-core`, implemented in `vel-storage`.

Suggested traits:

### `RunRepository`
- `create_run(run)`
- `get_run(id)`
- `list_runs(limit, offset or cursor)`
- `update_run(run)` or specific transition methods
- `append_run_event(event)`
- `list_run_events(run_id)`

### `ArtifactRepository`
- `create_artifact(...)`
- `get_artifact(id)`
- `list_artifacts_for_run(run_id)` if convenient
- write/update metadata

### `RefRepository`
- `create_ref(...)`
- `list_refs_from(...)`
- `list_refs_to(...)`

### `ContextService` or equivalent domain service
- `generate_today(...)`
- `generate_morning(...)`

Use names aligned to existing conventions.

---

# 11. Storage Implementation Specification

## 11.1 Transaction boundaries

The following operations should be transactionally sane:

### Create run
Should:
- insert run
- insert `run_created` event

Prefer one transaction.

### Start run
Should:
- update run status to `running`
- set `started_at`
- append event

Prefer one transaction.

### Complete run successfully
Should:
- persist output artifact(s) / structured output
- update run terminal state
- append terminal event
- persist provenance/refs

Use transactional grouping where practical across DB operations.

Note: filesystem writes cannot be fully rolled into SQLite transactions. Handle this explicitly.

## 11.2 Artifact write behavior

Artifact writes should be durable and reasonably atomic.

### Required pattern
1. write to temp file
2. flush/sync if appropriate
3. rename into final destination
4. compute metadata/checksum
5. persist DB row / metadata
6. link refs/provenance
7. emit `artifact_written` event if applicable

Avoid the anti-pattern:
- mark run succeeded
- then try to write artifact

That creates phantom success.

## 11.3 Checksum strategy

Recommended:
- SHA-256
- store as lowercase hex string

Implementation notes:
- compute once at write time
- do not re-read large files more than necessary
- if current artifact creation pipeline already streams content, consider hashing during write

## 11.4 Path handling

Artifact paths should be:

- relative to a configured artifact root, or
- stored canonically and consistently

Do not mix absolute and relative paths arbitrarily.

Recommended:
- store relative path in DB
- resolve against configured artifact root at runtime

This improves portability.

## 11.5 Error mapping

Storage errors should be mappable to domain-level failures without turning into string mush.

Recommended categories:
- not found
- invalid state / constraint violation
- serialization failure
- filesystem failure
- migration failure

---

# 12. Provenance Semantics Specification

## 12.1 What provenance must answer

For a generated context output, an operator should be able to determine:

- which run created it
- what type of run it was
- which source records contributed
- when it was generated
- what logic/template version generated it

## 12.2 Minimum provenance links for context generation

When `today` or `morning` runs complete, create links:

1. run → generated artifact
2. generated artifact → source captures (if available)
3. generated artifact → source artifacts (if applicable)

If a template/version string exists in code, store it in:
- `metadata_json` on artifact, or
- `output_json` in run, or
- event payload

At least one durable place must exist.

## 12.3 Preferred semantics

Suggested relation usage:

- `produced` or `attached_to` for run → artifact
- `generated_from` for generated output → source input
- `derived_from` where a transformed artifact came from another artifact

If you keep exactly the minimum relation types from earlier, map cleanly and document it.

---

# 13. Context Generation Runtime Specification

This section is normative for `today` and `morning`.

## 13.1 Existing behavior

Whatever current direct endpoint/CLI logic exists for `today` and `morning` should be refactored so the user-visible behavior still works, but is now backed by a run.

## 13.2 Run-backed flow

### `today`
1. construct run input payload
2. create run with kind `ContextGeneration`
3. emit `run_created`
4. transition to `running`
5. gather relevant records
6. produce output content
7. persist output artifact or structured output
8. create provenance refs
9. transition run to `succeeded`
10. emit terminal events
11. return/display result

### `morning`
Same lifecycle pattern.

## 13.3 Input payload shape

The run’s `input_json` should include enough information to reconstruct intent.

Suggested shape:

```json
{
  "context_kind": "today",
  "requested_at": "2026-03-14T12:00:00Z",
  "parameters": {
    "date": "2026-03-14"
  }
}
```

For morning:

```json
{
  "context_kind": "morning",
  "requested_at": "2026-03-14T08:00:00Z",
  "parameters": {
    "date": "2026-03-14"
  }
}
```

Adjust to actual available inputs.

## 13.4 Output payload shape

The run’s `output_json` should summarize what was produced, without necessarily duplicating full content.

Suggested shape:

```json
{
  "artifact_id": "art_123",
  "artifact_type": "context_brief",
  "summary": "Morning briefing generated",
  "source_count": 8
}
```

## 13.5 Failure payload shape

`error_json` should be structured.

Suggested shape:

```json
{
  "error_kind": "generation_failed",
  "message": "Failed to load source captures",
  "details": {
    "missing_capture_count": 1
  }
}
```

Do not store only a flat string if a structured object is easy.

## 13.6 Artifact behavior

The generated context should be persisted as:

- an artifact of type `context_brief` or `summary`
- likely mime type `text/markdown` or `text/plain`

Preferred:
- persist markdown artifact
- return displayable content from that same source

This makes outputs durable and inspectable.

## 13.7 Template / generator versioning

At minimum, store a generator version string somewhere durable.

Recommended:
- artifact `metadata_json`: `{ "generator_version": "context-v1" }`

This helps later when behavior changes.

---

# 14. CLI Specification

## 14.1 `vel doctor`

### Purpose

Diagnose the most common operator problems for a local-first install.

### Required checks

1. config resolution
2. DB path resolvable
3. DB connection succeeds
4. schema/migration version readable
5. artifact directory exists
6. artifact directory writable
7. daemon reachable, if CLI depends on daemon for some flows
8. major configuration mismatch detection where feasible

### Output format

Human-readable, for example:

```text
Config: OK
Database: OK (/path/to/vel.sqlite)
Schema: OK (latest migration 0004)
Artifacts: OK (/path/to/artifacts)
Daemon: OK
```

Or on failure:

```text
Config: OK
Database: FAIL (unable to open /path/to/vel.sqlite)
Schema: SKIP
Artifacts: OK
Daemon: WARN (not reachable)
```

### Exit codes

Suggested:
- `0` all required checks pass
- `1` one or more hard failures
- optionally different codes for specific failure classes if current CLI style supports it

### Notes

Do not make doctor output ornamental. It is a wrench, not a poem.

## 14.2 `vel runs`

### Purpose

List recent runs.

### Output columns

At minimum:
- id
- kind
- status
- created_at
- finished_at

Optional:
- summary string
- artifact count

### Default behavior

- show most recent N runs (e.g. 20)
- support later filtering without requiring it now

### Example

```text
RUN ID        KIND                STATUS      CREATED AT              FINISHED AT
run_01        context_generation  succeeded   2026-03-14T08:00:01Z   2026-03-14T08:00:02Z
run_02        context_generation  failed      2026-03-14T08:05:01Z   2026-03-14T08:05:01Z
```

## 14.3 `vel run inspect <id>`

### Purpose

Inspect one run in detail.

### Must display

- run id
- kind
- status
- created/started/finished timestamps
- input summary
- output summary
- error summary if failed
- ordered event list
- linked artifacts
- possibly linked source refs if easy

### Example high-level layout

```text
Run: run_01
Kind: context_generation
Status: succeeded
Created: ...
Started: ...
Finished: ...

Input:
  context_kind: morning
  date: 2026-03-14

Output:
  artifact_id: art_01
  source_count: 8

Events:
  [1] run_created
  [2] run_started
  [3] artifact_written
  [4] run_succeeded

Artifacts:
  - art_01 (context_brief, text/markdown, 4812 bytes)
```

### Error case

Should clearly display `error_json`.

---

# 15. API Specification

## 15.1 Endpoints

Implement at minimum:

- `GET /v1/runs`
- `GET /v1/runs/:id`

Optionally:
- `GET /v1/runs/:id/events`

## 15.2 Response DTOs

### RunSummary DTO

Fields:
- `id`
- `kind`
- `status`
- `created_at`
- `started_at`
- `finished_at`

### RunDetail DTO

Fields:
- `id`
- `kind`
- `status`
- `input`
- `output`
- `error`
- `created_at`
- `started_at`
- `finished_at`
- `events`
- `artifacts` optional but desirable

### Event DTO

Fields:
- `seq`
- `event_type`
- `payload`
- `created_at`

### Artifact summary DTO

Fields:
- `id`
- `artifact_type`
- `mime_type`
- `path`
- `size_bytes`
- `checksum`

Keep response naming consistent with existing API conventions.

## 15.3 Behavior expectations

### `GET /v1/runs`
- returns recent runs
- stable ordering: newest first recommended

### `GET /v1/runs/:id`
- 404 if not found
- returns detailed record

### `GET /v1/runs/:id/events` if implemented
- ordered ascending by `seq`

## 15.4 Error semantics

Follow current API error style. Do not invent a second religion here.

---

# 16. Service / Use Case Specification

## 16.1 Recommended application service shapes

You may already have similar service abstractions. Reuse where sensible.

Suggested services:

### `RunService`
Responsibilities:
- create runs
- transition runs
- fetch runs
- inspect runs
- append events in a consistent way

### `ContextGenerationService`
Responsibilities:
- execute `today` and `morning`
- gather source material
- build artifact content
- persist output/provenance through repositories
- drive run lifecycle

### `DoctorService`
Responsibilities:
- collect system health checks
- report status objects for CLI rendering

## 16.2 Avoid these anti-patterns

- endpoint handler directly mutates DB and filesystem with no service layer
- CLI command re-implements run logic
- storage repo silently sets business statuses
- giant “AppState” blob becomes universal dependency

---

# 17. Error Model Specification

## 17.1 Domain errors

Introduce or extend typed error categories:

- invalid transition
- validation failure
- not found
- persistence failure
- filesystem failure
- serialization failure
- context generation failure

## 17.2 Run failure capture

When a run fails, convert the relevant error into structured `error_json`.

This does not require serializing the entire Rust error chain, but it should preserve:
- category
- message
- useful details if available

## 17.3 CLI error behavior

- commands should print useful, concise errors
- `doctor` should distinguish warnings from failures
- `run inspect` for a missing run should say not found plainly

---

# 18. Observability Specification

## 18.1 Structured logs

If structured logging is already present, ensure new run operations log:
- run creation
- status transitions
- artifact writes
- failures

Do not rely on logs as the sole runtime record, but do keep them coherent.

## 18.2 Log content

Recommended log keys:
- `run_id`
- `run_kind`
- `status`
- `artifact_id`
- `event_type`

## 18.3 PII / content caution

Avoid dumping full artifact contents or large JSON payloads into logs by default.

---

# 19. Testing Specification

This section is required.

## 19.1 Unit tests

Add tests for:

### Run transitions
- queued -> running valid
- running -> succeeded valid
- running -> failed valid
- queued -> cancelled valid
- invalid transitions rejected

### Event ordering
- first event seq = 1 or 0 depending on chosen convention
- subsequent events monotonic
- uniqueness enforced

### Artifact metadata
- checksum generated
- size_bytes correct
- artifact_type/mime persisted

### Provenance relation creation
- correct relation types created
- relation lookup returns expected links

## 19.2 Storage integration tests

Required:

1. migration on fresh DB
2. migration on existing pre-runtime DB fixture if practical
3. create run persists run + created event
4. start run persists started status + event
5. complete run persists terminal state + event
6. artifact write persists file + DB metadata
7. refs/provenance persist and can be queried

Prefer using temp directories and isolated SQLite files.

## 19.3 Context generation integration tests

Required:

1. `today` creates run and output artifact
2. `morning` creates run and output artifact
3. output artifact linked to run
4. source refs/provenance created if source inputs exist
5. failure path stores `error_json` and failed event

## 19.4 CLI tests

Required:

1. `vel runs` output for populated DB
2. `vel run inspect <id>` detail view
3. `vel doctor` success case
4. `vel doctor` DB failure case
5. `vel doctor` artifact path failure case

Use project-appropriate testing style; golden tests acceptable where stable.

## 19.5 Test quality guidance

- prefer deterministic timestamps where possible
- use temp dirs
- avoid race-prone concurrency unless necessary
- do not make tests depend on daemon unless specifically testing daemon reachability

---

# 20. Implementation Steps

This section is intended to be executable by a coding agent.

## Phase A — Schema and core types

1. inspect current schema and artifact tables
2. add migration for `runs`
3. add migration for `run_events`
4. add migration/alterations for artifact metadata
5. add migration for refs/provenance if needed
6. add indexes
7. add core enums and structs
8. add validation / transition logic

### Exit criteria
- project compiles
- new types exist
- migrations apply on fresh DB

## Phase B — Repositories and storage wiring

1. add repository traits in core
2. implement repositories in storage
3. implement transactional run create/start/finish helpers
4. implement artifact metadata persistence
5. implement provenance persistence
6. add storage tests

### Exit criteria
- run rows + event rows persist correctly
- artifacts include checksum/size
- refs queryable

## Phase C — Services

1. add `RunService`
2. refactor current context generation into `ContextGenerationService`
3. wrap `today`/`morning` in run lifecycle
4. persist output artifacts and refs
5. add failure capture
6. add service tests

### Exit criteria
- context generation always creates a run
- success and failure paths both durable

## Phase D — API and CLI

1. add run DTOs
2. add `/v1/runs`
3. add `/v1/runs/:id`
4. optionally add `/v1/runs/:id/events`
5. add `vel runs`
6. add `vel run inspect`
7. add `vel doctor`
8. add CLI tests

### Exit criteria
- operator can inspect recent runs from CLI/API
- doctor useful for local troubleshooting

## Phase E — Documentation and cleanup

1. update README
2. add runtime concepts doc
3. add troubleshooting doc or doctor section
4. document run/event/provenance model
5. clean naming and module layout
6. remove any dead pre-refactor logic

### Exit criteria
- docs reflect reality
- no duplicate code path for old direct context behavior

---

# 21. Suggested Module Layout

This is a recommendation, not a mandate.

## `vel-core`
```text
src/
  run/
    mod.rs
    model.rs
    service.rs
    error.rs
  event/
    mod.rs
    model.rs
  provenance/
    mod.rs
    model.rs
  context/
    mod.rs
    service.rs
    model.rs
```

## `vel-storage`
```text
src/
  runs.rs
  events.rs
  artifacts.rs
  refs.rs
  migrations.rs
```

## `vel-cli`
```text
src/
  commands/
    doctor.rs
    runs.rs
    run_inspect.rs
```

Use current project patterns where they already exist.

---

# 22. Naming Recommendations

Be consistent and boring.

## Recommended nouns
- run
- event
- artifact
- ref
- provenance
- context_brief

## Avoid overly abstract names like
- execution thing
- unit
- task item
- node
- flow state

Unless those concepts already have a clean place, they will blur everything.

---

# 23. Compatibility and Upgrade Notes

## 23.1 Existing CLI/API behavior

Existing `today` and `morning` behavior should continue to work from a user perspective.

This is a refactor-plus-hardening, not a product break.

## 23.2 Existing data

If existing DBs/artifacts are present:
- migrations must preserve old data
- artifact metadata backfill can be partial initially
- missing checksum/size for legacy rows can be tolerated if code handles nulls gracefully, though a later backfill task is recommended

## 23.3 Forward compatibility

Design decisions should leave room for:
- job queue
- cancellation
- retries
- synthesis
- agent runs

Without requiring:
- renaming core tables
- rethinking run semantics from scratch

---

# 24. Risks and Mitigations

## Risk 1: Overengineering the runtime too early
Mitigation:
- keep runs simple
- event log minimal
- no generic workflow DAG

## Risk 2: Duplicating artifact/ref systems
Mitigation:
- inspect current schema first
- consolidate rather than layering redundant relation tables

## Risk 3: Hidden business logic in handlers
Mitigation:
- centralize transitions in domain/service layer

## Risk 4: Filesystem/DB inconsistency
Mitigation:
- artifact write order carefully designed
- test crash-adjacent failure paths where possible

## Risk 5: DTO drift from domain model
Mitigation:
- explicit mapping layer
- avoid exposing raw DB rows

---

# 25. Future Extension Hooks

These are not in current scope, but the implementation should not block them.

## 25.1 Job Queue
Add later:
- `jobs`
- `job_attempts`
- lease/heartbeat semantics

## 25.2 Cancellation
Later add:
- `cancel_requested`
or
- separate cancel-request mechanism

## 25.3 Resume / retry
Could later be represented via:
- new run statuses
- attempt counters
- child runs
- retry events

## 25.4 Synthesis
Use existing run and artifact model:
- run kind `synthesis`
- output artifact type `summary`

## 25.5 Agents
Represent agent work as runs with tool invocations/events layered on top.

That is the right direction. Do not invent an incompatible second execution model later.

---

# 26. Acceptance Checklist

The implementation is complete only when all boxes are true:

## Schema
- [ ] `runs` table exists
- [ ] `run_events` table exists
- [ ] artifact metadata strengthened
- [ ] provenance/refs support exists
- [ ] indexes added

## Domain
- [ ] `RunKind` implemented
- [ ] `RunStatus` implemented
- [ ] transition rules enforced
- [ ] event types implemented

## Storage
- [ ] run repository implemented
- [ ] run event repository implemented
- [ ] artifact metadata persistence implemented
- [ ] provenance persistence implemented

## Context
- [ ] `today` creates runs
- [ ] `morning` creates runs
- [ ] success path persists output artifact
- [ ] failure path persists structured error
- [ ] provenance links generated output to inputs

## CLI
- [ ] `vel doctor` implemented
- [ ] `vel runs` implemented
- [ ] `vel run inspect <id>` implemented

## API
- [ ] `GET /v1/runs` implemented
- [ ] `GET /v1/runs/:id` implemented
- [ ] event endpoint implemented or consciously deferred

## Tests
- [ ] unit tests added
- [ ] storage integration tests added
- [ ] context generation tests added
- [ ] CLI tests added

## Docs
- [ ] runtime concepts documented
- [ ] operator workflow documented
- [ ] doctor documented

---

# 27. Final Guidance to the Coding Agent

When making local decisions during implementation, prefer the option that is:

- more explicit
- easier to inspect
- easier to test
- less magical
- less likely to require schema redesign later

Do not chase clever abstraction symmetry. Chase operational clarity.

The north-star question is:

> If a context generation run fails halfway through, can an operator inspect what happened, see what was produced, see what inputs were involved, and understand what to retry?

If the implemented system answers that cleanly, this spec has been fulfilled.

If the answer is still “grep logs and pray,” keep going.