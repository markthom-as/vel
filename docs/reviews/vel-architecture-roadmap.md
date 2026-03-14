# vel — Architecture Roadmap (6–12 Months)

## Framing

Vel already has the bones of a strong system:

- local-first storage
- CLI / daemon split
- SQLite + migrations
- focused API surface
- a conceptual core around capture, recall, and orientation

The right next move is **not** to sprawl into a generic automation platform.

The right move is to deepen the core identity:

> **Vel as a local operating system for cognition**

Or, in plainer runtime terms:

> **a durable substrate for capture, retrieval, synthesis, and agent-assisted orientation**

---

## Guiding principles

1. **Local-first before distributed**

   Keep state inspectable, portable, and debuggable on one machine before introducing any remote complexity.

2. **Artifacts before vibes**

   Prefer durable, inspectable outputs over ephemeral agent chatter.

3. **Explicit state over hidden magic**

   Runs, jobs, context generation, and synthesis should all have observable state transitions.

4. **LLMs as a layer, not the foundation**

   The runtime should remain useful even when no model is configured.

5. **Human orientation is the product**

   The valuable thing is not "AI did a thing." The valuable thing is that the user becomes more oriented, more able to act, and less cognitively fragmented.

---

# Phase 1 — Harden the Core (next 4–6 weeks)

## Goals

Make Vel durable, inspectable, and boring in the best possible way.

## 1. Introduce a run / job model

Add tables and types for:

- `runs`
- `run_events`
- `jobs`
- `job_attempts`

Even if the implementation is minimal.

### Why

Right now the system likely models captures and retrieval, but anything asynchronous or resumable will eventually need a first-class execution record.

### Minimum viable semantics

`runs`:
- id
- kind
- status (`queued`, `running`, `succeeded`, `failed`, `cancelled`)
- created_at
- started_at
- finished_at
- input_json
- output_json
- error_json

`run_events`:
- id
- run_id
- seq
- event_type
- payload_json
- created_at

---

## 2. Add a replayable event log

Every meaningful action should emit an event.

Examples:
- `CAPTURE_CREATED`
- `ARTIFACT_WRITTEN`
- `SEARCH_EXECUTED`
- `CONTEXT_GENERATED`
- `JOB_STARTED`
- `JOB_RETRIED`
- `JOB_FAILED`

### Why

This unlocks:
- reproducibility
- operator debugging
- timeline views
- future auditing
- easier test assertions

### CLI implications

Add commands like:

```bash
vel runs
vel run inspect <id>
vel timeline
vel replay <run-id>
```

---

## 3. Add `vel doctor`

This should become the first-line support tool.

It should check:

- config resolution
- DB connectivity
- schema version
- artifact directory existence and writability
- daemon reachability
- worker health
- optional model provider configuration

### Why

Every local-first runtime needs one ruthless diagnostic entrypoint. Otherwise every support conversation turns into interpretive dance.

---

## 4. Make artifacts first-class

Strengthen the model around:

- artifact type
- mime type
- source
- filesystem path
- checksum
- provenance
- references to captures / runs

### Why

Artifacts are one of the most defensible parts of Vel's architecture. They should not be treated as decorative attachments.

### Suggested types

- `note`
- `url`
- `pdf`
- `image`
- `transcript`
- `summary`
- `search_result`
- `generated_context`

---

# Phase 2 — Retrieval and Context as a Runtime Primitive (6–10 weeks)

## Goals

Turn capture/search/context into a coherent memory system rather than a loose set of endpoints.

## 1. Split "capture" from "event"

`capture` is a user-facing action.

Storage-wise, you likely want a broader substrate:

- `events`
- `captures`
- `artifacts`
- `artifact_refs`

### Why

Otherwise `captures` turns into a junk drawer holding:
- raw notes
- agent outputs
- daily summaries
- imported feeds
- reminders
- system events

That becomes ugly quickly.

---

## 2. Unify search semantics

Define a search domain type with explicit modes:

- keyword
- semantic
- hybrid
- recency-weighted
- source-filtered

### Why

Search is not just an endpoint. It is one of the core verbs of the system. Treat it as such.

### Good future API shape

```json
{
  "query": "meeting notes budget",
  "mode": "hybrid",
  "sources": ["captures", "artifacts"],
  "time_range": {"from": "...", "to": "..."},
  "limit": 20
}
```

---

## 3. Define a context generation contract

