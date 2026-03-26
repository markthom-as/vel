# NAS Backup And Cold-Storage Export Spec

Status: future planning spec, not shipped behavior
Last updated: 2026-03-25

## Purpose

This document defines a future Vel capability for exporting durable operator data to a NAS-backed filesystem layout that supports both:

- Vel-native local-first recovery and re-ingestion
- offline analytics and experiments over derived parquet datasets

The intent is to extend Vel's existing backup and local-source model, not replace it with a generic data lake. Vel should continue to treat inspectable local files, typed snapshots, manifests, and verification evidence as the trustworthy substrate. Parquet should exist as a derived optimization layer for analysis workloads, not as the only durable truth.

## Non-Authority Note

This file is planning material only.

Current shipped behavior still follows:

- [MASTER_PLAN.md](../MASTER_PLAN.md)
- [README.md](../../README.md)
- [docs/user/backup-and-restore.md](../user/backup-and-restore.md)
- [docs/user/integrations/local-sources.md](../user/integrations/local-sources.md)
- [docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md](../cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md)
- [docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md](../cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md)
- [crates/veld/src/services/backup.rs](/home/jove/code/vel/crates/veld/src/services/backup.rs)

Those files describe the current local backup pack, local-source ingestion, and connector posture. This spec describes an additional future export lane that should be implemented through those seams.

## Problem

Vel already has a backup workflow centered on:

- SQLite snapshots
- durable artifacts
- public runtime config
- explicit omissions
- verification through a typed manifest

Vel also already prefers local-source ingestion for many important domains:

- notes directories
- local JSON snapshots
- local export files

That creates a gap for operators who want a stronger long-term data posture:

- copy important state to a NAS or other operator-controlled storage root
- keep raw source exports such as Google Takeout immutable
- normalize exported data into durable, provenance-bearing files
- run analytics and experiments without treating the live SQLite database as the only query surface
- preserve explainability and re-ingestability

Without a first-class export job, operators are left to improvise ad hoc scripts and directory structures. That weakens trust, increases schema drift, and makes future import/recovery work harder than it needs to be.

## Design Goal

Vel should support a first-class export job that writes to an operator-configured filesystem target such as a NAS, using a stable directory and manifest model that is:

- local-first
- inspectable
- versioned
- provenance-bearing
- reproducible
- verification-backed
- compatible with future replay and re-ingestion

The exported filesystem should support two distinct roles:

1. durable normalized records for Vel-facing trust and re-ingestion
2. derived columnar datasets for offline exploration and experiments

## Design Principles

### 1. Normalized Files Are The Durable Truth Outside The Live Database

The durable external truth should be normalized JSON or NDJSON files plus manifests, not only parquet.

Reasons:

- line-oriented JSON is easy to diff, inspect, sample, recover, and patch
- it preserves domain structure cleanly without forcing a single analytics engine
- it aligns with Vel's existing local snapshot and local file connector model
- it can be turned into parquet repeatedly without information loss if the normalized schema is well designed

### 2. Parquet Is Derived And Disposable

Parquet should be treated as a convenience tier for:

- DuckDB
- Polars
- Arrow-native notebooks
- aggregation-heavy analysis
- historical experiments

If parquet files are missing or stale, the normalized files and manifests should still be sufficient for trust, recovery, and future derivation.

### 3. Raw Exports Remain Immutable

External source exports such as Google Takeout should be copied into a raw lane unchanged.

Vel should never silently mutate or overwrite the raw export. Normalization and derivation happen into separate paths.

### 4. Export Is An Extension Of Backup, Not A Parallel Product

The operator should experience this as part of Vel's existing backup and trust posture:

- backup pack
- export root
- manifest
- verification
- retention
- inspect-before-trust

This should not become a second unrelated subsystem with incompatible state semantics.

### 5. Source Provenance Must Survive Every Stage

Every normalized record should be explainable from:

- its source family
- provider key
- source mode
- source path
- upstream identifier where available
- export run identifier
- transformation version
- timestamps for source generation and normalization

That provenance should also flow into derived parquet datasets.

## Intended Operator Outcomes

The operator should be able to:

