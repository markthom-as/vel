---
title: Ticket 501 - Upgrade suggestion engine to use project, task, and calendar evidence
status: proposed
owner: codex
priority: critical
---

# Goal

Turn suggestions from repeated-pattern heuristics into evidence-bearing, project-aware proposals.

# Current issue

Existing `services/suggestions.rs` only checks for repeated nudge patterns and inserts a thin pending suggestion.

That is a seed, not an engine.

# Files

## Changed
- `crates/veld/src/services/suggestions.rs`
- `migrations/0017_suggestions.sql` via forward migration
- `crates/vel-storage/src/db.rs`

## New migration
- `migrations/0032_suggestion_evidence.sql`

# Implementation

## Add suggestion candidate type
Suggested fields:
- `suggestion_type`
- `summary`
- `project_slug`
- `priority_score`
- `confidence`
- `evidence_ids`
- `proposal_kind`
- `payload_json`

## Candidate families
Implement at least:
- schedule adjustment
- project drift recovery
- overdue-task triage
- commitment clarification
- task labeling / project mapping repair
- prep window adjustment

## Ranking signals
Use:
- overdue count
- next-event proximity
- project inactivity window
- repeated unresolved uncertainty
- historical acceptance / rejection feedback

# Acceptance criteria

- suggestions cite evidence and project context
- suggestion ranking is deterministic and testable
- suggestions can create sync proposals or questions rather than only inert rows
