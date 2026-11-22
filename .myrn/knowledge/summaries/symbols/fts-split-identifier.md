---
id: fts-split-identifier
type: summary
status: unverified
title: fts::split_identifier
source: myrn summarize
about: sym:crates/myrn-store/src/fts.rs#fts::split_identifier
anchors:
- sym:crates/myrn-store/src/fts.rs#fts::split_identifier@a7eede889fdadc50
---
# fts::split_identifier

Splits a string into lowercase words on non-alphanumeric characters and at camel-case boundaries, so TokenStore::lookup_by_id yields token, store, lookup, by, id and HTTPServer yields http, server. Pure function with no I/O.

## Connections

- **Location:** `crates/myrn-store/src/fts.rs:8-33`
- **Signature:** `pub fn split_identifier(s: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-fts-rs]]
- **Called by:** [[resolver-resolve-method]], [[seed-task-words]], [[seed-seeds]], [[fts-index-node]]
- **Tests:** `splits_identifiers`
