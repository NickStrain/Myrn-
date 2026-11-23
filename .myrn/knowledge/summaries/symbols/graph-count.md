---
id: graph-count
type: summary
status: unverified
title: graph::count
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::count
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::count@85efae4c02996fae
map_version: '2'
---
# graph::count

Runs caller-supplied SQL with no parameters and returns the integer in the first column of the single result row.

## Behaviour

- 'SELECT COUNT(*) FROM nodes' -> row count as i64
- query returning no rows -> Err
- first column not an integer -> Err

## Terms

row count, total, tally, aggregate, number of rows, size

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:194-196`
- **Signature:** `pub fn count(conn: &Connection, sql: &str) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-status]], [[indexer-refresh]], [[sync-summary]]
- **Tests:** `forget_removes_the_record_and_what_only_it_used`
