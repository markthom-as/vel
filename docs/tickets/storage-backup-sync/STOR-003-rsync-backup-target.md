---
title: Rsync backup target
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-17
depends_on:
  - STOR-001-foundation-storage-target-and-backup-manifest-model.md
  - STOR-002-local-manifest-and-verification-cli.md
labels:
  - vel
  - storage
  - backup
  - rsync
---

Add an operator-managed `rsync` backup target for artifact mirroring and recovery workflows.

## Scope

- target configuration shape
- rsync command construction and dry-run support
- manifest-aware push behavior
- verification of mirrored files

## Acceptance criteria

- Vel can describe and run an `rsync`-based artifact backup job
- dry-run output is inspectable before execution
- manifest verification can confirm mirrored copies
