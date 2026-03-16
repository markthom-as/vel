---
id: TKT-002
status: proposed
title: Define Apple-facing shared models and sync contract for Vel core
priority: P0
estimate: 2-4 days
depends_on: [TKT-001]
owner: agent
---

## Goal

Define the contract between Vel core and Apple clients so the mobile surface is driven by stable domain objects rather than ad hoc JSON archaeology.

## Scope

Define client-visible models for at least:

- `TaskSummary`
- `RoutineInstance`
- `ReminderWindow`
- `MedicationEvent`
- `CheckinPrompt`
- `Suggestion`
- `RiskState`
- `UpcomingEvent`
- `QuickAction`
- `VoiceInboxItem`
- `SyncCursor`

Define endpoints or sync operations for:

- fetch today timeline
- fetch upcoming reminders
- mark done / snooze / skip / defer
- record med taken
- fetch suggestion stack
- ingest voice note / text capture
- incremental sync since cursor

## Implementation notes

- Prefer OpenAPI or JSON Schema checked into Vel core and mirrored/generated into Swift
- Add explicit enums for action states instead of string soup
- Include server timestamps and version fields for conflict resolution
- Add `source_device_id` and `client_generated_id` for idempotent writes

## Deliverables

- API contract document
- Generated or hand-authored Swift DTOs
- Mock fixtures for previews/tests

## Acceptance criteria

- iOS and watchOS can build against mock data without live backend
- Write operations are idempotent
- Contract includes enough fields to support offline queueing and reconciliation
