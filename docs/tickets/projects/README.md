---
title: Vel Projects page ticket pack
status: ready
owner: agent
created: 2026-03-16
---

# Vel Projects page ticket pack

This pack contains codex-ready markdown tickets for implementing a first-class Projects page and shared project workspace contract in Vel.

**Primary specs:**

- [docs/specs/vel-projects-page-spec.md](../../specs/vel-projects-page-spec.md)
- [docs/specs/vel-project-operations-substrate-spec.md](../../specs/vel-project-operations-substrate-spec.md)

## Repo-aware boundary notes

This pack assumes the current repo state already includes:
- `commitments` with `project` support
- Todoist sync that mirrors tasks into commitments
- transcript sync into `assistant_transcripts`
- web shell with Now / Inbox / Threads
- existing `/v1` and `/api` operator routes

Accordingly:
- do **not** introduce a second durable task authority unless explicitly justified
- do **not** treat `assistant_transcripts` as the final session/operator model
- do **not** make web and CLI invent separate workspace payloads

## Cross-pack dependencies

Depends on:

- current commitments/task substrate already present in the repo
- `integration-expansion/` when project workspace changes need provider/connection-aware integration metadata
- `task-hud/` when recurring ritual rendering or glance policies are being implemented on top of the shared routine substrate

Extended by:

- `connect-agent-launch/` for live launched-session metadata, instance/runtime-aware session fields, and launch-driven session controls

Overlap rule:

- this pack owns the shared project workspace contract and the base session/control-plane model
- this pack also owns the backend planning for project registry source mappings, project-facing tag normalization, dependency projection, and project-affine routine substrate
- Connect-backed launch work should extend that substrate rather than invent a parallel session or message-control model
- `task-hud/` should consume routine/dependency outputs for ranking and glance behavior rather than inventing a competing project/task backend

## Parallel lanes

Use these lanes when splitting work across agents or contributors:

### Lane A — Registry and typed contracts

- 01 — boundary + project registry foundation
- 02 — storage, migrations, DTO contracts
- 14 — registry source mappings and sync policies

### Lane B — Workspace projection and task semantics

- 03 — workspace projection service + APIs
- 05 — task tagging and operator ergonomics
- 15 — normalized tags and project match rules
- 16 — dependency and blocker projection

### Lane C — Task write-through and provider mutation

- 04 — Todoist write-through project/task mutations

### Lane D — Session and control plane

- 06 — agent session registry
- 07 — outbox, steering, feedback, session settings
- 08 — websocket/event contract

### Lane E — Surface consumers and rollout

- 09 — web Projects page shell
- 10 — web Tasks workspace
- 11 — web Chats/Agents workspace
- 12 — CLI project workspace
- 13 — tests, fixtures, docs, rollout

### Lane F — Routine follow-on

- 17 — routine definitions and project-affine anchors
- 18 — routine-aware schedule blocks and planning policy

Lane gates:

- start Lane B after Lane A has stabilized the registry and DTO boundaries
- Lane C can begin once Lane A defines source mappings and write-through policy
- start Lane D after Lane A has settled the shared project/session contract
- start Lane E after Lane B and Lane D expose stable projection/session payloads
- keep Lane F behind Lanes A and B; do not let routines invent a parallel planning or dependency authority

## Included
- 01 — boundary + project registry foundation
- 02 — storage, migrations, DTO contracts
- 03 — workspace projection service + APIs
- 04 — Todoist write-through project/task mutations
- 05 — task tagging and operator ergonomics
- 06 — agent session registry
- 07 — outbox, steering, feedback, session settings
- 08 — websocket/event contract
- 09 — web Projects page shell
- 10 — web Tasks workspace
- 11 — web Chats/Agents workspace
- 12 — CLI project workspace
- 13 — tests, fixtures, docs, rollout
- 14 — registry source mappings and sync policies
- 15 — normalized tags and project match rules
- 16 — dependency and blocker projection
- 17 — routine definitions and project-affine anchors
- 18 — routine-aware schedule blocks and planning policy

## Recommended implementation order
1. Establish project registry and typed contracts.
2. Build read-only project index/workspace projection.
3. Add commitment-backed task mutations and Todoist write-through.
4. Normalize tags and expose dependency projection.
5. Add session registry and control plane.
6. Ship the web page.
7. Add CLI parity.
8. Add routine definitions and only then consider routine-aware planning policy.
9. Harden tests/docs/realtime.

## Definition of done
A ticket is only done when:
- code compiles
- storage and route boundaries stay clean
- tests exist for meaningful behavior
- docs/status are updated where relevant
- feature flags are added when rollout risk is non-trivial
