# Daily Loop v0: SQL + API Contract (Check-in, Nudge Queue, Backup Recovery)

Working note only. This file is not shipped-behavior authority by itself.
If this work becomes active, promote the accepted contract into the relevant milestone packet, ticket, migration plan, and canonical architecture or API docs.

This document is the concrete v0 contract for implementation on top of existing routes:
- Base route style already in `crates/veld/src/app.rs` is `/v1/...`.
- This contract is internal operator-first; nudge/recovery APIs are intentionally **not yet frontend-safe**.

Canonical migration for this contract:
- `migrations/0051_v0_daily_loop_checks_nudges_backup.sql`

## 1) Schema contract (canonical)

### 1.1 check-in events (`daily_check_in_events`)

- Append-only event capture for `mood`, `body`, `sleep`, `dream`, `pain`, etc.
- Each request creates one row.
- Multiple rows per day/session/type are valid.
- Skip is persisted and reasoned.

**Columns required**
- `check_in_event_id TEXT PK`
- `session_id TEXT NOT NULL` (`daily_sessions.session_id`, cascade on delete)
- `prompt_id TEXT NOT NULL`
- `check_in_type TEXT NOT NULL` (`mood|body|sleep|dream|pain|other`)
- `session_phase TEXT NOT NULL` (`morning|standup`)
- `source TEXT NOT NULL` (`user|inferred`)
- `answered_at INTEGER`
- `text TEXT`
- `scale INTEGER CHECK BETWEEN -10 AND 10`
- `scale_min INTEGER DEFAULT -10`
- `scale_max INTEGER DEFAULT 10`
- `keywords_json TEXT DEFAULT '[]'`
- `confidence REAL`
- `schema_version INTEGER DEFAULT 1`
- `skipped INTEGER DEFAULT 0`
- `skip_reason_code TEXT`
- `skip_reason_text TEXT`
- `replaced_by_event_id TEXT` (`FK -> daily_check_in_events.check_in_event_id`, nullable)
- `meta_json TEXT DEFAULT '{}'`
- `created_at INTEGER`
- `updated_at INTEGER`
- `run_id TEXT`

### 1.2 skip reason registry (`daily_check_in_skip_reason_codes`)

- Optional governance surface for consistency.
- Fields: `reason_code`, `scope`, `label`, `description`, `user_visible`, `enabled`.

### 1.3 nudge queue state (`v0_nudge_queue`) and transitions (`v0_nudge_queue_events`)

#### `v0_nudge_queue` (current state)
- `nudge_id TEXT PK` (`nudges(nudge_id)` foreign key)
- `status TEXT` (`active|pending|snoozed|dismissed|resolved|expired|skipped`)
- `scheduled_for INTEGER`
- `expires_at INTEGER`
- `urgency INTEGER`
- `source TEXT` (`system|user`)
- `source_ref TEXT`
- `attempts INTEGER`
- `max_attempts INTEGER`
- `batch_id TEXT`
- `reason_code TEXT`, `reason_text TEXT`
- `actor`, `actor_id`, `updated_at`, `created_at`

#### `v0_nudge_queue_events` (append-only transition log)
- `event_id TEXT PK`
- `nudge_id TEXT`
- `batch_id TEXT`
- `event_type`:
  - `nudge_queued`
  - `nudge_presented`
  - `nudge_snoozed`
  - `nudge_dismissed`
  - `nudge_skipped`
  - `nudge_done`
  - `nudge_resurfaced`
  - `nudge_expired`
- `state_before`, `state_after`
- `actor_kind` (`user|system|automation`)
- `actor_id`
- `scheduled_for`
- `reason_code`, `reason_text`
- `metadata_json`
- `created_at`

### 1.4 backup queue/job telemetry (`v0_backup_jobs`), attempts, events

