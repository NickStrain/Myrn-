---
id: fts-query
type: summary
status: unverified
title: fts::query
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::query
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::query@262e3ce03d18fff8
map_version: '2'
---
# fts::query

Runs a prefix-matching full-text search over `search`, ORs the terms together, and returns up to limit (node id, score) pairs with higher scores first. Score is the negated bm25 with weights 4.0 name, 2.0 terms, 1.0 path, 0.5 doc.

## Behaviour

- terms [token, lookup] -> MATCH `"token"* OR "lookup"*`
- terms empty or all empty strings -> empty vec, no SQL runs
- each double quote is removed first: `a"b` -> `"ab"*`; a term of only quotes becomes `""*`

## Terms

full-text search, text search, ranking, bm25, relevance, relevance score, find nodes, prefix match

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:70-88`
- **Signature:** `pub fn query(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<(String, f64)>>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[seed-seeds]]