1. configure one or more export targets, including a NAS root
2. run or schedule export jobs from Vel itself
3. inspect what was written and why
4. verify that the export is internally consistent
5. browse raw, normalized, and derived outputs by domain
6. re-run normalization or parquet derivation without losing raw inputs
7. point future Vel import jobs back at normalized outputs
8. run offline experiments over parquet without endangering the live database

## Recommended Target Filesystem Shape

For a NAS root such as `/mnt/candnas/jove/knowledge/google`, the recommended structure is:

```text
<target-root>/
  README.txt
  manifests/
    export_run_2026-03-25T23-14-09Z.json
    export_run_2026-03-26T07-05-41Z.json
  raw/
    2026-03-25_google_takeout/
      source-manifest.json
      archive/
        Takeout.zip
      extracted/
        Takeout/
          ...
    2026-03-28_google_takeout/
      source-manifest.json
      archive/
        takeout-part-001.zip
        takeout-part-002.zip
      extracted/
        Takeout/
          ...
  normalized/
    calendar/
      schema-v1/
        2026/
          2026-03.ndjson
    contacts/
      schema-v1/
        contacts_current.ndjson
    drive/
      schema-v1/
        2026/
          2026-03.ndjson
    keep/
      schema-v1/
        2026/
          2026-03.ndjson
    location/
      schema-v1/
        2026/
          2026-03.ndjson
    mail/
      schema-v1/
        2026/
          2026-03.ndjson
    my_activity/
      schema-v1/
        2026/
          2026-03.ndjson
    photos/
      schema-v1/
        2026/
          2026-03.ndjson
    youtube/
      schema-v1/
        2026/
          2026-03.ndjson
  parquet/
    calendar/
      schema-v1/
        year=2026/month=03/part-000.parquet
    contacts/
      schema-v1/
        snapshot_date=2026-03-25/part-000.parquet
    drive/
      schema-v1/
        year=2026/month=03/part-000.parquet
    keep/
      schema-v1/
        year=2026/month=03/part-000.parquet
    location/
      schema-v1/
        year=2026/month=03/part-000.parquet
    mail/
      schema-v1/
        year=2026/month=03/part-000.parquet
    my_activity/
      schema-v1/
        year=2026/month=03/part-000.parquet
    photos/
      schema-v1/
        year=2026/month=03/part-000.parquet
    youtube/
      schema-v1/
        year=2026/month=03/part-000.parquet
  work/
    export_run_2026-03-25T23-14-09Z/
      logs/
      temp/
      reports/
```

## Why This Directory Shape

This structure intentionally separates:

- raw inputs
- normalized durable outputs
- derived analytical outputs
- manifests and verification records
- temporary working state

That separation preserves operator trust. It also makes it possible to:

- delete and rebuild parquet without touching normalized truth
- inspect raw inputs independently from transformed records
- track schema version upgrades per domain
- attach retention rules by lane
- diff export runs without parsing proprietary metadata

## Export Root Semantics

An export target should be modeled as a typed root with explicit behavior, not only a string path.

Proposed logical fields:

- `target_id`
- `label`
- `root_path`
- `target_kind`
- `durability_class`
- `allowed_export_lanes`
- `retention_policy`
- `writable`
- `verification_mode`

Suggested enums:

- `target_kind`: `local_disk | nas | removable | cloud_mounted`
- `durability_class`: `primary_backup | secondary_backup | cold_storage | analytics_only`
- `verification_mode`: `manifest_only | checksums | checksums_and_reopen`

## Data Classes

The export job should distinguish several kinds of output.

### 1. Backup Pack Data

This is the existing Phase 09 backup contract:

- SQLite snapshot
- artifacts
- public config
- omission list
- verification summary

This remains necessary for straightforward operator recovery.

### 2. Raw External Source Archives

This includes copied upstream exports such as:

- Google Takeout zip bundles
- split archive sequences
- local app exports
- source directory snapshots when a provider does not emit a single archive

These should be immutable once copied into the export root.

### 3. Normalized Domain Records

These are domain-specific records produced by Vel-owned normalization logic.

Properties:

- line-oriented where possible
- schema-versioned
- provenance-bearing
- domain-typed
- re-ingestable or at least mechanically transformable into future Vel import DTOs

### 4. Derived Analytical Tables

These are parquet outputs, partitioned by stable keys such as:

- year
- month
- snapshot date
- provider
- source account

These tables are for experiments and analysis, not the primary trust lane.

