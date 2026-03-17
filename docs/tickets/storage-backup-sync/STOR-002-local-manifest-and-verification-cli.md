---
title: Local manifest and verification CLI
status: in_progress
owner: agent
type: implementation
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - STOR-001-foundation-storage-target-and-backup-manifest-model.md
labels:
  - vel
  - storage
  - backup
  - cli
---

Add local manifest generation and verification tooling before any remote target integrations.

Current state this ticket evolves:

- `vel backup` only prints manual copy guidance
- `vel export` exports metadata, not artifact blobs or manifests
- `vel doctor` validates the local artifact directory, not backup integrity
- artifact metadata may already include hashes and size, but local files remain the primary source of truth for this phase

## Scope

- manifest writer for artifact inventory
- content-hash generation
- local verification command(s)
- inspection output for missing or mismatched artifact copies
- machine-readable output for automation
- mismatch taxonomy for local-first verification

## Required design decisions

1. Decide how `vel backup` evolves:
   - extend the existing command, or
   - add subcommands such as `vel backup manifest` and `vel backup verify`
2. Define the first manifest scope:
   - full local artifact inventory, or
   - filtered inventory by artifact class or path scope
3. Define verification mismatch classes at minimum:
   - missing file
   - unreadable file
   - hash mismatch
   - metadata without blob
   - blob without manifest entry
4. Decide when existing `content_hash` values can be reused and when fresh hashing must be computed.
5. Define operator output for both human-readable and `--json` modes.
6. Define reusable verification result structures so later `doctor` or status surfaces can summarize them without duplicating logic.

## Completed first slice

- `vel backup` remains the manual guidance entrypoint for operators who just need the current storage roots
- `vel backup manifest create` now emits a local-first manifest from the artifact inventory
- `vel backup manifest verify` now checks local files for missing blobs, unreadable files, hash mismatch, size mismatch, metadata-without-blob cases, and extra files under `artifact_root`
- both manifest create and verify support machine-readable JSON output

## Remaining work

- decide whether the local manifest JSON shape should become the persisted canonical manifest contract or a CLI-only precursor
- add fixture coverage around manifest generation and verification edge cases
- decide how verification summaries should later surface through `doctor` or adjacent operator status endpoints

## Acceptance criteria

- operators can generate a manifest from current local artifact state
- operators can verify the manifest against local files
- verification reports at least the core mismatch classes called out in this ticket
- the commands support both concise operator output and machine-readable output
- the flow reuses the current local artifact inventory under `artifact_root`
- this becomes the shared substrate for later provider targets
