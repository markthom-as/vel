---
title: "Initialize Vel Monorepo"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on: []
labels:
  - vel
  - chat-interface
---
Create the base repository structure and toolchain for Vel.

## Scope

Create:

```text
vel/
  Cargo.toml
  package.json
  pnpm-workspace.yaml
  /crates
  /clients/web
  /docs
```

## Tasks

- Create Rust workspace
- Initialize pnpm workspace
- Create top-level directories
- Verify workspace layout is coherent

## Acceptance Criteria

- `cargo build` runs successfully
- `pnpm install` works
- workspace structure is committed
- no broken references in root manifests

## Notes for Agent

Keep the monorepo minimal. Do not add speculative packages yet.
