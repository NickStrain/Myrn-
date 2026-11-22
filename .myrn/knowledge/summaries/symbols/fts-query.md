---
id: fts-query
type: summary
status: unverified
title: fts::query
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::query
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::query@262e3ce03d18fff8
---
# fts::query

Builds an OR of quoted prefix terms from the non-empty inputs with double quotes stripped, then returns up to limit (node id, score) pairs ordered by score descending. Score is -bm25 with column weights 4.0, 2.0, 1.0, 0.5, so higher is better; returns an empty vec if no usable terms remain.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:69-87`
- **Signature:** `pub fn query(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<(String, f64)>>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[seed-seeds]]
