---
id: fts-query-summaries
type: summary
status: unverified
title: fts::query_summaries
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::query_summaries
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::query_summaries@225a28a47b8c2d28
---
# fts::query_summaries

Runs the same prefix-OR term building against summary_search and returns up to limit node ids ordered by bm25 ascending, which is best first in SQLite. Returns ids only, with no scores, and an empty vec if no usable terms remain.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:90-105`
- **Signature:** `pub fn query_summaries(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<String>>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[seed-seeds]]
