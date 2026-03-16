---
id: vel-adaptive-config-006
title: Expose config CRUD, effective config, and explain APIs
status: proposed
priority: P1
owner: backend
---

## Summary
Add REST endpoints for persisted config CRUD, effective config retrieval, explain output, and policy listing/upsert.

## Scope
- add `routes/config.rs`
- mount in `app.rs`
- implement:
  - `GET /v1/config`
  - `PUT /v1/config`
  - `GET /v1/config/effective`
  - `GET /v1/config/explain`
  - `GET /v1/config/policies`
  - `PUT /v1/config/policies/:id`
  - `POST /v1/config/simulate`

## Acceptance Criteria
- API responses follow spec JSON shape
- explain endpoint returns operator-readable reasoning
- simulate endpoint performs dry-run only and records simulation audit event
- auth/ownership checks are enforced

## Tests
- route tests for happy path and validation errors
- explain endpoint golden tests
