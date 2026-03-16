---
id: VSM-002
title: Patch Proposal Schema
status: proposed
priority: P0
owner: platform
labels: [self-modification, schema, audit]
---

## Summary
Define the canonical typed object for all self-modification proposals.

## Why
If Vel is allowed to mutate anything, the unit of change cannot be “a vibe.” It must be a first-class proposal object with scope, evidence, validation plan, approval path, and rollback strategy.

## Scope
- Add proposal schema with versioning.
- Include trigger evidence, diagnosis, confidence, novelty, target paths, risk class, change summary, validation plan, approval requirement, rollout strategy, and rollback strategy.
- Include links to generated diff artifacts and logs.

## Suggested fields
- `id`
- `schema_version`
- `created_at`
- `trigger`
- `evidence_refs`
- `diagnosis`
- `confidence`
- `novelty`
- `scope`
- `target_paths`
- `risk_class`
- `change_plan`
- `validation_plan`
- `approval_state`
- `rollout_strategy`
- `rollback_strategy`
- `artifacts`
- `status`

## Implementation tasks
1. Define schema in the project’s typed language and serialization format.
2. Add validator and migration/version support.
3. Add stable ID generation.
4. Add persistence hooks for storage and retrieval.
5. Add fixtures for common proposal classes.

## Acceptance criteria
- Proposal schema is versioned and serializable.
- Invalid proposals fail validation with clear errors.
- Proposal IDs are stable and sortable enough for operator workflows.
- Schema can represent blocked, approved, applied, and rolled-back states.

## Dependencies
- VSM-001 for risk metadata.

