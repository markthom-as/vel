# Vel Runtime API (`/v1`)

This document describes the core runtime API exposed by `veld` under `/v1`.

For repo-wide implementation truth, see [`../status.md`](../status.md).

## Health

### `GET /v1/health`

- daemon status
- DB status
- version
- degraded flag (if relevant)

## Captures

### `POST /v1/captures`

- create a capture from text (supports `capture_type`, `source`).

## Context

### `GET /v1/context/today`
### `GET /v1/context/morning`
### `GET /v1/context/end-of-day`

- run-backed context endpoints (create a run, compute snapshot, write artifact, link run → artifact and refs).

### `GET /v1/context/current`

- returns the latest inferred current context (mode, morning_state, meds_status, windows, risk fields, etc.).

## Commitments

### `GET /v1/commitments`
### `GET /v1/commitments/:id`
### `POST /v1/commitments`
### `PATCH /v1/commitments/:id`

- see commitments section in `status.md` and `specs/vel-migrations-and-schema-spec.md`.

## Nudges

### `GET /v1/nudges`
### `GET /v1/nudges/:id`
### `POST /v1/nudges/:id/done`
### `POST /v1/nudges/:id/snooze`

- read and act on nudges; no generic `PATCH /v1/nudges/:id`.

## Risk

### `POST /v1/evaluate`

- runs inference → risk → nudges and persists risk state.

### `GET /v1/risk`
### `GET /v1/risk/:id`

- returns current risk state (read-only) from storage.

## Explain

### `GET /v1/explain/context`
### `GET /v1/explain/nudge/:id`
### `GET /v1/explain/commitment/:id`
### `GET /v1/explain/drift`

- exposes explainability surfaces for context, nudges, commitments, and drift.

## Synthesis

### `POST /v1/synthesis/week`
### `POST /v1/synthesis/project/:slug`

- weekly and project synthesis; see `specs/vel-weekly-synthesis-spec.md` and `specs/vel-agent-next-implementation-steps.md`.

