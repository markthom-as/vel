CREATE TABLE writeback_operations (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    risk TEXT NOT NULL,
    status TEXT NOT NULL,
    family TEXT NOT NULL,
    provider_key TEXT NOT NULL,
    project_id TEXT,
    connection_id TEXT,
    external_id TEXT,
    requested_payload_json TEXT NOT NULL DEFAULT '{}',
    result_payload_json TEXT NOT NULL DEFAULT '{}',
    provenance_json TEXT NOT NULL DEFAULT '{}',
    conflict_case_id TEXT,
    requested_by_node_id TEXT NOT NULL,
    ordering_stamp_json TEXT NOT NULL,
    requested_at INTEGER NOT NULL,
    applied_at INTEGER,
    updated_at INTEGER NOT NULL
);

CREATE TABLE conflict_cases (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    status TEXT NOT NULL,
    family TEXT NOT NULL,
    provider_key TEXT NOT NULL,
    project_id TEXT,
    connection_id TEXT,
    external_id TEXT,
    summary TEXT NOT NULL,
    local_payload_json TEXT NOT NULL DEFAULT '{}',
    upstream_payload_json TEXT NOT NULL DEFAULT '{}',
    resolution_payload_json TEXT NOT NULL DEFAULT '{}',
    opened_at INTEGER NOT NULL,
    resolved_at INTEGER,
    updated_at INTEGER NOT NULL
);

CREATE TABLE upstream_object_refs (
    id TEXT PRIMARY KEY,
    family TEXT NOT NULL,
    provider_key TEXT NOT NULL,
    project_id TEXT,
    local_object_kind TEXT NOT NULL,
    local_object_id TEXT NOT NULL,
    external_id TEXT NOT NULL,
    external_parent_id TEXT,
    ordering_stamp_json TEXT NOT NULL,
    last_seen_at INTEGER NOT NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE UNIQUE INDEX idx_upstream_object_refs_identity
    ON upstream_object_refs(family, provider_key, local_object_kind, local_object_id);
CREATE INDEX idx_writeback_operations_status
    ON writeback_operations(status, updated_at DESC);
CREATE INDEX idx_conflict_cases_status
    ON conflict_cases(status, updated_at DESC);
