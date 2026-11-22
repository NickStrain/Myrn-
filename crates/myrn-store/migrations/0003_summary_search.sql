-- Full-text search over code summaries (from summary notes), so a task can
-- match code by what it does, not only by its name. Rebuilt on every re-link.
CREATE VIRTUAL TABLE summary_search USING fts5(
    id UNINDEXED,
    text,
    tokenize = 'unicode61'
);
-- Re-parse every note so the next sync re-links and fills the new table.
DELETE FROM notes;
