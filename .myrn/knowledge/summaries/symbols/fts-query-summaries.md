---
id: fts-query-summaries
type: summary
status: unverified
title: fts::query_summaries
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::query_summaries
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::query_summaries@639fc246a4fd55a4
map_version: '2'
---
# fts::query_summaries

Runs a prefix-matching search over `summary_search` and returns up to limit node ids, best first. bm25 weights are id 0.0, summary 1.0, behaviour 1.5, terms 3.0, so matches in the terms column rank highest.

## Behaviour

- terms [lookup] -> MATCH `"lookup"*` over summary_search
- terms empty or all empty strings -> empty vec, no SQL runs
- bm25 is sorted ascending because lower is better, so this ordering is the reverse of the score sign used in query

## Terms

summary search, code map search, find notes, note search, ranked summaries, behaviour search, summary ranking, map lookup

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:91-108`
- **Signature:** `pub fn query_summaries(conn: &Connection, terms: &[String], limit: usize) -> Result<Vec<String>>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[seed-seeds]]
