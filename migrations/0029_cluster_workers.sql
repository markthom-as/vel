CREATE TABLE IF NOT EXISTS cluster_workers (
  worker_id TEXT PRIMARY KEY,
  node_id TEXT NOT NULL,
  node_display_name TEXT,
  worker_class TEXT,
  worker_classes_json TEXT NOT NULL DEFAULT '[]',
  capabilities_json TEXT NOT NULL DEFAULT '[]',
  status TEXT,
  max_concurrency INTEGER,
  current_load INTEGER,
  queue_depth INTEGER,
  reachability TEXT,
  latency_class TEXT,
  compute_class TEXT,
  power_class TEXT,
  recent_failure_rate REAL,
  tailscale_preferred INTEGER NOT NULL DEFAULT 0,
  sync_base_url TEXT,
  sync_transport TEXT,
  tailscale_base_url TEXT,
  preferred_tailnet_endpoint TEXT,
  tailscale_reachable INTEGER NOT NULL DEFAULT 0,
  lan_base_url TEXT,
  localhost_base_url TEXT,
  last_heartbeat_at INTEGER NOT NULL,
  started_at INTEGER,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cluster_workers_node
  ON cluster_workers(node_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_cluster_workers_heartbeat
  ON cluster_workers(last_heartbeat_at DESC);
