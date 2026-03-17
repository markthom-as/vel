---
title: Uncertainty Inspection Surfaces
status: proposed
priority: medium
owner: codex
---

# Goal

Make uncertainty visible to operators.

# Concrete file targets

- `crates/veld/src/routes/` add uncertainty route
- `crates/vel-api-types/src/lib.rs`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/client.rs`
- web UI if desired, but CLI/API first

# Concrete code changes

## Add routes
- `GET /v1/uncertainty`
- `GET /v1/uncertainty/:id`
- optional `POST /v1/uncertainty/:id/resolve`

## Add CLI
- `vel uncertainty`
- `vel uncertainty inspect <id>`

Display:
- subject
- decision kind
- confidence
- reasons
- missing evidence
- resolution mode
- status

# Acceptance criteria

- operator can inspect open uncertainty records
- uncertainty is no longer trapped inside logs or inline JSON blobs
