---
id: symbols-normalize
type: summary
status: unverified
title: symbols::normalize
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::normalize
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::normalize@f4d63a2dbb79f47f
map_version: '2'
---
# symbols::normalize

Collapses '.' and '..' segments and empty segments in a forward-slash path.

## Behaviour

- 'src/./a/../b.ts' -> 'src/b.ts'
- 'a//b' -> 'a/b'
- '../x' -> 'x' because '..' at the start has nothing to pop

## Terms

path normalization, dot segments, relative path, canonical path, path cleanup

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:891-903`
- **Signature:** `fn normalize(p: &str) -> String`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-js-import]]
