-- Ingested assistant/chat transcripts for capture extraction, thread discovery, project tagging.
CREATE TABLE IF NOT EXISTS assistant_transcripts (
  id TEXT PRIMARY KEY,
  source TEXT NOT NULL,
  conversation_id TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  role TEXT NOT NULL,
  content TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_assistant_transcripts_conversation ON assistant_transcripts(conversation_id);
CREATE INDEX IF NOT EXISTS idx_assistant_transcripts_timestamp ON assistant_transcripts(timestamp DESC);
