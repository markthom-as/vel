---
title: Now Surface Rust-Core Contract
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-21
updated: 2026-03-21
keywords:
  - now
  - rust core
  - task model
  - thread routing
  - offline
index_terms:
  - rust-owned now surface
  - now dto contract
  - platform portable now
related_files:
  - docs/product/now-surface-canonical-contract.md
  - docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - crates/vel-api-types/src/lib.rs
  - crates/vel-core/src/lib.rs
  - crates/veld/src/services/now.rs
summary: Architectural contract for implementing the canonical Now surface in platform-portable Rust core and shared transport, not in shell-specific logic.
---

# Purpose

Translate the product contract in [now-surface-canonical-contract.md](./../../product/now-surface-canonical-contract.md) into an explicit Rust-owned implementation rule.

The canonical `Now` surface must be implemented in platform-portable Rust core and service layers.

Web, Apple, CLI, and future shells may adapt layout and interaction density, but they must consume one shared `Now` contract instead of reconstructing product behavior locally.

# Core Rule

All `Now` product semantics belong in Rust-owned layers.

That includes:

- title and display metadata resolution
- icon-bar bucket definitions and counts
- sync/offline status and queued-write counts
- context resolution
- context one-liner generation and deterministic fallback
- nudge stack ordering and lifecycle state
- canonical task selection and ranking
- thread open priority and thread filter categories
- docked-input routing outcomes
- day-object boundary logic
- offline merge and stale-state markers where they affect the `Now` contract

Shells must not re-derive these semantics from raw state.

# Required Portable Contract Areas

## 1. Now header contract

Rust-owned contract must provide:

- resolved display title or the inputs needed to resolve it deterministically
- icon-bar buckets
- per-bucket counts
- urgency markers
- sync/offline summary
- queued-write count

## 2. Status-row contract

Rust-owned contract must provide:

- current date/time display inputs
- resolved `context_label`
- resolved `elapsed_time`
- explicit fallback states

Shells may format timestamps for locale, but they must not choose different context resolution rules.

## 3. Context-summary contract

Rust-owned layers own:

- backend-generated one-liner
- deterministic fallback summarizer
- update triggers
- thread target selection priority

The fallback summarizer must not live only in a single shell.

## 4. Nudge-stack contract

Rust-owned layers own:

- active bar list
- bar type
- severity
- action set
- snooze presets
- lifecycle state
- resurface and escalation metadata

Clients preserve order and render allowed actions; they do not invent local ranking.

## 5. Canonical task contract

Rust-owned layers must converge on one portable task model that supports:

- `task_kind`
- current task selection
- next ranked items
- recent completed items
- source badges
- project/container metadata
- timing and overrun markers
- reversible completion posture

`Now` must stop depending on shell-local distinctions between commitments, tasks, and nearby work.

## 6. Docked input routing contract

Rust-owned layers own:

- intent classification
- capture/request/routing result
- required thread artifact creation
- associated thread linkage
- inline versus thread versus inbox outcome

Voice and text parity rules belong in shared product logic even when microphone APIs are shell-owned.

## 7. Day and thread continuity contract

Rust-owned layers own:

- `day` identity and boundary rules
- `primary_thread_id` linking
- thread category filters
- open-target priority
- continuation status chips that remain visible on `Now`

## 8. Offline / mesh contract

Rust-owned layers own the authoritative state model for:

- sync health
- stale/local-only/synced distinctions
- offline write queue summary
- merge/conflict posture when state returns from another client

Shells may present transport diagnostics, but they do not define conflict rules.

# Platform-Portability Rule

If a `Now` behavior cannot be consumed by:

- web
- iPhone
- iPad
- Mac
- reduced Apple Watch

through one shared Rust-owned contract, then it is not implemented at the correct layer yet.

# Anti-Patterns

The following are explicitly disallowed:

- shell-only fallback summaries
- shell-only thread-category definitions
- shell-only task ranking or current-task resolution
- shell-specific snooze semantics
- shell-specific day-boundary logic
- shell-local connection/linking rules that affect `Now` truth

# Client Mesh Rule

Cross-client continuity and linking are part of the same portable product-core lane.

That means Rust-owned layers must also provide the shared state and policy needed for:

- client connection status
- linking and recovery summaries
- safe endpoint selection hints
- multi-client continuity markers shown in `Now` or support surfaces

Client-mesh onboarding may have shell-native presentation, but the connection and continuity model must remain shared and portable.
