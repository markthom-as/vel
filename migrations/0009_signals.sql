-- Signals from external adapters (calendar, todoist, activity) for inference.
CREATE TABLE IF NOT EXISTS signals (
  signal_id TEXT PRIMARY KEY,
  signal_type TEXT NOT NULL,
  source TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_signals_type ON signals(signal_type);
CREATE INDEX IF NOT EXISTS idx_signals_timestamp ON signals(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_signals_source ON signals(source);
