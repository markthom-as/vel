# Daily Loop v0 Decision Table (Morning / Today / Standup / Overdue)

Working note only. This file is not shipped-behavior authority by itself.
Treat it as a distilled planning artifact derived from the interview log below, not as the final contract surface.

Source: [daily-loop-doc-mode-interview](/home/jove/code/vel/docs/notes/daily-loop-doc-mode-interview.md)

## Core Session Flow

- Session source-of-truth for behavior: `docs/user/daily-use.md`.
- Flow: `morning` and `standup` are separate configurable sessions.
- Dependency: `morning` is a prerequisite for `standup`; `overdue` is part of `standup`.
- Skip model: both can be skipped (wholesale or per-item), with skip state persisted.

## Morning / Check-in Model

- Check-in capture: free text for user-facing prompts.
- Backend values: scale-backed extraction for each modality where feasible.
- Tracked modalities: `mood`, `body`, `sleep`, `dream`.
- Per modality persisted fields: `text`, `scale (-10..10)`, `keywords`, `confidence`, optional `skipped` + `skip_reason`, per-item `answered_at`.
- Data shape: separate record per item/type; each update creates a new append-only event.
- Multiple check-ins per day are allowed.
- Pain can be updated during day and can trigger inferred prompts.
- Check-in prompts can be both user-invoked and auto-scheduled.

## Check-in Nudge & Queue Behavior

- Nudge channels: shared schedule by default (web/desktop/mobile).
- Nudge action: include default action target (open thread capture in CLI or main threads).
- Snooze ladder defaults: `5, 10, 15, 30, 60` minutes (or similar units where applicable), default `5`, configurable by nudge/action type.
- Skip after repeated snoozes: mark as skipped (with reason).
- Queueing: nudge queueing during active sessions (resume later).
- Session conflict during session: queued (not hard-blocking).
- Expiry: action/type-specific with reason; configurable.
- Retry/attempt tracing: snooze attempts stored as run events and visible in timeline.
- Sort order: urgency-first with age/time secondary; keep top-N ranking surfaced in expanded nudge UI.
- Skip reasons: enum-backed, with optional detail text.
- Queue controls: support batch dismiss and batch snooze.

## Modes: silent / vacation / break

- Modes exist with both user and inferred controls.
- Inference should be supported with user confirmation by default.
- Multiple flags are orthogonal.
- If both user and inferred flags overlap, tie-break with confidence.
- Persist causes/sources/confidence for mode transitions.
- Mode transitions should be persisted in run-thread and surfaced in session summary (natural language).
- Existence of suppression modes should be user-configurable; not exposed in front-end yet.

## Conflict Resolution

- Sync conflict behavior: merge on conflict.
- Conflict view preference: thread-style contextual resolution (not just single global diff).
- Overdue in-flight conflict: wait to complete before merge.
- Undo scope: batch-level undo (default last-applied batch).

## Overdue Workflow

- `confirm` required before applying, with configurable override later.
- Actions can be mixed in one pass.
- Mixed confirms can use a single confirmation summary.
- Overdue completion should carry user intent metadata (confidence + rationale + context tags), plus optional `next_action` and `review_horizon`.
- No audit reason required for low-priority auto defaults.

## Standup Behavior

- Standup can complete with zero commitments or be fully skipped.
- If zero commitments, still emit no-commitments summary and continue overdue handling.

## Schema / Versioning / Validation

- Payload versioning: include schema version in canonical check-in and workflow payloads.
- Backward read decoding for prior versions in v0: not required (defer).
- Strict payload version requirement on writes: not required in v0.
- New optional fields evolution: recommended to support backward-compatible additive handling with non-breaking defaults.
- Validation mode: recommend strict validation on Rust boundary, with clear compatibility errors.

## AI/ML Readiness

- Confidence handling: keep both raw score and calibrated bucket (recommended for v1 but acceptable prep in v0).
- Lightweight model/auto-inference features: target in v1.
- New AI features behind explicit config flags.

## Eventing / Internals

- Prefer canonical event-driven architecture with one stream for traceability (configurable by domain if needed).
- Event ordering: recommended globally per user/session linear ordering for deterministic replay.
- Run-event retention: multiple weeks baseline, dev-configurable.
- Old/new schema strictness: avoid over-constraining in v0, but keep strict validation for active writes.

## Export / Backup / Cold Storage

- Backup targets: NAS / S3 / Google Drive.
- Cold format: Parquet export path (recommended) for long-tail analytics/ML workloads.
- Export payloads: include raw events + resolved projections + checksums.
- Export mode: user-initiated + scheduled.
- Export encryption: optional (default off) with per-backend policy overrides.
- Integrity verification: optional and configurable; defaults can be off if heavy.
- Backup schedule default: daily (24h) at configurable off-peak, [3..168h] range.
- Event-driven trigger default: flush at `>=500` new events or `>=5` minutes of unsynced queue.
- Backup execution: background with status visibility and optional foreground manual export.
- Backup jobs: exponential retry with backoff; non-recoverable failures degrade gracefully to local-only mode by default, with optional explicit safety-blocking mode.
- Recovery view: include a compact CLI backup/queue health dashboard, with an optional thread-level summary view.

## Open Decisions (still pending)

- Final conflict-UI details for mixed override scenarios.

## Resolved (this pass)

- Exact payload schema details and v0 command shapes are drafted in [daily-loop-v0-sql-and-api-contract.md](/home/jove/code/vel/docs/notes/daily-loop-v0-sql-and-api-contract.md), including endpoint list and request/response examples.
- Scheduled backup and queue semantics are codified there and include defaults for nudge/backup retries, expiry, and queue depth behavior.
