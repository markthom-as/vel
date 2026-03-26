# Daily Loop v0 Implementation Target (Architecture / Schema / Internals / UX)

Working note only. This file is not shipped-behavior authority by itself.
Use `docs/MASTER_PLAN.md`, `.planning/ROADMAP.md`, the active milestone packet, and canonical user or architecture docs as higher authority.

This is a concrete v0 execution target based on the interview log and approved defaults.

## 1) What v0 must do

- Align morning + standup + overdue as a coherent local-first loop with explicit, resumable state.
- Add mood/body/sleep/dream check-in as durable, typed events with skip/scale/confidence metadata.
- Keep flow configurable and partial (skip any step).
- Add nudges and suppression modes without breaking existing run loop.
- Preserve event auditability for all meaningful state transitions.

## 2) Proposed architecture changes

- `core` domain
  - Add typed enums/structs for check-in events and suppression modes in `vel-core`:
    - check-in kind: `mood`, `body`, `sleep`, `dream`
    - nudge event action: open thread capture / open CLI capture / open main thread
    - suppression mode flags: `silent`, `vacation`, `break` as orthogonal booleans
  - Keep DTO/transport concerns out of domain.

- `service` layer
  - Extend daily loop orchestration service to support:
    - morning and standup state machines as separately startable phases
    - check-in event creation and per-item skip capture
    - overdue proposal->confirm->apply->undo flow with mixed-action batches
  - Add a nudge planner service:
    - urgency scoring + per-type scheduling/snooze
    - queue policies (max depth, queueing during active sessions)
    - escalation rules (5, 10, 15, 30, 60 ladder by default)
  - Add mode service:
    - supports auto-infer modes with confirmation mode default
    - user-overridden flags and tie-break with confidence
    - mode transitions written as events

- `route` boundary
  - Update or add narrow DTOs and route handlers for:
    - check-in submission (`type`, `text`, `scale`, `skip`, `skip_reason`, `answered_at`)
    - nudge queue inspection/control (dismiss/snooze batch APIs)
    - suppression mode introspection/controls
    - backup/export status polling and retry actions
  - Keep errors mapped at boundary, transport-only DTOs.

- `storage`
  - Create/extend repositories for:
    - typed check-in event table or partitioned stream
    - nudge queue table
    - mode snapshot/state table with source/confidence
    - backup/export job status + error state
  - Preserve existing run/event persistence and append-only semantics for check-ins.

## 3) Data model (v0)

- `check_in_events`
  - `id`, `user_id`, `type`, `text`, `scale`, `keywords`, `confidence`
  - `skipped`, `skip_reason`, `answered_at`, `created_at`, `source`
  - `session_id` optional, `version`/`schema_version`
  - `meta` for extracted signal fields as JSON object

- `nudge_events` / `nudge_queue`
  - `id`, `user_id`, `nudge_type`, `urgency`, `scheduled_for`, `status`, `snooze_count`
  - `default_action`, `suggested_actions`, `skip_reason`, `source`
  - `expires_at`, `last_error`, `created_at`, `updated_at`

- `suppression_modes`
  - `user_id`, `mode`, `active`, `source`, `confidence`, `reason`
  - `effective_from`, `effective_to`, `notes`

- `backup_jobs`
  - `id`, `target`, `scope`, `status`, `scheduled_for`, `attempts`, `last_error`, `progress`

## 4) Sequencing and UX contract

- Morning
  - Present configurable check-in prompts, including pain/mood/sleep/dream paths.
  - Accept skip per item and whole block.
  - Persist each user interaction as event.

- Standup
  - Standup can run without commitments.
  - If zero commitments, emit compact no-commit summary and continue overdue stage.

- Overdue
  - Proposal stage -> confirm required (default)
  - Apply mixed actions in one batch
  - Undo: last applied batch only
  - Record intent metadata at completion.

- Nudges
  - Lower-priority than active session by default; queue for resume later.
  - Support shared queue with urgency ranking and top-N suggestions.
  - Batch operations for dismiss/snooze.

- Modes
  - Silent/vacation/break are supported with both user and inferred triggers.
  - Defaults are configurable and user-override-capable.

## 5) Sync and conflict behavior

- Merge strategy on conflicting writes.
- For contested active states, use contextual thread-style conflict resolution.
- Resolve overdue in-flight conflicts by waiting for complete/settled action then merge.
- Maintain deterministic ordering in event stream (global per user/session for v0).

## 6) Backup and retention

- Retention: multiple weeks baseline, dev-configurable.
- Scheduled backup default: 24h (range 3..168h).
- Event-driven backup trigger: >=500 events or >=5m unsynced queue.
- Backup targets: NAS/S3/Drive with optional parquet cold tier.
- Export payload: raw + resolved + checksums.
- Encryption optional by backend, default off in v0.
- Retry failed backups with exponential backoff, degrade to local-only on unrecoverable failure by default, with optional explicit safety-blocking mode.
- Add compact recovery/queue view in CLI, plus optional thread-level recovery summary view.

## 7) Suggested implementation order

1. Schema+storage first: check-in events, nudge queue, mode state, backup-job records.
2. Daily loop service flow: morning/standup/overdue sequencing + persistence.
3. Nudge planner and mode controller + queue resolution API.
4. Event/audit trail hardening and conflict merge paths.
5. Backup cold path and CLI recovery view.

### Canonical artifacts for this slice

- Migration: `migrations/0051_v0_daily_loop_checks_nudges_backup.sql`
- API contract: [daily-loop-v0-sql-and-api-contract.md](/home/jove/code/vel/docs/notes/daily-loop-v0-sql-and-api-contract.md)

## 8) v0 acceptance checklist

- No hard reset on sync for overdue updates due to persisted session + event merge model.
- Check-in capture supports free-text, scales, skip, and append-only updates.
- Standup can skip/empty and still continue.
- Overdue supports mixed decisions with confirm/apply/undo and evented completion metadata.
- Nudge queue, snooze, skip-reasons, and mode suppression operate without blocking standup by default.
- Run/event retention and backup controls are visible and configurable.
