---
id: WEB-002
title: Split query, resource, and realtime state
status: todo
priority: P1
write_scope:
  - clients/web/src/data/
  - clients/web/src/realtime/
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Make query, resource loading, and realtime sync a first-class web state layer.

# Acceptance criteria

1. query/resource/realtime ownership is explicit
2. realtime behavior is testable without component churn
3. repeated shaping work is reduced
