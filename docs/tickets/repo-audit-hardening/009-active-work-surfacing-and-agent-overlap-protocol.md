---
title: Active-work surfacing and agent overlap protocol
status: done
owner: agent
type: process
priority: medium
created: 2026-03-17
depends_on:
  - 003-ticket-pack-schema-and-maturity-normalization.md
labels:
  - vel
  - process
  - coordination
---

Make in-process work easier for overlapping agents to discover without overloading `status.md`.

## Scope

- active-work links from README and docs entrypoints
- conventions for marking ticket ownership and current status
- lightweight overlap notes for ongoing concurrent work

## Protocol

Use these surfaces in order:

1. [README.md](../../../README.md) for the current operator-facing active work pointer
2. [docs/README.md](../../README.md) for the active-plan entrypoints
3. [docs/tickets/README.md](../README.md) to choose the right pack and identify overlap risk
4. the pack `README.md` for current status and active focus within that lane

Agent overlap rules:

- do not treat `docs/status.md` as a scratchpad for in-flight work
- use pack READMEs for active lane state and current focus
- add or update a `Current focus` or `Current status` section when a pack has meaningful concurrent work
- keep ownership notes lightweight and avoid duplicating full changelogs in ticket docs
- if two packs overlap, the ticket index should say so explicitly and point to the dominant pack or caution note

## Completed outputs

- root and docs entrypoints link to the active hardening lane
- the ticket index includes overlap and caution guidance
- active packs now expose current status or current focus sections where that guidance is needed
