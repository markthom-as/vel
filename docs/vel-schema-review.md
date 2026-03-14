# vel — Schema and Migration Review

## Framing

The current migrations already suggest a healthy instinct:

- bootstrap first
- then search
- then artifact references

That means the data model is growing in response to real needs rather than pure fantasy football.

That said, early schema decisions quietly decide whether the system remains elegant or turns into a landfill with SQL.

---

# What looks good already

## 1. SQLite is the right choice

For Vel's current shape, SQLite is ideal:

- transactional
- local-first
- easy to ship
- easy to back up
- low operational burden

Unless you are trying to impress investors who enjoy unnecessary PostgreSQL, this is the correct move.

---

## 2. Migrations exist early

This matters more than people think.

Having explicit migrations means:
- schema changes are intentional
- setup is reproducible
- drift becomes visible
- future automation is easier

You already avoided one of the most common MVP sins: "the schema is whatever the ORM currently dreams about."

---

## 3. Artifact references are already emerging

The introduction of artifact refs is a strong signal.

It means the system is not only storing user text, but beginning to model relationships between memory objects and durable outputs.

That is one of the more interesting long-term parts of the design.

---

# Core schema concern

## `captures` is at risk of becoming a junk drawer

This is the single biggest schema risk I'd watch right now.

Early on, everything looks like a capture:
- a typed note
- a pasted link
- a transcript import
- a generated summary
- a reminder
- an agent output
- a context briefing

But these are not the same thing.

If they all get stuffed into one broad table with a few nullable columns and vibes, the schema will go feral.

---

# Recommended conceptual model

Think in layers.

## Layer 1: events

Events are things that happened in the system.

Examples:
- capture created
- artifact extracted
- search executed
- synthesis generated
- job failed

Suggested columns:
- `id`
- `event_type`
- `subject_type`
- `subject_id`
- `payload_json`
- `created_at`

### Why

Events give you auditability, replayability, and timeline construction.

---

## Layer 2: captures

Captures are user-initiated or imported memory entries.

Suggested columns:
- `id`
- `kind`
- `title`
- `body`
- `source`
- `created_at`
- `updated_at`
- `deleted_at` (optional soft delete)
- `metadata_json`

Suggested `kind` values:
- `note`
- `link`
- `clipboard`
- `transcript`
- `imported`
- `journal`

### Why

This keeps captures as a stable domain concept without forcing every other object to masquerade as one.

---

## Layer 3: artifacts

Artifacts are durable objects associated with captures, runs, or synthesis.

Suggested columns:
- `id`
- `artifact_type`
- `mime_type`
- `path`
- `checksum`
- `size_bytes`
- `created_at`
- `metadata_json`

Suggested artifact types:
- `markdown`
- `url`
- `pdf`
- `image`
- `html`
- `transcript`
- `summary`
- `search_result`
- `context_brief`

### Why

Artifacts should be first-class citizens, not decorative sidecars.

---

## Layer 4: references

A join model for relations.

Instead of hardcoding a hundred nullable foreign keys, use explicit reference tables.

Suggested tables:
- `capture_artifact_refs`
- `run_artifact_refs`
- `artifact_artifact_refs`
- or one generic polymorphic ref table if carefully designed

Example generic shape:
- `id`
- `from_type`
- `from_id`
- `to_type`
- `to_id`
- `relation_type`
- `created_at`

### Why

This gives you provenance and traversal without schema mutation every two weeks.

---

# Strong recommendation: add runs early

Even if you are not "doing agents yet," you will want a run model.

## Suggested `runs` table

- `id`
- `run_kind`
- `status`
- `input_json`
- `output_json`
- `error_json`
- `created_at`
- `started_at`
- `finished_at`

Suggested `run_kind` values:
- `search`
- `context_generation`
- `synthesis`
- `artifact_extraction`
- `embedding_index`
- `agent`

### Why

Once background work appears, lack of a run model becomes an architectural tax on everything.

---

# Search schema suggestions

If search is important, do not leave it as a side effect with no durable record.

## Suggested `search_queries` table

- `id`
- `query_text`
- `mode`
- `filters_json`
- `created_at`

## Suggested `search_results` table

- `id`
- `search_query_id`
- `result_type`
- `result_id`
- `rank`
- `score`
- `created_at`

### Why

This lets you:
- inspect retrieval quality
- support replay/debugging
- compare ranking modes later
- generate better context from prior searches

If you do not need persistence for every query, you can still sample or persist only user-invoked searches. But having a schema path matters.

---

# Provenance model

Generated outputs should carry provenance, always.

Suggested `generation_provenance` table:
- `id`
- `output_type`
- `output_id`
- `source_type`
- `source_id`
- `relation_type`
- `created_at`

Or, if you want fewer tables, fold this into a general refs table with a clear `relation_type`.

Useful relation types:
- `derived_from`
- `references`
- `generated_from`
- `attached_to`
- `extracted_from`

### Why

Without provenance, generated material becomes epistemically suspect very quickly.

---

# JSON columns: use them, but don't hide your entire soul in them

JSON is useful for:
- metadata
- optional per-type fields
- input/output payloads
- filters

But do not use JSON as an excuse to avoid modeling core concepts.

Bad pattern:
- one giant `captures` table
- `data_json` contains the entire universe

Better pattern:
- stable columns for core invariants
- JSON for flexible extensions

---

# Soft delete vs hard delete

For a cognition system, I'd lean toward soft delete for captures and artifacts.

Suggested pattern:
- `deleted_at`
- optional `deleted_reason`

### Why

It supports:
- auditability
- accidental recovery
- future undo
- better sync semantics later

Hard delete can still exist as a maintenance operation.

---

# Index recommendations

Likely useful indexes:

## captures
- `created_at`
- `kind`
- maybe FTS virtual table for body/title

## artifacts
- `artifact_type`
- `checksum`
- `created_at`

## refs
- `(from_type, from_id)`
- `(to_type, to_id)`
- `relation_type`

## runs
- `status`
- `run_kind`
- `created_at`

## search
- `created_at`
- maybe `mode`

### Important

If full-text search matters, treat it as a designed subsystem, not an afterthought. SQLite FTS can carry a surprising amount of weight before you need anything fancier.

---

# Migration strategy recommendations

## 1. Prefer additive migrations

Add tables / columns rather than rewriting history.

## 2. Backfill carefully

If introducing runs/events/provenance later, include explicit data migration scripts so earlier rows remain coherent.

## 3. Keep migrations readable

Resist generating giant opaque migration blobs if hand-written SQL is still manageable.

## 4. Track schema version in `doctor`

Operator tooling should clearly show:
- current schema version
- latest available migration
- whether DB is ahead/behind

---

# Future-safe schema milestones

In order of likely value:

1. `runs`
2. `run_events`
3. stronger artifact metadata
4. generalized refs/provenance
5. FTS strategy
6. job queue / attempts
7. synthesis output tables or artifact typing
8. topic / dossier tables if that feature materializes

---

# A schema north star

A good test for the schema is this:

> Can I explain where a piece of context came from, what generated it, what it references, and whether I can recreate it?

If the schema supports that, you're on the right track.

If the answer is "sort of, if I grep logs and trust my memory," the schema needs more spine.

---

# Final assessment

The schema direction is already promising.

The main risk is not that it is bad now. The risk is that early convenience hardens into an everything-table anti-pattern.

The cure is straightforward:

- keep captures narrow
- make artifacts first-class
- add runs/events before async work sprawls
- model provenance explicitly
- use JSON as garnish, not ontology

Do that, and the storage layer stays elegant instead of turning into a cursed pantry.
