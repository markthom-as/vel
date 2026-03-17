---
title: Vel storage, backup, and sync targets spec
status: proposed
owner: platform
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

Define the planned storage-target and backup/sync architecture for Vel artifacts and durable state.

This spec is about trust, portability, and recovery.

It is not a claim that remote backup is already implemented.

# Why this exists

Vel currently has:

- local database state
- local artifact storage rooted under `artifact_root`
- export support
- `vel backup` as guidance, not a real backup system

That is a reasonable first trust gesture, but it is not yet a storage strategy.

Vel needs explicit tickets for:

- object storage backups
- filesystem mirroring
- cloud drive targets
- manifests and restore verification

# Principles

## 1. Local-first remains the default

Remote targets are optional.

Vel should continue to function with:

- local database
- local artifact directory
- manual export and inspection

## 2. Backup and sync are not the same

Vel should distinguish:

- backup: durable copy for recovery
- mirror sync: bidirectional or one-way filesystem/object synchronization
- active artifact storage target: where new artifacts may be written

These should not be collapsed into one setting.

## 3. Manifests matter more than vendor APIs

Every remote target should be driven by a Vel-native manifest and verification model.

Providers differ, but Vel should preserve one common record of:

- what was backed up
- where it went
- content hash
- storage kind / target kind
- sync class
- last verified timestamp
- restore instructions or restore capability

## 4. Start with artifact backup, not database replication

The first remote-target work should focus on:

- artifact manifests
- artifact copy/push/pull
- verification
- restore tooling

Do not begin with raw sqlite replication or multi-master file sync.

# Planned target families

## Filesystem and operator-managed targets

- `rsync`

Use for:

- NAS
- external drives
- remote shell targets
- operator-controlled mirror jobs

## Object storage targets

- `s3`

Use for:

- S3 and S3-compatible stores
- versioned object backups
- cold archive and warm off-device copies

## Drive-sync targets

- `icloud_drive`
- `google_drive`
- `dropbox`

Use for:

- user-visible file sync roots
- personal cloud-backed storage
- recovery and portability workflows

Guardrail:

- these should begin as backup/mirror targets, not as canonical online runtime storage authorities

# Required model

Vel should gain explicit types for:

- storage target
- backup job
- backup manifest
- manifest entry
- verification result
- restore plan

Likely target classes:

- local_filesystem
- rsync
- s3
- icloud_drive
- google_drive
- dropbox

# First implementation sequence

1. define target model and manifest format
2. add local manifest and verification flow
3. add `rsync` target
4. add `s3` target
5. add cloud-drive targets
6. add restore verification and operator surfaces

# Non-goals for the first pass

- raw sqlite live sync
- true multi-master remote state replication
- treating Dropbox/Google Drive/iCloud as concurrent authoritative databases
- hiding provider differences behind fake sameness where restore semantics differ

# Acceptance criteria

- Vel has explicit tickets for `rsync`, `s3`, `icloud_drive`, `google_drive`, and `dropbox`
- backup targets are described as optional trust features, not current runtime truth
- the design distinguishes manifest-driven backup from broader cluster/client sync
