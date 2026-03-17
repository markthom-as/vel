# UI-V4-004 — Add integration policy controls and participation model

Status: todo
Priority: P1
Lane: C

## Why

The integration screenshots show strong adapter/status detail, but the system still needs a clearer policy model for source participation.

Evidence:

- `~/Downloads/localhost_5173_ (5).png`
- `~/Downloads/localhost_5173_ (6).png`

## Goal

Treat integrations as policy-configured sources, not just connected adapters.

## Required policy fields

- `enabled`
- `sync`
- `visible`
- `contributes_to_context`
- `trust_level`

## Ownership / likely write scope

- settings/integrations UI
- backend/config contract changes if required
- docs for source participation semantics

## Acceptance criteria

- operators can separately control source availability, visibility, context contribution, and trust
- the UI explains how a source participates in context, not just whether it exists
