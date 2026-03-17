---
title: Google Drive backup target
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
  - google-drive
---

Define and implement a `google_drive` backup target for manifest-driven artifact backup.

## Scope

- Drive folder and file layout
- manifest upload strategy
- local-mirror versus API-driven upload decision
- verification and restore-plan semantics

## Acceptance criteria

- Vel has a concrete `google_drive` backup target ticket and design
- backup semantics are manifest-driven rather than ad hoc file dumping
- restore and verification constraints are documented
