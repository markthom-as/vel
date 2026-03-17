CREATE TABLE IF NOT EXISTS integration_connections (
  id TEXT PRIMARY KEY,
  family TEXT NOT NULL,
  provider_key TEXT NOT NULL,
  status TEXT NOT NULL,
  display_name TEXT NOT NULL,
  account_ref TEXT,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_integration_connections_family
  ON integration_connections(family);

CREATE INDEX IF NOT EXISTS idx_integration_connections_provider
  ON integration_connections(provider_key);

CREATE INDEX IF NOT EXISTS idx_integration_connections_family_provider
  ON integration_connections(family, provider_key);

CREATE TABLE IF NOT EXISTS integration_connection_setting_refs (
  id TEXT PRIMARY KEY,
  connection_id TEXT NOT NULL,
  setting_key TEXT NOT NULL,
  setting_value TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY(connection_id) REFERENCES integration_connections(id) ON DELETE CASCADE,
  UNIQUE(connection_id, setting_key)
);

CREATE INDEX IF NOT EXISTS idx_integration_connection_setting_refs_connection
  ON integration_connection_setting_refs(connection_id);

CREATE TABLE IF NOT EXISTS integration_connection_events (
  id TEXT PRIMARY KEY,
  connection_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  payload_json TEXT NOT NULL DEFAULT '{}',
  timestamp INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY(connection_id) REFERENCES integration_connections(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_integration_connection_events_connection_timestamp
  ON integration_connection_events(connection_id, timestamp DESC, created_at DESC);
