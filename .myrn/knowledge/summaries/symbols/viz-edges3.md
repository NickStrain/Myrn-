---
id: viz-edges3
type: summary
status: unverified
title: viz::edges3
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::edges3
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::edges3@070555cfbfabbc04
map_version: '2'
---
# viz::edges3

Runs a query and collects the first three columns of each row as string triples, used for source, relation and target.

## Behaviour

- query selecting src, rel, dst -> list of (src, rel, dst) triples
- any row error makes the whole call fail
- rows come back in query order, with no deduplication

## Terms

triple list, three-column query, relation triples, sql helper

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:704-708`
- **Signature:** `fn edges3(c: &rusqlite::Connection, sql: &str) -> Result<Vec<(String, String, String)>>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Called by:** [[engine-viz-notes-graph]]
