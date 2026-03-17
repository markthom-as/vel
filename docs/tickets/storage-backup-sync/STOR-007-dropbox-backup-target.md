---
title: Dropbox backup target
status: todo
owner: agent
type: design
priority: medium
created: 2026-03-17
depends_on:
  - STOR-001-foundation-storage-target-and-backup-manifest-model.md
  - STOR-002-local-manifest-and-verification-cli.md
labels:
  - vel
  - storage
  - backup
  - dropbox
---

Define and implement a `dropbox` backup target for artifact backup and recovery workflows.

## Scope

- Dropbox-root or folder placement model
- manifest and artifact copy strategy
- verification under sync-lag conditions
- restore semantics and operator guidance

## Acceptance criteria

- Vel has a concrete `dropbox` backup target ticket and design
- the model distinguishes mirrored local folders from provider API behavior
- restore and verification expectations are explicit
