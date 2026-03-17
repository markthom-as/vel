---
title: Expose dependency and blocker projection in project workspaces
status: ready
owner: agent
priority: P0
area: projects
depends_on:
  - 02-project-storage-and-workspace-contract.md
---

# Goal

Use the existing `commitment_dependencies` substrate to make blocked and waiting work visible inside Projects instead of treating dependencies as hidden backend-only data.

## Scope

- project workspace task payload fields for `blocked_by`, `blocking`, and waiting state
- project index summary counts for blocked or dependency-pressured work
- project/task explainability for blocker relationships

## Requirements

- reuse `commitment_dependencies`; do not create a second dependency graph
- show whether a blocker is inside or outside the current project
- preserve dependency type information
- make blocked counts available on the project index without frontend graph reconstruction

## Suggested write scope

- workspace projector and DTO updates
- project APIs
- explain/debug payloads where needed
- tests for mixed in-project and cross-project blockers

## Acceptance criteria

- project task rows can render blocker/waiting state from one coherent contract
- project summaries surface blocked pressure
- dependency behavior is deterministic and explainable
