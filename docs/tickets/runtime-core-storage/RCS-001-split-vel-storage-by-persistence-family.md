---
id: RCS-001
title: Split vel-storage by persistence family
status: in_progress
priority: P0
write_scope:
  - crates/vel-storage/src/lib.rs
  - crates/vel-storage/src/db.rs
  - crates/vel-storage/src/infra.rs
  - crates/vel-storage/src/runs.rs
  - crates/vel-storage/src/run_refs.rs
  - crates/vel-storage/src/runtime_cluster.rs
  - crates/vel-storage/src/runtime_loops.rs
  - crates/vel-storage/src/integrations.rs
  - crates/vel-storage/src/threads.rs
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Replace the current `db.rs` concentration with persistence-family modules while preserving one stable `Storage` facade.

# Acceptance criteria

1. unrelated persistence work no longer collides in the same implementation file
2. storage family ownership is explicit
3. the external storage boundary remains stable
