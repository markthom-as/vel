-- Vel self-model: metrics for nudge effectiveness, feedback, suggestion acceptance (reflective tuning).
CREATE TABLE IF NOT EXISTS vel_self_metrics (
  id TEXT PRIMARY KEY,
  metric_type TEXT NOT NULL,
  metric_value REAL NOT NULL,
  context_json TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_vel_self_metrics_type_timestamp ON vel_self_metrics(metric_type, timestamp DESC);
