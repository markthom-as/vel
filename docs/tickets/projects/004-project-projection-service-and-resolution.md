---
id: VEL-PROJ-004
title: Build project projection service from commitments, Todoist, transcripts, and codex tags
status: proposed
priority: P0
estimate: 3-4 days
dependencies:
  - VEL-PROJ-001
  - VEL-PROJ-002
labels:
  - projects
  - inference
  - projection
---

# Goal

Create the read-side service that resolves projects and assembles a canonical workspace from existing sources.

# Scope

Implement a project projection service that:

- lists projects with stable slugs and summary health
- resolves work items from commitments
- associates Todoist project data
- links transcript sessions via `project_hint` and alias matching
- optionally incorporates codex-workspace tag conventions described in `docs/specs/vel-addendum-calendar-todoist-workflows.md`
- computes lightweight confidence scores for project/session association

# Deliverables

- service module for project list and project workspace assembly
- project resolution helpers
- projection tests using mixed fixtures:
  - Todoist project name
  - commitment `project`
  - transcript `metadata.project_hint`
  - alias collision case
- health summary rules (e.g. overdue count, active chats, drift signal, last activity)

# Acceptance criteria

- A deterministic `list_projects` and `get_project_workspace` service exists.
- Project aliases and slug normalization are covered by tests.
- Work items derive from commitments rather than a new task table.
- Session association exposes confidence and source evidence.
- The service can power both web and CLI without UI-specific branching.

# Notes for agent

Keep the inference legible. Hidden fuzzy matching is how you end up debugging ontology by séance.
