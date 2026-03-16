---
title: Vel Task HUD tickets
status: ready
owner: agent
created: 2026-03-15
---

# Vel Task HUD ticket pack

This pack contains agent-ready markdown tickets for implementing the Vel Task HUD subsystem.

**Spec:** [docs/specs/vel-task-hud-spec.md](../../specs/vel-task-hud-spec.md)

## Included
- 00-spec-vel-task-hud.md — high-level product and architecture spec
- 01..14 — implementation tickets in recommended order

## Assumptions
- Vel is a monorepo.
- Rust crates are the preferred place for core logic.
- UI layer may target desktop first, with mobile/watch/AR abstractions derived from shared view models.
- Risk engine integration already exists in partial form and should be consumed rather than duplicated.

## Recommended implementation order
1. Task core + DB
2. Actions + ranking + policy
3. HUD view model + desktop surface
4. Inference + rituals + risk integration
5. Voice + glance mode + ambient mode
6. AR protocol spec

## Definition of done
A task is complete when:
- code compiles,
- tests exist for core logic and policy,
- docs are updated,
- the surface is wired into Vel navigation or shell,
- feature flags exist where rollout risk is high.

