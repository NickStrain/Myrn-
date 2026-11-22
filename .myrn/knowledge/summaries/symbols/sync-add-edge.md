---
id: sync-add-edge
type: summary
status: unverified
title: sync::add_edge
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::add_edge
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::add_edge@db18db80a9d9ac62
---
# sync::add_edge

Inserts a resolved edge src -rel-> dst with evidence 'note:<note_id>', ignoring duplicates.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:308-315`
- **Signature:** `fn add_edge(tx: &Transaction<'_>, src: &str, rel: &str, dst: &str, note_id: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[sync-link]]
