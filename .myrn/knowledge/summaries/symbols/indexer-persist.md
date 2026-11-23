---
id: indexer-persist
type: summary
status: unverified
title: indexer::persist
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::persist
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::persist@782b0dc139dd249f
map_version: '2'
---
# indexer::persist

Writes a file node, its module chain and part_of edges, and when parsed its symbol and test nodes, defined_in edges, refs and imports, then upserts its files row with symbol count and error text.

## Behaviour

- parsed has_errors and no error given -> files.error set to 'syntax errors; indexed as far as possible'
- parsed None and error Some -> no symbols, files.error holds the message
- does not delete old rows; the caller must run delete_file first because refs and imports use plain INSERT

## Terms

write, store, upsert, save, insert rows, module chain, syntax error

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:208-345`
- **Signature:** `fn persist( conn: &Connection, st: &FileStat, hash: &str, parsed: Option<&Parsed>, error: Option<&str>, ) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[kind-as-str]], [[lang-name]], [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[id-parent-dir]], [[fts-index-node]], [[graph-upsert-node]], [[graph-insert-edge]]
- **Called by:** [[indexer-refresh]]
