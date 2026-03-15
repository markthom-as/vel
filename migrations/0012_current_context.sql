-- Singleton table: single row holding latest computed context snapshot (persistent current context).
CREATE TABLE IF NOT EXISTS current_context (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  computed_at INTEGER NOT NULL,
  context_json TEXT NOT NULL
);

-- Only one row ever; application must INSERT OR REPLACE or UPDATE where id = 1.
INSERT OR IGNORE INTO current_context (id, computed_at, context_json) VALUES (1, 0, '{}');