Today / morning context should become explicit jobs with predictable inputs and outputs.

Input:
- date / range
- relevant events
- recent captures
- recent artifacts
- open tasks (future)
- optionally search results

Output:
- a generated context artifact
- provenance
- generation metadata
- optional model metadata

### Why

Context generation is one of the highest-value behaviors in Vel. Make it inspectable.

---

## 4. Add provenance everywhere

For generated outputs, store:
- source item ids
- source artifact ids
- generation prompt / template version
- model / backend
- generation timestamp

### Why

Generated context without provenance becomes mystical sludge. Good for startups, bad for software.

---

# Phase 3 — Introduce an Agent Layer Carefully (2–4 months)

## Goals

Add agents without allowing the repo to mutate into orchestration soup.

## 1. Add agent execution as a thin layer over runs/jobs

Do **not** invent a new metaphysics for agents.

An agent execution is just:
- a run
- with declared inputs
- producing artifacts and events
- optionally using tools

### Model

`agent_run` can likely be represented as:
- run kind = `agent`
- config blob
- toolset reference
- output artifacts

---

## 2. Define a strict tool interface

Add a domain boundary like:

- `ToolSpec`
- `ToolInvocation`
- `ToolResult`
- `ToolError`
- `RetryPolicy`

### Why

If tool invocation is ad hoc, the runtime gets contaminated fast.

### Desired properties

- typed inputs/outputs
- timeout support
- structured errors
- idempotency hooks
- event emission

---

## 3. Add cancellation and resume semantics

Every long-running agent or synthesis job should be:
- cancellable
- inspectable
- resumable where practical

### Why

This is the difference between a demo and a runtime.

---

## 4. Keep agents artifact-oriented

The runtime should bias toward outputs like:
- summary markdown
- extracted references
- synthesized briefings
- generated task lists
- daily / weekly orientation docs

Not:
- giant hidden chat transcripts as the primary output

---

# Phase 4 — Synthesis as the Differentiator (4–6 months)

## Goals

Make Vel genuinely useful in daily life.

## 1. Daily and weekly synthesis

Add commands like:

```bash
vel synthesize day
vel synthesize week
vel synthesize topic "grant deadlines"
```

### Output should include

- summary
- relevant captures
- relevant artifacts
- open questions
- suggested next actions
- provenance

---

## 2. Orientation views

Evolve `morning` / `today` into structured orientation surfaces:

- what changed
- what matters
- what is unresolved
- what likely needs action
- what you may be forgetting

This is where Vel gets genuinely interesting.

---

## 3. Topic dossiers

Add the ability to maintain a running dossier for a topic:
- project
- person
- idea
- research thread

Example:

```bash
vel dossier "Mimesis budget"
vel dossier "Scylla/Charybdis"
```

Output:
- latest related captures
- key artifacts
- recent changes
- unresolved issues
- synthesized state

---

# Architecture constraints to defend ruthlessly

## 1. Do not become a notes app

Notes can exist inside Vel, but "notes app" is not the product.

## 2. Do not become a workflow engine

Once you start modeling generic DAG orchestration, you are one bad quarter away from becoming an internal Airflow with feelings.

## 3. Do not become LangChain-compatible by default

Compatibility gravity is real. Guard your abstractions.

## 4. Do not let LLM providers leak into core domain types

The domain should talk about:
- synthesis
- context generation
- retrieval
- artifact creation

Not about whichever vendor is currently fashionable.

---

# Suggested near-term deliverables

In order:

1. `vel doctor`
2. `runs` table + run inspection
3. `run_events` or event log
4. first-class artifact metadata
5. context generation provenance
6. job queue abstraction
7. daily synthesis
8. topic dossiers
9. agent layer on top of runs

---

# Product identity statement

A good internal sentence for Vel might be:

> Vel is a local-first cognition runtime that turns captures, artifacts, and retrieval into usable orientation.

Another, sharper version:

> Vel is Unix for personal cognition.

Slightly theatrical, but honestly not wrong.

---

# Final recommendation

The most important thing now is not breadth. It is **state discipline**.

If you keep:
- runs explicit
- artifacts durable
- provenance visible
- synthesis layered on top of stable primitives

then Vel can become unusually powerful.

If instead it sprawls sideways into generic notes/tasks/automation, it will dissolve into the usual mush.

The architecture already hints at the better path. Follow that one.