### 5. Export Run Metadata

Each export run should record:

- what it attempted
- what it wrote
- what it skipped
- what it derived
- what failed
- whether verification passed

## Normalized Record Envelope

Every normalized record should contain a shared metadata envelope plus domain payload.

Proposed minimum shape:

```json
{
  "schema_version": "v1",
  "record_kind": "google_keep_note",
  "source_family": "notes",
  "provider_key": "google_keep",
  "source_mode": "device_export",
  "export_run_id": "exp_20260325T231409Z_01",
  "source_export_id": "src_20260325_google_takeout",
  "source_path": "raw/2026-03-25_google_takeout/extracted/Takeout/Keep/note-1.json",
  "external_id": "keep_abc123",
  "account_ref": null,
  "record_timestamp": "2026-03-20T18:11:00Z",
  "normalized_at": "2026-03-25T23:14:21Z",
  "transform_version": "google-keep-v1",
  "content_hash": "sha256:...",
  "payload": {
    "title": "Ideas",
    "text": "Build Vel export lane",
    "labels": ["vel"]
  }
}
```

### Envelope Rules

- `schema_version` is the normalized schema version, not the upstream source schema version
- `record_kind` is the typed domain record identity
- `source_family` and `provider_key` must align with canonical integration vocabulary
- `source_path` should be relative to the export target root where feasible
- `external_id` should be stable when the upstream source offers one
- `content_hash` should identify the normalized payload or the raw source content, with the chosen rule documented clearly
- `payload` should contain typed domain fields, not arbitrary nested provenance again

## Domain Modeling Expectations

Each domain should define a narrow and useful normalized record family rather than dumping raw upstream JSON blobs.

### Calendar

Potential record kinds:

- `calendar_event`
- `calendar_event_instance`
- `calendar_calendar`

Expected fields:

- event title
- start and end time
- timezone
- all-day flag
- recurrence summary
- organizer
- attendees
- location
- notes
- source status

### Contacts

Potential record kinds:

- `contact_person`
- `contact_organization`
- `contact_method`

Expected fields:

- display name
- aliases
- email addresses
- phone numbers
- postal fields
- notes
- organization names

### Drive

Potential record kinds:

- `drive_file`
- `drive_revision`
- `drive_permission_summary`

Expected fields:

- file ID
- title
- mime type
- owners
- created and modified timestamps
- parent refs
- export format availability

### Keep

Potential record kinds:

- `google_keep_note`
- `google_keep_list_item`

Expected fields:

- note ID
- title
- body text
- labels
- archived and pinned flags
- reminder data
- collaborators if present

### Location

Potential record kinds:

- `location_visit`
- `location_segment`
- `location_point_sample`

Expected fields:

- timestamp range
- lat/lon
- place label
- confidence where provided
- source precision class

### Mail

Potential record kinds:

- `mail_message`
- `mail_thread`
- `mail_label`

Expected fields:

- message ID
- thread ID
- subject
- sender
- recipients
- sent and received timestamps
- label set
- snippet
- body pointers or extracted text summary

### My Activity

Potential record kinds:

- `activity_event`
- `activity_search`
- `activity_watch`
- `activity_navigation`

Expected fields:

- event timestamp
- product
- title
- URL
- device or client hint
- action class

### Photos

Potential record kinds:

- `photo_asset`
- `photo_album_membership`

Expected fields:

- asset ID
- original filename
- media type
- timestamps
- album membership
- geodata

### YouTube

Potential record kinds:

- `youtube_watch_event`
- `youtube_search_event`
- `youtube_subscription`

Expected fields:

- video ID
- title
- channel
- watch timestamp
- URL

## Raw Source Manifest

Each copied upstream export should include a small source manifest under its raw run folder.

Example:

```json
{
  "source_export_id": "src_20260325_google_takeout",
  "provider_key": "google_takeout",
  "source_family_set": [
    "calendar",
    "notes",
    "documents",
    "messaging"
  ],
  "copied_at": "2026-03-25T23:14:15Z",
  "copied_from_path": "/home/jove/Downloads/Takeout.zip",
  "archive_files": [
    {
      "relative_path": "archive/Takeout.zip",
      "size_bytes": 123456789,
      "sha256": "..."
    }
  ],
  "extracted_root": "extracted/Takeout",
  "notes": [
    "Copied as immutable raw source before normalization."
  ]
}
```

