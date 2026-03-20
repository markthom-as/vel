---
title: Apple Local-First Voice Continuity Contract
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-20
updated: 2026-03-20
keywords:
  - apple
  - iphone
  - voice
  - offline
  - continuity
index_terms:
  - local-first apple voice
  - iphone voice continuity
  - offline thread draft
related_files:
  - docs/cognitive-agent-architecture/apple/apple-embedded-runtime-contract.md
  - docs/cognitive-agent-architecture/apple/apple-action-loop-contracts.md
  - clients/apple/README.md
  - config/schemas/apple-local-voice-continuity.schema.json
  - config/examples/apple-local-voice-continuity.example.json
summary: Canonical Phase 38 contract for iPhone local-first voice continuity, queued quick actions, cached Now posture, and thread-draft merge behavior.
---

# Purpose

Define the bounded local-first iPhone voice loop Vel is allowed to ship in Phase 38.

This document is the authority for:

- what counts as the minimum offline/local baseline on iPhone
- how queued voice continuity should relate to canonical thread continuity
- how cached `Now`, offline-safe quick actions, and local draft state should fit together
- what still remains daemon-backed even after the local-first loop exists

# Current Truth

Phase 37 already introduced:

- the additive iPhone embedded-capable seam
- bounded local helper flows for cached `Now` hydration and quick-action preparation

Phase 38 builds on that seam. It does not replace backend-owned Apple voice routes, thread continuity, or daemon-backed authority.

# Canonical Local-First Baseline

The minimum acceptable iPhone local-first behavior is:

- cached `Now`
- local voice capture with immediate acknowledgment
- queued offline-safe quick actions
- local thread draft continuation

All four should be explained as one recovery story, not separate fallback tricks.

# Magic Flow

The target interaction is:

1. tap and speak
2. get instant acknowledgment
3. survive offline
4. later see the result appear correctly in canonical thread and `Now` continuity
5. avoid duplicate, lost, or confusing state

# Local Ownership Versus Daemon Ownership

## iPhone Local-First May Own

- push-to-talk UX
- transcript capture and local draft persistence
- embedded helper logic for packaging/normalization
- offline queue state for safe replayable actions
- cached `Now` display and compact local status messages

## Daemon-Backed Authority Still Owns

- backend voice interpretation and backend-only answers
- canonical thread persistence and merge authority
- heavy recall and heavy reasoning
- shared history synchronization
- integrations and sync infrastructure
- review/apply lanes

# Continuity Rules

- offline voice and online voice must remain one thread-backed product model
- the iPhone may keep a local draft or queued continuity record, but it must treat that as provisional until daemon-backed merge happens
- the shell must not invent a second “offline-only thread” identity model
- merge should prefer canonical thread continuity once reachable again

# Quick-Action Rules

Offline-safe quick actions in this phase are limited to:

- capture create
- commitment create
- commitment done
- nudge done
- nudge snooze

Anything outside that bounded list must fail closed to “backend required” rather than guessing locally.

# Cached `Now` Rules

- cached `Now` may be used for orientation and compact local voice/status replies
- cached `Now` must remain clearly cached/provisional when offline
- cached `Now` must not be used as permission to invent new planner, review, or explainability semantics locally

# Required Checked-In Artifact

The machine-readable contract for this Phase 38 boundary is:

- [apple-local-voice-continuity.schema.json](/home/jove/code/vel/config/schemas/apple-local-voice-continuity.schema.json)
- [apple-local-voice-continuity.example.json](/home/jove/code/vel/config/examples/apple-local-voice-continuity.example.json)

# Acceptance Criteria

1. The minimum iPhone local-first baseline is explicit.
2. Offline and online voice continuity are described as one thread-backed model.
3. Safe offline quick-action scope is explicit and bounded.
4. Cached `Now` use is allowed but does not widen into local authority claims.
5. Remaining daemon-backed limits are explicit.
