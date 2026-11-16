-- Initial schema. See design doc, "Code graph and index".
CREATE TABLE nodes  (id TEXT PRIMARY KEY, type TEXT, origin TEXT,
                     title TEXT, summary TEXT, path TEXT, span TEXT,
                     body_hash TEXT, status TEXT, confidence REAL);
CREATE TABLE edges  (src TEXT, rel TEXT, dst TEXT, status TEXT,
                     evidence TEXT, PRIMARY KEY (src, rel, dst));
CREATE TABLE anchors(note_id TEXT, symbol_id TEXT, body_hash TEXT);
CREATE TABLE notes  (id TEXT PRIMARY KEY, path TEXT, content_hash TEXT);
CREATE TABLE usage  (note_id TEXT, session_id TEXT,
                     injected INTEGER, used INTEGER, outcome TEXT);
-- TODO: full-text table over title, summary and note bodies
-- TODO: vector table (only when the embedding helper is enabled)
