---
id: INTG-006
title: Notes multi-vendor ingestion
status: proposed
priority: P1
estimate: 4-7 days
dependencies:
  - INTG-001
  - INTG-002
  - INTG-004
---

# Goal

Support Apple Notes, Obsidian, and filesystem notes under one canonical note/document ingestion contract.

# Scope

- Define provider-aware note document payloads.
- Preserve vault/notebook/folder provenance.
- Support file-backed and bridge-backed sources.
- Allow owner or mentioned-person linkage when source data exposes it.

# Deliverables

- canonical note object contract
- provider adapters or bridge interfaces
- migration path from current `notes_path`
- fixtures for Obsidian and Apple Notes imports

# Acceptance criteria

- Multiple note sources can coexist.
- Notes remain replay-safe and provenance-rich.
- People references from notes are not discarded into raw strings only.

# Notes

Obsidian and Apple Notes are not the same thing, but Vel should not need an entirely separate ontology for each.
