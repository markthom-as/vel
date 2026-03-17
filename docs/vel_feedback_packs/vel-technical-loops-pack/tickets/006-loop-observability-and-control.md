---
title: Loop Observability and Operator Controls
status: proposed
priority: medium
owner: codex
---

# Goal

Let operators see and control loop behavior.

# Concrete file targets

- `crates/veld/src/routes/loops.rs` (new)
- `crates/veld/src/app.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/client.rs`

# Concrete code changes

## Add API routes
- `GET /v1/loops`
- `GET /v1/loops/:kind`
- `PATCH /v1/loops/:kind` for enable/disable or interval updates

## Add CLI
- `vel loops`
- `vel loop inspect <kind>`
- `vel loop enable <kind>`
- `vel loop disable <kind>`

## Surface fields
- enabled
- interval_seconds
- last_started_at
- last_finished_at
- last_status
- last_error
- next_due_at

# Acceptance criteria

- operator can inspect loop health
- loop enable/disable is not a code change
- loop failures are visible without reading raw DB tables
