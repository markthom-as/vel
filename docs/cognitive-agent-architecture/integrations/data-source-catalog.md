---
title: Data Source Catalog
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - integrations
  - data sources
  - catalog
  - connectors
index_terms:
  - source inventory
  - integration inventory
  - provider catalog
related_files:
  - docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md
  - docs/user/integrations/README.md
  - crates/vel-core/src/integration.rs
summary: Canonical inventory of concrete Vel source providers, source modes, and rollout status by family.
---

# Purpose

Provide one concrete inventory of currently known Vel source providers and modes so architecture docs, user docs, and tickets reference the same source set.

# Catalog Rules

- Use `IntegrationFamily` names for top-level categories.
- Keep provider keys snake_case and stable.
- Every entry declares one source mode from the canonical connector contract.
- Do not add entries here without corresponding ticket and contract updates.

# Canonical Inventory

| Family | Provider Key | Source Mode | Status | Notes |
| --- | --- | --- | --- | --- |
| `calendar` | `local_ics` | `local_file` | shipped | Local `.ics` pull/sync path. |
| `calendar` | `google_calendar` | `credential_api` | shipped | Credential-backed sync with calendar selection controls. |
| `tasks` | `todoist_snapshot` | `local_snapshot` | shipped | Snapshot-backed local ingestion. |
| `tasks` | `todoist` | `credential_api` | shipped | API-backed task sync. |
| `activity` | `activity_snapshot` | `local_snapshot` | shipped | Workstation/activity snapshot ingestion. |
| `activity` | `apple_export` | `device_export` | planned | Apple-export oriented adapter lane. |
| `health` | `health_snapshot` | `local_snapshot` | shipped | Health snapshot ingestion. |
| `health` | `apple_export` | `device_export` | planned | Apple-export oriented adapter lane. |
| `git` | `git_snapshot` | `local_snapshot` | shipped | Repository activity snapshot ingestion. |
| `git` | `github` | `brokered_tool` | shipped | Bounded issue/comment/state write lane with typed project and people linkage. |
| `messaging` | `messaging_snapshot` | `local_snapshot` | shipped | Messaging snapshot ingestion. |
| `messaging` | `email` | `brokered_tool` | shipped | Draft-first reply lane with confirm-required send and typed people linkage. |
| `messaging` | `apple_export` | `device_export` | planned | Apple-export oriented adapter lane. |
| `notes` | `notes_path` | `local_directory` | shipped | File or directory-backed notes ingestion. |
| `transcripts` | `transcript_snapshot` | `local_snapshot` | shipped | Transcript snapshot ingestion. |
| `documents` | `local_export` | `local_snapshot` | planned | Document-import lane. |
| `gaming` | `local_export` | `local_snapshot` | planned | Personal-context signal lane. |
| `calendar` | `delegated_connector_runtime` | `delegated_connector` | planned | Connect/worker runtime integration path. |
| `tasks` | `delegated_connector_runtime` | `delegated_connector` | planned | Connect/worker runtime integration path. |

# Operational Notes

- Shipped status means available or represented in current runtime behavior; planned means architecture lane only.
- Local-source entries remain preferred defaults for reliability and inspectability.
- Credential-backed and delegated entries must follow capability mediation and explicit allowlists.
- `github` and `email` remain bounded write surfaces. They do not grant arbitrary provider mutation beyond the named Phase 06 operations.

# Acceptance Criteria

1. This file is the canonical concrete source inventory.
2. Integration and user docs can link here instead of restating provider lists ad hoc.
3. New source entries require matching ticket coverage and canonical contract updates.
