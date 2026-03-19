---
title: Backup And Operator Trust Contracts
doc_type: spec
status: draft
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - backup
  - restore
  - trust
  - phase-9
summary: Canonical Phase 09 contract vocabulary for typed backup packs, inspectable verification state, and manual-first restore posture.
---

# Purpose

Define the stable backup and operator-trust vocabulary before the runtime slices widen around backup export, status surfaces, or recovery flows.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Backup manifest schema and example | `config` | `config/schemas/backup-manifest.schema.json`, `config/examples/backup-manifest.example.json` |
| Backup trust DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |
| Operator backup guidance | `docs/user` | `docs/user/backup-and-restore.md` |
| Manifest registration | `config` | `config/contracts-manifest.json` |

# Stable Vocabulary

## Backup pack contents

A backup pack should tell the operator exactly what it contains:

- `backup_id`
- `created_at`
- `output_root`
- `database_snapshot_path`
- `artifact_coverage`
- `config_coverage`
- `explicit_omissions`
- `secret_omission_flags`
- `verification_summary`

This is a trust contract, not an opaque archive format. The operator should be able to inspect the manifest and understand what was captured, what was omitted, and whether verification succeeded.

## Coverage semantics

Coverage data must stay explicit and bounded:

- `artifact_coverage` lists durable artifact roots included in the pack and what was intentionally left out.
- `config_coverage` lists the non-secret runtime config files that were captured and the config surfaces that remain external.
- Coverage notes may explain why a path was omitted, but they must not bury omission rules in prose alone.

## Secret omission semantics

Backup packs are not a secret export mechanism.

- secret settings stay out of the pack by default
- integration tokens stay out of the pack by default
- local private key material stays out of the pack by default
- any later secret-migration contract must be explicit, narrow, and separately documented

## Verification semantics

Verification should be visible and checksum-backed:

- the manifest should record whether the pack was verified
- the checksum algorithm should be explicit
- checksum summary and inspected paths should be part of the operator-visible record
- verification failure should be obvious from status, not implied by the absence of a happy-path message

## Manual-first restore posture

Restore remains secondary to backup confidence in Phase 09.

- restore automation is intentionally not the center of the product
- the operator should inspect the backup pack before trusting it
- the docs should explain how to reason about restore manually before any automated restore path exists
- destructive restore behavior belongs in a later slice only if the roadmap explicitly widens it

# Contract Rules

- Keep the backup contract inspectable, typed, and explainable from persisted inputs.
- Do not turn backup/control into a generic configuration editor.
- Do not expose secret-bearing values in the manifest, example, or status surfaces.
- Use status surfaces to summarize trust posture, not to replace the backup manifest itself.

# Published Artifacts

- `config/schemas/backup-manifest.schema.json`
- `config/examples/backup-manifest.example.json`
- `config/contracts-manifest.json`
- `docs/user/backup-and-restore.md`

# Downstream Usage

- CLI and web surfaces should render backup confidence from typed manifest/status data, not ad hoc JSON blobs.
- Later runtime slices may add snapshot creation and inspection endpoints, but they should keep this contract stable.
- Manual restore guidance should continue to prefer inspect-before-trust semantics even when more automation arrives.
