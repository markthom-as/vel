---
title: Ticket-pack schema and maturity normalization
status: in_progress
owner: agent
type: documentation
priority: high
created: 2026-03-17
depends_on:
  - 001-docs-truth-repair-and-entrypoint-alignment.md
labels:
  - vel
  - tickets
  - docs
---

Define and roll out a shared schema for active, expansion, and speculative ticket packs.

## Scope

- required pack metadata
- allowed status words per pack
- class and authority framing
- audit of existing pack READMEs for consistency

## Acceptance criteria

- active packs clearly state class, authority, and source of truth
- ticket-pack index can classify packs consistently
- pack status language no longer varies arbitrarily where it causes confusion

## Current normalization artifact

- [docs/tickets/pack-schema.md](../pack-schema.md)

Initial normalization target set:

- [docs/tickets/README.md](../README.md)
- [docs/tickets/repo-feedback/README.md](../repo-feedback/README.md)
- [docs/tickets/multi-client-swarm/README.md](../multi-client-swarm/README.md)
- [docs/tickets/agent-runtime/README.md](../agent-runtime/README.md)
- [docs/tickets/full-spec-pack/README.md](../full-spec-pack/README.md)
