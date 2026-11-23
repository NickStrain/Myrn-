-- Code-map notes now also carry `## Behaviour` and `## Terms`: what the code
-- does on concrete inputs, and the words a bug report would use for it. They
-- are searched in their own columns so terms can weigh more than prose.
-- The table is rebuilt from the notes on the next re-link, which this forces.
DROP TABLE IF EXISTS summary_search;
CREATE VIRTUAL TABLE summary_search USING fts5(
    id UNINDEXED,
    text,
    behaviour,
    terms,
    tokenize = 'unicode61'
);
DELETE FROM meta WHERE key = 'notes_linked';
