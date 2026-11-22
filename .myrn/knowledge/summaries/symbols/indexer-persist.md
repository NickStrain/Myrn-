---
id: indexer-persist
type: summary
status: unverified
title: indexer::persist
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::persist
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::persist@782b0dc139dd249f
---
# indexer::persist

Writes the file node, its FTS entry, and its module chain up to the root directory. When parsed is Some, it also writes symbol and test nodes with defined_in edges, refs, and imports. Always writes the files row, with an error message if parsing failed or the parse had syntax errors. Does not delete existing rows, so the caller must call delete_file first.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:208-345`
- **Signature:** `fn persist( conn: &Connection, st: &FileStat, hash: &str, parsed: Option<&Parsed>, error: Option<&str>, ) -> Result<()>`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
- **Calls:** [[kind-as-str]], [[lang-name]], [[id-file-id]], [[id-module-id]], [[id-symbol-id]], [[id-test-id]], [[id-parent-dir]], [[fts-index-node]], [[graph-upsert-node]], [[graph-insert-edge]]
- **Called by:** [[indexer-refresh]]
