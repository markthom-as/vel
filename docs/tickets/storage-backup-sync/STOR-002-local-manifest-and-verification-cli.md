---
title: Local manifest and verification CLI
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-17
depends_on:
  - STOR-001-foundation-storage-target-and-backup-manifest-model.md
labels:
  - vel
  - storage
  - backup
  - cli
---

Add local manifest generation and verification tooling before any remote target integrations.

## Scope

- manifest writer for artifact inventory
- content-hash generation
- local verification command(s)
- inspection output for missing or mismatched artifact copies

## Acceptance criteria

- operators can generate a manifest from current local artifact state
- operators can verify the manifest against local files
- this becomes the shared substrate for later provider targets
