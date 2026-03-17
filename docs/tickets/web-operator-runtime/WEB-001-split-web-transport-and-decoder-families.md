---
id: WEB-001
title: Split web transport and decoder families
status: todo
priority: P0
write_scope:
  - clients/web/src/types.ts
  - clients/web/src/types.test.ts
  - clients/web/src/api/client.ts
  - clients/web/src/api/client.test.ts
created: 2026-03-17
updated: 2026-03-17
---

# Goal

Split the web transport contract into surface families instead of one giant type file.

# Acceptance criteria

1. decoder/type modules match real web surface families
2. web transport modules align cleanly with shared daemon and Apple-facing contract families
