---
id: NOW-005
status: proposed
title: Create a dedicated /v1/now endpoint and transport contract
owner: backend
priority: P1
---

## Goal

Stop composing the Now page from a bag of semi-related endpoints.

## Why

The current UI pulls from current context, context explain, drift explain, and commitments separately. That is brittle, redundant, and hard to make fresh.

## Files likely touched

- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/now.rs` (new)
- `crates/veld/src/routes/now.rs` (new)
- `crates/veld/src/app.rs`
- backend tests

## Requirements

1. Add `GET /v1/now`.
2. Add DTOs for:
   - summary cards
   - schedule
   - prioritized tasks
   - attention/drift
   - freshness
   - optional debug payload
3. Route must be read-only and must not call evaluate.
4. Service should assemble data from persisted sources:
   - current context
   - integration statuses
   - commitments
   - relevant event signals
5. Include both operator-facing labels and raw debug keys where helpful.

## Implementation notes

- Keep route thin.
- Put view-model shaping in service layer.
- Do not force the frontend to map a dozen unrelated raw fields.

## Tests

- happy-path `/v1/now`
- empty-state `/v1/now`
- partial freshness degradation
- debug payload presence/shape

## Acceptance criteria

- Frontend can render the Now page from one endpoint.
- Endpoint exposes freshness + operator labels.
