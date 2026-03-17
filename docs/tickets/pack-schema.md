---
title: Vel ticket-pack schema
status: active
owner: agent
created: 2026-03-17
updated: 2026-03-17
---

# Vel Ticket-Pack Schema

This document defines the shared schema for ticket-pack READMEs under [docs/tickets/](README.md).

It does not replace [docs/status.md](../status.md) as the shipped-behavior ledger.

Use this schema to make pack maturity, authority, and execution intent explicit.

## Required pack metadata

Every maintained ticket-pack README should declare:

- `title`
- `status`
- `owner`
- `class`
- `authority`
- `status_model`
- `source_of_truth`
- `created`
- `updated`

## Field meanings

### `class`

Allowed values:

- `convergence`
- `expansion`
- `speculative`

Meaning:

- `convergence`: tighten, repair, or finish the current system
- `expansion`: add a bounded new capability to the current system
- `speculative`: future-facing planning that should not outrank current convergence

### `authority`

Allowed values:

- `execution`
- `design`
- `historical`

Meaning:

- `execution`: use this pack to drive near-term implementation work
- `design`: use this pack for architectural direction and bounded planning
- `historical`: keep for context, imported planning, or prior review state

### `status_model`

List the allowed local ticket-status words for the pack.

Preferred normalized sets:

- execution packs: `todo | in_progress | done | deferred`
- design packs: `proposed | active | deferred | done`
- historical packs: `archived`

If a pack has stronger reasons to use a different vocabulary, the README should say why.

### `source_of_truth`

Usually:

- `docs/status.md` for shipped behavior

If a pack uses a different authority for its own scope, that should be stated explicitly without implying shipped runtime truth moved there.

## Required README sections

Every maintained pack README should include:

1. Purpose
2. Why this pack exists
3. Pack schema
4. Entry criteria
5. Exit criteria

Optional but recommended:

- execution order
- current status
- related specs
- cautions and boundaries

## Pack classification guidance

### Convergence

Use for:

- repo truth
- boundary cleanup
- hermeticity
- correctness
- active simplification

Do not use for:

- broad product expansion
- distant future architecture without immediate execution pressure

### Expansion

Use for:

- bounded new surfaces
- new subsystems that extend the current runtime
- implementation-ready product growth

Do not use for:

- unfinished convergence cleanup masquerading as new features

### Speculative

Use for:

- imported packets
- long-range design ideas
- future system sketches that still require reconciliation

Do not use as:

- justification to bypass current repo truth or convergence priorities

## Current normalization targets

These packs should be treated as the main reference examples:

- [repo-audit-hardening/README.md](repo-audit-hardening/README.md): convergence + execution
- [repo-feedback/README.md](repo-feedback/README.md): convergence + execution
- [multi-client-swarm/README.md](multi-client-swarm/README.md): expansion + design
- [agent-runtime/README.md](agent-runtime/README.md): expansion + design
- [full-spec-pack/README.md](full-spec-pack/README.md): speculative + historical

## Enforcement rule

If a pack README lacks schema fields or uses ambiguous status language, prefer the pack index plus [docs/status.md](../status.md) over the pack itself until it is normalized.
