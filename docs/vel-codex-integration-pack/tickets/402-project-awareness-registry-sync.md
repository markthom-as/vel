---
title: Ticket 402 - Import Codex project registry into stable project identities
status: proposed
owner: codex
priority: critical
---

# Goal

Use Codex Workspace project registry as the canonical naming layer for Vel project awareness.

# Grounding

Codex Workspace already has:
- `schemas/project-registry.md`
- `schemas/project-schema.md`
- `schemas/todoist-obsidian-alignment.md`

Vel currently stores project mostly as an optional freeform string on commitments.

That is too weak.

# Files

## New
- `crates/veld/src/services/project_awareness.rs`

## Changed
- `crates/veld/src/services/integrations.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/routes/integrations.rs`
- `crates/vel-storage/src/db.rs`

# Implementation

## Parse `schemas/project-registry.md`
Write a parser that:
- reads markdown table rows
- extracts project display name, brain path, status, type, owner, sync flags
- computes `slug`
- stores aliases:
  - display name
  - todoist name
  - derived folder basename

Do not require perfect markdown AST support. A disciplined table parser is enough.

## Project matching helper
Implement `match_project_identity(input: &str) -> Option<ProjectIdentity>` with:
- exact slug match
- exact display-name case-insensitive match
- exact alias match
- optional normalized punctuation-insensitive match

Do not do fuzzy AI matching here. Deterministic first.

# Route/API surface

Add:
- `GET /v1/projects`
- `GET /v1/projects/:slug`
- `POST /v1/integrations/projects/reload`

# Current-context usage

Update context generation so it can produce:
- active projects
- drifting projects
- projects with overdue commitments
- projects with upcoming calendar blocks

# Acceptance criteria

- project names across Todoist, calendar, and context resolve to a stable internal slug
- operator can inspect project registry through API
- context and suggestion systems no longer rely on raw string equality alone
