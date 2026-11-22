---
id: indexreport-changed
type: summary
status: unverified
title: IndexReport::changed
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport::changed
anchors:
- sym:crates/myrn-engine/src/indexer/mod.rs#indexer::IndexReport::changed@1e4f6fb02dad8e1b
---
# IndexReport::changed

Returns true if any file was parsed or removed, or any commits were indexed. Note-only or metadata-only refreshes return false.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/mod.rs:43-45`
- **Signature:** `pub fn changed(&self) -> bool`
- **In:** [[file-crates-myrn-engine-src-indexer-mod-rs]]
