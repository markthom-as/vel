---
title: "Configure Development Tooling"
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
Add formatting, linting, and type checking across Rust and TypeScript.

## Tasks

### Rust
- Add `cargo fmt`
- Add `cargo clippy`
- Create `rustfmt.toml`

### JS/TS
- Install `typescript`
- Install `eslint`
- Install `prettier`
- Add workspace scripts for lint and typecheck

## Acceptance Criteria

- `cargo fmt --all` succeeds
- `cargo clippy --workspace` succeeds
- `pnpm lint` succeeds
- `pnpm typecheck` succeeds

## Notes for Agent

Favor boring, standard defaults over ornate tooling theology.
