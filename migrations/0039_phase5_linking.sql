CREATE TABLE pairing_tokens (
    token_id TEXT PRIMARY KEY,
    token_code TEXT NOT NULL UNIQUE,
    issued_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    issued_by_node_id TEXT NOT NULL,
    scopes_json TEXT NOT NULL,
    redeemed_at INTEGER
);

CREATE TABLE linked_nodes (
    node_id TEXT PRIMARY KEY,
    node_display_name TEXT NOT NULL,
    status TEXT NOT NULL,
    scopes_json TEXT NOT NULL,
    linked_at INTEGER NOT NULL,
    last_seen_at INTEGER,
    transport_hint TEXT,
    revoked_at INTEGER
);

CREATE INDEX idx_pairing_tokens_expires_at
    ON pairing_tokens(expires_at);
CREATE INDEX idx_linked_nodes_status
    ON linked_nodes(status);
