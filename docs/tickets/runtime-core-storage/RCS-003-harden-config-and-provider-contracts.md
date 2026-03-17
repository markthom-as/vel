---
id: RCS-003
title: Harden config and provider contracts
status: todo
priority: P1
write_scope:
  - crates/vel-config/src/
  - crates/vel-llm/src/
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Normalize config/profile/provider boundaries so runtime and client-facing surfaces consume one clear shared contract.

# Acceptance criteria

1. provider/profile boundaries are explicit
2. config keys and runtime consumption remain aligned
3. shared config/provider logic stops drifting into unrelated modules
