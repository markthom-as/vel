# vel – Architecture & Repository Review

## Repository Structure

```
vel
├─ crates
│  ├─ vel-api-types
│  ├─ vel-cli
│  ├─ vel-config
│  ├─ vel-core
│  ├─ vel-storage
│  └─ veld        (daemon)
├─ docs
├─ migrations
├─ vel.md
```

This structure is strong. The project avoids the common mistake of collapsing everything into a single crate.

The layering appears to follow:

```
CLI
  ↓
HTTP API
  ↓
core domain
  ↓
storage
```

This is the correct direction of dependency.

---

# What's Strong

## 1. Local‑first architecture

Your runtime layout:

```
database: var/data/vel.sqlite
artifacts: var/artifacts
logs: var/logs
```

This is an excellent model for a **local knowledge runtime**.

Benefits:

- simpler debugging
- easy backups
- deterministic state
- minimal operational complexity

---

## 2. CLI / daemon split

```
vel-cli  → operator interface
veld     → runtime daemon
```

This enables future expansion:

- background workers
- scheduled tasks
- remote clients
- multiple interfaces

This was the correct architectural decision.

---

## 3. Domain separation

Crates are cleanly separated:

```
vel-api-types
vel-core
vel-storage
```

Recommended responsibility boundaries:

| Crate | Role |
|-----|-----|
| vel-api-types | boundary DTOs |
| vel-core | domain logic |
| vel-storage | persistence |
| vel-cli | user interface |
| veld | runtime daemon |

This separation will make the system easier to evolve.

---

## 4. SQLite with migrations

```
migrations/
 ├─ 0001_bootstrap.sql
 ├─ 0002_capture_search.sql
 └─ 0003_capture_artifact_refs.sql
```

SQLite is a strong choice for this stage:

- ACID transactions
- simple deployment
- WAL support
- easy backups

Excellent MVP database.

---

# Most Interesting Design Choice

The **capture → search → context** pipeline.

API endpoints:

```
POST /v1/captures
GET  /v1/search
GET  /v1/context/today
GET  /v1/context/morning
```

This suggests the system models **memory and recall**, not just notes.

Conceptually this becomes:

```
capture → memory
search → recall
context → orientation
```

This is a powerful conceptual framing.

---

# Improvements to Consider

## 1. Keep domain logic inside vel-core

Enforce dependency direction:

```
vel-cli
vel-api
   ↓
vel-core
   ↓
vel-storage
```

Only `vel-core` should understand business semantics.

Other crates should treat operations as opaque calls.

---

## 2. Prevent `captures` from becoming a junk drawer

Current schema pattern:

```
captures
artifacts
artifact_refs
```

This works early but may limit future evolution.

Consider introducing:

```
events
captures
artifacts
artifact_refs
```

Reason:

Future data types may include:

- tasks
- reminders
- decisions
- reflections
- agent outputs

Not all of these should be "captures".

---

## 3. Add a run / job concept early

Future runtime features will include:

- background indexing
- summarization
- embedding generation
- daily synthesis

Introduce a minimal run system:

```
runs
run_events
```

Even if unused initially, it prevents migration pain later.

---

# CLI Design Feedback

Current commands:

```
vel health
vel capture
vel search
vel today
vel morning
vel config show
```

UX is strong and readable.

Two useful additions:

```
vel inspect
vel doctor
```

Example usage:

```
vel inspect capture 123
vel doctor
```

Doctor should verify:

- database connection
- schema version
- artifact directory
- daemon health

---

# Background Processing

You already have early worker scaffolding.

Define a simple job abstraction early:

```
jobs
job_runs
```

Even if implemented on SQLite initially.

This will support:

- embeddings
- indexing
- synthesis
- agent operations

---

# Avoid This Common Rust Runtime Smell

Watch for excessive use of:

```
Arc<Mutex<AppState>>
```

Instead define a structured context:

```
AppContext
 ├─ storage
 ├─ config
 ├─ clock
```

Pass explicitly through functions.

This improves:

- testability
- clarity
- dependency boundaries

---

# Strategic Direction

Vel is not simply a note system.

Conceptually it resembles:

**a local operating system for cognition**

```
capture → memory
search → recall
context → orientation
```

Commands like:

```
vel morning
vel today
```

are the most distinctive part of the system.

This ritual‑style interaction model is worth emphasizing.

---

# High‑Impact Future Features

## 1. Replayable event log

Each operation generates an event:

```
CAPTURE_CREATED
SEARCH_EXECUTED
CONTEXT_GENERATED
```

This enables:

```
vel replay
vel audit
vel timeline
```

Extremely powerful for debugging and analysis.

---

## 2. Artifact pipeline

Artifacts should become first‑class entities.

Example pipeline:

```
capture
   ↓
artifact extraction
   ↓
artifact indexing
```

Artifacts may include:

- URLs
- documents
- images
- transcripts

---

## 3. Synthesis layer

Eventually:

```
vel synthesize day
vel synthesize week
```

This is where LLM integration becomes valuable.

But it should sit **on top of a stable core runtime**.

---

# Strategic Risk

The project could drift into becoming:

- a notes application
- a task manager
- a PKM system

Those are crowded markets.

Vel's real strength is its **runtime architecture**.

It should feel closer to:

```
Unix for personal cognition
```

rather than a traditional note tool.

---

# Overall Assessment

For an initial release, the repository is strong.

Positive indicators:

- clean crate boundaries
- CLI / daemon separation
- migrations already present
- minimal and focused API
- clear conceptual framing

This is a solid foundation for a long‑term system.
