---
id: indexreport-changed
type: summary
status: unverified
title: IndexReport::changed
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport::changed
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport::changed@1e4f6fb02dad8e1b
map_version: '2'
---
# IndexReport::changed

Returns true when this refresh parsed files, removed files, or indexed commits.

## Behaviour

- parsed, removed and commits all 0 -> false
- only mtime or size updates from touched files -> false
- errors, notes or symbol count alone -> not counted

## Terms

dirty, modified, has updates, any change, index changed, needs save

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:43-45`
- **Signature:** `pub fn changed(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
