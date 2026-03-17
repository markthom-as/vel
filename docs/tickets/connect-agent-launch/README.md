---
title: Connect-backed agent launch ticket pack
status: active
owner: agent
class: expansion
authority: execution
status_model:
  - todo
  - in_progress
  - done
  - deferred
source_of_truth: docs/status.md
created: 2026-03-17
updated: 2026-03-17
---

# Connect-backed agent launch ticket pack

Execution pack for launching external coding-agent runtimes on compatible Connect instances and surfacing those sessions through Vel web, CLI, and host-agent flows.

Primary spec:

- [docs/specs/vel-connect-agent-launch-spec.md](../../specs/vel-connect-agent-launch-spec.md)

Related specs:

- [docs/specs/vel-projects-page-spec.md](../../specs/vel-projects-page-spec.md)
- [docs/specs/vel-multi-vendor-integration-and-person-identity-spec.md](../../specs/vel-multi-vendor-integration-and-person-identity-spec.md)
- [docs/specs/vel-multi-client-swarm-spec.md](../../specs/vel-multi-client-swarm-spec.md)
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)

## Purpose

Turn the planned Connect-backed agent launch design into an implementable sequence that:

- discovers which instances can run which external agent runtimes,
- launches a runtime on the selected instance,
- records the resulting live session durably,
- lets operators interact with that session from Vel,
- lets Vel's host agent supervise the same session.

## Why this pack exists

The repo already has:

- connection-aware integration planning,
- project-linked session planning,
- session outbox/steering planning,
- cluster-aware worker planning.

But none of those packs alone owns the end-to-end launch path for external agent runtimes on compatible Connect instances.

This pack exists to close that gap without duplicating:

- provider/family/connection modeling from `integration-expansion/`,
- project workspace/session modeling from `projects/`,
- distributed authority and worker execution from `multi-client-swarm/`.

## Pack schema

- `class: expansion`
- `authority: execution`
- `status_model: todo | in_progress | done | deferred`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- implementing Connect-instance capability discovery for agent launch,
- extending agent sessions to represent live launched sessions,
- adding launch/message/stop/open flows for external coding agents,
- wiring Projects or host-agent surfaces to these launch flows.

Do not use this pack alone when:

- the work is only about generic provider/connection modeling,
- the work is only about session registry basics already covered by `projects/`,
- the work is only about generic cluster scheduling without external runtime launch.

## Boundaries

- Vel remains the canonical planner/integrator.
- External runtimes are bounded workers, not alternate truth authorities.
- `agent_sessions` should be extended, not replaced.
- Connect instance capability discovery should build on the integration family/provider/connection substrate where possible.
- Project workspace remains the primary operator surface for launched sessions.

## Cross-pack dependencies

Depends on:

- `integration-expansion/` for family/provider/connection modeling and provider capability manifests, especially `INTG-001` and `INTG-003`
- `projects/` for the base `agent_sessions` substrate and shared session control plane, especially tickets `06` and `07`
- `multi-client-swarm/` for host-agent supervision and worker authority semantics when external launched sessions are treated as bounded workers

Unlocks:

- Connect-aware Projects launch UX
- live launched-session representation in shared workspace projections
- host-agent delegation into external runtimes on compatible instances

Overlap rule:

- if a question is about connection/provider ontology, prefer `integration-expansion/`
- if a question is about shared session/operator UX contracts, prefer `projects/`
- if a question is about supervisor/worker authority or distributed orchestration, prefer `multi-client-swarm/`
- if a question is specifically about launching and interacting with external runtimes on compatible instances, prefer this pack

## Tickets

| ID | Title | Status |
|----|-------|--------|
| CAL-001 | Connect instance registry and capability manifest | todo |
| CAL-002 | Agent runtime catalog and compatibility selection | todo |
| CAL-003 | Live launched-session schema and session-model extensions | todo |
| CAL-004 | Launch service and Connect launch API | todo |
| CAL-005 | Session interaction actions and event lifecycle | todo |
| CAL-006 | Projects workspace launch flow and live session controls | todo |
| CAL-007 | Host-agent supervision and delegation integration | todo |
| CAL-008 | Fixtures, tests, docs, and rollout guards | todo |

## Recommended execution order

1. CAL-001
2. CAL-002
3. CAL-003
4. CAL-004
5. CAL-005
6. CAL-006
7. CAL-007
8. CAL-008

## Exit criteria

- compatible Connect instances can advertise launchable runtimes,
- Vel can launch at least one external runtime through a durable API/service path,
- launched sessions appear in shared workspace/session projections,
- operators and the host agent can interact with those sessions through Vel surfaces,
- tests/docs/status are updated without overstating shipped breadth.
