---
id: sync-link
type: summary
status: unverified
title: sync::link
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::link
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::link@75dc62fde3b96099
---
# sync::link

Rebuilds all note-derived rows from stored parses: earned non-episode nodes and their search rows, note-evidence edges, anchors, issues and summary search are deleted and re-inserted. Duplicate ids keep the shared copy, then path order. Summary notes create no node; their text is written to the summary field of their about targets, or anchor targets if there are none. Rows whose stored JSON no longer parses are deleted so the next scan re-parses them. It reads no files.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:318-508`
- **Signature:** `fn link(tx: &Transaction<'_>) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[relation-as-str]], [[fts-index-node]], [[graph-upsert-node]], [[note-summary-text]], [[note-display-title]], [[wikilink-extract]], [[resolver-note]], [[resolver-resolve-sync]], [[sync-add-edge]]
- **Called by:** [[sync-sync]]
