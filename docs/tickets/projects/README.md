---
title: Vel Projects page ticket pack
status: ready
owner: agent
created: 2026-03-16
---

# Vel Projects page ticket pack

This pack contains codex-ready markdown tickets for implementing a first-class Projects page and shared project workspace contract in Vel.

**Primary spec:** [docs/specs/vel-projects-page-spec.md](../../specs/vel-projects-page-spec.md)

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

Extended by:

- `connect-agent-launch/` for live launched-session metadata, instance/runtime-aware session fields, and launch-driven session controls

Overlap rule:

- this pack owns the shared project workspace contract and the base session/control-plane model
- Connect-backed launch work should extend that substrate rather than invent a parallel session or message-control model

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

## Recommended implementation order
1. Establish project registry and typed contracts.
2. Build read-only project index/workspace projection.
3. Add commitment-backed task mutations and Todoist write-through.
4. Add session registry and control plane.
5. Ship the web page.
6. Add CLI parity.
7. Harden tests/docs/realtime.

## Definition of done
A ticket is only done when:
- code compiles
- storage and route boundaries stay clean
- tests exist for meaningful behavior
- docs/status are updated where relevant
- feature flags are added when rollout risk is non-trivial