#### `v0_backup_jobs`
- `backup_job_id TEXT PK`
- `storage_target_id TEXT` (`storage_targets.storage_target_id`)
- `trigger_type` (`manual|scheduled|event_driven`)
- `scope` (`local_only|cold_storage|parquet_full|parquet_delta`)
- `status` (`queued|running|succeeded|failed|blocked|expired|cancelled`)
- `safety_mode` (`default_local_only|safety_blocking|off`)
- `requested_by` (`user|system|scheduler`)
- `requested_by_ref`
- `manifest_id` (`backup_manifests.backup_manifest_id`, optional)
- `urgency`
- `attempt`, `max_attempts`
- `base_backoff_minutes`
- `queue_confidence`
- `created_at`, `next_attempt_at`, `started_at`, `finished_at`, `completed_at`
- `last_error_code`, `last_error_message`, `last_error_transient`
- `policy_json`, `payload_json`

#### `v0_backup_job_attempts`
- per-attempt retry trace (`queued|running|succeeded|failed`, timestamps, exit_code, durations, error details)

#### `v0_backup_job_events`
- event ledger (`..._queued`, `..._started`, `..._progress`, `..._retried`, `..._failed`, `..._succeeded`, `..._blocked`, `..._retry_exhausted`, etc.)

### 1.5 Backward compatibility row-copy
- The migration includes a compatibility copy into existing `nudge_events` so legacy readers still observe nudge transitions while v0-specific structured nudge queue state lives in `v0_nudge_queue*`.

## 2) API contract (operator API)

All responses use existing `ApiResponse<T>` shape.

### 2.1 Check-in capture

#### `POST /v1/daily-loop/sessions/:session_id/check-ins`
Create one check-in event row (append-only).

Request:
```json
{
  "check_in_type": "mood",
  "session_phase": "morning",
  "source": "user",
  "prompt_id": "morning_mood_01",
  "text": "sleep was rough but focused now",
  "scale": -2,
  "keywords": ["sleep_debt", "focus"],
  "confidence": 0.84,
  "skipped": false,
  "replace_if_conflict": true
}
```

Headers:
- `Idempotency-Key` (strongly recommended)

Response:
```json
{
  "ok": true,
  "request_id": "req_x",
  "data": {
    "check_in_event_id": "ci_... ",
    "session_id": "...",
    "status": "recorded",
    "supersedes_event_id": null
  }
}
```

#### `POST /v1/daily-loop/check-ins/:check_in_event_id/skip`
Persist explicit skip + reason for an in-flight/invalid prior capture.

Request:
```json
{
  "reason_code": "not_now",
  "reason_text": "In meeting; will review later",
  "source": "user",
  "replace_existing": false
}
```

Response:
- `{ "ok": true, "data": { "check_in_event_id": "...", "skipped": true, "status": "applied", ... } }`

#### `GET /v1/daily-loop/sessions/:session_id/check-ins`
Query check-ins and skips.

Query params:
- `type` (`mood|body|sleep|dream|pain`)
- `phase` (`morning|standup`)
- `include_skipped` (`true|false`, default false)
- `limit` (default 50)
- `cursor` (pagination)

Returns:
- `items` array in `created_at` desc order.

### 2.2 Nudge queue state machine

#### `GET /v1/nudges/queue`
Read prioritized queue (queue view, not just raw nudge table).

Query:
- `status` repeated or comma-separated
- `top_n` default 10
- `min_urgency` default 0
- `include_expired` default false

Response includes:
- `queue`, `total_count`, `next_resume_at`, `generated_at`.

#### `POST /v1/nudges/:id/transition`
Transition endpoint for batch-safe single-item actions.

Request:
```json
{
  "transition": "snooze|skip|dismiss|done|expire|reactivate",
  "minutes": 15,
  "reason_code": "busy_meeting",
  "reason_text": "In person sync",
  "source": "user",
  "source_ref": "cli",
  "batch_id": "optional_batch"
}
```

Rules:
- `snooze` requires `minutes`.
- `skip` stores `reason_code` and transitions to `skipped`.
- `expire` is an auth-required internal/system action (also used by scheduler reconciliation).
- Response contains new computed queue state + `event_id`.

#### `POST /v1/nudges/:id/snooze`
Preserved compatibility endpoint.
- Backward compatible shape for current clients.
- New v0 payload extension allowed:
```json
{"minutes":10,"reason_code":"later","reason_text":"in meeting","actor_id":"usr_..."}
```

#### `POST /v1/nudges/batch/snooze`
Batch queue controls.

