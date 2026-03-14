-- Enforce monotonic sequence ordering per run: one event per (run_id, seq).
CREATE UNIQUE INDEX IF NOT EXISTS idx_run_events_run_id_seq ON run_events(run_id, seq);
