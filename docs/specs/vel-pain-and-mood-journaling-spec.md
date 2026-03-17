# Vel Pain and Mood Journaling Spec

## Purpose

Vel should support low-friction personal check-ins for mood and pain so those signals can shape daily orientation without turning the system into a medical analytics product.

This feature exists to answer practical questions such as:

- how am I doing right now
- is physical discomfort likely to distort my day
- has my state changed enough that Vel should frame attention and planning differently

## Product Intent

Mood and pain journaling are:

- fast manual check-ins
- private by default
- part of the same local-first capture system as other personal notes
- inputs to orientation, not diagnoses

They are not:

- a medical record
- a quantified-self dashboard
- a replacement for broader reflective journaling

## User Experience

### Mood entry

The user can record a mood check-in with:

- score from 1 to 10
- optional short label
- optional note

Examples:

- `vel journal mood 7 --label steady`
- `vel journal mood 4 --label fried --note "meeting overload"`

### Pain entry

The user can record a pain check-in with:

- severity from 0 to 10
- optional location
- optional note

Examples:

- `vel journal pain 3 --location neck`
- `vel journal pain 6 --location lower-back --note "worse after driving"`

## Data Model and Storage

This feature should reuse existing capture and signal infrastructure rather than introduce a separate journaling subsystem.

Canonical storage shape:

- each journal entry creates a private capture
- mood entries use capture type `mood_log`
- pain entries use capture type `pain_log`
- each entry also emits a structured signal with the same semantic type

Signal payloads:

- `mood_log` carries `capture_id`, `score`, optional `label`, optional `note`
- `pain_log` carries `capture_id`, `severity`, optional `location`, optional `note`

## Context and Explainability

The current-context snapshot should retain the latest mood and pain check-ins as typed summaries:

- `mood_summary`
- `pain_summary`

Those summaries must also be available through:

- `GET /v1/now`
- `GET /v1/explain/context`

Explain surfaces should show structured mood/pain summaries rather than forcing clients to interpret raw journal text.

## Constraints

- private by default
- local-first by default
- no derived medical claims
- no trend analytics requirement in the first slice
- no nudge policy changes are required in the first slice

## Initial Slice

The minimum viable implementation is:

1. API routes to record mood and pain entries
2. CLI commands for the same flows
3. capture plus signal persistence using existing storage primitives
4. latest mood/pain summaries included in current context, now, and explain
5. documentation and status updates that describe this as a shipped feature

## Deferred

- trends, streaks, charts, or longitudinal analytics
- symptom clustering or correlations
- medication or intervention recommendations driven by journal entries alone
- richer diary UIs on web or Apple clients
