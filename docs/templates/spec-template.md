---
title: Feature Or System Spec
doc_type: spec
status: proposed
owner: team-or-person
created: YYYY-MM-DD
updated: YYYY-MM-DD
keywords:
  - subsystem
  - architecture
  - feature
index_terms:
  - alternate subsystem name
  - acronym
  - likely design lookup phrase
related_files:
  - docs/MASTER_PLAN.md
  - docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md
  - docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md
summary: One or two sentences describing the system or behavior this spec defines.
---

# Purpose

Describe what this spec defines and why it exists.

# Problem

Describe the current gap, inconsistency, or need.

# Goals

- goal
- goal
- goal

# Non-Goals

- non-goal
- non-goal

# Current State

Describe the current shipped reality.

Link to [MASTER_PLAN.md](../MASTER_PLAN.md) when rollout truth matters.

# Proposed Design

If this is a durable architecture or concept document, place it under `docs/cognitive-agent-architecture/`.

If it is implementation work, prefer a ticket under `docs/tickets/phase-*/`.

## Concepts

Define the main entities, terms, and invariants.

## Behavior

Describe the expected system behavior.

## Data Model Or Contracts

Include fields, API shapes, event shapes, or storage rules as needed.

## Schema And Manifest Artifacts

If this proposal introduces or changes shared contracts, list:

- machine-readable schema or manifest artifacts
- canonical template or fixture artifacts
- versioning and migration policy
- owning ticket(s) for publication and fixture parity

## Boundaries

State what layer owns what.

# Cross-Cutting Traits

State how this proposal affects:

- modularity
- accessibility
- configurability
- data logging and observability
- rewind/replay
- composability

Mark traits as required, affected, or not applicable with one short sentence each.

# Operational Considerations

- observability
- failure modes
- rollout constraints
- migration concerns

# Acceptance Criteria

1. criterion
2. criterion
3. criterion

# Open Questions

- question
- question

# Related Terms

- canonical name:
- aliases:
- related packs or subsystems:

# Search Terms

- keyword:
- keyword:
- alternate phrase:
