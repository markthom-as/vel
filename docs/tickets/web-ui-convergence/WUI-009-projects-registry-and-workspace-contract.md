---
title: Build Projects registry and workspace projection contract
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-001-shell-ia-and-route-ownership.md
  - WUI-002-transport-decoder-and-query-boundaries.md
labels:
  - projects
  - backend
  - api
---

# Goal

Create the durable backend contract that makes Projects a first-class surface instead of a free-text filter.

## Scope

- project registry
- project workspace projection
- task/session DTOs for project detail

## Requirements

1. Projects become a registry, not only a commitment string field.
2. Commitment-backed task data is exposed through explicit project DTOs.
3. Session/outbox/feedback abstractions are modeled as operator-facing workspace data, not raw transcript rows.
4. The projection contract is shared enough to support both web and CLI surfaces.

## Write scope

- storage and service-layer project/session contracts
- project APIs and DTOs
- tests and docs for the workspace projection

## Acceptance criteria

- the frontend can build Projects from one coherent workspace contract
- tasks stay commitment-backed
- agent sessions are first-class operator objects
