---
title: Vel Uncertainty Runtime Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Purpose

Vel should not merely emit answers and hope they are correct. It should be able to say, in structured form:
- what it thinks
- how strongly it thinks it
- what evidence it used
- what is missing
- what it should do when confidence is too low

# Existing repo anchors

- `docs/specs/vel-uncertainty-architecture-spec.md`
- `migrations/0023_chat.sql` has a `confidence REAL`
- `migrations/0010_inferred_state.sql` has `confidence TEXT`
- `crates/veld/src/services/inference.rs` computes `attention_confidence`

The repo has ingredients, not yet a unified runtime.

# Target model

## Uncertainty record

Every meaningful decision step may emit an uncertainty record with:
- `subject_type` (`context`, `risk`, `suggestion`, `chat_reply`, `thread_link`, etc.)
- `subject_id`
- `decision_kind`
- `confidence_band`
- `score`
- `reasons_json`
- `missing_evidence_json`
- `resolution_mode`
- `resolved_at`

## Resolution modes

- `proceed`
- `ask_user`
- `ask_agent`
- `defer`
- `silent_hold`

# Concrete code changes

## A. Add persistence
Create:
- `migrations/0024_uncertainty_records.sql`

## B. Add domain model
Create:
- `crates/vel-core/src/uncertainty.rs`

## C. Add service helpers
Create:
- `crates/veld/src/services/uncertainty.rs`

Purpose:
- normalize scores into bands
- build durable reasons
- decide whether to proceed or escalate

# Acceptance criteria

- uncertainty is recorded in a consistent schema
- major decisions can emit structured uncertainty
- low-confidence situations can trigger explicit resolution behavior
