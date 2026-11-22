---
id: symbols-last-seg
type: summary
status: unverified
title: symbols::last_seg
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::last_seg
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::last_seg@381bd4f7d1618beb
---
# symbols::last_seg

Returns the final segment of a qualified name split on the given separator.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:882-884`
- **Signature:** `fn last_seg<'s>(q: &'s str, sep: &str) -> &'s str`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]], [[resolver-resolve-path]], [[resolver-resolve-method]]
