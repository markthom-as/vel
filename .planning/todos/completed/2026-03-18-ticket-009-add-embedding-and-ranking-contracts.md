---
created: 2026-03-18T07:25:40.260Z
title: Ticket 009 - add embedding model and hybrid ranking contracts
area: docs
files:
  - docs/tickets/phase-4/009-semantic-memory-rag.md
---

## Problem

Ticket 009 (Semantic Memory & Graph RAG) specifies sqlite-vec as the vector backend (confirmed) but leaves three implementation contracts undefined that affect the storage schema and retrieval design:

1. **Embedding model**: Who generates embeddings? Which model? At what dimensions? The sqlite-vec index schema is parameterized by vector dimensions — this must be decided before the migration is written.

2. **Index rebuild triggers**: When does a capture/artifact get embedded? On insert (synchronous)? Via a background job (processing_jobs table)? On-demand retrieval? The answer affects the storage design and the `processing_jobs` job types.

3. **Hybrid ranking policy**: How are lexical (FTS5) scores and semantic (cosine similarity) scores blended? What are the relative weights? This is the retrieval policy contract that needs to be explicit before the retrieval service is built.

## Solution

Add a "Contract Decisions" section to ticket 009 covering these three points before Phase 4 SP1 begins. The embedding model should route through `vel-llm` for consistency with the local-first approach. Embedding generation should use the `processing_jobs` queue (consistent with existing capture ingest pattern).
