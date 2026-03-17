---
title: S3 and S3-compatible backup target
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
  - s3
---

Add an `s3` target for object-store backup of artifacts and manifests.

## Scope

- S3 target configuration
- push/upload flow for artifacts and manifests
- hash-aware verification
- support for S3-compatible endpoints where feasible

## Acceptance criteria

- Vel can back up artifacts to S3-style object storage
- manifests record object locations and verification state
- the design supports both AWS S3 and compatible stores without changing the core model
