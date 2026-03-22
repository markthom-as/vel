CREATE TABLE integration_accounts (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    display_name TEXT NOT NULL,
    external_account_ref TEXT,
    auth_state TEXT NOT NULL,
    policy_profile TEXT NOT NULL,
    activation_state TEXT NOT NULL,
    sync_posture TEXT NOT NULL,
    metadata_json TEXT NOT NULL CHECK (json_valid(metadata_json)),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_integration_accounts_provider
    ON integration_accounts (provider, auth_state);

CREATE TABLE sync_links (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    integration_account_id TEXT NOT NULL,
    object_id TEXT NOT NULL,
    remote_id TEXT NOT NULL,
    remote_type TEXT NOT NULL,
    state TEXT NOT NULL,
    authority_mode TEXT NOT NULL,
    remote_version TEXT,
    metadata_json TEXT NOT NULL CHECK (json_valid(metadata_json)),
    linked_at INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL,
    FOREIGN KEY (integration_account_id) REFERENCES integration_accounts(id)
);

CREATE INDEX idx_sync_links_object
    ON sync_links (object_id, provider, integration_account_id, state);

CREATE UNIQUE INDEX idx_sync_links_remote_tuple
    ON sync_links (provider, integration_account_id, remote_id, remote_type);

CREATE TABLE runtime_records (
    id TEXT PRIMARY KEY,
    record_type TEXT NOT NULL,
    object_ref TEXT,
    status TEXT NOT NULL,
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_runtime_records_type_status
    ON runtime_records (record_type, status, created_at);

CREATE TABLE projections (
    id TEXT PRIMARY KEY,
    projection_type TEXT NOT NULL,
    object_id TEXT,
    source_summary_json TEXT CHECK (source_summary_json IS NULL OR json_valid(source_summary_json)),
    projection_json TEXT NOT NULL CHECK (json_valid(projection_json)),
    rebuild_token TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_projections_type_object
    ON projections (projection_type, object_id);
