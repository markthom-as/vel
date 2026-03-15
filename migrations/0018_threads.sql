-- First-class threads: projects, people, conversations, thematic strands.
CREATE TABLE IF NOT EXISTS threads (
  id TEXT PRIMARY KEY,
  thread_type TEXT NOT NULL,
  title TEXT NOT NULL,
  status TEXT NOT NULL,
  metadata_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_threads_type_status ON threads(thread_type, status);
CREATE INDEX IF NOT EXISTS idx_threads_updated_at ON threads(updated_at DESC);
