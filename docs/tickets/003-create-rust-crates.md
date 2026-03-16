---
title: "Create Rust Core Crates"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 001-initialize-monorepo
labels:
  - vel
  - chat-interface
---
Create the foundational Rust crates for Vel.

## Crates

- `vel-core`
- `vel-events`
- `vel-store`
- `vel-policy`
- `vel-server`

## Tasks

- Scaffold each crate
- Add all crates to workspace
- Ensure workspace compiles

## Acceptance Criteria

- `cargo build --workspace` succeeds
- all crates are referenced from root `Cargo.toml`
- crate naming is consistent

## Notes for Agent

Do not collapse all logic into `vel-server`. Preserve boundaries from the start.
