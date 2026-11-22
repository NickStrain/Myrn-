---
id: sync-segments
type: summary
status: unverified
title: sync::segments
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::segments
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::segments@3ddf7aea8186c394
---
# sync::segments

Splits a qualified name on '::' and '.' into non-empty segments.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:218-224`
- **Signature:** `fn segments(q: &str) -> Vec<String>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[resolver-resolve-sync]]
