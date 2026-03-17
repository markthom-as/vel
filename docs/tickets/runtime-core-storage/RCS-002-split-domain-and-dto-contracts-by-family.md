---
id: RCS-002
title: Split domain and DTO contracts by family
status: todo
priority: P1
write_scope:
  - crates/vel-core/src/
  - crates/vel-api-types/src/
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Split `vel-core` and `vel-api-types` by family so shared semantics and shared transport contracts stay aligned across daemon, CLI, web, and Apple.

# Acceptance criteria

1. domain families are explicit
2. DTO families are explicit
3. shared client contracts are easier for web and Apple to consume consistently
