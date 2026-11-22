---
id: symbols-common-prefix
type: summary
status: unverified
title: symbols::common_prefix
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::common_prefix
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::common_prefix@8e8b846487355d58
---
# symbols::common_prefix

Counts the leading characters two strings share; used to choose the closest of several same-file candidates.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:886-888`
- **Signature:** `fn common_prefix(a: &str, b: &str) -> usize`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Called by:** [[resolver-resolve-bare]]
