---
title: Vel storage backup sync ticket pack
status: in_progress
owner: agent
class: expansion
authority: design
status_model:
  - todo
  - in_progress
  - deferred
  - done
source_of_truth: docs/status.md
created: 2026-03-17
updated: 2026-03-17
---

# Vel Storage Backup Sync

Implementation-planning tickets for artifact backup, storage targets, manifests, verification, and restore workflows.

Primary spec:

- [docs/specs/vel-storage-backup-sync-spec.md](../../specs/vel-storage-backup-sync-spec.md)

## Why this pack exists

Vel currently has local-first artifact storage and backup guidance, but not a real remote-target backup system.

Current implemented truth:

- artifact rows describe source storage, not backup targets
- `vel backup` is still manual copy guidance
- `vel export` is metadata export, not recovery-ready backup
- `vel doctor` checks local runtime storage, not backup integrity

This pack covers:

- provider-neutral manifest and verification design
- filesystem and object-store targets
- cloud-drive backup targets
- restore and trust surfaces

## Pack schema

- `class: expansion`
- `authority: design`
- `status_model: todo | in_progress | deferred | done`
- `source_of_truth: docs/status.md`

## Entry criteria

Use this pack when:

- planning or implementing artifact backup targets,
- adding durable off-device storage for artifacts,
- improving restore and verification workflows,
- distinguishing backup targets from broader cluster/client sync.

Do not use this pack to imply that remote backup is already shipped.

## Ticket list

- `STOR-001-foundation-storage-target-and-backup-manifest-model.md` - `done`
- `STOR-002-local-manifest-and-verification-cli.md` - `in_progress`
- `STOR-003-rsync-backup-target.md`
- `STOR-004-s3-and-s3-compatible-backup-target.md`
- `STOR-005-icloud-drive-backup-target.md`
- `STOR-006-google-drive-backup-target.md`
- `STOR-007-dropbox-backup-target.md`
- `STOR-008-restore-plan-verification-and-operator-surfaces.md`

## Recommended execution order

1. STOR-001
2. STOR-002
3. STOR-003
4. STOR-004
5. STOR-005
6. STOR-006
7. STOR-007
8. STOR-008

## Exit criteria

- Vel has a manifest-driven storage-target model,
- at least one operator-managed and one remote-provider backup target are implemented,
- restore and verification flows are defined and inspectable,
- backup/storage-target work remains clearly separate from cluster/client sync.

## Current focus

- `STOR-001` is complete as the model and boundary definition lane.
- `STOR-002` is the active implementation lane for local manifests and verification, with the first CLI slice landed and follow-on schema/operator work still open.
