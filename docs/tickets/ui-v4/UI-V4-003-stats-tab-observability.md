# UI-V4-003 — Add a Stats tab for observability and runtime health

Status: todo
Priority: P0
Lane: A

## Why

The screenshot set shows observability currently spread across `Now` and multiple Settings tabs.

Evidence:

- `~/Downloads/localhost_5173_ (1).png`
- `~/Downloads/localhost_5173_ (4).png`
- `~/Downloads/localhost_5173_ (5).png`
- `~/Downloads/localhost_5173_ (6).png`
- `~/Downloads/localhost_5173_ (7).png`

## Goal

Create a dedicated `Stats` surface for runtime and source introspection.

## Ownership / likely write scope

- top-level navigation
- stats view and view-model contracts
- observability docs

## Required sections

- source health
- context formation
- system behavior
- loop performance

## Acceptance criteria

- operators have one obvious place to inspect runtime/system health
- `Now` no longer needs to carry broad observability detail
- the existing settings tabs can focus more on control than passive inspection
