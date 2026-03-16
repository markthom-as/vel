---
title: Vel Task HUD tickets
status: ready
owner: agent
created: 2026-03-15
---

# Vel Task HUD ticket pack

This pack contains agent-ready markdown tickets for implementing the Vel Task HUD subsystem.

**Spec:** [docs/specs/vel-task-hud-spec.md](../../specs/vel-task-hud-spec.md)

## Boundary first

This pack must **not** casually introduce a second source of truth for work.

Vel already has:

- **commitments** for actionable/reviewable items,
- **nudges** for interventions and reminders,
- **threads** for continuity/grouping,
- **risk** for urgency and threat semantics.

The Task HUD should default to being a **view-model / policy / surface layer over those existing concepts**.

Only introduce a new durable `Task` domain if the implementation can show, explicitly and in code/docs, that:

1. commitments cannot carry the required semantics cleanly,
2. the new domain will not duplicate risk/nudge/thread logic,
3. the ownership boundary between commitments and tasks is stated up front.

If that case is not made, prefer:

- commitment-backed ranking,
- HUD-specific grouping/view models,
- derived or ephemeral task-like items,
- narrow interfaces from risk and nudges into the HUD.

## Included
- 00-spec-vel-task-hud.md — high-level product and architecture spec
- 01..14 — implementation tickets in recommended order

## Assumptions
- Vel is a monorepo.
- Rust crates are the preferred place for core logic.
- UI layer may target desktop first, with mobile/watch/AR abstractions derived from shared view models.
- Risk engine integration already exists in partial form and should be consumed rather than duplicated.
- Commitment, nudge, thread, and risk semantics remain authoritative unless a ticket explicitly and convincingly narrows a different boundary.

## Recommended implementation order
1. Boundary decision: reuse commitments vs introduce a new durable task domain
2. Ranking + policy + HUD view model over the chosen source of truth
3. Desktop surface
4. Inference / rituals / risk integration
5. Voice + glance mode + ambient mode
6. AR protocol spec

## Definition of done
A task is complete when:
- code compiles,
- tests exist for core logic and policy,
- docs are updated,
- the surface is wired into Vel navigation or shell,
- feature flags exist where rollout risk is high.

For foundational tickets, "done" also requires the boundary decision to be explicit: what remains owned by commitments, nudges, threads, and risk, and what the HUD layer actually owns.
