-- Optional external identifier for deduplication (e.g. event id, task id).
ALTER TABLE signals ADD COLUMN source_ref TEXT;
CREATE INDEX IF NOT EXISTS idx_signals_source_ref ON signals(source, source_ref);
