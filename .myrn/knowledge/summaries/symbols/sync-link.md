---
id: sync-link
type: summary
status: unverified
title: sync::link
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::link
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::link@f2311709f4b4f34e
map_version: '2'
---
# sync::link

Clears every note-derived row, then rebuilds nodes, edges, anchors, summary text and issues from the stored note rows.

## Behaviour

- when two files share an id, the shared copy wins over private, then path order; the other gets a 'duplicate' issue and is not indexed
- a summary note writes its text onto the code nodes it is 'about' and creates no node; other notes create an earned node whose status is 'verified' for shared records only if local_verified matches the body hash, else 'unverified'
- a link or wikilink that resolves nothing adds a 'dangling' issue; then derive_states and shared_contradictions run

## Terms

rebuild links, relink, link pass, anchors, summaries, issue recording, node rebuild

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:328-545`
- **Signature:** `fn link(tx: &Transaction<'_>) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Calls:** [[status-as-str]], [[visibility-as-str]], [[relation-as-str]], [[fts-index-node]], [[graph-upsert-node]], [[note-extra-str]], [[note-summary-text]], [[note-summary-details]], [[note-display-title]], [[wikilink-extract]], [[sync-body-hash]], [[resolver-note]], [[resolver-resolve-sync]], [[sync-add-edge]], [[sync-shared-contradictions]], [[sync-derive-states]]
- **Called by:** [[sync-sync]]
