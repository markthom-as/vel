# Vel — Runtime Architecture

## Purpose

This document defines the runtime blueprint for Vel v0.

It translates the product spec, architecture principles, and data model into an executable system design.

Vel should be built as a **hybrid system**:

- a long-running daemon (`veld`)
- CLI and other clients as frontends
- optional sidecars for AI-heavy work
- local-first storage with degraded offline behavior

---

## Core Runtime Shape

Recommended v0 shape:

```text
veld
├── metadata store
├── artifact manager
├── ingestion pipeline
├── scheduler
├── suggestion engine
├── sync manager
├── HTTP API
├── model router
└── behavior/privacy control plane
```

Clients:
- `vel-cli`
- iPhone / Watch clients
- future desktop / dashboard clients
- optional voice surfaces

Optional sidecar:
- `vel-ai` for embeddings, transcription, LLM-heavy jobs

---

## veld Responsibilities

`veld` is the canonical process responsible for:

- serving the current context state
- reading/writing the metadata DB
- managing artifact references
- receiving captures
- launching ingestion jobs
- producing summaries and reflections
- generating suggestion candidates
- exposing the API
- enforcing privacy, quiet modes, and behavior controls
- managing sync metadata and degraded/offline mode

`veld` should be able to run:
- locally on workstation/laptop
- on NAS or server
- on VPS/cloud later if desired

---

## Process Model

### Recommended v0
Hybrid model:
- long-running daemon for stateful behavior
- CLI commands as thin clients
- scheduler inside daemon
- event-triggered jobs initiated by new data or time-based triggers

This supports:
- continuous context maintenance
- low-friction CLI
- future mobile/API clients
- auditability

---

## Language / Implementation Strategy

Recommended:
- core runtime in Rust
- optional AI sidecar in Python
- mobile clients in Swift

### Rust core
Best fit for:
- daemon
- DB access
- scheduler
- API server
- sync manager
- artifact manager
- permission / behavior control

### Python sidecar
Best fit for:
- transcription
- embeddings
- summarization experiments
- LLM orchestration that changes quickly

This keeps the runtime stable while preserving iteration speed.

---

## Storage Layers

### 1. Metadata Store
Primary:
- SQLite

Stores:
- object metadata
- relationships
- behavior configs
- timeline events
- processing jobs
- sync state

### 2. Artifact Store
Filesystem/object storage for:
- audio
- transcripts
- summaries
- exports
- raw notes
- imported documents

Preferred tiers:
- local machine
- NAS
- optional S3 backup
- selective phone cache

### 3. Indexes
- lexical search index
- semantic index
- content hashes
- optional embedding refs

---

## Job Model

Vel needs background jobs, but v0 should avoid distributed queue overkill.

### v0 recommendation
- internal scheduler
- `processing_jobs` table
- worker loop in `veld`

Job types:
- transcribe capture
- extract commitments
- create summary
- generate morning brief
- generate end-of-day summary
- detect dormant projects
- rank suggestions
- sync artifacts/indexes

This is enough before introducing Redis or a full queue.

---

## Processing Pipeline

### 1. Capture received
Input sources:
- CLI text capture
- voice capture
- imported file
- transcript import
- meeting recording upload

### 2. Artifact record created
Vel creates:
- metadata row
- blob reference
- timeline event

### 3. Job scheduling
Depending on capture type:
- transcription
- extraction
- summarization
- linking
- suggestion candidate generation

### 4. Derived objects created
Possible outputs:
- conversation
- task
- commitment
- summary artifact
- suggestion candidate

### 5. Feedback and correction
User may:
- confirm classification
- correct metadata
- suppress suggestion
- train system

---

## Failure Handling

### Capture interruption
If capture dies mid-stream:
- save partial artifact
- mark incomplete
- keep recoverable metadata
- allow later repair/merge with external source

### DB corruption
Support:
- periodic snapshots
- WAL/incremental backups where possible
- artifact-first rebuild path

### Missing artifact
If metadata exists but blob is missing:
- flag broken artifact
- surface repair path
- do not silently discard references

### Suggestion runaway
Provide:
- global quiet mode
- suggestion kill switch
- category-level suppression
- behavior reset path

---

## Sync Model

Vel should support multiple nodes:
- phone
- watch
- laptop
- workstation
- NAS
- optional cloud node

### v0 expectation
Not full sync brilliance. Just enough structure to avoid painting into a corner.

Each object/artifact should carry:
- sync status
- sync class
- last synced time
- allow later repair/merge with external source

