---
id: seed-seeds
type: summary
status: unverified
title: seed::seeds
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::seeds
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::seeds@b697555970c40320
---
# seed::seeds

Builds scored `Seed`s for a free-text task: runs `find` on code tokens and backticked identifiers (skipping queries with 0 or more than 12 hits, scoring 10/sqrt(hits) for up to 5 each), then does FTS over task words plus summaries, keeping nodes (not modules) whose name/path/title/doc/summary covers at least half the words (or any single word match when the task has one word), scored 6×relevance.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:191-268`
- **Signature:** `pub fn seeds(db: &Db, task: &str) -> Result<Vec<Seed>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-find]], [[seed-task-words]], [[seed-code-tokens]], [[seed-backticked]], [[fts-split-identifier]], [[fts-query]], [[fts-query-summaries]], [[graph-node-graph]]
- **Called by:** [[engine-context-excluding]]
