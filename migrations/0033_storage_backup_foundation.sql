CREATE TABLE IF NOT EXISTS storage_targets (
  storage_target_id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  role TEXT NOT NULL,
  label TEXT NOT NULL,
  root_uri TEXT NOT NULL,
  path_prefix TEXT,
  provider_ref TEXT,
  enabled INTEGER NOT NULL DEFAULT 1,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  last_success_at INTEGER,
  last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_storage_targets_kind
  ON storage_targets(kind);

CREATE INDEX IF NOT EXISTS idx_storage_targets_role
  ON storage_targets(role);

CREATE TABLE IF NOT EXISTS artifact_copies (
  artifact_id TEXT NOT NULL,
  storage_target_id TEXT NOT NULL,
  target_locator TEXT NOT NULL,
  copy_state TEXT NOT NULL,
  target_version TEXT,
  content_hash TEXT,
  size_bytes INTEGER,
  copied_at INTEGER,
  verified_at INTEGER,
  last_error TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (artifact_id, storage_target_id, target_locator),
  FOREIGN KEY (artifact_id) REFERENCES artifacts(artifact_id) ON DELETE CASCADE,
  FOREIGN KEY (storage_target_id) REFERENCES storage_targets(storage_target_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_artifact_copies_target
  ON artifact_copies(storage_target_id, copied_at DESC);

CREATE INDEX IF NOT EXISTS idx_artifact_copies_state
  ON artifact_copies(copy_state, verified_at DESC);

CREATE TABLE IF NOT EXISTS backup_manifests (
  backup_manifest_id TEXT PRIMARY KEY,
  storage_target_id TEXT NOT NULL,
  scope TEXT NOT NULL,
  state TEXT NOT NULL,
  started_at INTEGER NOT NULL,
  completed_at INTEGER,
  verified_at INTEGER,
  summary_json TEXT NOT NULL DEFAULT '{}',
  last_error TEXT,
  FOREIGN KEY (storage_target_id) REFERENCES storage_targets(storage_target_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_backup_manifests_target_started
  ON backup_manifests(storage_target_id, started_at DESC);

CREATE TABLE IF NOT EXISTS backup_manifest_entries (
  backup_manifest_id TEXT NOT NULL,
  artifact_id TEXT NOT NULL,
  artifact_copy_locator TEXT NOT NULL,
  source_storage_uri TEXT,
  source_storage_kind TEXT,
  sync_class TEXT NOT NULL,
  expected_content_hash TEXT,
  expected_size_bytes INTEGER,
  target_version TEXT,
  entry_state TEXT NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  PRIMARY KEY (backup_manifest_id, artifact_id, artifact_copy_locator),
  FOREIGN KEY (backup_manifest_id) REFERENCES backup_manifests(backup_manifest_id) ON DELETE CASCADE,
  FOREIGN KEY (artifact_id) REFERENCES artifacts(artifact_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_backup_manifest_entries_artifact
  ON backup_manifest_entries(artifact_id);

CREATE TABLE IF NOT EXISTS verification_records (
  verification_id TEXT PRIMARY KEY,
  subject_type TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  status TEXT NOT NULL,
  observed_content_hash TEXT,
  observed_size_bytes INTEGER,
  failure_reason TEXT,
  checked_at INTEGER NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_verification_records_subject
  ON verification_records(subject_type, subject_id, checked_at DESC);

CREATE TABLE IF NOT EXISTS restore_plans (
  restore_plan_id TEXT PRIMARY KEY,
  source_target_id TEXT NOT NULL,
  plan_state TEXT NOT NULL,
  requested_at INTEGER NOT NULL,
  prepared_at INTEGER,
  executed_at INTEGER,
  destination_root TEXT NOT NULL,
  summary_json TEXT NOT NULL DEFAULT '{}',
  last_error TEXT,
  FOREIGN KEY (source_target_id) REFERENCES storage_targets(storage_target_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_restore_plans_target_requested
  ON restore_plans(source_target_id, requested_at DESC);

CREATE TABLE IF NOT EXISTS restore_plan_items (
  restore_plan_id TEXT NOT NULL,
  artifact_id TEXT NOT NULL,
  target_locator TEXT NOT NULL,
  target_version TEXT,
  planned_destination TEXT NOT NULL,
  item_state TEXT NOT NULL,
  failure_reason TEXT,
  PRIMARY KEY (restore_plan_id, artifact_id, target_locator),
  FOREIGN KEY (restore_plan_id) REFERENCES restore_plans(restore_plan_id) ON DELETE CASCADE,
  FOREIGN KEY (artifact_id) REFERENCES artifacts(artifact_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_restore_plan_items_artifact
  ON restore_plan_items(artifact_id);
