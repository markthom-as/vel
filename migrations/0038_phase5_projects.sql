CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    family TEXT NOT NULL,
    status TEXT NOT NULL,
    primary_repo_path TEXT NOT NULL,
    primary_notes_root TEXT NOT NULL,
    secondary_repo_paths_json TEXT NOT NULL DEFAULT '[]',
    secondary_notes_roots_json TEXT NOT NULL DEFAULT '[]',
    upstream_ids_json TEXT NOT NULL DEFAULT '{}',
    pending_provision_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    archived_at INTEGER
);

CREATE TABLE project_aliases (
    alias TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    source TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_project_aliases_project_id
    ON project_aliases(project_id);
CREATE INDEX IF NOT EXISTS idx_projects_family
    ON projects(family);
