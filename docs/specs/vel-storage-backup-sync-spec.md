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

Current implemented baseline:

- artifact rows store `storage_uri`, `storage_kind`, `sync_class`, `content_hash`, and size metadata
- managed artifacts are usually local files under `artifact_root`
- some artifacts are logical or external references rather than local files
- `vel backup` is still manual copy guidance
- `vel export` exports metadata, not recovery-ready artifact copies or manifests

This means Vel can explain where some artifacts live, but it cannot yet answer:

- what backup targets exist
- which artifacts should exist on which targets
- whether copies were verified
- how to restore a given artifact or tree from a specific target

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

The manifest is the Vel-native source of truth for backup state.

Provider APIs are transport mechanisms, not the primary durable record.

## 4. Start with artifact backup, not database replication

The first remote-target work should focus on:

- artifact manifests
- artifact copy/push/pull
- verification
- restore tooling

Do not begin with raw sqlite replication or multi-master file sync.

## 5. Preserve the current artifact model as the source artifact record

The current artifact row is still useful and should remain the canonical record for:

- artifact identity
- local or logical source location
- artifact privacy and sync temperature
- base content metadata

Backup work should add target, copy, and manifest concepts around artifacts.

Do not stretch `storage_uri` or `storage_kind` into a full backup system.

## 6. Operator surfaces should extend current CLI and doctor flows

Vel already has operator footholds:

- `vel backup`
- `vel export`
- `vel config show`
- `vel doctor`
- `vel inspect artifact`
- `vel artifact latest`

Backup and verification work should extend these surfaces rather than introduce a separate parallel operator model.

The first new flows should still be CLI-first and local-first.

## 7. Keep backup targets separate from runtime roots

`db_path` and `artifact_root` remain runtime-local settings.

Backup target configuration is separate. Remote target configuration should not redefine the local runtime storage roots.

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
- artifact copy
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

Minimum target model:

- `storage_target_id`
- `storage_target_kind`
- operator-visible label
- target role:
  - backup_only
  - mirror_sync
  - active_storage
- target root or prefix semantics
- enabled or disabled status
- last_success_at
- last_error

Minimum artifact-copy model:

- `artifact_id`
- `storage_target_id`
- target object path or provider location
- copy state
- copied hash
- copied size
- copied_at
- verified_at
- last_error

Minimum manifest model:

- `backup_manifest_id`
- `storage_target_id`
- manifest scope or type
- manifest state
- created_at
- completed_at
- verified_at

Minimum manifest-entry model:

- `artifact_id`
- provider path, key, or logical location on target
- expected content hash
- expected size
- source storage metadata snapshot
- optional version or generation identifier where the provider supports it

Minimum verification model:

- subject type:
  - local_artifact
  - artifact_copy
  - manifest
- status:
  - pending
  - verified
  - mismatch
  - missing
  - unreadable
  - error
- checked_at
- observed hash and size
- failure reason

Normalization rule:

- `storage_uri` remains either:
  - a relative managed path under `artifact_root`, or
  - an opaque external or logical URI
- backup logic must not depend on parsing arbitrary `storage_uri` strings as the primary backup contract
- provider-specific target locations belong on copy or manifest records, not on the base artifact row

Policy rule:

- `sync_class` remains a temperature or priority hint
- target selection, retention, and verification policy live in backup-target configuration and later policy structures

Restore rule:

- first-pass restore planning can be schema-only
- the model must still carry enough metadata to reconstruct a local artifact tree or fetch a specific artifact copy from a target

## Current gaps this spec closes

Today Vel has no first-class model for:

- backup targets
- per-target artifact copies
- manifest snapshots
- verification results
- restore planning

It also overloads `storage_uri` for three distinct things:

- local managed relative paths
- external opaque locations
- logical synthetic URIs such as `vel://...`

This spec resolves that by adding surrounding storage-target records rather than redefining artifacts themselves.

# Proposed first-pass model draft

This section is the concrete starting point for `STOR-001`.

It is a schema and boundary draft, not a claim that these tables or API shapes already exist.

## Core-domain additions

Add provider-neutral domain concepts around artifacts:

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

Recommended semantics:

- `StorageTarget`
  - describes a destination or managed storage surface
  - does not hold provider secrets directly in domain rows
- `ArtifactCopy`
  - represents one artifact on one target
  - is the durable per-target copy record
- `BackupManifest`
  - represents one manifest generation or sync snapshot for one target
- `VerificationRecord`
  - records observed verification outcomes for a local artifact, copy, or manifest
