CREATE INDEX IF NOT EXISTS idx_captures_occurred_at ON captures(occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_captures_created_at ON captures(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_captures_type_device ON captures(capture_type, source_device);

CREATE VIRTUAL TABLE IF NOT EXISTS captures_fts USING fts5(
    capture_id UNINDEXED,
    content_text,
    tokenize = 'unicode61'
);

INSERT INTO captures_fts (capture_id, content_text)
SELECT capture_id, content_text
FROM captures
WHERE NOT EXISTS (
    SELECT 1 FROM captures_fts WHERE captures_fts.capture_id = captures.capture_id
);

CREATE TRIGGER IF NOT EXISTS captures_ai AFTER INSERT ON captures BEGIN
    INSERT INTO captures_fts (capture_id, content_text)
    VALUES (new.capture_id, new.content_text);
END;

CREATE TRIGGER IF NOT EXISTS captures_ad AFTER DELETE ON captures BEGIN
    INSERT INTO captures_fts (captures_fts, rowid, capture_id, content_text)
    VALUES ('delete', old.rowid, old.capture_id, old.content_text);
END;

CREATE TRIGGER IF NOT EXISTS captures_au AFTER UPDATE ON captures BEGIN
    INSERT INTO captures_fts (captures_fts, rowid, capture_id, content_text)
    VALUES ('delete', old.rowid, old.capture_id, old.content_text);
    INSERT INTO captures_fts (capture_id, content_text)
    VALUES (new.capture_id, new.content_text);
END;
