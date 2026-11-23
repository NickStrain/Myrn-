---
id: seed-seeds
type: summary
status: unverified
title: seed::seeds
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::seeds
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::seeds@ab72885890c3ac57
map_version: '2'
---
# seed::seeds

Builds scored starting nodes for a free-text task from two sources: find on code tokens and backticked identifiers, and full-text word search over names, docs, summaries and behaviour notes. Output is not deduplicated.

## Behaviour

- a token whose find returns more than 12 hits is skipped; otherwise its first 5 hits are kept with score 10 divided by the square root of the hit count
- word-search nodes need relevance of at least 0.5 over up to 4 words, unless the task has one word matched once; module nodes are skipped
- for bug reports, a bonus from summary-search order (first hit gets the full 4.0, later ones less) is added

## Terms

seed list, starting points, task search, relevance, ranking, candidate nodes, summary search

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:209-309`
- **Signature:** `pub fn seeds(db: &Db, task: &str) -> Result<Vec<Seed>>`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[seed-find]], [[seed-task-words]], [[seed-code-tokens]], [[seed-backticked]], [[seed-asks-about-bug]], [[fts-split-identifier]], [[fts-query]], [[fts-query-summaries]], [[fts-summary-details]], [[graph-node-graph]]
- **Called by:** [[engine-context-excluding]]
