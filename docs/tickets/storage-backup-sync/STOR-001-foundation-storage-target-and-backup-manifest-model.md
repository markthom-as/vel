---
title: Foundation storage target and backup manifest model
status: done
owner: agent
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on: []
labels:
  - vel
  - storage
  - backup
  - trust
---

Define the provider-neutral storage target and backup manifest model for Vel artifacts.

Current state this ticket addresses:

- artifact rows store `storage_uri`, `storage_kind`, `sync_class`, hash, and size
- managed artifacts are usually local files under `artifact_root`
- some artifacts are logical or external references, not local files
- there is no first-class target, artifact-copy, manifest, verification, or restore-plan model

This ticket should add the surrounding model, not overload the base artifact row.

## Scope

- target kinds and target configuration
- target role and target status
- artifact-copy model per target
- backup manifest format
- manifest entry hashing and verification fields
- restore-plan metadata
- separation between backup, mirror sync, and active storage target
- normalization rules for `storage_uri`
- policy boundary between `sync_class` and target selection

## Required design decisions

1. Define a provider-neutral `storage_target_kind` set that includes:
   - `local_filesystem`
   - `rsync`
   - `s3`
   - `icloud_drive`
   - `google_drive`
   - `dropbox`
2. Define target roles separately from kinds:
   - `backup_only`
   - `mirror_sync`
   - `active_storage`
3. Define how target config is represented without embedding provider secrets into artifact rows.
4. Define an `artifact_copy` or equivalent record keyed by `artifact_id + storage_target_id`.
5. Define a manifest model that can capture provider path or key, expected hash and size, verification status, and provider version metadata where available.
6. Define restore-plan metadata sufficient for later restore tooling.
7. Define the rule that `sync_class` remains a temperature or priority hint, not the target-selection system.
8. Define the rule that `storage_uri` remains source-location metadata and must not be parsed as the primary backup contract.

## Expected outputs

- core-domain model additions or a concrete schema design
- storage-layer schema plan
- API and CLI boundary notes for later tickets
- doc updates that clearly distinguish current truth from planned backup behavior

## Current draft direction

The first concrete draft should define:

### Core-domain concepts

- `StorageTarget`
- `StorageTargetKind`
- `StorageTargetRole`
- `ArtifactCopy`
- `ArtifactCopyState`
- `BackupManifest`
- `BackupManifestScope`
- `BackupManifestState`
- `BackupManifestEntry`
- `VerificationRecord`
- `VerificationStatus`
- `RestorePlan`
- `RestorePlanItem`

### Storage-layer tables

- `storage_targets`
- `artifact_copies`
- `backup_manifests`
- `backup_manifest_entries`
- `verification_records`
- `restore_plans`
- `restore_plan_items`

### Boundary rules

- artifact rows remain the canonical source artifact record
- backup records are additive around artifacts, not replacements for them
- `storage_uri` remains source-location metadata
- target placement belongs on copy or manifest records
- runtime-local config remains separate from backup-target config

### Operator surface direction

- evolve `vel backup` into manifest and verification subcommands
- keep target inspection and verification summary compatible with later `doctor` or API surfaces
- do not treat `vel export` as the primary recovery system

## Completed outputs

- provider-neutral backup model and boundary draft in [docs/specs/vel-storage-backup-sync-spec.md](../../specs/vel-storage-backup-sync-spec.md)
- explicit storage-layer table plan for targets, copies, manifests, verification, and restore planning
- first-pass core domain scaffolding for those concepts in `vel-core`
- first-pass storage migration scaffold for those tables under `migrations/0033_storage_backup_foundation.sql`

## Acceptance criteria

- target kinds include `rsync`, `s3`, `icloud_drive`, `google_drive`, and `dropbox`
- the model also distinguishes target roles from target kinds
- the design introduces an artifact-copy record or equivalent per-target copy representation
- manifests record hashes, remote location, sync class, verification state, and timestamps
- verification state includes at least status, checked time, and failure reason
- restore-plan metadata is defined, even if workflows remain for later tickets
- runtime-local config remains separate from backup-target config
- the design does not require raw sqlite replication for the first pass
