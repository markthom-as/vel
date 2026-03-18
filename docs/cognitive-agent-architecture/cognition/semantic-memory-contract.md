# Semantic Memory Contract

This document defines the Phase 4 contract layer for semantic memory before runtime indexing is implemented.

## Purpose

Semantic memory extends lexical search with embedding-backed retrieval while preserving Vel's provenance and local-first trust model.

## Core Contracts

- `SemanticMemoryRecord`: persisted semantic chunk with explicit source linkage and embedding revision.
- `SemanticQuery`: caller-supplied retrieval request with explicit strategy and optional hybrid policy.
- `SemanticHit`: retrieval result carrying lexical, semantic, and combined scores plus provenance.

## Hard Rules

- every semantic record must point back to durable runtime entities through provenance
- embedding revision and model identity must be explicit so rebuilds are inspectable
- hybrid retrieval policy must be configured explicitly rather than hidden in scorer code
- retrieval output must remain explainable from lexical score, semantic score, and provenance

## Published Artifacts

- schema: `config/schemas/semantic-query.schema.json`
- schema: `config/schemas/semantic-memory-record.schema.json`
- example: `config/examples/semantic-query.example.json`
- example: `config/examples/semantic-memory-record.example.json`
- template: `config/templates/semantic-query.template.json`
- template: `config/templates/semantic-memory-record.template.json`
