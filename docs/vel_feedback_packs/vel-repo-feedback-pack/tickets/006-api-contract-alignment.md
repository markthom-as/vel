---
title: API Contract Alignment
status: proposed
priority: high
owner: codex
---

# Goal

Align route names, comments, and docs so the API surface says exactly what it does.

# Current issues to fix

- `routes::risk::compute_and_list` is read-only but still sounds mutating
- some docs still read historically rather than contractually
- suggestions API carries state transitions but no explicit action semantics

# Concrete code changes

## A. Rename route handler functions for clarity
Update:
- `crates/veld/src/routes/risk.rs`
- `crates/veld/src/app.rs`

Suggested renames:
- `compute_and_list` -> `list_latest_risk`
- `get_commitment_risk` can remain

## B. Tighten API docs
Update:
- `docs/api.md`
- `docs/api/runtime.md`
- `docs/status.md`

Document:
- `GET /v1/risk` is persisted latest snapshot only
- `POST /v1/evaluate` is the recompute boundary

## C. Improve suggestion action semantics
Optionally extend `SuggestionUpdateRequest` to allow explicit action names:
- `accept`
- `reject`
- `reopen`

Internally you can still map these to state values. The goal is a cleaner operator contract.

# Acceptance criteria

- route function names match behavior
- docs no longer imply GET recomputes
- suggestion actions are easier to understand from the API layer
