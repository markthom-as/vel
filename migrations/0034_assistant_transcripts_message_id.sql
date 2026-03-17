-- Transcript ingestion may retain optional message IDs from snapshots for deduplication and traceability.
ALTER TABLE assistant_transcripts ADD COLUMN message_id TEXT;
