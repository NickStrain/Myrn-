---
id: graph-count
type: summary
status: unverified
title: graph::count
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::count
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::count@85efae4c02996fae
---
# graph::count

Runs the caller's SQL with no parameters and returns the first column of the first row as i64. The SQL is not parameterized, so it must be trusted and must yield an integer.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:194-196`
- **Signature:** `pub fn count(conn: &Connection, sql: &str) -> Result<i64>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-status]], [[indexer-refresh]], [[sync-summary]]
