-- Relate threads to entities (commitments, captures, signals, artifacts, suggestions).
CREATE TABLE IF NOT EXISTS thread_links (
  id TEXT PRIMARY KEY,
  thread_id TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  entity_id TEXT NOT NULL,
  relation_type TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_thread_link_unique
  ON thread_links(thread_id, entity_type, entity_id, relation_type);
CREATE INDEX IF NOT EXISTS idx_thread_links_thread ON thread_links(thread_id);
CREATE INDEX IF NOT EXISTS idx_thread_links_entity ON thread_links(entity_type, entity_id);
