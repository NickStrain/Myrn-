---
id: ids
type: summary
status: unverified
title: ids
source: myrn summarize
about: sym:crates/myrn-engine/tests/gate0.rs#ids
anchors:
- sym:crates/myrn-engine/tests/gate0.rs#ids@4fa74998397b2a48
map_version: '2'
---
# ids

Collects the id field of each link into a BTreeSet of strings, dropping duplicates and sorting.

## Behaviour

- links with ids x, x, y -> {x, y}
- empty iterator -> empty set
- link status and other fields are ignored

## Terms

link ids, collect ids, id set, sorted ids

## Connections

- **Location:** `crates/myrn-engine/tests/gate0.rs:89-93`
- **Signature:** `fn ids<'a>( links: impl IntoIterator<Item = &'a myrn_engine::pack::cards::Link>, ) -> BTreeSet<String>`
- **In:** [[file-crates-myrn-engine-tests-gate0-rs]]
- **Called by:** [[engine-map-body]], [[check]]
- **Tests:** `gate0_rust`, `gate0_python`, `gate0_typescript`, `incremental_refresh_tracks_edits_and_deletes`
