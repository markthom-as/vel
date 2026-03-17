---
title: Extend project registry with source mappings and sync policies
status: ready
owner: agent
priority: P0
area: projects
depends_on:
  - 01-project-boundary-and-registry.md
  - 02-project-storage-and-workspace-contract.md
---

# Goal

Make the project registry carry the real substrate needed for multi-source project operations, not just a slug and display name.

## Scope

- extend `projects` records for owner, type, repo/url, local workspace hints, transcript match rules, and sync policy fields
- document which fields are durable operator truth versus source hints
- expose typed DTO fields needed by web, CLI, and ingest/project-linking services

## Requirements

- keep `slug` canonical
- external mapping fields stay optional
- local path/workspace hints are not treated as authoritative existence proof
- sync participation policy is explicit and inspectable
- transcript/project keyword mapping rules live in backend state, not only frontend config

## Suggested write scope

- project registry schema and migrations
- `vel-api-types` project DTOs
- project service projector
- docs/specs for project substrate

## Acceptance criteria

- project registry can represent the same practical metadata class proven useful in local workspace tooling
- web/CLI do not need ad hoc side config to understand project source mappings
- source mapping and sync behavior are explainable through typed contracts
