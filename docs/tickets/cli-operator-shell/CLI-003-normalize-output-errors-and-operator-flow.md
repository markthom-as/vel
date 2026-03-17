---
id: CLI-003
title: Normalize output, errors, and operator flow
status: todo
priority: P1
write_scope:
  - crates/vel-cli/src/client.rs
  - crates/vel-cli/src/commands/
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Unify output, error, JSON, and human-readable behavior across CLI command families.

# Acceptance criteria

1. JSON and human output paths are consistent
2. command failures are easier to interpret
3. operator flows feel like one shell rather than unrelated tools
