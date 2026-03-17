---
title: Semantic Memory & Graph RAG Implementation
status: planned
owner: staff-eng
type: feature
priority: medium
created: 2026-03-17
updated: 2026-03-17
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

# Cross-Cutting Trait Impact

- **Modularity**: required — semantic index must not collapse storage/service boundaries.
- **Accessibility**: affected — retrieval diagnostics should be readable to operators.
- **Configurability**: required — model/backend and index policies must be explicit.
- **Data Logging**: required — retrieval provenance and index lifecycle should be inspectable.
- **Rewind/Replay**: affected — index rebuild and retrieval behavior should be reproducible.
- **Composability**: required — integrate with existing search and context pathways.

# Implementation Steps (The How)

1. **Contract pass**: define semantic record/query contracts and backend seam.
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
