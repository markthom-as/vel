---
title: Semantic Memory & Graph RAG Implementation
status: planned
owner: staff-eng
type: feature
priority: medium
created: 2026-03-17
labels:
  - vel-storage
  - vector-db
  - rag
---

Integrate a local Vector Database (e.g., LanceDB) into `vel-storage` to enable advanced semantic search and Graph-Augmented Retrieval (Graph RAG) for the Chat Assistant.

## Technical Details
- **Vector Storage**: Incorporate LanceDB as a storage backend within `vel-storage`.
- **Local Embedding**: Use the `candle` crate or similar for local text embedding generation within a background `EmbeddingWorker`.
- **Graph Integration**: Create links between semantic vector chunks and structured Thread/Artifact records.
- **Retrieval Engine**: Develop a retrieval strategy that combines lexical search (FTS5) with semantic vector lookups.

## Acceptance Criteria
- Artifacts and Captures are automatically indexed into the local vector DB.
- Chat Assistant can retrieve relevant context via semantic similarity.
- Search performance remains within acceptable latency (< 200ms for retrieval).
