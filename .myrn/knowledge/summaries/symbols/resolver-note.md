---
id: resolver-note
type: summary
status: unverified
title: Resolver::note
source: myrn summarize
about: sym:crates/myrn-store/src/sync.rs#sync::Resolver::note
anchors:
- sym:crates/myrn-store/src/sync.rs#sync::Resolver::note@7af51700cdab19b6
---
# Resolver::note

Looks up a note id case-insensitively and returns its canonical id, or None.

## Connections

- **Location:** `crates/myrn-store/src/sync.rs:231-233`
- **Signature:** `fn note(&self, t: &str) -> Option<String>`
- **In:** [[file-crates-myrn-store-src-sync-rs]]
- **Called by:** [[resolver-resolve-sync]], [[sync-link]]
