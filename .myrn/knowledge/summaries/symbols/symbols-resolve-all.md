---
id: symbols-resolve-all
type: summary
status: unverified
title: symbols::resolve_all
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::resolve_all
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::resolve_all@5bfda718dd3963ac
---
# symbols::resolve_all

Deletes existing calls, imports and tests edges, loads files, symbols and refs from the database, then inserts import edges followed by resolved call edges (plus tests edges from test symbols). It uses INSERT OR IGNORE, so only newly inserted rows are counted.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:943-1045`
- **Signature:** `pub fn resolve_all(conn: &Connection, root: &Path) -> Result<EdgeCounts>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[lang-parse-name]], [[resolver-new]], [[resolver-sym]], [[resolver-add-import]], [[resolver-resolve]], [[resolver-index-of]], [[symbols-rust-crates]], [[id-file-id]], [[status-as-str]]
- **Called by:** [[engine-record]], [[indexer-refresh]]
