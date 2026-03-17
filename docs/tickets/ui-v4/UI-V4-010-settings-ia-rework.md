# UI-V4-010 — Rework Settings IA around policy, observability, and control

Status: todo
Priority: P1
Lane: C

## Why

The screenshot set shows useful controls across `General`, `Integrations`, `Components`, and `Loops`, but the IA is still fragmented.

Evidence:

- `~/Downloads/localhost_5173_ (4).png`
- `~/Downloads/localhost_5173_ (5).png`
- `~/Downloads/localhost_5173_ (6).png`
- `~/Downloads/localhost_5173_ (7).png`

## Goal

Make Settings feel like a coherent control plane, with clearer separation between:

- policy
- observability
- runtime control

## Ownership / likely write scope

- settings navigation and grouping
- related view models
- docs for settings categories

## Acceptance criteria

- policy controls are easier to find
- passive observability no longer dominates settings tabs when a `Stats` surface exists
- runtime controls remain available without feeling like an unstructured dump of subsystems
