---
id: VEL-DOC-002
title: Align docs with implemented risk and nudges contracts
status: proposed
priority: P0
owner: backend / docs
---

# Goal

Resolve concrete API contract mismatches for risk and nudges.

# Scope

- `docs/status.md`
- canonical API docs
- any route-specific documentation mentioning risk or nudges

# Problem details

## Risk

Docs currently imply `GET /v1/risk` recomputes risk, but implementation is read-only and evaluation happens through `POST /v1/evaluate`.

## Nudges

Docs currently advertise `PATCH /v1/nudges/:id`, but the server exposes read plus action routes such as done and snooze.

# Required changes

## 1. Rewrite risk route documentation to match code

Document current truth:
- `GET /v1/risk` returns current risk state
- `POST /v1/evaluate` triggers evaluation/recomputation

If desired future behavior includes recompute-on-GET, document it in a clearly separate “planned” section only.

## 2. Rewrite nudges route documentation to match code

Document current truth:
- `GET /v1/nudges/:id` exists
- mutation happens through explicit action endpoints
- remove `PATCH /v1/nudges/:id` from current implementation docs unless it is actually implemented.

## 3. Update status wording in `docs/status.md`

Wherever risk or nudge behavior is summarized, make sure the summary reflects current mutation model.

# Acceptance criteria

- no current-state doc claims that `GET /v1/risk` recomputes risk.
- no current-state doc claims that `PATCH /v1/nudges/:id` exists unless code is added.
- API docs clearly distinguish read routes from mutation/action routes.

# Suggested implementation steps

1. inspect current route registration in `crates/veld/src/app.rs` and route modules.
2. rewrite route descriptions in the API docs first.
3. update `docs/status.md` summaries to match.
4. grep the repo for `PATCH /v1/nudges/:id` and `GET /v1/risk` wording and fix all stale references.

