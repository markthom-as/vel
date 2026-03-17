---
title: Architecture Spec Draft
doc_type: spec
status: draft
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - architecture
  - draft
  - spec
index_terms:
  - spec draft
  - architecture draft
  - default spec path
related_files:
  - docs/MASTER_PLAN.md
  - docs/templates/spec-template.md
  - docs/cognitive-agent-architecture/architecture/README.md
summary: Default landing document for new cross-subsystem architecture spec drafts before they are renamed into durable docs.
---

# Purpose

This file exists as the default draft destination for new architecture specs created before a precise file name has been chosen.

# Problem

The repo now routes new architecture spec drafts into `docs/cognitive-agent-architecture/architecture/`, but that path needs a concrete starter file so agents and contributors land in a real resource instead of a dead destination.

# Goals

- provide a valid spec scaffold at the default draft path
- make placement rules explicit for architecture-vs-ticket work
- encourage renaming or copying this file into a focused durable spec

# Non-Goals

- acting as the canonical source of shipped behavior
- replacing the Master Plan or execution tickets

# Current State

Current shipped truth lives in [MASTER_PLAN.md](../../MASTER_PLAN.md).

This file is only a drafting scaffold. Its presence does not imply that any described behavior is implemented.

# Proposed Design

If you are starting a new cross-subsystem architecture doc:

1. use this file as the initial scaffold
2. rename or copy it to a focused file name such as `authority-runtime-contracts.md`
3. update links from [README.md](README.md) or the parent architecture pack when the new doc becomes durable

If the work is implementation-specific, create or update a ticket under `docs/tickets/phase-*/` instead.

## Concepts

- **architecture draft**: a non-authoritative design doc that clarifies boundaries, contracts, or control flow
- **durable architecture doc**: a named spec that should remain discoverable after the current implementation task ends
- **execution ticket**: a bounded implementation work item with acceptance and verification criteria

## Behavior

This file should stay lightweight and reusable. Do not turn it into a second master plan or a catch-all dumping ground.

## Data Model Or Contracts

Use this section only when the draft needs to describe concrete interfaces, data shapes, or invariants.

## Boundaries

- `docs/MASTER_PLAN.md` owns shipped-truth and queue-shape authority
- `docs/tickets/phase-*/` owns implementation work
- `docs/cognitive-agent-architecture/architecture/` owns durable cross-subsystem architecture docs

# Cross-Cutting Traits

- modularity: required — drafts should identify ownership seams before proposing shared abstractions
- accessibility: affected — drafts should remain searchable and navigable through the architecture pack
- configurability: n/a — this scaffold itself does not add runtime configuration
- data logging and observability: n/a — this scaffold itself does not define execution behavior
- rewind/replay: n/a — this scaffold itself does not define sequence-sensitive flows
- composability: required — new architecture docs should define reusable contracts rather than one-off coupling

# Operational Considerations

- rename or copy this file before treating the draft as durable
- keep the parent `README.md` updated when new architecture docs become authoritative references

# Acceptance Criteria

1. The default architecture draft path exists and is valid.
2. The file explains when to use an architecture doc versus an implementation ticket.
3. The parent architecture sub-pack can point to this file without dead links.

# Open Questions

- Should future doc generation use a machine-readable manifest for draft destinations?
- Should sub-pack readmes eventually exist for every major architecture subtree?

# Related Terms

- canonical name: architecture spec draft
- aliases: spec draft, default architecture draft
- related packs or subsystems: architecture pack, ticket queue, master plan

# Search Terms

- architecture draft
- spec draft
- default spec path
