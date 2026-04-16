---
title: Semantic Memory & Graph RAG Implementation
status: planned
owner: staff-eng
type: feature
priority: medium
created: 2026-03-17
updated: 2026-04-16
depends_on:
  - 007-day-simulation-harness
  - 017-execution-tracing-reviewability
labels:
  - vel-storage
  - memory
  - rag
  - phase-4
---

# Context & Objectives

Vel currently ships lexical/local retrieval and artifact/run inspection, but does not yet have semantic vector indexing or graph-linked retrieval.

This ticket introduces semantic memory indexing and retrieval as a local-first extension of existing search and artifact/thread structures.

# Impacted Files & Symbols

- **Crate**: `vel-storage`
  - **Symbols**: semantic index persistence, retrieval query surfaces, migrations
- **Crate**: `veld`
  - **Symbols**: retrieval orchestration and ranking blend with lexical search
- **Crate**: `vel-core`
  - **Symbols**: semantic memory/query contracts
- **Docs**: retrieval and memory operation contracts in architecture/user docs

# Technical Requirements

- **Vector Backend Abstraction**: introduce a backend seam for semantic index storage.
- **Embedding Pipeline**: local-first embedding generation/index refresh path.
- **Graph Linkage**: connect semantic chunks to durable captures/artifacts/threads.
- **Hybrid Retrieval**: combine lexical and semantic ranking with explicit scoring policy.
- **Observability**: index/update/query paths emit inspectable run or trace events.

## Contract Decisions

The semantic-memory implementation must lock these contracts before storage migrations or retrieval services widen.

### Embedding Model Boundary

Embedding generation routes through Vel's local-first model boundary, not direct provider calls from storage or route handlers. The active embedding profile must record:

- model identifier.
- embedding revision.
- vector dimension.
- backend family.

Persisted semantic records must keep `embedding_model` and `embedding_revision` so rebuilds and mixed-index states are inspectable. Vector dimension is a schema-level decision for the selected backend and must be verified before migration.

### Index Rebuild Triggers

Embedding/index refresh is background-job driven through the existing `processing_jobs` queue. Inserts or updates to indexable captures, artifacts, and threads enqueue deterministic indexing jobs instead of blocking user-facing capture paths.

Rebuild jobs must be idempotent and keyed by source object plus embedding revision. A model or revision change creates a new rebuild lane rather than silently overwriting provenance.

### Hybrid Ranking Policy

Retrieval combines lexical and semantic candidates through an explicit scoring policy. The first implementation should use deterministic weighted blending over normalized lexical and vector scores, with provenance and component scores preserved in diagnostics.

The ranking policy must remain configurable enough to tune weights, but every response must be explainable from source object IDs, lexical score, semantic score, embedding revision, and final blended score.

# Cross-Cutting Trait Impact

- **Modularity**: required — semantic index must not collapse storage/service boundaries.
- **Accessibility**: affected — retrieval diagnostics should be readable to operators.
- **Configurability**: required — model/backend and index policies must be explicit.
- **Data Logging**: required — retrieval provenance and index lifecycle should be inspectable.
- **Rewind/Replay**: affected — index rebuild and retrieval behavior should be reproducible.
- **Composability**: required — integrate with existing search and context pathways.

# Implementation Steps (The How)

1. **Contract pass**: define semantic record/query contracts, embedding profile fields, rebuild triggers, hybrid ranking policy, and backend seam.
2. **Index pass**: implement embedding/indexing lifecycle with migrations.
3. **Retrieval pass**: add hybrid lexical+semantic retrieval strategy.
4. **Inspection pass**: expose retrieval provenance and diagnostics.

# Acceptance Criteria

1. [ ] Semantic index can ingest and retrieve from core runtime entities.
2. [ ] Hybrid retrieval improves relevant-context recall over lexical-only baseline.
3. [ ] Retrieval/index operations are inspectable and test-covered.
4. [ ] Local-first operation remains viable without mandatory cloud dependencies.

# Verification & Regression

- **Unit Test**: index write/read and ranking logic.
- **Integration Test**: end-to-end ingest and retrieval for representative scenarios.
- **Smoke Check**: local semantic query against seeded data.
- **Invariants**: retrieval output includes provenance links to durable inputs.
