---
title: Project, Action, and Linking Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-18
updated: 2026-03-18
keywords:
  - project
  - action
  - linking
  - phase-5
summary: Canonical Phase 05 contract vocabulary for typed projects, surfaced operator action items, review counts, and scoped linking records.
---

# Purpose

Publish the stable Phase 05 vocabulary before storage, service, and client slices widen around it.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Project substrate | `vel-core` | `crates/vel-core/src/project.rs` |
| Operator action/intervention queue | `vel-core` | `crates/vel-core/src/operator_queue.rs` |
| Linking and pairing scopes | `vel-core` | `crates/vel-core/src/linking.rs` |
| Transport DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |

# Stable Vocabulary

## Project families

The family vocabulary is fixed for Phase 05:

- `Personal`
- `Creative`
- `Work`

These are project families, not projects. They are stable grouping metadata shared across web, CLI, Apple, and later sync/bootstrap surfaces.

## Action kinds

The surfaced operator vocabulary is fixed for Phase 05:

- `next_step`
- `intervention`
- `review`
- `freshness`
- `blocked`
- `conflict`
- `linking`

`Now` consumes the small ranked set. `Inbox` consumes the broader triage queue. Both surfaces share the same backend-owned action vocabulary.

## Review snapshot counts

Review counts move through one named contract:

- `open_action_count`
- `triage_count`
- `projects_needing_review`

This snapshot exists to keep daily/weekly review surfaces typed and inspectable instead of ad hoc count blobs.

## Linking scopes

Guided linking uses exactly these scope fields:

- `read_context`
- `write_safe_actions`
- `execute_repo_tasks`

These scopes must be shown explicitly during pairing so operators can see what a linked node may read, write, or execute.

# Contract Rules

- Do not deepen `commitment.project: Option<String>` into the long-term project boundary.
- Do not append opaque JSON placeholders to `Now`, `Inbox`, or sync/bootstrap for Phase 05 project or action state.
- Keep linking local, scoped, and fail-closed. Pairing records describe access; they do not hand out broad long-lived credentials.
- Treat action/intervention evidence as first-class. Every surfaced item should remain explainable from persisted inputs or rules.

# Published Artifacts

- `config/schemas/project-workspace.schema.json`
- `config/examples/project-workspace.example.json`
- `config/schemas/operator-action-item.schema.json`
- `config/examples/operator-action-item.example.json`

# Downstream Usage

- Storage and service slices should persist or project these contracts directly, not invent parallel names.
- Route handlers should map domain contracts to DTOs at the boundary.
- Clients may cache these records, but ranking and durable policy stay in Rust backend layers.
