---
id: CLI-001
title: Split command tree and registration
status: todo
priority: P0
write_scope:
  - crates/vel-cli/src/main.rs
  - crates/vel-cli/src/commands/mod.rs
  - crates/vel-cli/src/command_lang/
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Replace the monolithic CLI entry definition with family-oriented command registration.

# Acceptance criteria

1. command families register through clear module seams
2. `main.rs` stops being the only change point for unrelated CLI work
