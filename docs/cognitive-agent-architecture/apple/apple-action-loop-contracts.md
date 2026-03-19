---
title: Apple Action Loop Contracts
doc_type: spec
status: complete
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - apple
  - ios
  - watch
  - voice
  - health
  - phase-7
summary: Canonical Phase 07 contract vocabulary for backend-owned Apple voice turns, schedule snapshots, and bounded behavior summaries.
---

# Purpose

Publish the stable Phase 07 Apple contract vocabulary before backend behavior or Apple client surfaces widen.

# Owner Modules

| Contract Surface | Owner | Primary File |
| --- | --- | --- |
| Apple domain vocabulary | `vel-core` | `crates/vel-core/src/apple.rs` |
| Transport DTOs | `vel-api-types` | `crates/vel-api-types/src/lib.rs` |
| Apple shared client decoding | `VelAPI` | `clients/apple/VelAPI/Sources/VelAPI/Models.swift` |
| Schema/example publication | config assets | `config/schemas/apple-voice-turn.schema.json`, `config/schemas/apple-behavior-summary.schema.json` |

# Stable Vocabulary

## Voice intent categories

Phase 07 Apple voice turns may only classify intents into these backend-owned categories:

- `capture`
- `morning_briefing`
- `current_schedule`
- `next_commitment`
- `active_nudges`
- `explain_why`
- `behavior_summary`
- `complete_commitment`
- `snooze_nudge`

Clients may suggest or request intents through typed fields, but backend services remain the authority on interpretation, execution, and fallback behavior.

## Requested operation types

Apple voice turns use one explicit operation field:

- `capture_only`
- `query_only`
- `capture_and_query`
- `mutation`

This field exists so transcript persistence, query response, and safe action execution remain inspectable instead of being inferred from ad hoc client state.

## Behavior metric scope

Phase 07 behavior summaries are intentionally narrow. The only summary metrics are:

- `step_count`
- `stand_hours`
- `exercise_minutes`

Do not widen this summary vocabulary to sleep, heart analytics, or broader wellness interpretation in Phase 07.

# Contract Rules

- Persist transcript provenance before answering or mutating.
- Derive Apple schedule snapshots from backend-owned `Now` output instead of local Swift heuristics.
- Keep reasons and evidence first-class in the response so Apple surfaces can explain what they rendered.
- Swift renders backend decisions, queues low-risk offline work, and handles Apple permissions. Swift does not synthesize queries, rankings, or explanations locally.
- Bounded Apple behavior summaries must carry freshness so stale data is visible instead of guessed around.

# Published Artifacts

- `config/schemas/apple-voice-turn.schema.json`
- `config/examples/apple-voice-turn.example.json`
- `config/schemas/apple-behavior-summary.schema.json`
- `config/examples/apple-behavior-summary.example.json`

# Downstream Usage

- Backend routes and services should map domain contracts to DTOs at the boundary.
- Apple clients may cache these records, but policy ownership stays in Rust backend layers.
- Watch and iPhone surfaces should consume the same typed voice, schedule, and behavior payloads to avoid policy forks.
