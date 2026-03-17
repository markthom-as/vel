---
title: iCloud Drive backup target
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
  - icloud
---

Define and implement an `icloud_drive` backup target suitable for Apple-first personal storage workflows.

## Scope

- target-root semantics for iCloud Drive mirrored folders
- manifest and file placement rules
- platform and operator constraints
- verification behavior under delayed cloud sync

## Acceptance criteria

- Vel has a concrete `icloud_drive` target model and ticketed implementation path
- the design accounts for local mirror behavior rather than pretending iCloud is a direct object API
- verification semantics are explicit about eventual cloud propagation