- `RestorePlan`
  - records a recoverability plan without requiring restore execution in the first pass

## Storage schema draft

Recommended first-pass tables:

### `storage_targets`

- `storage_target_id`
- `kind`
- `role`
- `label`
- `root_uri`
- `path_prefix`
- `provider_ref`
- `enabled`
- `metadata_json`
- `created_at`
- `updated_at`
- `last_success_at`
- `last_error`

Notes:

- `provider_ref` should point to config or credential material indirectly
- `root_uri` is target-root metadata, not an artifact path
- `metadata_json` can carry provider-specific non-secret options during the first pass

### `artifact_copies`

- `artifact_id`
- `storage_target_id`
- `copy_state`
- `target_locator`
- `target_version`
- `content_hash`
- `size_bytes`
- `copied_at`
- `verified_at`
- `last_error`
- `metadata_json`

Primary identity:

- unique on `artifact_id + storage_target_id + target_locator`

Notes:

- `target_locator` is the provider path, key, or object identifier on the target
- `target_version` is optional provider generation or version metadata

### `backup_manifests`

- `backup_manifest_id`
- `storage_target_id`
- `scope`
- `state`
- `started_at`
- `completed_at`
- `verified_at`
- `summary_json`
- `last_error`

### `backup_manifest_entries`

- `backup_manifest_id`
- `artifact_id`
- `artifact_copy_locator`
- `source_storage_uri`
- `source_storage_kind`
- `sync_class`
- `expected_content_hash`
- `expected_size_bytes`
- `target_version`
- `entry_state`
- `metadata_json`

### `verification_records`

- `verification_id`
- `subject_type`
- `subject_id`
- `status`
- `observed_content_hash`
- `observed_size_bytes`
- `failure_reason`
- `checked_at`
- `metadata_json`

### `restore_plans`

- `restore_plan_id`
- `source_target_id`
- `plan_state`
- `requested_at`
- `prepared_at`
- `executed_at`
- `destination_root`
- `summary_json`
- `last_error`

### `restore_plan_items`

- `restore_plan_id`
- `artifact_id`
- `target_locator`
- `target_version`
- `planned_destination`
- `item_state`
- `failure_reason`

## Boundary rules

Artifact rows remain the canonical source artifact record.

That means:

- `artifacts.storage_uri` stays on the artifact row
- `artifacts.storage_kind` stays on the artifact row
- `artifacts.sync_class` stays on the artifact row

New backup records are additive:

- target placement lives on `artifact_copies`
- snapshot membership lives on `backup_manifest_entries`
- observed integrity results live on `verification_records`

## Config boundary draft

Runtime config remains:

- `db_path`
- `artifact_root`

Backup-target config should be added separately, for example through a later `backup_targets` config block or a persisted target registry.

The first pass should support indirect provider references such as:

- local named target config
- env-backed provider credentials
- operator-managed external configuration

Do not put raw provider credentials into artifact or manifest rows.

## CLI and API surface draft

First-pass CLI evolution:

- `vel backup manifest create`
- `vel backup manifest verify`
- `vel backup target list`
- `vel backup target inspect <id>`

Early API surface, if added later:

- target listing and target inspection
- manifest listing and manifest inspection
- verification summary endpoints
- restore-plan inspection

The first shipped implementation can remain CLI-first as long as the underlying model is provider-neutral.

# First implementation sequence

1. define target model and manifest format
2. add local manifest and verification flow
3. add `rsync` target
4. add `s3` target
5. add cloud-drive targets
6. add restore verification and operator surfaces

Phase expectations:

1. STOR-001 defines the provider-neutral model and operator-facing boundary
2. STOR-002 turns `vel backup` from manual instructions into manifest generation plus local verification
3. provider targets reuse the same manifest and verification substrate
4. restore surfaces build on manifest state instead of bypassing it

# Non-goals for the first pass

- raw sqlite live sync
- true multi-master remote state replication
- treating Dropbox/Google Drive/iCloud as concurrent authoritative databases
- hiding provider differences behind fake sameness where restore semantics differ

# Acceptance criteria

- Vel has explicit tickets for `rsync`, `s3`, `icloud_drive`, `google_drive`, and `dropbox`
- backup targets are described as optional trust features, not current runtime truth
- the design distinguishes manifest-driven backup from broader cluster/client sync
- the spec defines target, artifact-copy, manifest, verification, and restore-plan concepts
- the spec keeps runtime-local config separate from backup-target config
- the spec preserves CLI-first operator surfaces for the first implementation pass
