-- Chat interface: conversations, messages, interventions, event_log.
-- See docs/api/chat.md and docs/MASTER_PLAN.md

CREATE TABLE IF NOT EXISTS conversations (
  id TEXT PRIMARY KEY,
  title TEXT,
  kind TEXT NOT NULL DEFAULT 'general',
  pinned INTEGER NOT NULL DEFAULT 0,
  archived INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON conversations(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_conversations_archived ON conversations(archived);

CREATE TABLE IF NOT EXISTS messages (
  id TEXT PRIMARY KEY,
  conversation_id TEXT NOT NULL,
  role TEXT NOT NULL,
  kind TEXT NOT NULL,
  content_json TEXT NOT NULL,
  status TEXT,
  importance TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER,
  FOREIGN KEY (conversation_id) REFERENCES conversations(id)
);

CREATE INDEX IF NOT EXISTS idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(conversation_id, created_at DESC);

CREATE TABLE IF NOT EXISTS interventions (
  id TEXT PRIMARY KEY,
  message_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  state TEXT NOT NULL,
  surfaced_at INTEGER NOT NULL,
  resolved_at INTEGER,
  snoozed_until INTEGER,
  confidence REAL,
  source_json TEXT,
  provenance_json TEXT,
  FOREIGN KEY (message_id) REFERENCES messages(id)
);

CREATE INDEX IF NOT EXISTS idx_interventions_state ON interventions(state);
CREATE INDEX IF NOT EXISTS idx_interventions_message_id ON interventions(message_id);
CREATE INDEX IF NOT EXISTS idx_interventions_surfaced_at ON interventions(surfaced_at DESC);

CREATE TABLE IF NOT EXISTS event_log (
  id TEXT PRIMARY KEY,
  event_name TEXT NOT NULL,
  aggregate_type TEXT,
  aggregate_id TEXT,
  payload_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_event_log_created_at ON event_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_event_log_aggregate ON event_log(aggregate_type, aggregate_id);
