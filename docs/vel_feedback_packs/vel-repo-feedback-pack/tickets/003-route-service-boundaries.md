---
title: Route and Service Boundary Hardening
status: proposed
priority: high
owner: codex
---

# Goal

Ensure routes remain thin and business logic stays in services.

# Why now

The repo has already moved in this direction, but there is still implicit logic in route naming, route comments, and route-shape drift. This ticket finishes the cleanup in a low-risk way.

# Concrete file targets

- `crates/veld/src/app.rs`
- `crates/veld/src/routes/*.rs`
- `crates/veld/src/services/mod.rs`
- `docs/specs/vel-service-boundary-contract.md`

# Concrete code changes

## A. Add service boundary comments
At the top of each route module, add:
- route purpose
- read-only vs recompute
- primary service owner

## B. Normalize "read-only" semantics
Audit these routes:
- `/v1/risk`
- `/v1/risk/:id`
- `/v1/explain/*`
- `/v1/context/current`
- `/v1/context/timeline`
- `/v1/suggestions`

Verify none of them call recompute services.

## C. Introduce a tiny route test helper
Add one helper test module to assert that read routes do not change row counts for:
- `commitment_risk`
- `inferred_state`
- `current_context` timestamp unless explicitly refreshed
- `suggestions`

This helper belongs in backend tests, not in route code.

# Suggested implementation details

Create an integration-style backend test file:
- `crates/veld/tests/read_routes_are_read_only.rs`

That file should:
1. seed captures/commitments/signals
2. call `POST /v1/evaluate`
3. snapshot relevant row counts
4. call read routes
5. assert counts unchanged

# Acceptance criteria

- route modules describe their ownership and mutation contract
- read-only routes are guarded by tests
- service orchestration remains the place where recomputation happens
