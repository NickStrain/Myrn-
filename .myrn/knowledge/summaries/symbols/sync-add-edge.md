---
id: sync-add-edge
type: summary
status: unverified
title: sync::add_edge
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::add_edge
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::add_edge@db18db80a9d9ac62
map_version: '2'
---
# sync::add_edge

Inserts a resolved edge from src to dst with the given relation, tagged with evidence 'note:<note id>'.

## Behaviour

- a duplicate of an existing (src, rel, dst) edge is ignored silently
- status is always 'resolved'
- evidence string is 'note:' followed by note_id

## Terms

insert edge, relation, evidence, graph link

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:318-325`
- **Signature:** `fn add_edge(tx: &Transaction<'_>, src: &str, rel: &str, dst: &str, note_id: &str) -> Result<()>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[sync-link]]
