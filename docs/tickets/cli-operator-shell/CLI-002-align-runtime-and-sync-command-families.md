---
id: CLI-002
title: Align runtime and sync command families
status: todo
priority: P1
write_scope:
  - crates/vel-cli/src/commands/sync.rs
  - crates/vel-cli/src/commands/connect.rs
  - crates/vel-cli/src/commands/integrations.rs
  - crates/vel-cli/src/commands/loops.rs
  - crates/vel-cli/src/commands/doctor.rs
  - crates/vel-cli/src/commands/health.rs
  - crates/vel-cli/src/commands/config.rs
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Make sync/runtime-facing commands match the current daemon/control-plane surfaces coherently.

# Acceptance criteria

1. sync/runtime commands match current APIs
2. connect/integration/runtime families are operator-legible
