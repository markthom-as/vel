---
id: VEL-PROJ-001
title: Project registry and shared workspace view-model contracts
status: proposed
priority: P0
estimate: 2-3 days
dependencies: []
labels:
  - projects
  - api-types
  - architecture
---

# Goal

Establish stable project identity and the typed contracts that every surface will consume.

# Why

Right now Vel has project-ish fragments (`commitment.project`, Todoist project names, transcript `project_hint`), but no stable identity model. Without this, a Projects page will collapse into brittle string matching and UI folklore.

# Scope

- Add canonical project and workspace DTOs in `vel-api-types`.
- Add core enums/types for:
  - project status
  - project source refs
  - project health summary
  - work item projection
  - agent session summary
  - queued message summary
  - capability flags
- Define project slug normalization and alias rules in a shared module.
- Document mapping precedence:
  1. explicit project registry link
  2. Todoist project id/name
  3. commitment `project`
  4. transcript `metadata.project_hint`
  5. codex-workspace tag inference

# Deliverables

- `crates/vel-api-types/src/lib.rs` additions for projects/session/outbox DTOs
- `crates/vel-core` or `crates/veld` shared project slug normalization module
- JSON fixture examples for project workspace payloads
- dev doc describing project identity resolution and confidence semantics

# Acceptance criteria

- A `ProjectWorkspaceData` contract exists and is renderer-neutral.
- Project identity resolution rules are explicit and deterministic.
- DTOs cover both web and CLI requirements.
- JSON round-trip tests exist for the new DTOs.
- No write-path assumptions are embedded in read contracts.

# Notes for agent

Keep the registry narrow. This is identity and projection plumbing, not permission to invent a grand unified theory of projects.
