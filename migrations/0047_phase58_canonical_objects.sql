CREATE TABLE canonical_objects (
    id TEXT PRIMARY KEY,
    object_type TEXT NOT NULL,
    object_class TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    revision INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL,
    provenance_json TEXT NOT NULL CHECK (json_valid(provenance_json)),
    facets_json TEXT NOT NULL CHECK (json_valid(facets_json)),
    source_summary_json TEXT CHECK (source_summary_json IS NULL OR json_valid(source_summary_json)),
    deleted_at INTEGER,
    archived_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_canonical_objects_class_type
    ON canonical_objects (object_class, object_type);

CREATE TABLE canonical_registry_objects (
    id TEXT PRIMARY KEY,
    registry_type TEXT NOT NULL,
    namespace TEXT NOT NULL,
    slug TEXT NOT NULL,
    display_name TEXT NOT NULL,
    version TEXT NOT NULL,
    status TEXT NOT NULL,
    manifest_ref TEXT NOT NULL,
    overlay_json TEXT NOT NULL CHECK (json_valid(overlay_json)),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX idx_canonical_registry_namespace_slug
    ON canonical_registry_objects (namespace, slug);

CREATE TABLE canonical_relations (
    id TEXT PRIMARY KEY,
    relation_type TEXT NOT NULL,
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    direction TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1,
    source_json TEXT NOT NULL CHECK (json_valid(source_json)),
    confidence REAL,
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    UNIQUE (from_id, relation_type, to_id, direction)
);

CREATE INDEX idx_canonical_relations_from
    ON canonical_relations (from_id, relation_type, active);

CREATE INDEX idx_canonical_relations_to
    ON canonical_relations (to_id, relation_type, active);
