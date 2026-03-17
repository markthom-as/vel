---
title: Foundation storage target and backup manifest model
status: todo
owner: agent
type: architecture
priority: high
created: 2026-03-17
depends_on: []
labels:
  - vel
  - storage
  - backup
  - trust
---

Define the provider-neutral storage target and backup manifest model for Vel artifacts.

## Scope

- target kinds and target configuration
- backup manifest format
- manifest entry hashing and verification fields
- restore-plan metadata
- separation between backup, mirror sync, and active storage target

## Acceptance criteria

- target kinds include `rsync`, `s3`, `icloud_drive`, `google_drive`, and `dropbox`
- manifests record hashes, remote location, sync class, verification state, and timestamps
- the design does not require raw sqlite replication for the first pass
