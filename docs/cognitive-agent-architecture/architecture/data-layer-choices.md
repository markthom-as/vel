---
title: Data Layer Choices
doc_type: guide
status: complete
owner: staff-eng
created: 2026-03-25
updated: 2026-03-25
keywords:
  - data
  - storage
  - layers
  - rust
summary: Concise helper guide for where data shape, persistence rules, DTOs, and read models belong in Vel.
---

# Purpose

Give agents and contributors a fast answer to one recurring question:

Where should this data shape live?

# Canonical Layer Split

## `vel-core`

Put data here when it expresses:

- domain meaning
- invariants
- identity
- cross-surface business semantics

Examples:

- prefixed IDs
- domain enums
- durable object meanings
- run lifecycle semantics

Do not put transport DTOs or storage-specific row shapes here.

## `vel-storage`

Put data here when it expresses:

- persistence schema
- repository records
- transaction boundaries
- migration-backed storage concerns

Examples:

- row structs
- repository helpers
- storage-only denormalized fields

Do not make storage the owner of API contracts.

## `vel-api-types`

Put data here when it expresses:

- transport DTOs
- request and response shapes
- boundary serialization contracts

Examples:

- route payloads
- typed API responses
- client-consumed transport models

Do not let DTO needs leak down into domain or storage ownership.

## `veld` services

Put logic here when it expresses:

- orchestration
- application flow
- multi-repository coordination
- policy application over domain and storage seams

Services may map between layers, but they should not collapse all layers into one.

## Clients

Put data here when it expresses:

- presentation state
- view-specific derivation
- shell-local interaction state

Clients should not become the hidden owner of durable product semantics.

# Quick Decision Rules

If you are unsure, ask:

1. Is this true even without HTTP, SQLite, or React?
   If yes, it probably belongs in `vel-core`.
2. Is this only about how it is stored?
   If yes, it belongs in `vel-storage`.
3. Is this only about what crosses the wire?
   If yes, it belongs in `vel-api-types`.
4. Is this coordinating multiple repos or policies?
   If yes, it belongs in a `veld` service.
5. Is this only about rendering or local interaction?
   If yes, it belongs in a client shell.

# Read Models Versus Canonical Data

Vel should keep canonical data separate from rebuildable views.

- canonical data: durable truth with ownership and provenance
- read model: optimized, rebuildable, presentation-friendly or query-friendly projections

Do not mistake a convenient read model for canonical truth.

# JSON Guidance

- prefer typed structs over unbounded JSON when the shape is known
- if structured dynamic data is necessary, prefer `serde_json::Value` over raw JSON strings
- keep JSON serialization at the boundary where possible
- do not deepen `current_context` or similar seams as catch-all blobs

# Anti-Patterns

- route handlers returning service-owned business objects as if they were already DTOs
- storage repositories returning `vel-api-types`
- clients inventing local truth because the backend seam is inconvenient
- new “universal metadata” blobs that bypass typed ownership
- adding unrelated persistence behavior back into `db.rs` when a focused repo exists

# Agent Checklist

Before adding a data shape, check:

1. which layer owns the meaning
2. which layer owns persistence
3. which layer owns transport
4. whether the shape is canonical truth or a rebuildable read model
5. whether a nearby typed shape already exists
