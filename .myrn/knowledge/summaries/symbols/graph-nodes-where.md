---
id: graph-nodes-where
type: summary
status: unverified
title: graph::nodes_where
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::nodes_where
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::nodes_where@5216bae5c903f74b
---
# graph::nodes_where

Runs SELECT ... FROM nodes WHERE {clause} with the clause interpolated as raw SQL and args bound as parameters. Callers must never build the clause from untrusted input.

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:133-141`
- **Signature:** `pub fn nodes_where( conn: &Connection, clause: &str, args: &[&dyn rusqlite::ToSql], ) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-brief]], [[engine-commit-ref]], [[rank-rank]], [[engine-recall]], [[seed-find]], [[walks-members]], [[engine-summarize]], [[engine-map-body]], [[resolver-resolve-sync]]
