-- Dependency graph among commitments (e.g. meeting -> prep, meeting -> commute).
CREATE TABLE IF NOT EXISTS commitment_dependencies (
  id TEXT PRIMARY KEY,
  parent_commitment_id TEXT NOT NULL,
  child_commitment_id TEXT NOT NULL,
  dependency_type TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_commitment_dependency_unique
  ON commitment_dependencies(parent_commitment_id, child_commitment_id, dependency_type);
CREATE INDEX IF NOT EXISTS idx_commitment_dependencies_parent ON commitment_dependencies(parent_commitment_id);
CREATE INDEX IF NOT EXISTS idx_commitment_dependencies_child ON commitment_dependencies(child_commitment_id);
