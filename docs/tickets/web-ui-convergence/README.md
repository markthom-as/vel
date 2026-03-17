---
title: Vel web UI convergence ticket pack
status: active
owner: agent
class: convergence
authority: execution
status_model:
  - todo
  - in_progress
  - done
  - deferred
source_of_truth: docs/status.md
labels:
  - planning
  - web
  - ui
  - operator-surface
  - tickets
created: 2026-03-17
updated: 2026-03-17
---

# Vel Web UI Convergence — Ticket Pack

This is the canonical execution pack for web and global operator UX/UI work.

Primary spec:

- [docs/specs/vel-web-operator-surface-spec.md](../../specs/vel-web-operator-surface-spec.md)

## Purpose

Turn the current spread of web/UI planning into one executable queue that can be assigned in parallel across backend, web, and docs lanes.

## Why this pack exists

Relevant work is currently split across:

- the shipped chat/web shell work
- `now-page-fixes`
- `ui-v4`
- `projects`
- frontend decomposition and state-management cleanup tickets
- several architecture and operator-surface specs

Those packets still contain useful detail, but they are no longer a good primary execution entrypoint.

This pack consolidates them into one queue with explicit write scopes and dependencies.

## Pack schema

- `class: convergence`
- `authority: execution`
- `status_model: todo | in_progress | done | deferred`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- changing the global web shell or top-level navigation
- changing shared transport/query/realtime behavior in `clients/web`
- changing cross-surface context, freshness, or degraded-state UX rules
- changing `Now`, `Inbox`, `Threads`, `Suggestions`, `Projects`, `Stats`, or `Settings`
- reconciling multiple older web/UI ticket packets into one implementation sequence

## Boundaries

- do not move domain semantics out of runtime services into the web client
- do not introduce a second task authority beside commitments
- do not let `Now` re-absorb observability that belongs in `Stats`
- do not let `Settings` become a passive diagnostics landfill
- do not fork separate web and CLI project workspace contracts

## Legacy input packets

Use these as source material, not as the primary queue:

- [docs/tickets/ui-v4/README.md](../ui-v4/README.md)
- [docs/tickets/projects/README.md](../projects/README.md)
- `docs/tickets/now-page-fixes/*.md`
- [docs/tickets/repo-audit-hardening/007-frontend-surface-decomposition-plan.md](../repo-audit-hardening/007-frontend-surface-decomposition-plan.md)
- [docs/tickets/repo-feedback/006-harden-web-client-state-management-and-realtime-sync.md](../repo-feedback/006-harden-web-client-state-management-and-realtime-sync.md)

## Parallel lanes

### Lane A — Shell, transport, and realtime

- WUI-001
- WUI-002
- WUI-003

### Lane B — Operational surfaces

- WUI-004
- WUI-005
- WUI-006

### Lane C — Cognitive and continuity surfaces

- WUI-007
- WUI-008

### Lane D — Project workspace

- WUI-009
- WUI-010

Lane dependency note:

- treat Lane D as downstream of the shared project substrate in [docs/tickets/projects/README.md](../projects/README.md), especially tickets 01, 02, 03, 14, 15, and 16
- do not let web-only DTOs, task tagging, or dependency semantics diverge from the project pack

### Lane E — Docs, tests, and rollout

- WUI-011

## Tickets

| ID | Title | Status | Priority | Lane |
|----|-------|--------|----------|------|
| WUI-001 | Canonicalize shell IA and top-level route ownership | done | P0 | A |
| WUI-002 | Decompose web transport decoders and resource/query boundaries by domain | todo | P0 | A |
| WUI-003 | Harden websocket, optimistic mutations, and shared invalidation behavior | todo | P0 | A |
| WUI-004 | Converge Now, context inspection, and Stats around purpose-built contracts | in_progress | P0 | B |
| WUI-005 | Rework Settings and integrations into control-first IA | todo | P1 | B |
| WUI-006 | Normalize global page-state, freshness, degraded-state, and recovery UX | todo | P1 | B |
| WUI-007 | Realign Inbox, Threads, and Suggestions to distinct operator roles | todo | P1 | C |
| WUI-008 | Complete chat/provenance surface polish under the shared operator model | todo | P2 | C |
| WUI-009 | Build Projects registry and workspace projection contract | todo | P0 | D |
| WUI-010 | Ship web Projects tasks and sessions workspaces | todo | P0 | D |
| WUI-011 | Reconcile docs, regression coverage, and historical packet references | todo | P0 | E |

## Recommended execution order

1. WUI-001, WUI-002, WUI-003
2. WUI-004 and WUI-009 in parallel once shared foundations are clear
3. WUI-005, WUI-006, WUI-007
4. WUI-008 and WUI-010
5. WUI-011

## Exit criteria

- one canonical web/global UX execution queue exists
- shared web architecture is explicit enough to stop duplicate decoder/fetch/realtime patterns
- each top-level web surface has a single clear role
- project workspace work is integrated into the same shell model rather than a side packet
- docs and status references point to this pack as the execution entrypoint
