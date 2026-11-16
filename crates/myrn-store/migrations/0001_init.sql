-- Initial schema. See design doc, "Code graph and index".
-- Everything here is rebuildable from the code and the notes.

CREATE TABLE meta (
    key   TEXT PRIMARY KEY,
    value TEXT
);

-- Indexing state per source file, used to re-parse only what changed.
CREATE TABLE files (
    path         TEXT PRIMARY KEY,
    lang         TEXT,
    content_hash TEXT NOT NULL,
    size         INTEGER NOT NULL,
    mtime        INTEGER NOT NULL,
    symbols      INTEGER NOT NULL DEFAULT 0,
    error        TEXT
);

CREATE TABLE nodes (
    id          TEXT PRIMARY KEY,
    type        TEXT NOT NULL,
    origin      TEXT NOT NULL,
    kind        TEXT,          -- function, method, class, struct, ...
    title       TEXT,          -- short name
    qname       TEXT,          -- qualified name
    summary     TEXT,          -- written by an agent (later phases)
    doc         TEXT,          -- doc comment or docstring from the code
    signature   TEXT,
    path        TEXT,
    lang        TEXT,
    start_line  INTEGER,
    end_line    INTEGER,
    body_hash   TEXT,
    status      TEXT,
    confidence  REAL,
    extra       TEXT           -- JSON
);
CREATE INDEX nodes_path  ON nodes(path);
CREATE INDEX nodes_type  ON nodes(type);
CREATE INDEX nodes_title ON nodes(title);
CREATE INDEX nodes_qname ON nodes(qname);

CREATE TABLE edges (
    src      TEXT NOT NULL,
    rel      TEXT NOT NULL,
    dst      TEXT NOT NULL,
    status   TEXT,             -- resolved | inferred (derived edges)
    evidence TEXT,
    PRIMARY KEY (src, rel, dst)
) WITHOUT ROWID;
CREATE INDEX edges_dst ON edges(dst, rel);

-- Raw references found by the parser, resolved into calls/imports/tests edges
-- after every refresh so edges into changed files stay correct.
CREATE TABLE refs (
    src       TEXT NOT NULL,   -- enclosing symbol/test id, or file id
    path      TEXT NOT NULL,
    kind      TEXT NOT NULL,   -- call | import
    name      TEXT NOT NULL,
    qualifier TEXT,            -- receiver/path for calls, local alias for imports
    scope     TEXT,            -- Rust module path at the reference site
    line      INTEGER
);
CREATE INDEX refs_path ON refs(path);

-- Files changed by each indexed commit.
CREATE TABLE commit_files (
    sha    TEXT NOT NULL,
    path   TEXT NOT NULL,
    change TEXT NOT NULL,      -- added | modified | deleted | renamed
    old_path TEXT,
    PRIMARY KEY (sha, path)
) WITHOUT ROWID;
CREATE INDEX commit_files_path ON commit_files(path);

CREATE TABLE anchors (note_id TEXT, symbol_id TEXT, body_hash TEXT);
CREATE TABLE notes   (id TEXT PRIMARY KEY, path TEXT, content_hash TEXT);
CREATE TABLE usage   (note_id TEXT, session_id TEXT,
                      injected INTEGER, used INTEGER, outcome TEXT);

-- Full-text search. rowid = nodes.rowid.
CREATE VIRTUAL TABLE search USING fts5(
    name, terms, path, doc,
    tokenize = 'unicode61'
);