Request:
```json
{
  "ids": ["nud_1", "nud_2"],
  "minutes": 10,
  "reason_code": "low_confidence",
  "reason_text": "wait for clarifying signal",
  "batch_id": "nb_2026_03_25_001",
  "source": "system"
}
```

#### `POST /v1/nudges/batch/skip`
Batch skip action.

Request:
```json
{
  "ids": ["nud_1", "nud_2"],
  "reason_code": "already_done",
  "reason_text": "handled by different tool",
  "source": "user",
  "source_ref": "cli"
}
```

#### `POST /v1/nudges/batch/dismiss`
Batch dismiss action; optional safety.

Request:
```json
{
  "ids": ["nud_3", "nud_4"],
  "reason_code": "noise",
  "reason_text": "Not relevant for today"
}
```

All batch endpoints include:
- `batch_id` (server generated if missing)
- per-item result array with success/failure and conflict version.

#### `POST /v1/nudges/:id/expire`
Internal/system endpoint to force expiry.
- idempotent.
- used when `expires_at < now`.

### 2.3 Recovery dashboard + backup queue

#### `GET /v1/backup/jobs`
List current jobs for queue dashboard and diagnostics.
- Query: `status`, `target`, `trigger_type`, `from`, `to`, `limit`.

#### `POST /v1/backup/jobs/:job_id/retry`
Retry a failed job.
- Request: `{ "force": false }`
- `force: true` bypasses safety/circuit-break rules.

#### `POST /v1/backup/jobs/:job_id/cancel`
Cancel/abandon a queued or failed job.

#### `POST /v1/backup/jobs/:job_id/ack-fail-safe-mode`
- marks unrecoverable failures as intentionally tolerated in local-only mode.

#### `GET /v1/backup/jobs/:job_id`
Inspect one job with full attempt/event timeline.

#### `GET /v1/backup/recovery-dashboard`
Compact status surface (CLI-focused).

Response shape:
```json
{
  "queue": {
    "queued": 2,
    "running": 1,
    "failed": 4,
    "blocked": 0,
    "last_updated": "2026-03-25T00:00:00Z"
  },
  "backoff": {
    "next_retry_at": "2026-03-25T10:00:00Z",
    "max_retry_attempts": 6
  },
  "local_only_mode": {
    "enabled": true,
    "reason": "unrecoverable_manifest_error"
  },
  "critical_alerts": [
    {"level":"warn","code":"high_queue_age","message":"nudge queue older than 24h"}
  ],
  "top_n": 10
}
```

#### `POST /v1/backup/create`
Existing endpoint remains.
- In v0, this should emit a `v0_backup_jobs` row with:
  - `trigger_type = manual`
  - `status = queued`
  - transition into `running`/`succeeded/failed` with events in `v0_backup_job_events`.

## 3) State-machine rules (minimum behavior)

### Check-in
- No row mutation. Every submit/skip is insert-only.
- If `replace_if_conflict=true`, set `replaced_by_event_id` on previous most-recent unresolved item in same session/type/phase.

### Nudge
- All transitions emit:
  1) one row in `v0_nudge_queue_events`
  2) corresponding `nudge_events` compatibility row
- `batch_id` is used for mixed/expanded actions and traceability.
- Expiry is deterministic: if `expires_at` < now, transition to `expired` unless already terminal.

### Backup
- Exponential backoff default in service layer:
  - base 2 min
  - jitter optional
  - cap per `max_attempts`
- Failed unrecoverable event should set `status=blocked` only if safety policy allows; default is `status=local_only` continuation.
- Recovery dashboard must show `status + last_error_transient + next_attempt_at`.

## 4) Suggested immediate execution order for implementation

1. Migrate DB with `0051_v0_daily_loop_checks_nudges_backup.sql`.
2. Add check-in write/read DTO + route in `crates/veld/src/routes/daily_loop.rs`.
3. Add `v0_nudge_queue*` service methods + batch transition route(s) in `crates/veld/src/routes/nudges.rs`.
4. Add `v0_backup_jobs*` route slices in `crates/veld/src/routes/backup.rs`.
5. Implement `/v1/backup/recovery-dashboard`.
