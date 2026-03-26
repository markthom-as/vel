-- 0051_v0_daily_loop_checks_nudges_backup.sql
--
-- V0 contract slice:
-- 1) check-in events are append-only and typed
-- 2) nudge queue supports explicit state transitions (snooze/skip/expire + batch)
-- 3) backup/job telemetry supports scheduler + retry + compact recovery dashboard
--
PRAGMA foreign_keys = ON;

-- Check-in capture history (one row per interaction, including skips)
CREATE TABLE IF NOT EXISTS daily_check_in_events (
  check_in_event_id TEXT PRIMARY KEY,
  session_id TEXT NOT NULL,
  prompt_id TEXT NOT NULL,
  check_in_type TEXT NOT NULL
    CHECK(check_in_type IN ('mood', 'body', 'sleep', 'dream', 'pain', 'other')),
  session_phase TEXT NOT NULL
    CHECK(session_phase IN ('morning', 'standup')),
  source TEXT NOT NULL CHECK(source IN ('user', 'inferred')),
  answered_at INTEGER,
  text TEXT,
  scale INTEGER CHECK(scale BETWEEN -10 AND 10),
  scale_min INTEGER NOT NULL DEFAULT -10,
  scale_max INTEGER NOT NULL DEFAULT 10,
  keywords_json TEXT NOT NULL DEFAULT '[]',
  confidence REAL,
  schema_version INTEGER NOT NULL DEFAULT 1,
  skipped INTEGER NOT NULL DEFAULT 0 CHECK(skipped IN (0,1)),
  skip_reason_code TEXT,
  skip_reason_text TEXT,
  replaced_by_event_id TEXT,
  meta_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  run_id TEXT,
  FOREIGN KEY (session_id) REFERENCES daily_sessions(session_id) ON DELETE CASCADE,
  FOREIGN KEY (replaced_by_event_id) REFERENCES daily_check_in_events(check_in_event_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_daily_check_in_events_session
  ON daily_check_in_events(session_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_daily_check_in_events_type
  ON daily_check_in_events(check_in_type, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_daily_check_in_events_state
  ON daily_check_in_events(skipped, answered_at DESC);

-- Optional enum-like registry for check-in skip reasons
CREATE TABLE IF NOT EXISTS daily_check_in_skip_reason_codes (
  reason_code TEXT PRIMARY KEY,
  scope TEXT NOT NULL CHECK(scope IN ('mood', 'body', 'sleep', 'dream', 'pain', 'generic')),
  label TEXT NOT NULL,
  description TEXT,
  user_visible INTEGER NOT NULL DEFAULT 1,
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_daily_check_in_skip_scope_code
  ON daily_check_in_skip_reason_codes(scope, reason_code);

-- Nudge queue runtime state (higher-order than legacy nudges state for v0 loop control)
CREATE TABLE IF NOT EXISTS v0_nudge_queue (
  nudge_id TEXT PRIMARY KEY,
  status TEXT NOT NULL CHECK(status IN ('active', 'pending', 'snoozed', 'dismissed', 'resolved', 'expired', 'skipped')),
  scheduled_for INTEGER NOT NULL,
  expires_at INTEGER,
  urgency INTEGER NOT NULL DEFAULT 0,
  source TEXT NOT NULL CHECK(source IN ('system', 'user')),
  source_ref TEXT,
  attempts INTEGER NOT NULL DEFAULT 0,
  max_attempts INTEGER NOT NULL DEFAULT 10,
  batch_id TEXT,
  reason_code TEXT,
  reason_text TEXT,
  actor TEXT,
  actor_id TEXT,
  updated_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (nudge_id) REFERENCES nudges(nudge_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_v0_nudge_queue_status
  ON v0_nudge_queue(status, scheduled_for ASC);

CREATE INDEX IF NOT EXISTS idx_v0_nudge_queue_next
  ON v0_nudge_queue(urgency DESC, scheduled_for ASC);

CREATE INDEX IF NOT EXISTS idx_v0_nudge_queue_batch
  ON v0_nudge_queue(batch_id, created_at DESC);

-- Append-only transition log for queue/state machine actions
CREATE TABLE IF NOT EXISTS v0_nudge_queue_events (
  event_id TEXT PRIMARY KEY,
  nudge_id TEXT NOT NULL,
  batch_id TEXT,
  event_type TEXT NOT NULL
    CHECK(event_type IN (
      'nudge_queued',
      'nudge_presented',
      'nudge_snoozed',
      'nudge_dismissed',
      'nudge_skipped',
      'nudge_done',
      'nudge_resurfaced',
      'nudge_expired'
    )),
  state_before TEXT,
  state_after TEXT,
  actor_kind TEXT NOT NULL CHECK(actor_kind IN ('user', 'system', 'automation')),
  actor_id TEXT,
  scheduled_for INTEGER,
  reason_code TEXT,
  reason_text TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  FOREIGN KEY (nudge_id) REFERENCES nudges(nudge_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_v0_nudge_queue_events_nudge
  ON v0_nudge_queue_events(nudge_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_v0_nudge_queue_events_type
  ON v0_nudge_queue_events(event_type, created_at DESC);

-- Backup jobs + attempts for v0 recovery dashboard / retry behavior
CREATE TABLE IF NOT EXISTS v0_backup_jobs (
  backup_job_id TEXT PRIMARY KEY,
  storage_target_id TEXT NOT NULL,
  trigger_type TEXT NOT NULL CHECK(trigger_type IN ('manual', 'scheduled', 'event_driven')),
  scope TEXT NOT NULL CHECK(scope IN ('local_only', 'cold_storage', 'parquet_full', 'parquet_delta')),
  status TEXT NOT NULL CHECK(status IN ('queued', 'running', 'succeeded', 'failed', 'blocked', 'expired', 'cancelled')),
  safety_mode TEXT NOT NULL CHECK(safety_mode IN ('default_local_only', 'safety_blocking', 'off')),
  requested_by TEXT NOT NULL CHECK(requested_by IN ('user', 'system', 'scheduler')),
  requested_by_ref TEXT,
  manifest_id TEXT,
  urgency INTEGER NOT NULL DEFAULT 0,
  attempt INTEGER NOT NULL DEFAULT 0,
  max_attempts INTEGER NOT NULL DEFAULT 6,
  base_backoff_minutes INTEGER NOT NULL DEFAULT 2,
  queue_confidence REAL,
  created_at INTEGER NOT NULL,
  next_attempt_at INTEGER,
  started_at INTEGER,
  finished_at INTEGER,
  completed_at INTEGER,
  last_error_code TEXT,
  last_error_message TEXT,
  last_error_transient INTEGER NOT NULL DEFAULT 1,
  policy_json TEXT NOT NULL DEFAULT '{}',
  payload_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (storage_target_id) REFERENCES storage_targets(storage_target_id) ON DELETE RESTRICT,
  FOREIGN KEY (manifest_id) REFERENCES backup_manifests(backup_manifest_id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_v0_backup_jobs_status
  ON v0_backup_jobs(status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_v0_backup_jobs_next
  ON v0_backup_jobs(status, next_attempt_at ASC)
  WHERE status IN ('queued', 'failed');

CREATE TABLE IF NOT EXISTS v0_backup_job_attempts (
  attempt_id TEXT PRIMARY KEY,
  backup_job_id TEXT NOT NULL,
  attempt_no INTEGER NOT NULL,
  state TEXT NOT NULL CHECK(state IN ('queued', 'running', 'succeeded', 'failed')),
  scheduled_at INTEGER NOT NULL,
  started_at INTEGER,
  finished_at INTEGER,
  exit_code INTEGER,
  duration_ms INTEGER,
  error_code TEXT,
  error_message TEXT,
  error_is_transient INTEGER,
  retry_wait_seconds INTEGER,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (backup_job_id) REFERENCES v0_backup_jobs(backup_job_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_v0_backup_job_attempts_job
  ON v0_backup_job_attempts(backup_job_id, attempt_no DESC);

CREATE TABLE IF NOT EXISTS v0_backup_job_events (
  event_id TEXT PRIMARY KEY,
  backup_job_id TEXT NOT NULL,
  event_type TEXT NOT NULL
    CHECK(event_type IN (
      'backup_job_queued',
      'backup_job_started',
      'backup_job_progress',
      'backup_job_retried',
      'backup_job_succeeded',
      'backup_job_failed',
      'backup_job_blocked',
      'backup_job_expired',
      'backup_job_retry_exhausted',
      'backup_job_resumed'
    )),
  state_before TEXT,
  state_after TEXT,
  reason_code TEXT,
  reason_text TEXT,
  created_at INTEGER NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  FOREIGN KEY (backup_job_id) REFERENCES v0_backup_jobs(backup_job_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_v0_backup_job_events_job
  ON v0_backup_job_events(backup_job_id, created_at DESC);

-- Compatibility aliases for current v1 API consumers that read nudge_events
INSERT OR IGNORE INTO nudge_events (id, nudge_id, event_type, payload_json, timestamp, created_at)
SELECT
  e.event_id,
  e.nudge_id,
  e.event_type,
  COALESCE(e.metadata_json, '{}'),
  e.created_at,
  e.created_at
FROM v0_nudge_queue_events e
WHERE NOT EXISTS (
  SELECT 1 FROM nudge_events ne WHERE ne.id = e.event_id
);

