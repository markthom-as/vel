---
title: Uncertainty & Clarification Ticket Pack
status: todo
owner: core
labels:
  - planning
  - uncertainty
  - tickets
created: 2026-03-15
---

# Uncertainty & Clarification — Ticket Pack

Implementation tickets for first-class uncertainty handling: domain model, scoring, clarification policy, ledger persistence, resolvers (user, agent, retrieval, validation), ask-before-acting preferences, UI (panel, inbox, assumption review), telemetry, and agent output contract.

**Spec:** [docs/specs/vel-uncertainty-architecture-spec.md](../../specs/vel-uncertainty-architecture-spec.md)

## Tickets

| ID | Title | Priority |
|----|--------|----------|
| TICKET-001 | Uncertainty domain model | P0 |
| TICKET-002 | Confidence scoring and normalization | P0 |
| TICKET-003 | Clarification policy engine | P0 |
| TICKET-004 | Uncertainty ledger persistence | P0 |
| TICKET-005 | User clarification resolver | P1 |
| TICKET-006 | Inter-agent consultation resolver | P1 |
| TICKET-007 | Retrieval and validation resolvers | P1 |
| TICKET-008 | Ask-before-acting preferences | P1 |
| TICKET-009 | Uncertainty panel and clarification inbox | P1 |
| TICKET-010 | Assumption review surface | P2 |
| TICKET-011 | Telemetry and calibration | P2 |
| TICKET-012 | Agent output contract update | P0 |

## Status convention

- `open` / `todo`
- `in_progress`
- `blocked`
- `review`
- `done`
