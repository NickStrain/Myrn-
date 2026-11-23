---
id: symbols-resolve-all
type: summary
status: unverified
title: symbols::resolve_all
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::resolve_all
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::resolve_all@5bfda718dd3963ac
map_version: '2'
---
# symbols::resolve_all

Deletes existing calls, imports and tests edges, then rebuilds them from the stored refs and symbols, inserting with OR IGNORE and returning counts.

## Behaviour

- refs of kind 'import' -> import edges when the target differs from the source file, deduplicated
- ref whose source id is not a known symbol -> skipped
- test source symbol -> also writes a tests edge for each target

## Terms

rebuild edges, refresh edges, edges table, sqlite, edge rebuild, resolution pass

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:943-1045`
- **Signature:** `pub fn resolve_all(conn: &Connection, root: &Path) -> Result<EdgeCounts>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-parse-name]], [[resolver-new]], [[resolver-sym]], [[resolver-add-import]], [[resolver-resolve]], [[resolver-index-of]], [[symbols-rust-crates]], [[id-file-id]], [[status-as-str]]
- **Called by:** [[engine-record]], [[indexer-refresh]]
