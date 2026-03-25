---
title: Artifact Store and Run Linkage
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 027-sqlite-run-store
labels:
  - artifacts
  - vel-cli
  - storage
  - phase-5
---

# Context & Objectives

Provide durable artifact output for runs using filesystem-backed storage with typed metadata references in SQLite.

# Impacted Files & Symbols

- **Crate**: `crates/vel-storage`
  - **Symbols**: artifact metadata persistence and lookup
- **Crate**: `crates/veld`
  - **Symbols**: artifact emit path from run execution
- **Directory**: `var/artifacts/`
  - **Symbols**: artifact payload files

# Technical Requirements

- Persist artifact metadata (`artifact_id`, `run_id`, kind, mime/type, path, created_at).
- Store artifact payloads on local filesystem under scoped directories.
- Ensure run deletion/archive policy preserves auditability expectations.
- Expose retrieval API for `vel artifacts <run-id>`.

# Implementation Steps (The How)

1. Add storage APIs for artifact write/index/read.
2. Define safe path strategy and artifact naming conventions.
3. Wire artifact emission into execution service and CLI query path.
4. Add tests for missing-file and stale-index failure cases.

# Acceptance Criteria

1. [x] A run can persist one or more artifacts with metadata linkage.
2. [x] Artifact listing by `run_id` is deterministic and complete.
3. [x] Missing-file artifacts return explicit recoverable errors.
4. [x] Artifact payload writes stay in configured artifact root.

# Verification & Regression

- **Integration Test**: emit artifacts and list/retrieve by run id.
- **Integration Test**: stale metadata path reports clear error without panic.
- **Security Test**: path traversal attempt is rejected.
