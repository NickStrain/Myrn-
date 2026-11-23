---
id: symbols-strip-suffix
type: summary
status: unverified
title: symbols::strip_suffix
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::strip_suffix
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::strip_suffix@c1246b537b506862
map_version: '2'
---
# symbols::strip_suffix

Removes a trailing '#' followed only by digits from a qualified name, leaving other '#' text intact.

## Behaviour

- 'a#1#2' -> 'a#1' (only the last '#' is checked)
- 'f#' -> 'f' (empty digit tail counts as all digits)
- 'f#2b' -> 'f#2b' unchanged

## Terms

collision suffix, hash stripping, numeric suffix, disambiguator, trim suffix

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:56-61`
- **Signature:** `fn strip_suffix(q: &str) -> &str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[sym-base-qname]], [[tests-sym]]
