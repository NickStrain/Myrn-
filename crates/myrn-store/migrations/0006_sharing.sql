-- Sharing: shared records are shown as verified only when this clone verified
-- them itself. The note file keeps the state it was written with elsewhere.
-- Keyed by the body's hash, so an edited record has to be verified again.
-- Not derivable from the files, so `rebuild` carries these rows over.
CREATE TABLE local_verified (
    note_id   TEXT PRIMARY KEY,
    body_hash TEXT NOT NULL,
    reason    TEXT,
    t         INTEGER
) WITHOUT ROWID;
