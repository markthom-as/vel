CREATE TABLE IF NOT EXISTS work_assignment_receipts (
  receipt_id TEXT PRIMARY KEY,
  work_request_id TEXT NOT NULL,
  worker_id TEXT NOT NULL,
  worker_class TEXT,
  capability TEXT,
  status TEXT NOT NULL,
  assigned_at INTEGER NOT NULL,
  started_at INTEGER,
  completed_at INTEGER,
  result TEXT,
  error_message TEXT,
  last_updated INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_work_assignment_by_worker
  ON work_assignment_receipts(worker_id, last_updated DESC);

CREATE INDEX IF NOT EXISTS idx_work_assignment_by_request
  ON work_assignment_receipts(work_request_id, last_updated DESC);
