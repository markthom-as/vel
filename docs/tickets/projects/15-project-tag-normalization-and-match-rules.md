---
title: Add normalized tag substrate and project match rules
status: ready
owner: agent
priority: P0
area: projects
depends_on:
  - 02-project-storage-and-workspace-contract.md
  - 05-project-task-tagging-and-filters.md
---

# Goal

Turn tags into a typed project/task substrate and make project matching rules explicit for transcript and external-source ingestion.

## Scope

- normalize commitment/project task tags into explicit typed DTO fields
- preserve source-native labels in structured metadata when needed
- add project keyword/tag match rules for transcript/session linking
- expose explainable matching provenance

## Requirements

- canonical operator tag slug is deterministic
- duplicate tags collapse predictably
- provider-native labels are preserved when round-trip fidelity matters
- project linking preference order is explicit:
  1. operator link
  2. source-native mapping
  3. configured rule
  4. explainable inference

## Suggested write scope

- commitment/project projection helpers
- transcript/session project-linking services
- typed DTO updates for tags and match provenance
- explain/debug payloads where links are surfaced

## Acceptance criteria

- tasks and project workspaces expose normalized tags directly
- transcript/session project matches can cite the rule or source that caused the link
- tag and match-rule behavior is documented and test-covered
