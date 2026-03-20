# Semantic Memory Contract

This document defines the Phase 4 semantic-memory contract and the currently shipped baseline.

## Current Status

Implemented today:

- capture ingestion writes deterministic local semantic records alongside lexical search state
- retrieval uses a local token-overlap backend with explicit `embedding_model` and `embedding_revision`
- context generation emits `search_executed` run events with retrieval provenance and combined scores

Still planned:

- artifact, thread, and message indexing beyond capture-backed records
- pluggable embedding/vector backends beyond the local deterministic baseline
- broader operator-facing retrieval diagnostics and ranking controls

## Purpose

Semantic memory extends lexical search with embedding-backed retrieval while preserving Vel's provenance and local-first trust model.

## Core Contracts

- `SemanticMemoryRecord`: persisted semantic chunk with explicit source linkage and embedding revision.
- `SemanticQuery`: caller-supplied retrieval request with explicit strategy and optional hybrid policy.
- `SemanticHit`: retrieval result carrying lexical, semantic, and combined scores plus provenance.
- `RecallContextPack`: bounded recall-oriented context bundle built from semantic hits, explicit source counts, and provenance-bearing snippets for assistant/context assembly.

## Hard Rules

- every semantic record must point back to durable runtime entities through provenance
- embedding revision and model identity must be explicit so rebuilds are inspectable
- hybrid retrieval policy must be configured explicitly rather than hidden in scorer code
- retrieval output must remain explainable from lexical score, semantic score, and provenance
- recall-oriented context assembly must reuse semantic hits and provenance instead of inventing assistant-only memory state

## Published Artifacts

- schema: `config/schemas/semantic-query.schema.json`
- schema: `config/schemas/semantic-memory-record.schema.json`
- example: `config/examples/semantic-query.example.json`
- example: `config/examples/semantic-memory-record.example.json`
- template: `config/templates/semantic-query.template.json`
- template: `config/templates/semantic-memory-record.template.json`