### DB corruption
Support:
- periodic snapshots
- WAL/incremental backups where possible
- artifact-first rebuild path

### Missing artifact
If metadata exists but blob is missing:
- flag broken artifact
- surface repair path
- do not silently discard references

### Suggestion runaway
Provide:
- global quiet mode
- suggestion kill switch
- category-level suppression
- behavior reset path

---

## Sync Model

Vel should support multiple nodes:
- phone
- watch
- laptop
- workstation
- NAS
- optional cloud node

### v0 expectation
Not full sync brilliance. Just enough structure to avoid painting into a corner.

Each object/artifact should carry:
- sync status
- sync class
- last synced time
- availability tier

### Conflict handling
Conflicts should create:
- conflict artifacts or conflict records
- optional merge path later
- never silent overwrite of meaningful user data

---

## API Surface

Recommended v0 protocol:
- HTTP + JSON
- typed request/response schemas
- keep room for streaming later

Suggested endpoints:
- `POST /capture`
- `GET /context/today`
- `GET /search`
- `GET /project/:id`
- `GET /goal/:id`
- `POST /suggestion/:id/feedback`
- `POST /behavior/update`
- `POST /jobs/run`
- `GET /health`

CLI and mobile clients should consume the same core API where practical.

---

## Suggestion Pipeline

### Stage 1: detectors
Rule-based detectors scan DB/timeline state.

Examples:
- dormant project threshold hit
- due commitment approaching
- zero time on important goal
- repeated task deferral

### Stage 2: contextualizer
Optional LLM or lightweight reasoning layer:
- rank candidates
- decide phrasing
- suppress noise
- adapt tone to current mode

### Stage 3: delivery
Suggestions are surfaced through:
- CLI
- morning brief
- mobile notification later
- dashboard later

### Stage 4: feedback
User can:
- dismiss
- correct
- never show again
- train system

Feedback updates behavior config and suggestion history.

---

## Behavior / Privacy Control Plane

Vel needs explicit behavioral controls from day one.

### Behavior controls
- quiet mode
- quiet hours
- reminder strictness
- category nag levels
- contextual modes
- global suggestions off

### Privacy controls
- privacy classes
- do-not-record
- sensitive retention rules
- selective sync blocking
- redaction/review hooks

These are not optional polish. They are core trust infrastructure.

---

## Model Routing

Vel should route tasks based on:
- sensitivity
- complexity
- latency tolerance
- offline availability
- available local compute

### Rough policy
- basic recall / local summaries → local or cached
- search/ranking → non-LLM first, LLM optional
- deep synthesis → workstation/cloud allowed
- sensitive/private data → local-preferred

### Fallback chain
Recommended:
- local model / heuristic
- stronger local node if available
- remote model if permitted
- degraded deterministic fallback

---

## Client Types

### vel-cli
Main operator shell for:
- capture
- search
- recall
- project continuity
- manual jobs
- debugging suggestions

### Mobile
Primary use cases:
- voice/text capture
- daily brief
- approvals / feedback
- lightweight recall

### Watch
Primary use cases:
- quick capture
- quick reminder
- approve/dismiss
- basic “what’s next?”

### Desktop/dashboard
Later:
- visual project graph
- timeline
- reflections
- review/reconciliation interfaces

---

## Security / Execution Boundaries

Vel should distinguish:
- cognition
- orchestration
- execution

Execution should be permissioned and configurable.

Examples:
- repo-scoped tool access
- approval-required actions
- automation modes
- version control preferred

If external execution engines like ProvenAct are used, they should sit beneath Vel as optional infrastructure.

---

## Observability

Vel must be inspectable.

Store:
- job logs
- suggestion source info
- processing errors
- feedback history
- sync state
- model routing decisions where useful

This supports:
- debugging
- trust
- self-tuning
- future productization

---

## v0 Runtime Priorities

Build in this order:

1. `veld` skeleton
2. SQLite metadata layer
3. artifact manager
4. capture ingestion
5. search / recall endpoints
6. morning and end-of-day summaries
7. suggestion detector + feedback loop
8. behavior controls
9. mobile-friendly API

---

## Non-Goals for v0 Runtime

Avoid early implementation of:
- distributed queue stacks
- full graph engine
- real-time collaboration
- polished push notification orchestration
- universal passive listening
- multi-tenant auth complexity

---

## Summary

Vel v0 should feel like:

- a daemon-backed personal context system
- a CLI-first operator shell
- a reliable capture and recall layer
- a modest but useful executive assistant

The runtime should be disciplined enough to evolve, but simple enough to ship.