## Export Run Manifest

The existing backup manifest schema should remain stable for the current backup pack. This export job likely needs a sibling schema rather than silently overloading the current one.

Proposed new manifest concept:

- `export-run-manifest.schema.json`

This manifest would capture:

- export run identity
- target root
- source inputs copied
- domains normalized
- domains derived to parquet
- record counts
- file counts
- warnings
- failures
- verification summary
- tool or transform versions

Example fields:

- `export_run_id`
- `started_at`
- `finished_at`
- `target_root`
- `job_kind`
- `backup_ref`
- `source_exports`
- `normalized_outputs`
- `derived_outputs`
- `verification_summary`
- `warnings`
- `errors`

## Relationship To Existing Backup Packs

The export job should be able to reference or include a conventional backup pack, but the concepts should stay distinct.

### Recommended Relationship

- the existing backup pack remains focused on direct recovery of Vel runtime state
- the export run produces operator-facing knowledge exports and cold-storage datasets
- an export run may optionally embed or link to a backup pack created in the same run

That enables two useful modes:

1. `backup-only`
2. `backup-plus-export`

## CLI Shape

The CLI should eventually expose a bounded family of commands such as:

```bash
vel export create --target nas-main
vel export create --target nas-main --include google_takeout
vel export create --target nas-main --normalized-only
vel export create --target nas-main --derive parquet
vel export inspect <export_root>
vel export verify <export_root>
vel export derive-parquet <export_root> --domain keep
vel export re-normalize <export_root> --domain mail
```

Alternative:

- keep `vel backup` and add export sub-options there

Recommended posture:

- use a distinct `vel export` command family
- allow shared backend code with `vel backup`
- preserve conceptual clarity between runtime recovery and knowledge export

## API Shape

Possible future endpoints:

- `POST /v1/export/create`
- `POST /v1/export/inspect`
- `POST /v1/export/verify`
- `POST /v1/export/derive`
- `GET /v1/export/status`

The route handlers should stay thin:

- parse request
- auth and capability check
- invoke export service
- map typed service result into DTOs

The service layer should own:

- file planning
- copy logic
- normalization orchestration
- parquet derivation orchestration
- verification
- manifest writing

## Configuration Shape

This feature should be config-driven and explicit.

Potential config section:

```toml
[exports]
enabled = true
default_target = "nas-main"

[[exports.targets]]
id = "nas-main"
label = "CandNAS"
root_path = "/mnt/candnas/jove/knowledge"
target_kind = "nas"
durability_class = "primary_backup"
writable = true
verification_mode = "checksums"

[exports.schedule]
enabled = true
cron = "0 3 * * *"
create_backup_pack = true
normalize_exports = true
derive_parquet = false
```

Additional config should cover:

- included domains
- retention
- max work directory size
- whether raw source copy is enabled
- whether delete or pruning is allowed
- allowed writable targets

## Scheduling Model

The export job should support:

- operator-triggered runs
- scheduled runs
- dry runs
- one-shot derivation without source copy

### Recommended Scheduling Rules

- only one export run per target at a time
- runs should emit stable run IDs and lifecycle events
- scheduled runs should skip or queue when the previous run is still active
- retries should be bounded and visible

## Job Stages

A full run might have these stages:

1. resolve target and config
2. validate write access and free space
3. create work directory and run record
4. optionally create linked backup pack
5. discover or copy raw source exports
6. normalize selected domains
7. verify normalized outputs
8. optionally derive parquet outputs
9. verify derived outputs
10. write export manifest
11. persist terminal state and summary
12. apply retention rules

Each stage should produce structured logs and run events.

## Verification Requirements

Verification should be stronger than "files exist."

### Raw Lane Verification

- archive count matches manifest
- sizes recorded
- checksums recorded
- extracted root exists when extraction was requested

### Normalized Lane Verification

- every output file parses
- every line parses for NDJSON outputs
- schema version is explicit
- record counts match manifest
- required envelope fields exist
- relative source paths resolve within the export root

### Parquet Lane Verification

- file opens through the chosen parquet reader
- row count matches derivation summary
- schema fingerprint is recorded
- partition directories match declared partition keys

### Cross-Lane Verification

