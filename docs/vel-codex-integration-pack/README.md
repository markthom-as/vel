---
title: Vel × Codex Workspace Integration Pack
status: proposed
owner: codex
generated_on: 2026-03-16
---

This pack is grounded in:
- the current Vel repo state
- the current Codex Workspace repo state
- the existing Vel run / signal / commitment / context architecture
- Codex Workspace canonical schemas for tasks, projects, calendar, and source-of-truth rules

## Grounding references

### Vel
- `migrations/0008_commitments.sql`
- `migrations/0009_signals.sql`
- `migrations/0012_current_context.sql`
- `migrations/0015_commitment_risk.sql`
- `migrations/0017_suggestions.sql`
- `crates/veld/src/adapters/todoist.rs`
- `crates/veld/src/adapters/calendar.rs`
- `crates/veld/src/services/integrations.rs`
- `crates/veld/src/services/suggestions.rs`

### Codex Workspace
- `schemas/unified-schema-map.md`
- `schemas/project-schema.md`
- `schemas/project-registry.md`
- `schemas/task-schema.md`
- `schemas/calendar-schema.md`
- `schemas/todoist-obsidian-alignment.md`
- `schemas/source-registry.md`
- `tools/todoist/src/cli.js`
- `tools/google-calendar/src/cli.js`

## Intent

Vel should **not** duplicate Codex Workspace.  
Vel should become the interpretation, synthesis, and action layer **over** those external systems.

That means:
- Todoist remains canonical for actionable tasks
- Google Calendar remains canonical for events
- Codex Workspace project registry remains canonical for project naming / taxonomy
- Vel stores normalized evidence, derived context, commitments, and proposals

## Recommended implementation order
1. 401 schema extensions
2. 400 source registry / ontology
3. 402 project awareness
4. 403 todoist hardening
5. 404 calendar hardening
6. 405 normalized item pipeline
7. 406 commitment linkage
8. 407 context enrichment
9. 408 proposed writeback
10. 409 surfaces
11. 410 tests and backfill
