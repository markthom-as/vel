---
title: Vel UI v4 ticket pack
status: active
owner: agent
class: expansion
authority: design
status_model:
  - todo
  - in_progress
  - deferred
  - done
source_of_truth: docs/status.md
labels:
  - planning
  - ui
  - operator-surface
  - tickets
created: 2026-03-17
updated: 2026-03-17
---

# Vel UI v4 — Ticket Pack

This pack turns the imported UI v4 redesign packet and its screenshot feedback into repo-local tickets that are specific enough to execute in parallel.

**Primary spec:** [docs/specs/vel-ui-v4-spec.md](../../specs/vel-ui-v4-spec.md)

## Why this pack exists

The imported `~/Downloads/vel-ui-v4-spec-pack.zip` was directionally useful but too compressed to operate as a durable repo packet.

The screenshot set made the actionable issues clearer:

- `Now` is overloaded
- context explanation and raw debug are fused
- `Suggestions` looks inspector-heavy rather than decision-first
- `Threads` does not yet read as a continuity/process surface
- `Settings` needs a clearer policy/observability split

This pack captures those observations as parallelizable work lanes.

## Pack schema

- `class: expansion`
- `authority: design`
- `status_model: todo | in_progress | deferred | done`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- redesigning operator IA
- moving debug/observability concerns out of `Now`
- improving context explanation modes
- adding source-policy controls
- making thread/inbox/suggestion surfaces clearer and more role-specific

Do not use this pack alone to claim the redesign is shipped.

## Screenshot evidence set

Primary references from `~/Downloads/`:

- `localhost_5173_.png` — baseline `Now`
- `localhost_5173_ (1).png` — overloaded `Now` with freshness/debug
- `localhost_5173_ (2).png` — current thread/chat + context rail
- `localhost_5173_ (3).png` — suggestions detail view
- `localhost_5173_ (4).png` — settings general
- `localhost_5173_ (5).png` — integrations settings
- `localhost_5173_ (6).png` — components settings
- `localhost_5173_ (7).png` — loops settings

## Parallel work lanes

### Lane A — Now / Context / Stats

- UI-V4-001
- UI-V4-002
- UI-V4-003

### Lane B — Threads / Inbox / Suggestions IA

- UI-V4-005
- UI-V4-006
- UI-V4-009

### Lane C — Integration policy and settings

- UI-V4-004
- UI-V4-010

### Lane D — Visual language and polish

- UI-V4-007
- UI-V4-008

## Tickets

| ID | Title | Status | Priority | Lane |
|----|-------|--------|----------|------|
| UI-V4-001 | Refactor context panel into State / Why / Debug modes | todo | P0 | A |
| UI-V4-002 | Clean up the Now page around actionable work only | todo | P0 | A |
| UI-V4-003 | Add a Stats tab for observability and runtime health | todo | P0 | A |
| UI-V4-004 | Add integration policy controls and participation model | todo | P1 | C |
| UI-V4-005 | Upgrade threads into process-oriented continuity surfaces | todo | P1 | B |
| UI-V4-006 | Realign inbox with the new information architecture | todo | P1 | B |
| UI-V4-007 | Add phase-1 metaball head state encoding | deferred | P2 | D |
| UI-V4-008 | Add attention-token system language and polish | deferred | P2 | D |
| UI-V4-009 | Reframe suggestions as a decision-first steering surface | todo | P1 | B |
| UI-V4-010 | Rework Settings IA around policy, observability, and control | todo | P1 | C |

## Exit criteria

- the redesign work is either implemented, deferred, or re-scoped
- `Now`, `Context`, `Threads`, `Suggestions`, `Stats`, and `Settings` have explicit roles
- observability no longer depends on overloading the main action surface
- screenshot-derived feedback has concrete acceptance criteria in the ticket set
- docs/status reflect the final shipped subset rather than the design packet alone
