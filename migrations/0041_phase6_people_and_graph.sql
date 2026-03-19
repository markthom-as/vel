CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    given_name TEXT,
    family_name TEXT,
    relationship_context TEXT,
    birthday TEXT,
    last_contacted_at INTEGER,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_people_updated_at
    ON people(updated_at DESC, created_at DESC);

CREATE TABLE IF NOT EXISTS person_aliases (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    handle TEXT NOT NULL,
    display TEXT,
    source_ref_json TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL,
    UNIQUE(platform, handle),
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_person_aliases_person
    ON person_aliases(person_id, created_at ASC);
