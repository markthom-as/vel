---
title: Suggestion API and CLI Operator Surfaces
status: proposed
priority: medium
owner: codex
---

# Goal

Make suggestions easy to inspect, accept, reject, and trace from CLI and API.

# Concrete file targets

- `crates/veld/src/routes/suggestions.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/vel-cli/src/main.rs`
- `crates/vel-cli/src/client.rs`

# Concrete code changes

## A. Add evidence endpoint
Create:
- `GET /v1/suggestions/:id/evidence`

## B. Add CLI commands
Add:
- `vel suggestions`
- `vel suggestion inspect <id>`
- `vel suggestion accept <id>`
- `vel suggestion reject <id> [--reason "..."]`

If these already partially exist, upgrade them to display:
- title
- summary
- priority
- confidence
- evidence refs

## C. Improve update semantics
Prefer explicit action endpoints or action enums over raw freeform state strings.

# Acceptance criteria

- operator can understand a suggestion without reading raw payload JSON
- suggestion evidence is visible from API and CLI
- accept/reject flows are ergonomic
