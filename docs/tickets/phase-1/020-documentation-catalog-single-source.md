---
title: Documentation Catalog Single Source & Surface Parity
status: complete
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 011-documentation-truth-repair
labels:
  - docs
  - developer-experience
  - repo-wide
  - dry
  - phase-1
---

# Context & Objectives

Vel now has a consistent authority chain, but the surfaced documentation catalogs are still duplicated across multiple code paths:

- the CLI docs command
- the web Settings documentation cards
- the Apple documentation catalog
- the repo verifier and related maintenance scripts

That duplication is already a DRY and consistency risk. This ticket creates one canonical catalog source and derives surfaced views from it instead of hand-maintaining the same doc list in several languages.

# Impacted Files & Symbols

- **File**: `crates/vel-cli/src/commands/docs.rs`
  - **Symbols**: `DOCS`, `run`
- **File**: `clients/web/src/components/SettingsPage.tsx`
  - **Symbols**: documentation card data
- **File**: `clients/apple/VelAPI/Sources/VelAPI/VelDocumentation.swift`
  - **Symbols**: `VelDocumentationCatalog`
- **File**: `scripts/verify-repo-truth.mjs`
  - **Symbols**: authority and queue checks
- **File**: `docs/README.md`
  - **Symbols**: authority entrypoints, queue entrypoints

# Technical Requirements

- **Single Source**: Define one canonical documentation catalog or manifest that owns the current repo entrypoints.
- **Derived Surfaces**: CLI, web, and Apple surfaces should derive from that source or from generated artifacts rather than hand-curated duplicate arrays.
- **Scope Discipline**: Allow surface-specific subsets, but not surface-specific truth.
- **Legacy Rejection**: No surfaced catalog should point to `docs/status.md`, `docs/architecture.md`, or other retired authorities.
- **Verification**: Add tests or checks that fail when surfaced catalogs diverge from the canonical source.

# Cross-Cutting Trait Impact
- **Modularity**: required — documentation discovery should have one owning manifest or generation boundary.
- **Accessibility**: required — every surfaced catalog should expose the same current authority entrypoints.
- **Configurability**: affected — surface-specific filtering rules should be explicit if they differ by client.
- **Data Logging**: affected — verifier failures should clearly identify which surfaced catalog drifted.
- **Rewind/Replay**: n/a — the catalog itself is static metadata rather than replayable runtime state.
- **Composability**: required — the canonical catalog should be reusable from Rust, TypeScript, Swift, and scripts without copy/paste drift.

# Implementation Steps (The "How")

1. **Inventory**: Enumerate all places that surface repo documentation entrypoints.
2. **Manifest**: Define the canonical catalog format and ownership location.
3. **Derive**: Update CLI, web, Apple, and verification surfaces to consume the canonical source or generated artifacts.
4. **Verify**: Add tests or script checks that fail on catalog divergence.

# Acceptance Criteria

1. [ ] One canonical source defines the surfaced documentation catalog.
2. [ ] CLI, web, and Apple doc surfaces no longer hand-maintain independent authority lists.
3. [ ] Legacy documentation paths cannot re-enter surfaced catalogs without a failing check.
4. [ ] The queue and docs guide explain how the shared catalog should be maintained.

# Verification & Regression

- **Rust Test**: `cargo test -p vel-cli docs_catalog_points_at_current_authority_docs`
- **Web Test**: `npm -C clients/web test -- --run src/components/SettingsPage.test.tsx`
- **Repo Check**: `node scripts/verify-repo-truth.mjs`
- **Invariants**: surfaced catalogs must contain the Master Plan and must not contain retired authority files

## Lane A Status Notes (2026-03-17)

- [x] Added automated retired-authority rejection for documentation catalogs (`docs/status.md`, `docs/architecture.md`) in both `scripts/sync-documentation-catalog.mjs` and `scripts/verify-repo-truth.mjs`.
- [x] Added automated invariant check that surfaced catalogs include `docs/MASTER_PLAN.md`.
- [x] Verified with: `node scripts/sync-documentation-catalog.mjs --check` and `node scripts/verify-repo-truth.mjs`.

# Agent Guardrails

- **No New Shadow Catalogs**: Do not solve this by adding another hardcoded list in a different language.
- **Prefer Machine-Readable Sources**: Favor a format that multiple surfaces can consume with minimal translation.
- **Keep It Small**: The catalog should represent durable entrypoints, not every doc in the repository.
