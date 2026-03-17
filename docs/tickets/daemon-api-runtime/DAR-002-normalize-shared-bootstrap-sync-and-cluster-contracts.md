---
id: DAR-002
title: Normalize shared bootstrap, sync, and cluster contracts
status: in_progress
priority: P0
write_scope:
  - crates/veld/src/routes/sync.rs
  - crates/veld/src/routes/cluster.rs
  - crates/veld/src/routes/connect.rs
  - crates/veld/src/services/client_sync.rs
  - crates/veld/src/services/connect.rs
  - crates/veld/src/worker.rs
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Make bootstrap, sync, cluster, connect, and worker-control behavior one explicit shared runtime seam for all clients.

# Acceptance criteria

1. client bootstrap and sync semantics are clearer for both web and Apple
2. worker/control-plane logic is separated from unrelated read-model assembly
3. route files stay thin
