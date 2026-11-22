---
id: seed-myrn-index-lang
type: summary
status: unverified
title: seed::myrn_index_lang
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::myrn_index_lang
anchors:
- sym:crates/myrn-engine/src/retrieve/seed.rs#retrieve::seed::myrn_index_lang@93dbf5d1ccb3fb08
---
# seed::myrn_index_lang

Returns true if the query's path extension maps to a language the indexer supports (via `indexer::lang::Lang::from_path`); used by `find` to decide whether a bare filename should be treated as a file lookup.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/seed.rs:95-97`
- **Signature:** `fn myrn_index_lang(q: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-retrieve-seed-rs]]
- **Calls:** [[lang-from-path]]
- **Called by:** [[seed-find]]
