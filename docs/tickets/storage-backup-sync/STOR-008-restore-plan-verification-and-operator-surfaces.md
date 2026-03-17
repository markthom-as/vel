---
title: Restore plan verification and operator surfaces
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
  - restore
---

Add restore-plan, verification, and inspection surfaces for storage targets and backup jobs.

## Scope

- restore-plan generation
- verification summaries
- CLI or API inspection surfaces
- operator guidance for partial or failed restore states

## Acceptance criteria

- operators can inspect target status and verification summaries
- restore planning is explicit rather than implied by raw file copies
- backup trust surfaces improve beyond manual text instructions
