-- Files a commit added, or added a new definition to (a function, class or
-- type, whole within an inserted hunk). It tells "how code gets added here":
-- which other files such commits changed together.
CREATE TABLE commit_adds (
    sha  TEXT NOT NULL,
    path TEXT NOT NULL,
    PRIMARY KEY (sha, path)
) WITHOUT ROWID;
CREATE INDEX commit_adds_path ON commit_adds(path);
