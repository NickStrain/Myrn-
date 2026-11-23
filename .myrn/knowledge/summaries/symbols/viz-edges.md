---
id: viz-edges
type: summary
status: unverified
title: viz::edges
source: myrn summarize
about: sym:crates/myrn-engine/src/viz.rs#viz::edges
anchors:
- sym:crates/myrn-engine/src/viz.rs#viz::edges@1d5a0b334b29fff9
map_version: '2'
---
# viz::edges

Runs a query and collects the first two columns of each row as string pairs.

## Behaviour

- query selecting two columns -> list of (column 0, column 1) pairs
- any row error makes the whole call fail
- rows come back in query order, with no deduplication

## Terms

two-column query, pair list, edge pairs, sql helper, string tuples

## Connections

- **Location:** `crates/myrn-engine/src/viz.rs:698-702`
- **Signature:** `fn edges(c: &rusqlite::Connection, sql: &str) -> Result<Vec<(String, String)>>`
- **In:** [[file-crates-myrn-engine-src-viz-rs]]
- **Called by:** [[engine-viz-code-graph]], [[engine-viz-map]]
