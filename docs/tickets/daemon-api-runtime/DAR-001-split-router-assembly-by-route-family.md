---
id: DAR-001
title: Split router assembly by route family
status: todo
priority: P0
write_scope:
  - crates/veld/src/app.rs
  - crates/veld/src/routes/mod.rs
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Break the monolithic app router into family-level registration units with one readable assembly point.

# Acceptance criteria

1. route registration is grouped by family
2. top-level assembly remains easy to scan
3. route ownership is obvious without scanning the whole file