- normalized record counts are compatible with derived counts
- every derived table references an originating normalized dataset
- manifest checksums are stable

## Retention Policy

Retention must be explicit because raw exports and derived tables can grow quickly.

Suggested policies:

- keep all manifests
- keep all raw source runs unless the operator explicitly prunes them
- keep all normalized outputs by default
- allow parquet to be pruned and rebuilt
- allow work directories to be deleted automatically after success

Retention should be per lane, not one blanket delete policy.

## Import And Re-Ingestion Story

This export feature becomes much more valuable if future Vel import flows can read normalized exports directly.

That implies:

- normalized domain schemas should be designed for future import compatibility
- import jobs should accept normalized export roots as local sources
- manifest metadata should allow import code to discover available domains and schema versions

A future operator story should be:

1. copy raw export to NAS
2. Vel normalizes it
3. Vel later ingests from the normalized root without needing the original upstream archive every time

## Security And Trust Rules

### No Secret Expansion

This feature must not become a side door for exporting secrets into cold storage.

By default, it should omit:

- API credentials
- OAuth refresh tokens
- private keys
- decrypted secret material

If a future export mode needs encrypted secret escrow, that should be a separate explicit contract.

### Fail Closed On Unknown Sources

If a requested source export type is unknown:

- do not silently copy arbitrary directories
- record a visible denial or unsupported-source result

### Keep Read Scope Separate From Write Scope

The job may inspect broad source trees or exported archives, but writes should remain limited to the configured target root and work directory.

### Avoid Prompt-Visible Secrets

If an agentic or assistant-mediated orchestration path triggers exports, it must still rely on configured server-side target paths and bounded capabilities rather than freeform file writes.

## Observability

Export jobs should emit:

- stable export run IDs
- stage transitions
- counts written per domain
- bytes copied
- warnings
- failures
- verification outcomes
- retention actions

These should be inspectable from:

- CLI
- logs
- status endpoints
- future Settings or backup/export dashboards

## Recommended Implementation Split

### Phase A: Contract And Config

Add:

- export target config
- export run manifest schema
- example manifest
- authority docs and examples

Do not implement heavy transforms yet.

### Phase B: Raw Copy And Backup Linking

Add:

- target root resolution
- raw export copy lane
- source manifest
- optional linked backup pack

### Phase C: Normalized Domain Export

Start with a narrow proving set:

- notes-like sources
- local snapshot JSON sources
- one concrete external export family such as Google Takeout Keep or Calendar

### Phase D: Parquet Derivation

Add:

- domain-by-domain derivation
- verification
- partitioning strategy

### Phase E: Scheduled Runs And UI Status

Add:

- scheduled export jobs
- status surfaces
- retention management

## Why Start Narrow

The failure mode here is obvious: trying to turn "NAS export" into a universal knowledge platform in one step.

A narrow proving lane is better:

- one target root type
- one manifest family
- a few high-value domains
- one derivation engine
- bounded retention rules

That keeps the feature reviewable and consistent with Vel's architecture discipline.

## Explicit Non-Goals

This spec does not imply:

- replacing SQLite with parquet
- making parquet the primary runtime database
- exporting every possible provider on day one
- building a generic ETL platform inside Vel
- supporting arbitrary user-written transformations without review
- automatic destructive restore from NAS exports

## Open Questions

1. Should export manifests be a sibling schema to backup manifests or a superset with a typed `job_kind` discriminator?
2. Which normalization engine should own parquet derivation: Rust-native Arrow/parquet, DuckDB subprocess, or an external bounded tool runner?
3. Should raw Google Takeout archives be copied into Vel-managed storage automatically or only when the operator points at them explicitly?
4. Which first proving domains deliver the most value with the least schema churn?
5. Should normalized outputs be one-file-per-month NDJSON, one-file-per-run NDJSON, or append-only segmented logs with compacted snapshots?
6. How much of the export lane should be available to assistants versus operator-only surfaces?

## Implementation Recommendation

The first implementation should be opinionated:

- one configured NAS target
- one new export manifest schema
- explicit raw, normalized, parquet, manifests, and work lanes
- normalized NDJSON as durable truth
- parquet as optional derived output
- strong verification and explicit omissions

That is the smallest version that materially improves operator trust without eroding Vel's local-first architecture.
