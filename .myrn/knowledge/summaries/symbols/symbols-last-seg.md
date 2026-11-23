---
id: symbols-last-seg
type: summary
status: unverified
title: symbols::last_seg
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::last_seg
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::last_seg@381bd4f7d1618beb
map_version: '2'
---
# symbols::last_seg

Returns the part of a qualified name after its last separator.

## Behaviour

- 'a::b::c' with '::' -> 'c'
- 'f' with no separator -> 'f'
- empty string -> empty string

## Terms

last segment, final name part, tail name, short name, trailing component

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:882-884`
- **Signature:** `fn last_seg<'s>(q: &'s str, sep: &str) -> &'s str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
