-- Distinguish artifacts Vel manages (writes, checksum, size) from external references.
ALTER TABLE artifacts ADD COLUMN storage_kind TEXT NOT NULL DEFAULT 'external';
