---
id: graph-nodes-where
type: summary
status: unverified
title: graph::nodes_where
source: myrn summarize
about: sym:crates/myrn-store/src/graph.rs#graph::nodes_where
anchors:
- sym:crates/myrn-store/src/graph.rs#graph::nodes_where@5216bae5c903f74b
map_version: '2'
---
# graph::nodes_where

Selects all nodes matching a caller-supplied SQL WHERE clause, binding args as parameters.

## Behaviour

- clause 'type = ?1' with one arg -> matching nodes in table order
- no match -> empty Vec
- clause text is spliced into the SQL unescaped; only args are bound

## Terms

filter, query nodes, select nodes, search nodes, where clause, predicate

## Connections

- **Location:** `crates/myrn-store/src/graph.rs:133-141`
- **Signature:** `pub fn nodes_where( conn: &Connection, clause: &str, args: &[&dyn rusqlite::ToSql], ) -> Result<Vec<Node>>`
- **In:** [[file-crates-myrn-store-src-graph-rs]]
- **Called by:** [[engine-brief]], [[engine-commit-ref]], [[rank-rank]], [[engine-recall]], [[seed-find]], [[walks-members]], [[engine-summarize]], [[engine-map-body]], [[engine-promote]], [[engine-viz-code-graph]], [[engine-viz-map]], [[resolver-resolve-sync]]
