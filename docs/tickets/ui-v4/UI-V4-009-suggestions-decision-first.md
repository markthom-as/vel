# UI-V4-009 — Reframe suggestions as a decision-first steering surface

Status: todo
Priority: P1
Lane: B

## Why

The suggestions screenshot shows good evidence depth, but the default presentation still foregrounds payload and evidence detail more than the operator decision.

Evidence:

- `~/Downloads/localhost_5173_ (3).png`

## Goal

Make suggestions feel like steering decisions first, with payload and raw evidence available on demand.

## Ownership / likely write scope

- suggestions list/detail UI
- suggestion detail disclosure model
- docs for steering workflow

## Acceptance criteria

- accept/reject remains the primary action
- evidence is legible without forcing raw JSON to dominate the surface
- payload internals move behind progressive disclosure where appropriate
