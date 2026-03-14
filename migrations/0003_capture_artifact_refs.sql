-- Optional links from captures to artifacts (raw recording, derived transcript, etc.)
ALTER TABLE captures ADD COLUMN raw_artifact_id TEXT;
ALTER TABLE captures ADD COLUMN derived_artifact_id TEXT;
