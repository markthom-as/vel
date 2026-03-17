# Vel — Architecture

## System Principles

Vel should be:

- local-first
- distributed but degradable
- modular
- introspectable
- privacy-aware
- designed for iteration

Vel is not a monolithic app. It is a **distributed personal system** with multiple clients, storage tiers, and execution tiers.

---

## High-Level Shape

```text
clients
  ├─ vel-cli
  ├─ iPhone / Watch
  ├─ voice surfaces
  └─ desktop interfaces
        ↓
      veld
        ↓
  memory + scheduling + suggestions + APIs
        ↓
storage / models / agents / external systems
```

---

## Core Services

### veld
The long-running core runtime.

Responsibilities:
- memory graph access
- ingestion orchestration
- scheduler / background jobs
- suggestion engine
- API surface
- sync manager
- model routing
- privacy / behavior controls

### vel-cli
Operator shell for:
- capture
- search
- recall
- plan
- reflect
- debug

### Mobile clients
- quick capture
- daily summaries
- approvals
- reminders
- compact dashboard

### Optional AI sidecar
A separate process or service for:
- embeddings
- summarization
- LLM-heavy orchestration
- experimentation

---

## Storage Model

### Metadata
- SQLite first
- migration path to Postgres or distributed SQLite

### Artifacts
Filesystem / object storage:
- local machine
- NAS
- optional S3-compatible backup
- selective mobile cache

### Indexes
- lexical index
- semantic index
- content hashes

---

## Memory Graph Design

Use a relational core plus a generic relationships table.

Core objects:
- containers
- projects
- goals
- milestones
- tasks
- commitments
- people
- captures
- artifacts
- conversations
- suggestions
- reflections
- timeline events
- behavior configs

This gives stable structure without prematurely adopting a full graph database.

---

## Capture Pipelines

### Explicit capture first
Supported sources:
- text notes
- voice notes
- meeting recordings
- imported transcripts
- git activity
- tasks/calendar imports

### Pipeline shape
```text
capture
  ↓
artifact creation
  ↓
optional transcription / extraction
  ↓
timeline event
  ↓
memory links
  ↓
possible suggestion candidates
```

### Edge cases
- partial recording recovery
- duplicate source reconciliation
- low-confidence transcript handling
- sensitive-content review

---

## Alignment / Suggestion Engine

Vel should use a hybrid pipeline.

### Stage 1: rule detectors
Examples:
- dormant project > N days
- repeated deferral
- due commitment approaching
- no time on high-priority goal
- health reminder threshold

### Stage 2: contextualizer
LLM or lightweight reasoning layer:
- ranks signals
- suppresses noise
- frames nudges
- adapts tone / urgency

### Stage 3: feedback loop
Suggestions support:
- dismiss
- correct
- never show again
- train system

---

## Planning Horizons

Vel should support:
- morning brief
- end-of-day summary
- weekly review
- monthly / quarterly planning
- year-in-review later

---

## Distributed Node Model

Potential nodes:
- phone
- watch
- laptop
- Linux workstation
- NAS
- optional cloud/VPS

Vel should degrade across tiers:
- fully local basic mode
- LAN-enhanced mode
- cloud-assisted mode

---

## Model Routing

Vel should route work across:
- phone-local models where feasible
- workstation/NAS local models
- remote/cloud models when needed
- compatible Connect instances that can host external agent runtimes

Rule of thumb:
- capture / simple summaries / cached recall → local
- complex synthesis / deep reasoning → remote fallback allowed

Offline-first core features must remain usable.

## Connect-Aware Agent Execution

Vel should be able to treat compatible Connect instances as bounded execution targets for external agent runtimes.

Examples:
- Codex on a workstation
- Claude Code on a laptop
- Cursor or Copilot agent surfaces on a desktop machine
- OpenCode or Gemini CLI on a remote executor

Guardrails:
- instance capabilities should be discovered through manifests, not hardcoded UI checks
- launched sessions should flow back into Vel's session/operator surfaces
- Vel's host agent remains the supervisor and canonical integrator

---

## ProvenAct Boundary

ProvenAct is optional infrastructure, not Vel’s core identity.

If used, it should function as:
- execution engine
- policy gate
- provenance/receipt layer

Vel remains:
- context engine
- memory system
- planning system
- orchestration layer

---

## v0 Architecture Focus

v0 should prioritize:
1. capture
2. recall
3. daily orientation
4. suggestion feedback
5. reliable storage / restore

Avoid early overreach into:
- autonomous swarms
- full ambient recording
- complex distributed queues
- productized multi-tenant features

---

## Repo Direction

Suggested conceptual layout:

```text
vel/
├── AGENTS.md
├── vel.md
├── docs/
│   ├── product-spec.md
│   ├── architecture.md
│   ├── data-model.md
│   ├── mvp.md
│   └── runtime.md
├── crates/
├── services/
└── apps/
```
