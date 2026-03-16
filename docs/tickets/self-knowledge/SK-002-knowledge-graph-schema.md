---
id: SK-002
title: Create self-knowledge graph schema and query layer
status: proposed
priority: P0
owner: nav-core
area: self-knowledge
last_updated: 2026-03-16
---

# Goal

Create the normalized graph-backed system model that represents entities and relationships across code and docs.

# Scope

Entity types:

- Repository
- Package
- Module
- File
- Symbol
- Function
- Struct
- Class
- Interface
- Test
- Command
- DatabaseTable
- Migration
- Document
- ADR
- Ticket

Relationship types:

- imports
- exports
- depends_on
- implements
- documents
- tested_by
- supersedes
- references
- affects

# Tasks

1. Design SQLite schema for entities and relationships.
2. Add stable entity IDs and source location mapping.
3. Implement graph hydration from parsed artifacts.
4. Add a query service for dependency and documentation traversal.
5. Add reverse-edge queries and basic entity summaries.

# Acceptance Criteria

- Given a module, the system can list directly related docs, tests, and dependencies.
- Relationship queries are available via internal API.
- Entity summaries include source file, type, and basic connected artifacts.
- Graph rebuild is deterministic from the same repo state.

# Deliverables

- schema migration(s)
- graph service module
- internal query API

