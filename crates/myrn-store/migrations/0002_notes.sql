-- Note indexing. Rows are keyed by file, not id, so two files claiming one id
-- (a hand copy, or a private and a shared copy) are both seen and flagged.

DROP TABLE notes;
CREATE TABLE notes (
    path         TEXT PRIMARY KEY, -- `.myrn/knowledge/bugs/x.md` (shared) or `notes/bugs/x.md` (private)
    id           TEXT NOT NULL,
    type         TEXT NOT NULL,
    visibility   TEXT NOT NULL,    -- private | shared
    content_hash TEXT NOT NULL,
    size         INTEGER NOT NULL,
    mtime        INTEGER NOT NULL,
    data         TEXT NOT NULL,    -- the parsed note, JSON
    problems     TEXT              -- header problems, JSON array
);
CREATE INDEX notes_id ON notes(id);

-- Everything wrong with the notes, recomputed on every re-link:
-- kind = header | dangling | duplicate.
CREATE TABLE note_issues (
    path    TEXT NOT NULL,
    note_id TEXT NOT NULL,
    kind    TEXT NOT NULL,
    detail  TEXT NOT NULL
);
