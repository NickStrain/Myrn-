-- Learning loop: when each item was injected (to attribute later use to it),
-- and embedding vectors for semantic search (optional).
ALTER TABLE usage ADD COLUMN t INTEGER;
CREATE INDEX usage_note ON usage(note_id);
CREATE INDEX usage_session ON usage(session_id);

CREATE TABLE vectors (
    id    TEXT PRIMARY KEY,  -- node id
    hash  TEXT NOT NULL,     -- hash of the embedded text, to re-embed only what changed
    model TEXT NOT NULL,
    vec   BLOB NOT NULL      -- little-endian f32, L2-normalised
) WITHOUT ROWID;
