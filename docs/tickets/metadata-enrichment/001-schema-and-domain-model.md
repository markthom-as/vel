---
id: VEL-META-001
title: Canonical metadata enrichment schema and domain model
status: proposed
priority: P0
estimate: 2-3 days
dependencies: []
---

# Goal

Introduce the core domain model for normalized metadata snapshots, gaps, candidates, actions, and preferences.

# Scope

- Add Rust domain types for:
  - source object reference
  - canonical metadata envelope
  - field quality state
  - gap record
  - enrichment candidate
  - enrichment action
  - preference/policy primitives
- Add serialization/deserialization tests.
- Define stable enums for object types, gap types, approval modes, and risk levels.

# Deliverables

- `vel-core/src/metadata/schema.rs`
- `vel-core/src/metadata/gaps.rs`
- `vel-core/src/metadata/candidate.rs`
- `vel-core/src/metadata/policy.rs`
- migration or persistence model scaffolding
- fixture examples in JSON

# Acceptance criteria

- Core types compile and are documented.
- All enums are forward-compatible.
- JSON fixtures round-trip cleanly.
- No source-specific assumptions leak into canonical types.

# Notes

Design for extensibility. Today it is tags and locations; tomorrow it is half the symbolic bureaucracy of a life.
